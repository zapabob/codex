//! Protocol definitions for the agent orchestration communication layer.
//!
//! This module defines the versioned message envelope and RPC operations
//! for the orchestrator protocol.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Protocol version
pub const PROTOCOL_VERSION: &str = "1.0";

/// Message envelope for all protocol messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    /// Protocol version
    pub v: String,
    
    /// Unique message ID
    pub id: String,
    
    /// Timestamp (RFC3339)
    pub ts: DateTime<Utc>,
    
    /// Message type
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    
    /// Operation name
    pub op: String,
    
    /// Optional session ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    
    /// Optional agent info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentInfo>,
    
    /// Optional idempotency key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idem_key: Option<String>,
    
    /// Message body
    pub body: serde_json::Value,
}

impl Envelope {
    /// Create a new request envelope
    pub fn new_request(op: String, body: serde_json::Value) -> Self {
        Self {
            v: PROTOCOL_VERSION.to_string(),
            id: Uuid::new_v4().to_string(),
            ts: Utc::now(),
            msg_type: MessageType::Request,
            op,
            session: None,
            agent: None,
            idem_key: None,
            body,
        }
    }
    
    /// Create a response envelope for a request
    pub fn response_for(request: &Envelope, status: ResponseStatus, body: serde_json::Value) -> Self {
        Self {
            v: PROTOCOL_VERSION.to_string(),
            id: Uuid::new_v4().to_string(),
            ts: Utc::now(),
            msg_type: MessageType::Response,
            op: request.op.clone(),
            session: request.session.clone(),
            agent: None,
            idem_key: None,
            body: serde_json::json!({
                "status": status.status,
                "code": status.code,
                "message": status.message,
                "request_id": request.id,
                "data": body,
            }),
        }
    }
    
    /// Create an event envelope
    pub fn new_event(topic: String, body: serde_json::Value) -> Self {
        Self {
            v: PROTOCOL_VERSION.to_string(),
            id: Uuid::new_v4().to_string(),
            ts: Utc::now(),
            msg_type: MessageType::Event,
            op: topic,
            session: None,
            agent: None,
            idem_key: None,
            body,
        }
    }
}

/// Message type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Request,
    Response,
    Event,
}

/// Agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub role: String,
}

/// Response status
#[derive(Debug, Clone)]
pub struct ResponseStatus {
    pub status: String,
    pub code: Option<u16>,
    pub message: Option<String>,
}

impl ResponseStatus {
    pub fn ok() -> Self {
        Self {
            status: "ok".to_string(),
            code: None,
            message: None,
        }
    }
    
    pub fn error(code: u16, message: String) -> Self {
        Self {
            status: "error".to_string(),
            code: Some(code),
            message: Some(message),
        }
    }
    
    /// Conflict - preimage/base mismatch
    pub fn conflict(message: String) -> Self {
        Self::error(409, message)
    }
    
    /// Rate limit / backpressure
    pub fn rate_limit(retry_after: u64) -> Self {
        Self {
            status: "error".to_string(),
            code: Some(429),
            message: Some(format!("Rate limited, retry after {} seconds", retry_after)),
        }
    }
}

/// Lock operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum LockOp {
    Status,
    Acquire { owner: String, timeout_ms: Option<u64> },
    Release { owner: String },
}

/// Lock status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockStatus {
    pub locked: bool,
    pub owner: Option<String>,
    pub acquired_at: Option<DateTime<Utc>>,
}

/// File system operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum FsOp {
    Read { path: String },
    Write { path: String, content: String, preimage_sha: Option<String> },
    Patch { unified_diff: String, base_commit: Option<String> },
}

/// VCS operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum VcsOp {
    Diff,
    Commit { message: String },
    Push { remote: String, branch: String },
}

/// Agent operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AgentOp {
    Register {
        capabilities: Vec<String>,
        heartbeat_ms: u64,
        version: String,
    },
    Heartbeat {
        stats: HashMap<String, serde_json::Value>,
    },
    List,
}

/// Task operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum TaskOp {
    Submit {
        kind: String,
        payload: serde_json::Value,
        deps: Vec<String>,
    },
    Cancel {
        id: String,
    },
}

/// Token operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum TokenOp {
    ReportUsage {
        agent_id: String,
        prompt_tokens: u64,
        completion_tokens: u64,
        model: String,
    },
    GetBudget,
}

/// Session operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum SessionOp {
    Start {
        meta: HashMap<String, String>,
    },
    End {
        id: String,
    },
}

/// Pub/Sub operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum PubSubOp {
    Subscribe {
        topics: Vec<String>,
    },
    Unsubscribe {
        topics: Vec<String>,
    },
}

/// Event topics
pub mod topics {
    pub const LOCK_CHANGED: &str = "lock.changed";
    pub const FS_CHANGED: &str = "fs.changed";
    pub const VCS_CHANGED: &str = "vcs.changed";
    pub const TOKENS_UPDATED: &str = "tokens.updated";
    pub const AGENT_JOIN: &str = "agent.join";
    pub const AGENT_LEAVE: &str = "agent.leave";
    pub const TASK_PROGRESS: &str = "task.progress";
    pub const TASK_COMPLETED: &str = "task.completed";
    pub const TASK_FAILED: &str = "task.failed";
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_envelope_serialization() {
        let body = serde_json::json!({"test": "value"});
        let envelope = Envelope::new_request("test.op".to_string(), body);
        
        let json = serde_json::to_string(&envelope).unwrap();
        let deserialized: Envelope = serde_json::from_str(&json).unwrap();
        
        assert_eq!(envelope.v, deserialized.v);
        assert_eq!(envelope.op, deserialized.op);
        assert_eq!(envelope.msg_type, deserialized.msg_type);
    }
    
    #[test]
    fn test_response_envelope() {
        let request = Envelope::new_request(
            "test.op".to_string(),
            serde_json::json!({}),
        );
        
        let response = Envelope::response_for(
            &request,
            ResponseStatus::ok(),
            serde_json::json!({"result": "success"}),
        );
        
        assert_eq!(response.msg_type, MessageType::Response);
        assert_eq!(response.op, request.op);
    }
}
