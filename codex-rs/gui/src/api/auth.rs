use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub db: Arc<SqlitePool>,
    pub jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub user: UserInfo,
    pub expires_at: String,
}

pub async fn login(
    axum::extract::Extension(state): axum::extract::Extension<AuthState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // Initialize database if needed
    init_db(&state.db).await?;

    // Find user by email
    let user = sqlx::query_as!(
        UserRecord,
        "SELECT id, email, password_hash, name FROM users WHERE email = ?",
        request.email
    )
    .fetch_optional(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    let user = user.ok_or_else(|| AuthError::InvalidCredentials)?;

    // Verify password
    bcrypt::verify(&request.password, &user.password_hash)
        .map_err(|_| AuthError::InvalidCredentials)?;

    // Generate JWT token
    let now = Utc::now();
    let exp = (now + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user.id.clone(),
        exp,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|e| AuthError::TokenGeneration(e.to_string()))?;

    // Create session record
    let session_id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)",
        session_id,
        user.id,
        exp
    )
    .execute(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            name: user.name,
        },
    }))
}

pub async fn register(
    axum::extract::Extension(state): axum::extract::Extension<AuthState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // Initialize database if needed
    init_db(&state.db).await?;

    // Check if user already exists
    let existing = sqlx::query!(
        "SELECT id FROM users WHERE email = ?",
        request.email
    )
    .fetch_optional(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    if existing.is_some() {
        return Err(AuthError::UserExists);
    }

    // Hash password
    let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AuthError::PasswordHash(e.to_string()))?;

    // Create user
    let user_id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO users (id, email, password_hash, name) VALUES (?, ?, ?, ?)",
        user_id,
        request.email,
        password_hash,
        request.name
    )
    .execute(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    // Generate JWT token
    let now = Utc::now();
    let exp = (now + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.clone(),
        exp,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|e| AuthError::TokenGeneration(e.to_string()))?;

    // Create session record
    let session_id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)",
        session_id,
        user_id,
        exp
    )
    .execute(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user: UserInfo {
            id: user_id,
            email: request.email,
            name: request.name,
        },
    }))
}

pub async fn logout(
    axum::extract::Extension(state): axum::extract::Extension<AuthState>,
    Json(request): Json<LogoutRequest>,
) -> Result<StatusCode, AuthError> {
    // Remove session
    sqlx::query!("DELETE FROM sessions WHERE id = ?", request.session_id)
        .execute(&*state.db)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub session_id: String,
}

pub async fn get_session(
    axum::extract::Extension(state): axum::extract::Extension<AuthState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<SessionInfo>, AuthError> {
    let token = params
        .get("token")
        .ok_or_else(|| AuthError::InvalidToken)?;
    // Decode JWT token
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    let user_id = token_data.claims.sub;

    // Get user info
    let user = sqlx::query_as!(
        UserRecord,
        "SELECT id, email, password_hash, name FROM users WHERE id = ?",
        user_id
    )
    .fetch_optional(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?
    .ok_or_else(|| AuthError::UserNotFound)?;

    // Get session expiry
    let session = sqlx::query!(
        "SELECT expires_at FROM sessions WHERE user_id = ? ORDER BY expires_at DESC LIMIT 1",
        user_id
    )
    .fetch_optional(&*state.db)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    let expires_at = session
        .map(|s| s.expires_at.to_string())
        .unwrap_or_else(|| Utc::now().to_rfc3339());

    Ok(Json(SessionInfo {
        user: UserInfo {
            id: user.id,
            email: user.email,
            name: user.name,
        },
        expires_at,
    }))
}

struct UserRecord {
    id: String,
    email: String,
    password_hash: String,
    name: Option<String>,
}

async fn init_db(pool: &SqlitePool) -> Result<(), AuthError> {
    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            name TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    // Create sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User already exists")]
    UserExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Database error: {0}")]
    Database(String),
    #[error("Password hash error: {0}")]
    PasswordHash(String),
    #[error("Token generation error: {0}")]
    TokenGeneration(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::UserExists => (StatusCode::CONFLICT, "User already exists"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, &msg),
            AuthError::PasswordHash(msg) => (StatusCode::INTERNAL_SERVER_ERROR, &msg),
            AuthError::TokenGeneration(msg) => (StatusCode::INTERNAL_SERVER_ERROR, &msg),
        };

        let body = Json(serde_json::json!({
            "error": message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
