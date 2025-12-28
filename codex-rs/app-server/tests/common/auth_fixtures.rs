#![allow(dead_code)]

// Test authentication fixtures
use serde::Deserialize;
use serde::Serialize;

pub fn test_api_key() -> String {
    "test-api-key".to_string()
}

pub fn test_user_id() -> String {
    "test-user-id".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGptAuthFixture {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGptIdTokenClaims {
    pub sub: String,
    pub email: String,
    pub exp: u64,
}

pub fn encode_id_token(_claims: &ChatGptIdTokenClaims) -> String {
    "test-encoded-token".to_string()
}

pub fn write_chatgpt_auth(_fixture: &ChatGptAuthFixture) -> String {
    "test-auth-data".to_string()
}
