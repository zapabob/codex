// Dummy implementation for build
use anyhow::Result;
use std::path::Path;

use super::Connection;
use super::Transport;
use super::TransportInfo;
use async_trait::async_trait;

pub struct TcpTransport;

impl TcpTransport {
    pub async fn new(_port: u16, _dir: &Path) -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl Transport for TcpTransport {
    fn info(&self) -> TransportInfo {
        TransportInfo::Tcp {
            host: "127.0.0.1".to_string(),
            port: 0,
        }
    }

    async fn accept(&mut self) -> Result<Box<dyn Connection>> {
        anyhow::bail!("Not implemented")
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}
