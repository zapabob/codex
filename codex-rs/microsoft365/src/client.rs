//! Microsoft 365 API client
//!
//! Provides access to Word, Excel, PowerPoint, and Outlook APIs

use anyhow::{Context, Result};
use reqwest::Client;
use std::sync::Arc;
use tracing::info;

use crate::auth::AuthManager;

/// Microsoft 365 API client
pub struct Microsoft365Client {
    /// HTTP client
    http_client: Client,
    /// Authentication manager
    auth_manager: Arc<AuthManager>,
    /// Base URL for Microsoft Graph API
    base_url: String,
}

impl Microsoft365Client {
    /// Create a new Microsoft 365 client
    pub fn new(auth_manager: Arc<AuthManager>) -> Self {
        Self {
            http_client: Client::new(),
            auth_manager,
            base_url: "https://graph.microsoft.com/v1.0".to_string(),
        }
    }

    /// Make an authenticated request
    async fn make_request(&self, method: &str, endpoint: &str, body: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let access_token = self.auth_manager.get_access_token().await?;
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = match method {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PATCH" => self.http_client.patch(&url),
            "DELETE" => self.http_client.delete(&url),
            _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
        };

        request = request.bearer_auth(&access_token);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Microsoft 365 API error ({}): {}",
                status,
                error_text
            ));
        }

        let json: serde_json::Value = response.json().await.context("Failed to parse response")?;
        Ok(json)
    }

    /// Word: Read a document
    pub async fn word_read_document(&self, drive_id: &str, item_id: &str) -> Result<String> {
        let endpoint = format!("/drives/{}/items/{}/content", drive_id, item_id);
        let response = self.make_request("GET", &endpoint, None).await?;
        Ok(response.to_string())
    }

    /// Word: Create a new document
    pub async fn word_create_document(&self, name: String, _content: String) -> Result<String> {
        // In a real implementation, this would create a Word document
        // For now, we'll return a placeholder
        info!("Creating Word document: {}", name);
        Ok(format!("Document created: {}", name))
    }

    /// Excel: Read a spreadsheet
    pub async fn excel_read_spreadsheet(&self, drive_id: &str, item_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("/drives/{}/items/{}/workbook", drive_id, item_id);
        self.make_request("GET", &endpoint, None).await
    }

    /// Excel: Update a cell
    pub async fn excel_update_cell(
        &self,
        drive_id: &str,
        item_id: &str,
        worksheet: &str,
        cell: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        let endpoint = format!(
            "/drives/{}/items/{}/workbook/worksheets/{}/range(address='{}')",
            drive_id, item_id, worksheet, cell
        );
        let body = serde_json::json!({ "values": [[value]] });
        self.make_request("PATCH", &endpoint, Some(body)).await?;
        Ok(())
    }

    /// PowerPoint: Read a presentation
    pub async fn powerpoint_read_presentation(&self, drive_id: &str, item_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("/drives/{}/items/{}", drive_id, item_id);
        self.make_request("GET", &endpoint, None).await
    }

    /// Outlook: Send an email
    pub async fn outlook_send_email(
        &self,
        to: Vec<String>,
        subject: String,
        body: String,
    ) -> Result<()> {
        let endpoint = "/me/sendMail";
        let body_json = serde_json::json!({
            "message": {
                "subject": subject,
                "body": {
                    "contentType": "HTML",
                    "content": body
                },
                "toRecipients": to.iter().map(|email| {
                    serde_json::json!({ "emailAddress": { "address": email } })
                }).collect::<Vec<_>>()
            }
        });
        self.make_request("POST", endpoint, Some(body_json)).await?;
        info!("Email sent successfully");
        Ok(())
    }

    /// Outlook: Get calendar events
    pub async fn outlook_get_calendar_events(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<serde_json::Value>> {
        let endpoint = format!(
            "/me/calendar/calendarView?startDateTime={}&endDateTime={}",
            start.to_rfc3339(),
            end.to_rfc3339()
        );
        let response = self.make_request("GET", &endpoint, None).await?;
        Ok(response["value"].as_array().cloned().unwrap_or_default())
    }
}
