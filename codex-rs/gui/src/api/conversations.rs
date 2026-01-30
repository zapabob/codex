use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::GuiError;
use crate::state::AppState;
use crate::types::{Conversation, Message};

pub async fn list_conversations(State(state): State<AppState>) -> Json<Vec<Conversation>> {
    let conversations = state.conversations.read().await.clone();
    Json(conversations)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationRequest {
    pub model: String,
    pub initial_message: Option<String>,
}

pub async fn create_conversation(
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Json<Conversation> {
    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        model: request.model.clone(),
        status: "active".to_string(),
        created_at: Utc::now(),
        last_activity: Utc::now(),
        message_count: if request.initial_message.is_some() {
            1
        } else {
            0
        },
        summary: None,
    };

    // Add initial message if provided
    if let Some(content) = request.initial_message {
        let message = Message {
            id: Uuid::new_v4().to_string(),
            role: "user".to_string(),
            content,
            timestamp: Utc::now(),
        };

        let mut messages = state.messages.write().await;
        messages.insert(conversation.id.clone(), vec![message]);
    }

    // Add conversation to state
    let mut conversations = state.conversations.write().await;
    conversations.push(conversation.clone());

    Json(conversation)
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Result<Json<Vec<Message>>, GuiError> {
    let messages = state.messages.read().await;
    let conversation_messages = messages.get(&conversation_id).cloned().unwrap_or_default();

    Ok(Json(conversation_messages))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    pub content: String,
    pub role: Option<String>,
}

pub async fn send_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<Message>, GuiError> {
    let message = Message {
        id: Uuid::new_v4().to_string(),
        role: request.role.unwrap_or_else(|| "user".to_string()),
        content: request.content.clone(),
        timestamp: Utc::now(),
    };

    // Add message to conversation
    let mut messages = state.messages.write().await;
    let conversation_messages = messages
        .entry(conversation_id.clone())
        .or_insert_with(Vec::new);
    conversation_messages.push(message.clone());

    // Update conversation metadata
    let mut conversations = state.conversations.write().await;
    if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
        conversation.last_activity = Utc::now();
        conversation.message_count += 1;
    }

    Ok(Json(message))
}
