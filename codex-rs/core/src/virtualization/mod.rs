//! Virtual OS Layer
//!
//! Provides virtualization layer for running applications in isolated environments
//! with macOS-style UI/UX, application creation interface, and internet connectivity.

pub mod app_creator;
pub mod macos_emulator;
pub mod network;
pub mod terminal;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Virtual OS layer trait
pub trait VirtualOSLayer {
    /// Initialize the virtual OS layer
    fn initialize(&mut self) -> Result<()>;

    /// Shutdown the virtual OS layer
    fn shutdown(&mut self) -> Result<()>;

    /// Get the virtual OS type
    fn os_type(&self) -> VirtualOSType;

    /// Check if the virtual OS is running
    fn is_running(&self) -> bool;
}

/// Virtual OS types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VirtualOSType {
    MacOS,
    Linux,
    Windows,
}

/// Virtual OS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualOSConfig {
    /// OS type
    pub os_type: VirtualOSType,
    /// Workspace directory
    pub workspace_path: PathBuf,
    /// Enable network access
    pub enable_network: bool,
    /// Memory limit in MB
    pub memory_mb: usize,
    /// CPU count
    pub cpu_count: usize,
    /// Enable GPU access
    pub enable_gpu: bool,
}

impl Default for VirtualOSConfig {
    fn default() -> Self {
        Self {
            os_type: VirtualOSType::MacOS,
            workspace_path: PathBuf::from("./virtual-os-workspace"),
            enable_network: true,
            memory_mb: 4096,
            cpu_count: 4,
            enable_gpu: false,
        }
    }
}

/// Virtual OS instance
pub struct VirtualOSInstance {
    /// Instance ID
    pub id: String,
    /// OS type
    pub os_type: VirtualOSType,
    /// Configuration
    pub config: VirtualOSConfig,
    /// Is running
    pub is_running: bool,
}

impl VirtualOSInstance {
    pub fn new(config: VirtualOSConfig) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            os_type: config.os_type,
            config,
            is_running: false,
        }
    }
}

/// Virtual OS Manager
pub struct VirtualOSManager {
    instances: Vec<VirtualOSInstance>,
}

impl VirtualOSManager {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    /// Create a new virtual OS instance
    pub fn create_instance(&mut self, config: VirtualOSConfig) -> Result<String> {
        let instance = VirtualOSInstance::new(config);
        let id = instance.id.clone();
        self.instances.push(instance);
        Ok(id)
    }

    /// Get an instance by ID
    pub fn get_instance(&self, id: &str) -> Option<&VirtualOSInstance> {
        self.instances.iter().find(|i| i.id == id)
    }

    /// Get a mutable instance by ID
    pub fn get_instance_mut(&mut self, id: &str) -> Option<&mut VirtualOSInstance> {
        self.instances.iter_mut().find(|i| i.id == id)
    }

    /// List all instances
    pub fn list_instances(&self) -> Vec<&VirtualOSInstance> {
        self.instances.iter().collect()
    }

    /// Remove an instance
    pub fn remove_instance(&mut self, id: &str) -> Result<()> {
        self.instances.retain(|i| i.id != id);
        Ok(())
    }
}

impl Default for VirtualOSManager {
    fn default() -> Self {
        Self::new()
    }
}

pub use app_creator::AppCreator;
pub use macos_emulator::MacOSEmulator;
pub use network::VirtualNetwork;
pub use terminal::{TerminalCommand, TerminalManager, TerminalResult, TerminalSession};
