use crate::state::AppState;
use crate::types::User;
use axum::{Json, extract::State};

pub async fn get_current_user(State(state): State<AppState>) -> Json<Option<User>> {
    let user = state.current_user.read().await.clone();

    // If no user is set, create a default user
    if user.is_none() {
        let default_user = User {
            id: "default-user".to_string(),
            name: "Codex User".to_string(),
            email: "user@codex.local".to_string(),
            avatar_url: None,
        };

        let mut current_user = state.current_user.write().await;
        *current_user = Some(default_user.clone());
        return Json(Some(default_user));
    }

    Json(user)
}
