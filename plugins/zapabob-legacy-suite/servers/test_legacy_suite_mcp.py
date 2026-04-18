import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SERVER_DIR = Path(__file__).resolve().parent
if str(SERVER_DIR) not in sys.path:
    sys.path.insert(0, str(SERVER_DIR))

import legacy_suite_mcp as legacy_suite


def init_git_repo(path: Path) -> None:
    subprocess.run(
        ["git", "init", str(path)],
        check=True,
        capture_output=True,
        text=True,
    )
    subprocess.run(
        [
            "git",
            "-C",
            str(path),
            "-c",
            "user.name=Codex Test",
            "-c",
            "user.email=codex@example.com",
            "commit",
            "--allow-empty",
            "-m",
            "initial",
        ],
        check=True,
        capture_output=True,
        text=True,
    )


class FakeAppServerClient:
    def __init__(self, ws_url: str, *, timeout: float = 5.0):
        self.ws_url = ws_url
        self.timeout = timeout

    def __enter__(self) -> "FakeAppServerClient":
        return self

    def __exit__(self, exc_type, exc, tb) -> None:
        return None

    def notify(self, method: str, params=None) -> None:
        return None

    def request(self, method: str, params):
        if method == "initialize":
            return {}
        if method == "git4d/capabilities/read":
            return {
                "requestedMode": params["mode"],
                "effectiveMode": "desktop" if params["mode"] == "ar" else params["mode"],
                "nativeSupported": params["mode"] != "ar",
                "platform": "Desktop" if params["mode"] == "ar" else "WebXR",
                "deviceName": None,
                "fallbackReason": (
                    "OpenXR runtime missing" if params["mode"] == "ar" else None
                ),
            }
        if method == "git4d/session/list":
            return {
                "sessions": [
                    {
                        "sessionId": "git4d-existing",
                        "repositoryPath": self.repo_path,
                        "requestedMode": "desktop",
                        "effectiveMode": "desktop",
                        "status": "active",
                        "platform": "Desktop",
                        "deviceName": None,
                        "fallbackReason": None,
                        "uptimeMs": 1200,
                        "idleMs": 100,
                    }
                ]
            }
        if method == "git4d/session/start":
            return {
                "session": {
                    "sessionId": "git4d-launched",
                    "repositoryPath": params["repositoryPath"],
                    "requestedMode": params["mode"],
                    "effectiveMode": "desktop",
                    "status": "starting",
                    "platform": "Desktop",
                    "deviceName": None,
                    "fallbackReason": "OpenXR runtime missing",
                    "uptimeMs": 0,
                    "idleMs": 0,
                }
            }
        raise AssertionError(f"Unexpected method: {method}")


class LegacySuiteTests(unittest.TestCase):
    def test_git4d_repo_summary_prefers_app_server(self) -> None:
        with tempfile.TemporaryDirectory() as tmp_dir:
            repo_path = Path(tmp_dir)
            init_git_repo(repo_path)
            fake_client = type(
                "ConfiguredFakeClient",
                (FakeAppServerClient,),
                {"repo_path": str(repo_path)},
            )

            with mock.patch.object(legacy_suite, "AppServerRpcClient", fake_client):
                result = legacy_suite.handle_git4d_repo_summary(
                    {
                        "repoPath": str(repo_path),
                        "mode": "ar",
                        "launch": True,
                        "appServerWsUrl": "ws://127.0.0.1:8765",
                    }
                )

        structured = result["structuredContent"]
        self.assertTrue(structured["ok"])
        self.assertEqual(structured["liveSource"], "app-server")
        self.assertEqual(structured["capability"]["transport"], "app-server-json-rpc")
        self.assertEqual(structured["launchResult"]["sessionId"], "git4d-launched")
        self.assertEqual(structured["activeSessions"][0]["sessionId"], "git4d-existing")

    def test_vr_ar_report_falls_back_to_gui_when_app_server_is_unavailable(self) -> None:
        gui_capability = {
            "requestedMode": "vr",
            "effectiveMode": "desktop",
            "deviceAvailable": False,
            "platform": "Desktop",
            "deviceName": None,
            "fallbackReason": "GUI bridge fallback",
            "transport": "sse",
        }

        with mock.patch.object(
            legacy_suite,
            "fetch_git4d_capability_from_app_server",
            return_value={"ok": False, "status": None, "error": "ws down", "body": None},
        ), mock.patch.object(
            legacy_suite,
            "fetch_git4d_capabilities",
            return_value={"ok": True, "status": 200, "body": gui_capability},
        ):
            result = legacy_suite.handle_vr_ar_capability_report(
                {
                    "mode": "vr",
                    "projectPath": ".",
                    "appServerWsUrl": "ws://127.0.0.1:8765",
                    "guiBaseUrl": "http://127.0.0.1:8787",
                }
            )

        structured = result["structuredContent"]
        self.assertEqual(structured["liveSource"], "gui")
        self.assertEqual(structured["capability"]["transport"], "sse")
        self.assertEqual(structured["recommendedMode"], "desktop")

    def test_vr_ar_report_uses_local_fallback_when_live_bridges_are_unavailable(self) -> None:
        with mock.patch.object(
            legacy_suite,
            "fetch_git4d_capability_from_app_server",
            return_value={"ok": False, "status": None, "error": "ws down", "body": None},
        ), mock.patch.object(
            legacy_suite,
            "fetch_git4d_capabilities",
            return_value={"ok": False, "status": None, "error": "gui down", "body": None},
        ), mock.patch.dict(os.environ, {"OPENXR_RUNTIME_JSON": ""}, clear=False):
            result = legacy_suite.handle_vr_ar_capability_report(
                {
                    "mode": "vr",
                    "projectPath": ".",
                    "appServerWsUrl": "ws://127.0.0.1:8765",
                    "guiBaseUrl": "http://127.0.0.1:8787",
                }
            )

        structured = result["structuredContent"]
        self.assertEqual(structured["liveSource"], "local")
        self.assertFalse(structured["deviceAvailable"])
        self.assertEqual(structured["recommendedMode"], "desktop")


if __name__ == "__main__":
    unittest.main()
