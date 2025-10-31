//! Transport layer for orchestrator communication.
//!
//! Supports Unix Domain Sockets, Windows Named Pipes, and TCP fallback.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, warn};

#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

#[cfg(windows)]
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};

use crate::protocol::Envelope;

/// Transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Socket path for Unix Domain Socket (Unix only)
    #[cfg(unix)]
    pub socket_path: PathBuf,
    
    /// Named pipe name (Windows only)
    #[cfg(windows)]
    pub pipe_name: String,
    
    /// TCP fallback configuration
    pub tcp_fallback: Option<TcpConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            #[cfg(unix)]
            socket_path: PathBuf::from(".codex/orchestrator.sock"),
            #[cfg(windows)]
            pipe_name: r"\\.\pipe\codex-orchestrator".to_string(),
            tcp_fallback: Some(TcpConfig {
                host: "127.0.0.1".to_string(),
                port: 0, // ephemeral port
            }),
        }
    }
}

/// Transport server
pub enum TransportServer {
    #[cfg(unix)]
    Unix(UnixListener),
    #[cfg(windows)]
    NamedPipe(NamedPipeServer),
    Tcp(TcpListener),
}

impl TransportServer {
    /// Create a new transport server based on configuration
    pub async fn new(config: TransportConfig) -> Result<Self> {
        // Try Unix Domain Socket first on Unix
        #[cfg(unix)]
        {
            match Self::new_unix(&config.socket_path).await {
                Ok(server) => {
                    info!("Transport server listening on Unix socket: {:?}", config.socket_path);
                    return Ok(server);
                }
                Err(e) => {
                    warn!("Failed to create Unix socket: {}, falling back to TCP", e);
                }
            }
        }
        
        // Try Named Pipe on Windows
        #[cfg(windows)]
        {
            match Self::new_named_pipe(&config.pipe_name).await {
                Ok(server) => {
                    info!("Transport server listening on named pipe: {}", config.pipe_name);
                    return Ok(server);
                }
                Err(e) => {
                    warn!("Failed to create named pipe: {}, falling back to TCP", e);
                }
            }
        }
        
        // Fall back to TCP
        if let Some(tcp_config) = config.tcp_fallback {
            let addr = format!("{}:{}", tcp_config.host, tcp_config.port);
            let listener = TcpListener::bind(&addr)
                .await
                .with_context(|| format!("Failed to bind TCP listener to {}", addr))?;
            
            let local_addr = listener.local_addr()?;
            info!("Transport server listening on TCP: {}", local_addr);
            
            // Store port in .codex/orchestrator.port
            let port_file = PathBuf::from(".codex/orchestrator.port");
            if let Some(parent) = port_file.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            tokio::fs::write(&port_file, local_addr.port().to_string()).await?;
            
            Ok(TransportServer::Tcp(listener))
        } else {
            anyhow::bail!("No transport available");
        }
    }
    
    #[cfg(unix)]
    async fn new_unix(path: &PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Remove existing socket if present
        let _ = tokio::fs::remove_file(path).await;
        
        let listener = UnixListener::bind(path)
            .with_context(|| format!("Failed to bind Unix socket at {:?}", path))?;
        
        // Set permissions to 0700
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o700);
            std::fs::set_permissions(path, permissions)?;
        }
        
        Ok(TransportServer::Unix(listener))
    }
    
    #[cfg(windows)]
    async fn new_named_pipe(name: &str) -> Result<Self> {
        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(name)?;
        Ok(TransportServer::NamedPipe(server))
    }
    
    /// Accept a new connection
    pub async fn accept(&self) -> Result<TransportConnection> {
        match self {
            #[cfg(unix)]
            TransportServer::Unix(listener) => {
                let (stream, _) = listener.accept().await?;
                Ok(TransportConnection::Unix(stream))
            }
            #[cfg(windows)]
            TransportServer::NamedPipe(_) => {
                // TODO: Windows named pipe accept not yet implemented
                // Need to create a new pipe instance for each connection
                // See: https://docs.microsoft.com/en-us/windows/win32/ipc/named-pipes
                anyhow::bail!("Named pipe accept not implemented yet - use TCP fallback");
            }
            TransportServer::Tcp(listener) => {
                let (stream, addr) = listener.accept().await?;
                debug!("Accepted TCP connection from {}", addr);
                Ok(TransportConnection::Tcp(stream))
            }
        }
    }
}

/// Transport connection
pub enum TransportConnection {
    #[cfg(unix)]
    Unix(UnixStream),
    #[cfg(windows)]
    NamedPipe(NamedPipeServer),
    Tcp(TcpStream),
}

impl TransportConnection {
    /// Read a JSON-lines message
    pub async fn read_message(&mut self) -> Result<Envelope> {
        match self {
            #[cfg(unix)]
            TransportConnection::Unix(stream) => {
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                reader.read_line(&mut line).await?;
                serde_json::from_str(&line).context("Failed to parse JSON")
            }
            #[cfg(windows)]
            TransportConnection::NamedPipe(_) => {
                // TODO: Windows named pipe read not yet implemented
                // Use TCP transport on Windows for now
                anyhow::bail!("Named pipe read not implemented yet - use TCP fallback");
            }
            TransportConnection::Tcp(stream) => {
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                reader.read_line(&mut line).await?;
                serde_json::from_str(&line).context("Failed to parse JSON")
            }
        }
    }
    
    /// Write a JSON-lines message
    pub async fn write_message(&mut self, envelope: &Envelope) -> Result<()> {
        let json = serde_json::to_string(envelope)?;
        let line = format!("{}\n", json);
        
        match self {
            #[cfg(unix)]
            TransportConnection::Unix(stream) => {
                stream.write_all(line.as_bytes()).await?;
                stream.flush().await?;
            }
            #[cfg(windows)]
            TransportConnection::NamedPipe(_) => {
                // TODO: Windows named pipe write not yet implemented
                // Use TCP transport on Windows for now
                anyhow::bail!("Named pipe write not implemented yet - use TCP fallback");
            }
            TransportConnection::Tcp(stream) => {
                stream.write_all(line.as_bytes()).await?;
                stream.flush().await?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tcp_transport() {
        let config = TransportConfig {
            #[cfg(unix)]
            socket_path: PathBuf::from("/tmp/test.sock"),
            #[cfg(windows)]
            pipe_name: r"\\.\pipe\test".to_string(),
            tcp_fallback: Some(TcpConfig {
                host: "127.0.0.1".to_string(),
                port: 0,
            }),
        };
        
        let server = TransportServer::new(config).await.unwrap();
        
        // Server should be created successfully
        match server {
            TransportServer::Tcp(_) => {
                // Expected on systems without Unix sockets
            }
            #[cfg(unix)]
            TransportServer::Unix(_) => {
                // Expected on Unix systems
            }
            #[cfg(windows)]
            TransportServer::NamedPipe(_) => {
                // Expected on Windows
            }
        }
    }
}
