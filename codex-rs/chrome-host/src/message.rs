use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeMessage {
    pub version: String,
    pub id: String,
    pub r#type: String,
    pub origin: Option<serde_json::Value>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeResponse {
    pub version: String,
    pub id: String,
    pub r#type: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl NativeResponse {
    pub fn success(id: String, message_type: String, data: serde_json::Value) -> Self {
        Self {
            version: "1.0".to_string(),
            id,
            r#type: message_type,
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(id: String, message_type: String, error: String) -> Self {
        Self {
            version: "1.0".to_string(),
            id,
            r#type: message_type,
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

pub fn read_message() -> Result<NativeMessage> {
    let mut length_bytes = [0u8; 4];
    io::stdin()
        .read_exact(&mut length_bytes)
        .context("Failed to read message length")?;

    let length = u32::from_le_bytes(length_bytes) as usize;
    if length == 0 || length > 1024 * 1024 {
        anyhow::bail!("Invalid message length: {}", length);
    }

    let mut buffer = vec![0u8; length];
    io::stdin()
        .read_exact(&mut buffer)
        .context("Failed to read message body")?;

    let message: NativeMessage =
        serde_json::from_slice(&buffer).context("Failed to parse message JSON")?;

    Ok(message)
}

pub fn write_response(response: &NativeResponse) -> Result<()> {
    let json = serde_json::to_vec(response).context("Failed to serialize response")?;
    let length = json.len() as u32;

    io::stdout()
        .write_all(&length.to_le_bytes())
        .context("Failed to write response length")?;
    io::stdout()
        .write_all(&json)
        .context("Failed to write response body")?;
    io::stdout().flush().context("Failed to flush stdout")?;

    Ok(())
}
