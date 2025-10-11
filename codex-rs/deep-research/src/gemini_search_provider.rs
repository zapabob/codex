// Gemini CLI Search Provider - Google Search via Gemini API
// Integrates Gemini CLI to use Google Search with Grounding
use crate::provider::ResearchProvider;
use crate::types::Source;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::process::Command;
use tracing::debug;
use tracing::info;

/// Gemini CLI search provider that uses Google Search via Gemini API Grounding
/// Requires: gemini CLI installed and GOOGLE_API_KEY environment variable
pub struct GeminiSearchProvider {
    api_key: String,
    model: String,
    max_retries: u8,
}

impl Default for GeminiSearchProvider {
    fn default() -> Self {
        Self {
            api_key: std::env::var("GOOGLE_API_KEY").unwrap_or_default(),
            model: "gemini-1.5-pro".to_string(),
            max_retries: 3,
        }
    }
}

impl GeminiSearchProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gemini-1.5-pro".to_string()),
            max_retries: 3,
        }
    }

    /// Execute search using Gemini CLI with Google Search grounding
    async fn execute_gemini_search(&self, query: &str) -> Result<Vec<GeminiSearchResult>> {
        info!("🔍 Executing Gemini CLI search for: {}", query);

        // Check if gemini CLI is installed
        self.check_gemini_cli_installed()?;

        // Construct Gemini CLI command with grounding (Google Search)
        // Example: gemini "Search for: <query>" --api-key $GOOGLE_API_KEY --model gemini-1.5-pro --grounding
        let output = Command::new("gemini")
            .arg(format!("Search for: {}", query))
            .arg("--api-key")
            .arg(&self.api_key)
            .arg("--model")
            .arg(&self.model)
            .arg("--grounding")
            .arg("--json") // Request JSON output if available
            .output()
            .context("Failed to execute gemini CLI command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Gemini CLI failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        debug!("Gemini CLI output: {}", stdout);

        // Parse JSON response from Gemini CLI
        self.parse_gemini_response(&stdout)
    }

    /// Check if gemini CLI is installed
    fn check_gemini_cli_installed(&self) -> Result<()> {
        let output = Command::new("gemini")
            .arg("--version")
            .output()
            .context("gemini CLI not found. Please install it from: https://github.com/google/generative-ai-go")?;

        if !output.status.success() {
            anyhow::bail!("gemini CLI is not properly installed");
        }

        Ok(())
    }

    /// Parse Gemini CLI JSON response
    fn parse_gemini_response(&self, json_str: &str) -> Result<Vec<GeminiSearchResult>> {
        // Try to parse JSON response
        if let Ok(response) = serde_json::from_str::<GeminiApiResponse>(json_str) {
            return Ok(response
                .candidates
                .into_iter()
                .flat_map(|c| c.search_results)
                .collect());
        }

        // Fallback: Parse text response manually
        info!("⚠️  JSON parsing failed, using text fallback");
        Ok(self.parse_text_response(json_str))
    }

    /// Parse text response as fallback
    fn parse_text_response(&self, text: &str) -> Vec<GeminiSearchResult> {
        // Simple text parsing to extract URLs and titles
        let mut results = Vec::new();

        // Look for patterns like "Title: ...", "URL: ...", or markdown links [title](url)
        let re_markdown = regex::Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();

        for cap in re_markdown.captures_iter(text) {
            let title = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let url = cap.get(2).map_or("", |m| m.as_str()).to_string();

            results.push(GeminiSearchResult {
                title: title.clone(),
                url: url.clone(),
                snippet: format!("Result from Gemini search: {}", title),
            });
        }

        // If no markdown links found, try to extract plain URLs
        if results.is_empty() {
            let re_url = regex::Regex::new(r"https?://[^\s]+").unwrap();
            for (i, url) in re_url.find_iter(text).enumerate() {
                results.push(GeminiSearchResult {
                    title: format!("Search Result {}", i + 1),
                    url: url.as_str().to_string(),
                    snippet: "Result from Gemini search".to_string(),
                });
            }
        }

        results
    }

    /// Execute search with retry logic
    async fn search_with_retry(
        &self,
        query: &str,
        max_results: usize,
    ) -> Result<Vec<GeminiSearchResult>> {
        let mut last_error = None;

        for attempt in 0..self.max_retries {
            match self.execute_gemini_search(query).await {
                Ok(results) => {
                    info!("✅ Gemini search succeeded on attempt {}", attempt + 1);
                    return Ok(results.into_iter().take(max_results).collect());
                }
                Err(e) => {
                    tracing::warn!("Gemini search attempt {} failed: {}", attempt + 1, e);
                    last_error = Some(e);

                    // Wait before retry
                    if attempt < self.max_retries - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retry attempts failed")))
    }
}

#[async_trait]
impl ResearchProvider for GeminiSearchProvider {
    async fn search(&self, query: &str, max_results: u8) -> Result<Vec<Source>> {
        let gemini_results = self.search_with_retry(query, max_results as usize).await?;

        let sources: Vec<Source> = gemini_results
            .into_iter()
            .map(|result| Source {
                url: result.url,
                title: result.title,
                snippet: result.snippet,
                relevance_score: 0.90, // Gemini-grounded results are high quality
            })
            .collect();

        info!(
            "✅ Found {} sources via Gemini CLI for: {}",
            sources.len(),
            query
        );

        Ok(sources)
    }

    async fn retrieve(&self, url: &str) -> Result<String> {
        // Use Gemini to summarize content from URL
        info!("📥 Retrieving content from {} via Gemini", url);

        let output = Command::new("gemini")
            .arg(format!("Summarize the content from this URL: {}", url))
            .arg("--api-key")
            .arg(&self.api_key)
            .arg("--model")
            .arg(&self.model)
            .output()
            .context("Failed to execute gemini CLI for content retrieval")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Gemini CLI content retrieval failed: {}", stderr);
        }

        let content = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(content)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiApiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiCandidate {
    #[serde(rename = "searchResults")]
    search_results: Vec<GeminiSearchResult>,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiSearchResult {
    title: String,
    url: String,
    snippet: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires gemini CLI and API key
    async fn test_gemini_search() {
        let provider = GeminiSearchProvider::default();
        let sources = provider.search("Rust async programming", 3).await.unwrap();

        assert!(!sources.is_empty());
        assert!(sources[0].relevance_score > 0.8);
    }

    #[test]
    fn test_parse_text_response() {
        let provider = GeminiSearchProvider::default();
        let text = r#"
        Here are some results:
        [Rust Async Book](https://rust-lang.github.io/async-book/)
        [Tokio Documentation](https://tokio.rs)
        "#;

        let results = provider.parse_text_response(text);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Rust Async Book");
        assert_eq!(results[0].url, "https://rust-lang.github.io/async-book/");
    }
}
