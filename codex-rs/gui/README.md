# Codex GUI Service

This crate exposes a lightweight HTTP service that translates curated UI actions into Codex CLI invocations.
It powers the web front-end located in `gui/`.

## Running locally

```bash
# 1. Start the HTTP service
cd codex-rs
cargo run -p codex-gui

# Optional: override settings
# CODEX_GUI_PORT=8080 CODEX_GUI_CLI_PATH=./target/debug/codex cargo run -p codex-gui
```

The front-end expects the backend at `http://localhost:8787` by default.

## API Endpoints

### Actions
- `GET /api/actions` – Fetches metadata about the available orchestration playbooks.
- `POST /api/actions/{id}/execute` – Runs the selected playbook.

### Authentication
- `POST /api/auth/login` – User login
- `POST /api/auth/register` – User registration
- `POST /api/auth/logout` – User logout
- `GET /api/auth/session` – Get current session

### Plans
- `GET /api/plans` – List all plans
- `POST /api/plans` – Create a new plan
- `GET /api/plans/{id}` – Get plan details
- `POST /api/plans/{id}/approve` – Approve a plan
- `POST /api/plans/{id}/reject` – Reject a plan
- `POST /api/plans/{id}/execute` – Execute a plan
- `GET /api/plans/{id}/export` – Export a plan
- `POST /api/plans/mode` – Toggle plan mode
- `GET /api/plans/mode/status` – Get plan mode status

### VR/AR
- `GET /api/vr/status` – Get VR/AR support status
- `POST /api/vr/session` – Create VR/AR session

### Visualization
- `POST /api/visualization/git4d` – Launch Git4D visualization
- `GET /api/visualization/git4d/sessions` – List Git4D sessions
- `GET /api/visualization/git4d/capabilities/{mode}` – Report whether `desktop`, `vr`, or `ar` can run natively and what fallback mode will be used
- `GET /api/visualization/git4d/{session_id}/events` – Stream Git4D events (SSE)

### System
- `GET /api/system/metrics` – Get system metrics
- `GET /api/mcp/connections` – List MCP connections
- `GET /api/user` – Get current user

### Conversations
- `GET /api/conversations` – List conversations
- `POST /api/conversations` – Create conversation
- `GET /api/conversations/{id}/messages` – Get messages
- `POST /api/conversations/{id}/messages` – Send message

## Environment Variables

- `CODEX_GUI_PORT` - Port to listen on (default: 8787)
- `CODEX_GUI_CLI_PATH` - Path to codex CLI executable (default: "codex")
- `CODEX_GUI_DB_URL` - SQLite database URL (default: "sqlite:codex-gui.db")
- `CODEX_GUI_JWT_SECRET` - JWT secret for authentication (required in production)
- `RUST_LOG` - Log level (default: "info")

## Database

The service uses SQLite for storing:
- User accounts
- Sessions
- Plan metadata (plans are managed by CLI)

Database migrations are in `migrations/` directory.

## Dependencies

- `axum` - Web framework
- `sqlx` - SQL database toolkit
- `jsonwebtoken` - JWT authentication
- `bcrypt` - Password hashing
- `chrono` - Date/time handling
- `uuid` - UUID generation

## Security

- Passwords are hashed with bcrypt
- JWT tokens for authentication
- SQLite database for data storage
- CORS configuration for frontend access
