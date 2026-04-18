#!/usr/bin/env python3
"""Lightweight MCP server for the Zapabob Legacy Suite plugin."""

from __future__ import annotations

import json
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any
from urllib import error, request


TOOLS = [
    {
        "name": "deepresearch_brief",
        "description": (
            "Create a DeepResearch-style plan that routes work through the "
            "official Codex plugin, app, and browsing surfaces."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "topic": {"type": "string"},
                "goal": {"type": "string"},
                "constraints": {
                    "type": "array",
                    "items": {"type": "string"},
                },
            },
            "required": ["topic"],
            "additionalProperties": False,
        },
    },
    {
        "name": "git4d_repo_summary",
        "description": (
            "Summarize a repository in a Git4D-compatible format and, when the "
            "lightweight GUI bridge is available, include live session, launch, "
            "SSE, and capability information."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "repoPath": {"type": "string"},
                "maxCommits": {"type": "integer", "minimum": 1, "maximum": 50},
                "mode": {
                    "type": "string",
                    "enum": ["desktop", "vr", "ar"],
                },
                "launch": {"type": "boolean"},
                "appServerWsUrl": {"type": "string"},
                "guiBaseUrl": {"type": "string"},
            },
            "additionalProperties": False,
        },
    },
    {
        "name": "vr_ar_capability_report",
        "description": (
            "Report whether immersive VR or AR prerequisites are available and, "
            "when possible, use the lightweight GUI bridge capability endpoint "
            "before falling back to local heuristics."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "mode": {
                    "type": "string",
                    "enum": ["desktop", "vr", "ar"],
                },
                "target": {
                    "type": "string",
                    "enum": ["webxr", "vrchat", "generic"],
                },
                "projectPath": {"type": "string"},
                "appServerWsUrl": {"type": "string"},
                "guiBaseUrl": {"type": "string"},
            },
            "additionalProperties": False,
        },
    },
]


def response(
    request_id: Any,
    result: dict[str, Any] | None = None,
    error_payload: dict[str, Any] | None = None,
) -> dict[str, Any]:
    payload: dict[str, Any] = {
        "jsonrpc": "2.0",
        "id": request_id,
    }
    if error_payload is not None:
        payload["error"] = error_payload
    else:
        payload["result"] = result or {}
    return payload


def text_result(text: str, structured: dict[str, Any] | None = None) -> dict[str, Any]:
    result: dict[str, Any] = {
        "content": [{"type": "text", "text": text}],
    }
    if structured is not None:
        result["structuredContent"] = structured
    return result


def run_git(repo_path: Path, *args: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(repo_path), *args],
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    return completed.stdout.strip()


def resolve_mode(arguments: dict[str, Any], *, default: str) -> str:
    mode = str(arguments.get("mode", "") or "").strip().lower()
    if mode in {"desktop", "vr", "ar"}:
        return mode

    target = str(arguments.get("target", "") or "").strip().lower()
    if target in {"webxr", "vrchat"}:
        return "vr"
    return default


def bool_argument(arguments: dict[str, Any], name: str, default: bool) -> bool:
    value = arguments.get(name, default)
    if isinstance(value, bool):
        return value
    return default


def gui_base_url(arguments: dict[str, Any]) -> str:
    explicit = str(arguments.get("guiBaseUrl", "") or "").strip()
    if explicit:
        return explicit.rstrip("/")

    env_value = str(os.environ.get("CODEX_GUI_BASE_URL", "") or "").strip()
    if env_value:
        return env_value.rstrip("/")

    port = str(os.environ.get("CODEX_GUI_PORT", "8787") or "8787").strip() or "8787"
    return f"http://127.0.0.1:{port}"


def app_server_ws_url(arguments: dict[str, Any]) -> str:
    explicit = str(arguments.get("appServerWsUrl", "") or "").strip()
    if explicit:
        return explicit

    return str(os.environ.get("CODEX_APP_SERVER_WS_URL", "") or "").strip()


def http_json(
    method: str,
    url: str,
    payload: dict[str, Any] | None = None,
    *,
    timeout: float = 5.0,
) -> dict[str, Any]:
    body_bytes = None
    if payload is not None:
        body_bytes = json.dumps(payload).encode("utf-8")

    req = request.Request(
        url,
        data=body_bytes,
        headers={
            "Accept": "application/json",
            "Content-Type": "application/json",
        },
        method=method,
    )

    try:
        with request.urlopen(req, timeout=timeout) as resp:
            raw = resp.read().decode("utf-8", errors="replace")
            parsed = json.loads(raw) if raw else None
            return {
                "ok": True,
                "status": getattr(resp, "status", 200),
                "body": parsed,
                "raw": raw,
            }
    except error.HTTPError as err:
        raw = err.read().decode("utf-8", errors="replace")
        try:
            parsed = json.loads(raw) if raw else None
        except json.JSONDecodeError:
            parsed = None
        return {
            "ok": False,
            "status": err.code,
            "body": parsed,
            "raw": raw,
            "error": f"HTTP {err.code}",
        }
    except (error.URLError, OSError, TimeoutError, json.JSONDecodeError) as err:
        return {
            "ok": False,
            "status": None,
            "body": None,
            "raw": "",
            "error": str(err),
        }


class AppServerRpcClient:
    def __init__(self, ws_url: str, *, timeout: float = 5.0):
        self.ws_url = ws_url
        self.timeout = timeout
        self._connection: Any | None = None
        self._backend = "uninitialized"
        self._next_id = 1

    def __enter__(self) -> "AppServerRpcClient":
        self._connection, self._backend = self._open_connection()
        self.request(
            "initialize",
            {
                "clientInfo": {
                    "name": "zapabob-legacy-suite",
                    "version": "1.2.0",
                },
                "capabilities": {
                    "experimentalApi": True,
                },
            },
        )
        self.notify("initialized")
        return self

    def __exit__(self, exc_type: Any, exc: Any, tb: Any) -> None:
        if self._connection is not None:
            self._connection.close()
            self._connection = None

    def _open_connection(self) -> tuple[Any, str]:
        try:
            from websocket import create_connection  # type: ignore

            return create_connection(self.ws_url, timeout=self.timeout), "websocket-client"
        except ImportError:
            pass

        try:
            from websockets.sync.client import connect  # type: ignore

            return (
                connect(
                    self.ws_url,
                    open_timeout=self.timeout,
                    close_timeout=self.timeout,
                ),
                "websockets-sync",
            )
        except ImportError as err:
            raise RuntimeError(
                "No websocket client library available for CODEX_APP_SERVER_WS_URL; "
                "install websocket-client or websockets."
            ) from err

    def notify(self, method: str, params: dict[str, Any] | None = None) -> None:
        self._send(
            {
                "jsonrpc": "2.0",
                "method": method,
                "params": params or {},
            }
        )

    def request(self, method: str, params: dict[str, Any]) -> dict[str, Any]:
        request_id = self._next_id
        self._next_id += 1
        self._send(
            {
                "jsonrpc": "2.0",
                "id": request_id,
                "method": method,
                "params": params,
            }
        )

        while True:
            message = self._recv_json()
            if message.get("id") != request_id:
                continue
            if "error" in message:
                error_payload = message.get("error") or {}
                raise RuntimeError(
                    f"{method} failed: {error_payload.get('message', 'unknown error')}"
                )
            result = message.get("result")
            return result if isinstance(result, dict) else {}

    def _send(self, payload: dict[str, Any]) -> None:
        if self._connection is None:
            raise RuntimeError("app-server websocket is not connected")
        self._connection.send(json.dumps(payload))

    def _recv_json(self) -> dict[str, Any]:
        if self._connection is None:
            raise RuntimeError("app-server websocket is not connected")
        raw = self._connection.recv()
        if isinstance(raw, bytes):
            raw = raw.decode("utf-8", errors="replace")
        message = json.loads(raw)
        if not isinstance(message, dict):
            raise RuntimeError("app-server websocket returned a non-object payload")
        return message


def fetch_git4d_capabilities(base_url: str, mode: str) -> dict[str, Any]:
    return http_json("GET", f"{base_url}/api/visualization/git4d/capabilities/{mode}")


def fetch_git4d_sessions(base_url: str) -> dict[str, Any]:
    return http_json("GET", f"{base_url}/api/visualization/git4d/sessions")


def launch_git4d_session(base_url: str, repo_path: Path, mode: str) -> dict[str, Any]:
    return http_json(
        "POST",
        f"{base_url}/api/visualization/git4d",
        {
            "mode": mode,
            "repositoryPath": str(repo_path),
        },
    )


def summarize_bridge_response(endpoint: str, response_payload: dict[str, Any]) -> dict[str, Any]:
    if response_payload.get("ok"):
        return {
            "available": True,
            "endpoint": endpoint,
            "status": response_payload.get("status"),
            "error": None,
        }
    return {
        "available": False,
        "endpoint": endpoint,
        "status": response_payload.get("status"),
        "error": response_payload.get("error") or "bridge unavailable",
    }


def normalize_app_server_capability(body: dict[str, Any], requested_mode: str) -> dict[str, Any]:
    return {
        "requestedMode": str(body.get("requestedMode", requested_mode)),
        "effectiveMode": str(body.get("effectiveMode", "desktop")),
        "deviceAvailable": bool(body.get("nativeSupported")),
        "platform": body.get("platform"),
        "deviceName": body.get("deviceName"),
        "fallbackReason": body.get("fallbackReason"),
        "transport": "app-server-json-rpc",
    }


def normalize_app_server_session(body: dict[str, Any]) -> dict[str, Any]:
    return {
        "sessionId": body.get("sessionId"),
        "repositoryPath": body.get("repositoryPath"),
        "requestedMode": body.get("requestedMode"),
        "effectiveMode": body.get("effectiveMode"),
        "mode": body.get("effectiveMode"),
        "status": body.get("status"),
        "platform": body.get("platform"),
        "deviceName": body.get("deviceName"),
        "fallbackReason": body.get("fallbackReason"),
        "uptimeMs": body.get("uptimeMs"),
        "idleMs": body.get("idleMs"),
        "transport": "app-server-json-rpc",
    }


def fetch_git4d_state_from_app_server(
    ws_url: str,
    repo_path: Path,
    requested_mode: str,
    should_launch: bool,
) -> dict[str, Any]:
    try:
        with AppServerRpcClient(ws_url) as client:
            capability_result = client.request(
                "git4d/capabilities/read",
                {"mode": requested_mode},
            )
            session_list_result = client.request("git4d/session/list", {})
            launch_result = (
                client.request(
                    "git4d/session/start",
                    {
                        "repositoryPath": str(repo_path),
                        "mode": requested_mode,
                    },
                )
                if should_launch
                else None
            )
    except (OSError, RuntimeError, json.JSONDecodeError) as err:
        return {
            "ok": False,
            "status": None,
            "error": str(err),
            "body": None,
        }

    sessions = session_list_result.get("sessions")
    normalized_sessions = []
    if isinstance(sessions, list):
        normalized_sessions = [
            normalize_app_server_session(session)
            for session in sessions
            if isinstance(session, dict)
            and same_resolved_path(session.get("repositoryPath", ""), str(repo_path))
        ]

    normalized_launch = None
    if isinstance(launch_result, dict):
        launch_session = launch_result.get("session")
        if isinstance(launch_session, dict):
            normalized_launch = normalize_app_server_session(launch_session)

    return {
        "ok": True,
        "status": "jsonrpc",
        "error": None,
        "body": {
            "capability": normalize_app_server_capability(capability_result, requested_mode),
            "sessions": normalized_sessions,
            "launch": normalized_launch,
        },
    }


def fetch_git4d_capability_from_app_server(ws_url: str, requested_mode: str) -> dict[str, Any]:
    try:
        with AppServerRpcClient(ws_url) as client:
            capability_result = client.request(
                "git4d/capabilities/read",
                {"mode": requested_mode},
            )
    except (OSError, RuntimeError, json.JSONDecodeError) as err:
        return {
            "ok": False,
            "status": None,
            "error": str(err),
            "body": None,
        }

    return {
        "ok": True,
        "status": "jsonrpc",
        "error": None,
        "body": normalize_app_server_capability(capability_result, requested_mode),
    }


def same_resolved_path(left: str, right: str) -> bool:
    return str(Path(left).expanduser().resolve()).lower() == str(
        Path(right).expanduser().resolve()
    ).lower()


def handle_deepresearch_brief(arguments: dict[str, Any]) -> dict[str, Any]:
    topic = str(arguments.get("topic", "")).strip()
    if not topic:
        return text_result("`topic` is required.", {"ok": False})

    goal = str(arguments.get("goal", "")).strip() or "Produce a citation-backed answer."
    constraints = arguments.get("constraints", [])
    constraints_text = ""
    if isinstance(constraints, list) and constraints:
        rendered_constraints = "\n".join(f"- {item}" for item in constraints if str(item).strip())
        if rendered_constraints:
            constraints_text = f"\nConstraints:\n{rendered_constraints}"

    body = (
        "# DeepResearch Brief\n\n"
        f"Topic: {topic}\n"
        f"Goal: {goal}\n\n"
        "Recommended flow:\n"
        "1. Use the plugin mention `plugin://zapabob-legacy-suite@zapabob-repo-local`.\n"
        "2. Prefer official browsing plus the bundled GitHub and Hugging Face connectors.\n"
        "3. Keep outputs citation-oriented and degrade to text-first summaries when GUI-only affordances are unavailable."
        f"{constraints_text}"
    )
    return text_result(
        body,
        {
            "ok": True,
            "topic": topic,
            "goal": goal,
            "recommendedApps": ["github", "hugging-face", "vercel"],
        },
    )


def handle_git4d_repo_summary(arguments: dict[str, Any]) -> dict[str, Any]:
    repo_path = Path(str(arguments.get("repoPath", ".") or ".")).expanduser().resolve()
    max_commits = arguments.get("maxCommits", 8)
    try:
        max_commits = max(1, min(int(max_commits), 50))
    except (TypeError, ValueError):
        max_commits = 8

    if not repo_path.exists():
        return text_result(
            f"Repository path does not exist: {repo_path}",
            {"ok": False, "repoPath": str(repo_path)},
        )

    requested_mode = resolve_mode(arguments, default="desktop")
    should_launch = bool_argument(arguments, "launch", False)
    ws_url = app_server_ws_url(arguments)
    base_url = gui_base_url(arguments)

    try:
        branch = run_git(repo_path, "rev-parse", "--abbrev-ref", "HEAD")
        head = run_git(repo_path, "rev-parse", "--short", "HEAD")
        status = run_git(repo_path, "status", "--short")
        log_output = run_git(repo_path, "log", f"-n{max_commits}", "--oneline")
    except (subprocess.CalledProcessError, FileNotFoundError) as err:
        return text_result(
            f"Git4D bridge could not inspect `{repo_path}`: {err}",
            {"ok": False, "repoPath": str(repo_path)},
        )

    app_server_resp = (
        fetch_git4d_state_from_app_server(ws_url, repo_path, requested_mode, should_launch)
        if ws_url
        else {
            "ok": False,
            "status": None,
            "error": "CODEX_APP_SERVER_WS_URL is not set",
            "body": None,
        }
    )
    app_server_status = summarize_bridge_response(ws_url or "app-server disabled", app_server_resp)

    gui_sessions_resp = {
        "ok": False,
        "status": None,
        "error": "not attempted",
        "body": None,
    }
    gui_capability_resp = {
        "ok": False,
        "status": None,
        "error": "not attempted",
        "body": None,
    }
    gui_launch_resp = None
    gui_status = summarize_bridge_response(base_url, gui_sessions_resp)

    live_source = "local"
    capability_body = None
    live_sessions: list[dict[str, Any]] = []
    launch_body = None
    launch_status = None

    app_server_body = app_server_resp.get("body")
    if app_server_resp.get("ok") and isinstance(app_server_body, dict):
        capability_body = app_server_body.get("capability")
        sessions_body = app_server_body.get("sessions")
        if isinstance(sessions_body, list):
            live_sessions = [session for session in sessions_body if isinstance(session, dict)]
        launch_body = app_server_body.get("launch")
        live_source = "app-server"
        launch_status = summarize_bridge_response(ws_url, {"ok": True, "status": "jsonrpc"})
    else:
        gui_sessions_resp = fetch_git4d_sessions(base_url)
        gui_capability_resp = fetch_git4d_capabilities(base_url, requested_mode)
        gui_launch_resp = (
            launch_git4d_session(base_url, repo_path, requested_mode) if should_launch else None
        )
        gui_status = summarize_bridge_response(base_url, gui_sessions_resp)
        capability_status = summarize_bridge_response(
            f"{base_url}/api/visualization/git4d/capabilities/{requested_mode}",
            gui_capability_resp,
        )
        launch_status = (
            summarize_bridge_response(f"{base_url}/api/visualization/git4d", gui_launch_resp)
            if gui_launch_resp is not None
            else None
        )

        sessions_body = gui_sessions_resp.get("body")
        if isinstance(sessions_body, list):
            live_sessions = [
                session
                for session in sessions_body
                if same_resolved_path(session.get("repositoryPath", ""), str(repo_path))
            ]

        capability_body = gui_capability_resp.get("body")
        launch_body = gui_launch_resp.get("body") if gui_launch_resp is not None else None
        if isinstance(capability_body, dict) or live_sessions or isinstance(launch_body, dict):
            live_source = "gui"
    capability_status = summarize_bridge_response(
        live_source if live_source != "local" else "local fallback",
        {"ok": live_source != "local", "status": None, "error": None},
    )

    status_lines = status.splitlines()[:12] if status else []
    status_text = (
        "\n".join(f"- {line}" for line in status_lines)
        if status_lines
        else "- working tree clean"
    )
    log_lines = log_output.splitlines()
    log_text = "\n".join(f"- {line}" for line in log_lines) if log_lines else "- no commits found"

    capability_text = "- live bridge unavailable; using repository-only fallback"
    if isinstance(capability_body, dict):
        capability_text = "\n".join(
            [
                f"- live source: {live_source}",
                f"- requested mode: {capability_body.get('requestedMode', requested_mode)}",
                f"- effective mode: {capability_body.get('effectiveMode', 'desktop')}",
                f"- device available: {capability_body.get('deviceAvailable', False)}",
                f"- platform: {capability_body.get('platform', 'unknown')}",
                f"- transport: {capability_body.get('transport', live_source)}",
                (
                    f"- fallback reason: {capability_body.get('fallbackReason')}"
                    if capability_body.get("fallbackReason")
                    else "- fallback reason: none"
                ),
            ]
        )

    session_text = "- no active bridge sessions for this repository"
    if live_sessions:
        session_text = "\n".join(
            (
                f"- {session.get('sessionId')} [{session.get('status')}] "
                f"mode={session.get('mode')} transport={session.get('transport', live_source)} "
                f"events={session.get('eventsPath') or 'git4d/session/watch'}"
            )
            for session in live_sessions
        )

    launch_text = "- launch not requested"
    if isinstance(launch_body, dict):
        launch_text = "\n".join(
            [
                f"- transport: {launch_body.get('transport', live_source)}",
                f"- status: {launch_body.get('status', 'unknown')}",
                f"- session: {launch_body.get('sessionId', 'n/a')}",
                f"- effective mode: {launch_body.get('effectiveMode', requested_mode)}",
                f"- events: {launch_body.get('eventsPath', 'git4d/session/watch')}",
            ]
        )
        if launch_body.get("fallbackReason"):
            launch_text += f"\n- fallback reason: {launch_body.get('fallbackReason')}"
    elif should_launch and launch_status is not None:
        launch_text = f"- launch failed: {launch_status.get('error')}"

    body = (
        "# Git4D Bridge Summary\n\n"
        f"Repository: {repo_path}\n"
        f"Branch: {branch}\n"
        f"HEAD: {head}\n"
        f"Requested mode: {requested_mode}\n\n"
        "Bridge status:\n"
        f"- live source: {live_source}\n"
        f"- app-server available: {app_server_status.get('available')}\n"
        f"- app-server endpoint: {ws_url or 'not configured'}\n"
        f"- gui bridge available: {gui_status.get('available')}\n"
        f"- gui base URL: {base_url}\n"
        f"- capability available: {capability_status.get('available')}\n"
        f"- Launch attempted: {should_launch}\n\n"
        f"Live capability:\n{capability_text}\n\n"
        f"Active sessions:\n{session_text}\n\n"
        f"Launch result:\n{launch_text}\n\n"
        f"Recent commits:\n{log_text}\n\n"
        f"Working tree:\n{status_text}\n\n"
        "Note: this plugin prefers official plugin and app-server seams, and degrades to text-first summaries when the GUI bridge is unavailable."
    )
    return text_result(
        body,
        {
            "ok": True,
            "repoPath": str(repo_path),
            "branch": branch,
            "head": head,
            "requestedMode": requested_mode,
            "liveSource": live_source,
            "recentCommits": log_lines,
            "workingTree": status_lines,
            "bridgeStatus": capability_status,
            "appServerStatus": app_server_status,
            "guiStatus": gui_status,
            "capabilityStatus": capability_status,
            "launchStatus": launch_status,
            "capability": capability_body,
            "activeSessions": live_sessions,
            "launchResult": launch_body,
        },
    )


def handle_vr_ar_capability_report(arguments: dict[str, Any]) -> dict[str, Any]:
    mode = resolve_mode(arguments, default="vr")
    project_path = Path(str(arguments.get("projectPath", ".") or ".")).expanduser().resolve()
    ws_url = app_server_ws_url(arguments)
    base_url = gui_base_url(arguments)

    openxr_runtime = os.environ.get("OPENXR_RUNTIME_JSON")
    python_available = shutil.which("python") is not None
    webxr_hint = (project_path / "package.json").is_file() or (project_path / "web").is_dir()
    vrchat_hint = (project_path / "Packages").is_dir() or (project_path / "Assets").is_dir()

    app_server_resp = (
        fetch_git4d_capability_from_app_server(ws_url, mode)
        if ws_url
        else {
            "ok": False,
            "status": None,
            "error": "CODEX_APP_SERVER_WS_URL is not set",
            "body": None,
        }
    )
    app_server_status = summarize_bridge_response(ws_url or "app-server disabled", app_server_resp)

    gui_capability_resp = {
        "ok": False,
        "status": None,
        "error": "not attempted",
        "body": None,
    }
    gui_status = summarize_bridge_response(base_url, gui_capability_resp)

    capability_body = app_server_resp.get("body") if app_server_resp.get("ok") else None
    live_source = "app-server" if isinstance(capability_body, dict) else "local"
    if not isinstance(capability_body, dict):
        gui_capability_resp = fetch_git4d_capabilities(base_url, mode)
        gui_status = summarize_bridge_response(base_url, gui_capability_resp)
        capability_body = gui_capability_resp.get("body")
        if isinstance(capability_body, dict):
            live_source = "gui"

    if isinstance(capability_body, dict):
        recommended_mode = str(capability_body.get("effectiveMode", "desktop"))
        fallback_reason = capability_body.get("fallbackReason")
        device_available = bool(capability_body.get("deviceAvailable"))
        platform_name = capability_body.get("platform")
    else:
        device_available = bool(openxr_runtime)
        recommended_mode = mode if device_available else "desktop"
        fallback_reason = (
            None if device_available else "No live bridge available; using local fallback"
        )
        platform_name = "OpenXR" if device_available else "Desktop"

    body = (
        "# VR or AR Capability Report\n\n"
        f"Mode: {mode}\n"
        f"Project path: {project_path}\n"
        f"Platform: {platform.system()} {platform.release()}\n"
        f"Live source: {live_source}\n"
        f"App-server available: {app_server_status.get('available')}\n"
        f"App-server endpoint: {ws_url or 'not configured'}\n"
        f"GUI bridge available: {gui_status.get('available')}\n"
        f"GUI base URL: {base_url}\n"
        f"OpenXR runtime configured: {'yes' if openxr_runtime else 'no'}\n"
        f"Python available: {'yes' if python_available else 'no'}\n"
        f"WebXR project hints: {'yes' if webxr_hint else 'no'}\n"
        f"VRChat project hints: {'yes' if vrchat_hint else 'no'}\n"
        f"Device available: {'yes' if device_available else 'no'}\n"
        f"Recommended mode: {recommended_mode}\n"
        f"Platform hint: {platform_name}\n"
        f"Fallback reason: {fallback_reason or 'none'}\n\n"
        "If immersive support is unavailable, keep the request on official Codex surfaces and answer with a non-device summary instead of reviving legacy GUI behavior."
    )
    return text_result(
        body,
        {
            "ok": True,
            "mode": mode,
            "projectPath": str(project_path),
            "liveSource": live_source,
            "bridgeStatus": gui_status if live_source == "gui" else app_server_status,
            "appServerStatus": app_server_status,
            "guiStatus": gui_status,
            "capability": capability_body,
            "openxrConfigured": bool(openxr_runtime),
            "pythonAvailable": python_available,
            "webxrHint": webxr_hint,
            "vrchatHint": vrchat_hint,
            "deviceAvailable": device_available,
            "recommendedMode": recommended_mode,
            "fallbackReason": fallback_reason,
        },
    )


def handle_tool_call(name: str, arguments: dict[str, Any]) -> dict[str, Any]:
    if name == "deepresearch_brief":
        return handle_deepresearch_brief(arguments)
    if name == "git4d_repo_summary":
        return handle_git4d_repo_summary(arguments)
    if name == "vr_ar_capability_report":
        return handle_vr_ar_capability_report(arguments)
    return {
        "isError": True,
        "content": [{"type": "text", "text": f"Unknown tool: {name}"}],
    }


def main() -> int:
    for raw_line in sys.stdin:
        line = raw_line.strip()
        if not line:
            continue

        try:
            request_payload = json.loads(line)
        except json.JSONDecodeError as err:
            print(
                json.dumps(
                    response(
                        None,
                        error_payload={"code": -32700, "message": f"Parse error: {err}"},
                    )
                ),
                flush=True,
            )
            continue

        method = request_payload.get("method")
        request_id = request_payload.get("id")
        params = request_payload.get("params") or {}

        if method == "notifications/initialized":
            continue
        if method == "initialize":
            print(
                json.dumps(
                    response(
                        request_id,
                        {
                            "protocolVersion": "2024-11-05",
                            "capabilities": {"tools": {}},
                            "serverInfo": {
                                "name": "zapabob-legacy-suite-fallbacks",
                                "version": "1.1.0",
                            },
                        },
                    )
                ),
                flush=True,
            )
            continue
        if method == "tools/list":
            print(json.dumps(response(request_id, {"tools": TOOLS})), flush=True)
            continue
        if method == "tools/call":
            tool_name = str(params.get("name", ""))
            arguments = params.get("arguments") or {}
            if not isinstance(arguments, dict):
                arguments = {}
            print(
                json.dumps(response(request_id, handle_tool_call(tool_name, arguments))),
                flush=True,
            )
            continue

        if request_id is not None:
            print(
                json.dumps(
                    response(
                        request_id,
                        error_payload={
                            "code": -32601,
                            "message": f"Method not found: {method}",
                        },
                    )
                ),
                flush=True,
            )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
