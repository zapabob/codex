mod api;
mod error;
mod state;
mod types;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::extract::Extension;
use axum::routing::{get, post};
use http::Method;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::error::GuiError;
use crate::state::AppState;
use crate::types::action_definitions;

#[tokio::main]
async fn main() -> Result<(), GuiError> {
    init_tracing();

    let port = std::env::var("CODEX_GUI_PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(8787);

    let cli_path = std::env::var("CODEX_GUI_CLI_PATH").unwrap_or_else(|_| "codex".to_string());

    // Initialize SQLite database
    let db_url =
        std::env::var("CODEX_GUI_DB_URL").unwrap_or_else(|_| "sqlite:codex-gui.db".to_string());
    let db = SqlitePool::connect(&db_url)
        .await
        .map_err(|e| GuiError::Database(e.to_string()))?;

    // JWT secret (in production, use a secure random secret)
    let jwt_secret = std::env::var("CODEX_GUI_JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let state = AppState::new(cli_path.clone(), action_definitions());
    let cli_path_for_log = state.cli_path.clone();

    // Auth state
    let auth_state = api::auth::AuthState {
        db: Arc::new(db.clone()),
        jwt_secret,
    };

    // Plans state
    let plans_state = api::plans::PlansState {
        db: Arc::new(db.clone()),
        cli_path: Arc::new(cli_path.clone()),
    };

    // VR state
    let vr_state = api::vr::VRState {};

    let app = Router::new()
        // Existing routes
        .route("/api/actions", get(api::actions::list_actions))
        .route("/api/health", get(api::system::health))
        .route(
            "/api/actions/{id}/execute",
            post(api::actions::execute_action),
        )
        .route("/api/mcp/connections", get(api::mcp::list_mcp_connections))
        .route("/api/system/metrics", get(api::system::get_system_metrics))
        .route(
            "/api/conversations",
            get(api::conversations::list_conversations),
        )
        .route(
            "/api/conversations",
            post(api::conversations::create_conversation),
        )
        .route(
            "/api/conversations/{id}/messages",
            get(api::conversations::get_messages),
        )
        .route(
            "/api/conversations/{id}/messages",
            post(api::conversations::send_message),
        )
        .route("/api/user", get(api::user::get_current_user))
        .route(
            "/api/visualization/git4d",
            post(api::git4d::launch_git4d_visualization),
        )
        .route(
            "/api/visualization/git4d/sessions",
            get(api::git4d::list_git4d_sessions),
        )
        .route(
            "/api/visualization/git4d/capabilities/{mode}",
            get(api::git4d::get_git4d_capabilities),
        )
        .route(
            "/api/visualization/git4d/{session_id}/events",
            get(api::git4d::git4d_events_stream),
        )
        // Auth routes
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/auth/register", post(api::auth::register))
        .route("/api/auth/logout", post(api::auth::logout))
        .route("/api/auth/session", get(api::auth::get_session))
        // Plans routes
        .route("/api/plans", get(api::plans::list_plans))
        .route("/api/plans", post(api::plans::create_plan))
        .route("/api/plans/{id}", get(api::plans::get_plan))
        .route("/api/plans/{id}/approve", post(api::plans::approve_plan))
        .route("/api/plans/{id}/reject", post(api::plans::reject_plan))
        .route("/api/plans/{id}/execute", post(api::plans::execute_plan))
        .route("/api/plans/{id}/export", get(api::plans::export_plan))
        .route("/api/plans/mode", post(api::plans::toggle_plan_mode))
        .route(
            "/api/plans/mode/status",
            get(api::plans::get_plan_mode_status),
        )
        // VR routes
        .route("/api/vr/status", get(api::vr::get_vr_status))
        .route("/api/vr/session", post(api::vr::create_vr_session))
        .with_state(state)
        .layer(Extension(auth_state))
        .layer(Extension(plans_state))
        .layer(Extension(vr_state))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.map_err(GuiError::from)?;

    info!(
        port,
        cli_path = cli_path_for_log.as_str(),
        "listening on http://0.0.0.0:{port}"
    );

    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    server.await.map_err(GuiError::from)
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
