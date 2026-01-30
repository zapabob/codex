use chrono::{DateTime, Utc};
use serde::Serialize;

// MCP Connection structures
#[derive(Serialize)]
pub struct MCPConnection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub status: String,
    pub url: Option<String>,
    pub last_connected: Option<DateTime<Utc>>,
    pub request_count: Option<u32>,
    pub avg_response_time: Option<f64>,
}

// System Metrics structures
#[derive(Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: Option<f64>,
    pub active_processes: u32,
    pub uptime: u64,
}

// Conversation structures
#[derive(Serialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub model: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub message_count: u32,
    pub summary: Option<String>,
}

// Message structures
#[derive(Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

// User structures
#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}
