use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Deserialize)]
pub struct NativeMessage {
    pub version: String,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub origin: Option<serde_json::Value>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct NativeResponse {
    pub version: String,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl NativeResponse {
    pub fn success(id: String, response_type: String, data: serde_json::Value) -> Self {
        Self {
            version: "1.0".to_string(),
            id,
            r#type: response_type,
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(id: String, response_type: String, error: String) -> Self {
        Self {
            version: "1.0".to_string(),
            id,
            r#type: response_type,
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Read a message from stdin following Native Messaging API protocol.
/// Messages are prefixed with a 4-byte length (little-endian).
pub fn read_message() -> Result<NativeMessage> {
    let mut len_bytes = [0u8; 4];
    io::stdin()
        .read_exact(&mut len_bytes)
        .context("Failed to read message length")?;

    let len = u32::from_le_bytes(len_bytes) as usize;
    if len == 0 {
        anyhow::bail!("Message length is zero");
    }
    if len > 1024 * 1024 {
        anyhow::bail!("Message too large: {} bytes", len);
    }

    let mut buffer = vec![0u8; len];
    io::stdin()
        .read_exact(&mut buffer)
        .context("Failed to read message body")?;

    let json_str = String::from_utf8(buffer).context("Invalid UTF-8 in message")?;
    let message: NativeMessage = serde_json::from_str(&json_str)
        .context("Failed to parse message JSON")?;

    Ok(message)
}

/// Write a response to stdout following Native Messaging API protocol.
/// Messages are prefixed with a 4-byte length (little-endian).
pub fn write_response(response: &NativeResponse) -> Result<()> {
    let json = serde_json::to_string(response).context("Failed to serialize response")?;
    let bytes = json.as_bytes();
    let len = bytes.len() as u32;

    let mut stdout = io::stdout();
    stdout
        .write_all(&len.to_le_bytes())
        .context("Failed to write response length")?;
    stdout
        .write_all(bytes)
        .context("Failed to write response body")?;
    stdout.flush().context("Failed to flush stdout")?;

    Ok(())
}
