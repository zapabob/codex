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


def summarize_bridge_response(base_url: str, response_payload: dict[str, Any]) -> dict[str, Any]:
    if response_payload.get("ok"):
        return {
            "available": True,
            "baseUrl": base_url,
            "status": response_payload.get("status"),
            "error": None,
        }
    return {
        "available": False,
        "baseUrl": base_url,
        "status": response_payload.get("status"),
        "error": response_payload.get("error") or "bridge unavailable",
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

    sessions_resp = fetch_git4d_sessions(base_url)
    capability_resp = fetch_git4d_capabilities(base_url, requested_mode)
    launch_resp = (
        launch_git4d_session(base_url, repo_path, requested_mode) if should_launch else None
    )

    bridge_status = summarize_bridge_response(base_url, sessions_resp)
    capability_status = summarize_bridge_response(base_url, capability_resp)
    launch_status = (
        summarize_bridge_response(base_url, launch_resp) if launch_resp is not None else None
    )

    sessions_body = sessions_resp.get("body")
    live_sessions = []
    if isinstance(sessions_body, list):
        live_sessions = [
            session
            for session in sessions_body
            if same_resolved_path(session.get("repositoryPath", ""), str(repo_path))
        ]

    capability_body = capability_resp.get("body")
    launch_body = launch_resp.get("body") if launch_resp is not None else None

    status_lines = status.splitlines()[:12] if status else []
    status_text = "\n".join(f"- {line}" for line in status_lines) if status_lines else "- working tree clean"
    log_lines = log_output.splitlines()
    log_text = "\n".join(f"- {line}" for line in log_lines) if log_lines else "- no commits found"

    capability_text = "- GUI bridge unavailable"
    if isinstance(capability_body, dict):
        capability_text = "\n".join(
            [
                f"- requested mode: {capability_body.get('requestedMode', requested_mode)}",
                f"- effective mode: {capability_body.get('effectiveMode', 'desktop')}",
                f"- device available: {capability_body.get('deviceAvailable', False)}",
                f"- platform: {capability_body.get('platform', 'unknown')}",
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
                f"mode={session.get('mode')} events={session.get('eventsPath')}"
            )
            for session in live_sessions
        )

    launch_text = "- launch not requested"
    if isinstance(launch_body, dict):
        launch_text = "\n".join(
            [
                f"- status: {launch_body.get('status', 'unknown')}",
                f"- session: {launch_body.get('sessionId', 'n/a')}",
                f"- effective mode: {launch_body.get('effectiveMode', requested_mode)}",
                f"- events: {launch_body.get('eventsPath', 'n/a')}",
            ]
        )
        if launch_body.get("fallbackReason"):
            launch_text += f"\n- fallback reason: {launch_body.get('fallbackReason')}"
    elif launch_resp is not None and launch_status is not None:
        launch_text = f"- launch failed: {launch_status.get('error')}"

    body = (
        "# Git4D Bridge Summary\n\n"
        f"Repository: {repo_path}\n"
        f"Branch: {branch}\n"
        f"HEAD: {head}\n"
        f"Requested mode: {requested_mode}\n\n"
        f"Bridge status:\n- GUI bridge available: {bridge_status.get('available')}\n"
        f"- GUI base URL: {base_url}\n"
        f"- Capability endpoint available: {capability_status.get('available')}\n"
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
            "recentCommits": log_lines,
            "workingTree": status_lines,
            "bridgeStatus": bridge_status,
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
    base_url = gui_base_url(arguments)

    openxr_runtime = os.environ.get("OPENXR_RUNTIME_JSON")
    python_available = shutil.which("python") is not None
    webxr_hint = (project_path / "package.json").is_file() or (project_path / "web").is_dir()
    vrchat_hint = (project_path / "Packages").is_dir() or (project_path / "Assets").is_dir()

    capability_resp = fetch_git4d_capabilities(base_url, mode)
    capability_body = capability_resp.get("body")
    bridge_status = summarize_bridge_response(base_url, capability_resp)

    if isinstance(capability_body, dict):
        recommended_mode = str(capability_body.get("effectiveMode", "desktop"))
        fallback_reason = capability_body.get("fallbackReason")
        device_available = bool(capability_body.get("deviceAvailable"))
        platform_name = capability_body.get("platform")
    else:
        device_available = bool(openxr_runtime)
        recommended_mode = mode if device_available else "desktop"
        fallback_reason = None if device_available else "GUI bridge unavailable; using local fallback"
        platform_name = "OpenXR" if device_available else "Desktop"

    body = (
        "# VR or AR Capability Report\n\n"
        f"Mode: {mode}\n"
        f"Project path: {project_path}\n"
        f"Platform: {platform.system()} {platform.release()}\n"
        f"GUI bridge available: {bridge_status.get('available')}\n"
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
            "bridgeStatus": bridge_status,
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
