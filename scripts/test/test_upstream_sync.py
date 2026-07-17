from __future__ import annotations

import argparse
import sys
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPTS_DIR = REPO_ROOT / "scripts"
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))

import fast_build
import resolve_merge_conflicts
import upstream_sync


class UpstreamSyncClassificationTests(unittest.TestCase):
    def test_classify_strategy_marks_gui_paths_for_plugin_migration(self) -> None:
        self.assertEqual(
            fast_build.classify_strategy("gui/src/app/page.tsx"), "plugin-migrate"
        )
        self.assertEqual(
            fast_build.classify_strategy("codex-gui-x/src/main.tsx"),
            "plugin-migrate",
        )

    def test_classify_strategy_marks_deepresearch_for_reinjection(self) -> None:
        self.assertEqual(
            fast_build.classify_strategy("codex-rs/deep-research/src/lib.rs"),
            "upstream-plus-reinject",
        )

    def test_build_report_payload_groups_paths_into_plan_buckets(self) -> None:
        args = argparse.Namespace(
            baseline_ref="rust-v0.121.0",
            remote="upstream",
            branch="main",
            base_branch="codex/upstream-sync-2026-04-17",
            create_branch="codex/upstream-sync-2026-04-17",
        )
        merge = upstream_sync.MergeOutcome(
            performed=False,
            conflicts=[],
            unresolved_conflicts=[],
        )
        workspace_repair = upstream_sync.WorkspaceRepairOutcome(
            performed=True,
            success=True,
            missing_members=["tools", "plugin"],
            restored_members=["plugin", "tools"],
            overlaid_members=["plugin", "tools"],
            missing_from_baseline=[],
        )
        validation = upstream_sync.ValidationOutcome(
            performed=True,
            success=True,
            steps=[],
        )
        build_release = upstream_sync.BuildOutcome(
            performed=True,
            success=True,
            command=["cargo", "build"],
            cwd="codex-rs",
            returncode=0,
            binary_path="codex-rs/target/release/codex.exe",
        )
        windows_install = upstream_sync.WindowsInstallOutcome(
            performed=True,
            success=True,
            command=["powershell", "-Command", "& 'scripts/install_with_kill.ps1' ..."],
            returncode=0,
            install_path=r"C:\Users\downl\.cargo\bin\codex.exe",
            resolved_command_path=r"C:\Users\downl\.cargo\bin\codex.exe",
            codexapp_before=[
                {
                    "Id": 1234,
                    "Path": r"C:\Program Files\WindowsApps\OpenAI.Codex_x\app\Codex.exe",
                }
            ],
            codexapp_after=[
                {
                    "Id": 1234,
                    "Path": r"C:\Program Files\WindowsApps\OpenAI.Codex_x\app\Codex.exe",
                }
            ],
            surviving_codexapp_pids=[1234],
            version_output="codex 3.1.0",
            app_server_help_ok=True,
        )
        payload = upstream_sync.build_report_payload(
            args=args,
            branch_name="main",
            candidate_paths=[
                "codex-rs/cli/src/main.rs",
                "codex-rs/deep-research/src/lib.rs",
                "gui/src/app/page.tsx",
                "gui/src/components/virtual-os/VirtualDesktop.tsx",
                "scripts/upstream_sync.py",
            ],
            custom_commits=["abc123 implement plugin migration"],
            range_diff="",
            merge=merge,
            workspace_repair=workspace_repair,
            validation=validation,
            build_release_outcome=build_release,
            windows_install=windows_install,
        )

        counts = payload["summary"]["classification_counts"]
        self.assertEqual(counts["upstream-first"], 1)
        self.assertEqual(counts["upstream-plus-reinject"], 1)
        self.assertEqual(counts["plugin-migrate"], 1)
        self.assertEqual(counts["retire-after-parity"], 1)
        self.assertEqual(counts["keep-fork"], 1)

        markdown = upstream_sync.build_markdown_report(payload)
        self.assertIn("Workspace Repair", markdown)
        self.assertIn("Windows Install", markdown)
        self.assertIn("Migrated To Plugin", markdown)
        self.assertIn("Retire After Parity", markdown)
        self.assertIn("Kept Fork-Specific", markdown)
        self.assertIn("Blocked on environment", markdown)


class UpstreamSyncWorkspaceRepairTests(unittest.TestCase):
    def test_find_missing_workspace_members_detects_absent_directories(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            workspace_root = Path(temp_dir) / "codex-rs"
            (workspace_root / "cli").mkdir(parents=True)
            missing = upstream_sync.find_missing_workspace_members(
                ["cli", "tools", "plugin"],
                workspace_root,
            )
        self.assertEqual(missing, ["plugin", "tools"])

    def test_select_post_tag_overlay_members_only_returns_tracked_members(self) -> None:
        overlay = upstream_sync.select_post_tag_overlay_members(
            ["plugin", "tools", "cli", "utils/plugins", "deep-research"],
        )
        self.assertEqual(overlay, ["plugin", "tools", "utils/plugins"])

    def test_build_windows_install_command_uses_explicit_array_literals(self) -> None:
        args = argparse.Namespace(
            install_helper=Path(r"C:\repo\scripts\install_with_kill.ps1"),
            install_target=Path(r"C:\Users\downl\.cargo\bin\codex.exe"),
            codexapp_exclude_prefix=[r"C:\Program Files\WindowsApps\OpenAI.Codex_"],
        )

        command = upstream_sync.build_windows_install_command(
            args,
            Path(r"C:\repo\codex-rs\target\release\codex.exe"),
        )

        self.assertEqual(
            command[:5],
            ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-Command"],
        )
        script = command[5]
        self.assertIn(
            "-ProcessNames @('codex', 'codex-tui', 'codex-gui', 'opencode')",
            script,
        )
        self.assertIn(
            "-ExcludePathPrefixes @('C:\\Program Files\\WindowsApps\\OpenAI.Codex_')",
            script,
        )
        self.assertIn(
            "& 'C:\\repo\\scripts\\install_with_kill.ps1'",
            script,
        )

    def test_detect_validation_environment_blocker_for_windows_v8_symlink_failure(
        self,
    ) -> None:
        original_platform = upstream_sync.sys.platform
        try:
            upstream_sync.sys.platform = "win32"
            step = upstream_sync.StepOutcome(
                name="cargo-test",
                command=["cargo", "test", "--workspace"],
                cwd=r"C:\repo\codex-rs",
                returncode=101,
                stdout_tail="error: failed to run custom build command for `v8 v146.4.0`",
                stderr_tail=(
                    "symlink_dir failed: Os { code: 1314, kind: Uncategorized, "
                    'message: "クライアントは要求された特権を保有していません。" }\n'
                    "thread 'main' panicked\nFailed to create symlink"
                ),
                status="failed",
            )
            blocker = upstream_sync.detect_validation_environment_blocker(step)
        finally:
            upstream_sync.sys.platform = original_platform

        self.assertIsNotNone(blocker)
        assert blocker is not None
        self.assertEqual(blocker[0], "environment_prerequisite_missing")
        self.assertIn("Developer Mode", blocker[1])

    def test_build_report_payload_renders_environment_blocker_note(self) -> None:
        args = argparse.Namespace(
            baseline_ref="rust-v0.121.0",
            remote="upstream",
            branch="main",
            base_branch="codex/upstream-sync-2026-04-18",
            create_branch="codex/upstream-sync-2026-04-18",
        )
        validation = upstream_sync.ValidationOutcome(
            performed=True,
            success=False,
            steps=[
                upstream_sync.StepOutcome(
                    name="cargo-test",
                    command=["cargo", "test", "--workspace"],
                    cwd=r"C:\repo\codex-rs",
                    returncode=101,
                    stdout_tail="",
                    stderr_tail="",
                    status="environment-prerequisite-missing",
                    failure_kind="environment_prerequisite_missing",
                    failure_summary="Native Windows full workspace validation requires Developer Mode.",
                )
            ],
            blocked_on_environment=True,
            failure_kind="environment_prerequisite_missing",
            failure_summary="Native Windows full workspace validation requires Developer Mode.",
        )
        payload = upstream_sync.build_report_payload(
            args=args,
            branch_name="main",
            candidate_paths=[],
            custom_commits=[],
            range_diff="",
            merge=upstream_sync.MergeOutcome(),
            workspace_repair=upstream_sync.WorkspaceRepairOutcome(),
            validation=validation,
            build_release_outcome=upstream_sync.BuildOutcome(),
            windows_install=upstream_sync.WindowsInstallOutcome(),
        )

        markdown = upstream_sync.build_markdown_report(payload)
        self.assertIn("Validation Note", markdown)
        self.assertIn("environment_prerequisite_missing", markdown)
        self.assertIn("Blocked on environment: yes", markdown)


class ResolveMergeConflictTests(unittest.TestCase):
    def test_plugin_migrate_prefers_upstream_block(self) -> None:
        resolved = resolve_merge_conflicts.resolve_block(
            "legacy gui line\n",
            "official plugin line\n",
            "plugin-migrate",
        )
        self.assertEqual(resolved, "official plugin line\n")

    def test_upstream_plus_reinject_keeps_unique_local_lines(self) -> None:
        resolved = resolve_merge_conflicts.resolve_block(
            "shared line\nlocal advantage\n",
            "shared line\nupstream line\n",
            "upstream-plus-reinject",
        )
        self.assertEqual(resolved, "shared line\nupstream line\nlocal advantage\n")


if __name__ == "__main__":
    unittest.main()
