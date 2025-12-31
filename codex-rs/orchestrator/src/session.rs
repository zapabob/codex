//! Session management for orchestrator
//!
//! Provides secure session management with session ID generation,
//! expiration handling, and session fixation attack prevention.

use chrono::DateTime;
use chrono::Utc;
use rand::Rng;
use sha2::Digest;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Session information
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub session_id: String,
    /// User ID
    pub user_id: String,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// Session expiration time
    pub expires_at: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Session data (custom key-value pairs)
    pub data: HashMap<String, String>,
}

/// Session manager
pub struct SessionManager {
    /// Active sessions (session_id -> Session)
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    /// Session timeout in seconds
    session_timeout_sec: u64,
    /// Maximum session lifetime in seconds
    max_session_lifetime_sec: u64,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(session_timeout_sec: u64, max_session_lifetime_sec: u64) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_timeout_sec,
            max_session_lifetime_sec,
        }
    }

    /// Generate a cryptographically secure session ID
    fn generate_session_id() -> String {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);

        // Hash the random bytes for additional security
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = hasher.finalize();

        BASE64_STANDARD.encode(&hash[..])
    }

    /// Create a new session
    pub async fn create_session(&self, user_id: String) -> String {
        let session_id = Self::generate_session_id();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(self.session_timeout_sec as i64);

        let session = Session {
            session_id: session_id.clone(),
            user_id,
            created_at: now,
            expires_at,
            last_activity: now,
            data: HashMap::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        session_id
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Validate session (check expiration and update last activity)
    pub async fn validate_session(&self, session_id: &str) -> Result<Session, SessionError> {
        let mut sessions = self.sessions.write().await;

        let session = sessions
            .get_mut(session_id)
            .ok_or(SessionError::SessionNotFound)?;

        let now = Utc::now();

        // Check if session expired
        if session.expires_at < now {
            sessions.remove(session_id);
            return Err(SessionError::SessionExpired);
        }

        // Check maximum session lifetime
        let max_lifetime =
            session.created_at + chrono::Duration::seconds(self.max_session_lifetime_sec as i64);
        if max_lifetime < now {
            sessions.remove(session_id);
            return Err(SessionError::SessionExpired);
        }

        // Update last activity
        session.last_activity = now;
        // Extend expiration on activity
        session.expires_at = now + chrono::Duration::seconds(self.session_timeout_sec as i64);

        Ok(session.clone())
    }

    /// Regenerate session ID (for session fixation attack prevention)
    pub async fn regenerate_session_id(
        &self,
        old_session_id: &str,
    ) -> Result<String, SessionError> {
        let mut sessions = self.sessions.write().await;

        let session = sessions
            .remove(old_session_id)
            .ok_or(SessionError::SessionNotFound)?;

        // Create new session with same data
        let new_session_id = Self::generate_session_id();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(self.session_timeout_sec as i64);

        let new_session = Session {
            session_id: new_session_id.clone(),
            user_id: session.user_id,
            created_at: session.created_at, // Keep original creation time
            expires_at,
            last_activity: now,
            data: session.data,
        };

        sessions.insert(new_session_id.clone(), new_session);

        Ok(new_session_id)
    }

    /// Invalidate session (logout)
    pub async fn invalidate_session(&self, session_id: &str) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        sessions
            .remove(session_id)
            .ok_or(SessionError::SessionNotFound)?;
        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) {
        let now = Utc::now();
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, session| session.expires_at > now);
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: &str) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.user_id == user_id)
            .cloned()
            .collect()
    }
}

/// Session error
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Session not found")]
    SessionNotFound,
    #[error("Session expired")]
    SessionExpired,
    #[error("Session invalid")]
    SessionInvalid,
}
