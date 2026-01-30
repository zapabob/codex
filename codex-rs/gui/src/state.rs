use crate::types::{ActionDefinition, Conversation, Message, User};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub cli_path: Arc<String>,
    pub actions: Arc<Vec<ActionDefinition>>,
    pub conversations: Arc<RwLock<Vec<Conversation>>>,
    pub messages: Arc<RwLock<HashMap<String, Vec<Message>>>>,
    pub current_user: Arc<RwLock<Option<User>>>,
}

impl AppState {
    pub fn new(cli_path: String, actions: Vec<ActionDefinition>) -> Self {
        Self {
            cli_path: Arc::new(cli_path),
            actions: Arc::new(actions),
            conversations: Arc::new(RwLock::new(Vec::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
            current_user: Arc::new(RwLock::new(None)),
        }
    }

    pub fn find_action(&self, id: &str) -> Option<ActionDefinition> {
        self.actions.iter().find(|&action| action.id == id).cloned()
    }
}
