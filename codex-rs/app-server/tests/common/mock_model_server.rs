// Mock model server for testing
use std::net::SocketAddr;

pub struct MockModelServer {
    addr: SocketAddr,
}

impl MockModelServer {
    pub fn new() -> Self {
        Self {
            addr: "127.0.0.1:0".parse().unwrap(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

pub fn create_mock_chat_completions_server() -> MockModelServer {
    MockModelServer::new()
}

pub fn create_mock_chat_completions_server_unchecked() -> MockModelServer {
    MockModelServer::new()
}
