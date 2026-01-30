use serde::{Deserialize, Serialize};

/// LINE message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineMessageType {
    Text,
    Image,
    File,
    Location,
    Sticker,
}

/// LINE message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineMessage {
    pub message_id: String,
    pub message_type: LineMessageType,
    pub text: Option<String>,
    pub sender_id: String,
    pub sender_name: String,
    pub timestamp: i64,
    pub reply_token: Option<String>,
}

/// Development command from LINE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentCommand {
    ExecuteCode { code: String, language: String },
    CreateFile { path: String, content: String },
    ReadFile { path: String },
    RunTests,
    DeployApp,
    StatusCheck,
    CustomCommand { command: String, args: Vec<String> },
}

/// LINE API response
#[derive(Debug, Deserialize)]
pub struct LineApiResponse {
    #[serde(rename = "message")]
    pub message: Option<String>,
}

/// LINE Communicator configuration
#[derive(Debug, Clone)]
pub struct LineConfig {
    pub channel_access_token: String,
    pub channel_secret: String,
    pub webhook_url: String,
}

/// Development session for each user
#[derive(Debug, Clone)]
pub struct DevelopmentSession {
    pub user_id: String,
    pub user_name: String,
    pub current_project: Option<String>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub permissions: Vec<String>,
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub execution_time_ms: u64,
    pub error: Option<String>,
}
