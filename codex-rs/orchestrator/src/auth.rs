// Dummy implementation for build
use anyhow::Result;
use std::path::Path;

#[derive(Clone)]
pub struct AuthManager;

impl AuthManager {
    pub fn new(_path: &Path) -> Result<Self> {
        Ok(Self)
    }

    pub fn from_auth_for_testing<T>(_auth: T) -> Self {
        Self
    }
}

pub struct AuthHeader;

pub struct HmacAuthenticator;

#[derive(Clone)]
pub struct CodexAuth;
