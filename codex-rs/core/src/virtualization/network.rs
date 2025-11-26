//! Virtual Network Layer
//!
//! Provides network access for virtual OS instances with security policies.

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tracing::info;
use tracing::warn;

/// Network security policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSecurityPolicy {
    /// No network access
    DenyAll,
    /// Allow only specific domains
    Whitelist,
    /// Block specific domains
    Blacklist,
    /// Full network access (use with caution)
    AllowAll,
}

/// Network rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRule {
    pub domain: String,
    pub allowed: bool,
}

/// Virtual Network
pub struct VirtualNetwork {
    security_policy: NetworkSecurityPolicy,
    whitelist: Vec<String>,
    blacklist: Vec<String>,
    active_connections: HashMap<String, NetworkConnection>,
}

/// Network connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub id: String,
    pub url: String,
    pub method: String,
    pub status: ConnectionStatus,
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Pending,
    Connected,
    Failed,
    Blocked,
}

impl VirtualNetwork {
    pub fn new(security_policy: NetworkSecurityPolicy) -> Self {
        Self {
            security_policy,
            whitelist: Vec::new(),
            blacklist: Vec::new(),
            active_connections: HashMap::new(),
        }
    }

    /// Check if a URL is allowed
    pub fn is_allowed(&self, url: &str) -> bool {
        match self.security_policy {
            NetworkSecurityPolicy::DenyAll => false,
            NetworkSecurityPolicy::AllowAll => true,
            NetworkSecurityPolicy::Whitelist => {
                self.whitelist.iter().any(|domain| url.contains(domain))
            }
            NetworkSecurityPolicy::Blacklist => {
                !self.blacklist.iter().any(|domain| url.contains(domain))
            }
        }
    }

    /// Add a domain to whitelist
    pub fn add_whitelist(&mut self, domain: String) {
        if !self.whitelist.contains(&domain) {
            self.whitelist.push(domain);
            info!("Added to whitelist: {}", domain);
        }
    }

    /// Add a domain to blacklist
    pub fn add_blacklist(&mut self, domain: String) {
        if !self.blacklist.contains(&domain) {
            self.blacklist.push(domain);
            info!("Added to blacklist: {}", domain);
        }
    }

    /// Create a network connection
    pub async fn create_connection(&mut self, url: String, method: String) -> Result<String> {
        if !self.is_allowed(&url) {
            warn!("Connection blocked by security policy: {}", url);
            return Err(anyhow::anyhow!("Connection blocked: {}", url));
        }

        let connection_id = uuid::Uuid::new_v4().to_string();
        let connection = NetworkConnection {
            id: connection_id.clone(),
            url: url.clone(),
            method,
            status: ConnectionStatus::Pending,
        };

        self.active_connections
            .insert(connection_id.clone(), connection);
        info!("Created network connection: {}", url);

        // TODO: Implement actual network request
        // For now, mark as connected
        if let Some(conn) = self.active_connections.get_mut(&connection_id) {
            conn.status = ConnectionStatus::Connected;
        }

        Ok(connection_id)
    }

    /// Close a network connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<()> {
        self.active_connections.remove(connection_id);
        info!("Closed network connection: {}", connection_id);
        Ok(())
    }

    /// Get active connections
    pub fn get_active_connections(&self) -> Vec<&NetworkConnection> {
        self.active_connections.values().collect()
    }

    /// Set security policy
    pub fn set_security_policy(&mut self, policy: NetworkSecurityPolicy) {
        self.security_policy = policy;
        info!("Security policy changed to: {:?}", policy);
    }

    /// Get security policy
    pub fn get_security_policy(&self) -> NetworkSecurityPolicy {
        self.security_policy
    }
}

impl Default for VirtualNetwork {
    fn default() -> Self {
        Self::new(NetworkSecurityPolicy::Whitelist)
    }
}
