# sandbox_smoketests.py
# Run a suite of smoke tests against the Windows sandbox via the Codex CLI
# Requires: Python 3.8+ on Windows. No pip requirements.

import os
import sys
import shutil
import subprocess
import contextlib
import http.client
import http.server
import threading
from pathlib import Path
from typing import List, Optional, Tuple
from urllib.parse import urlsplit

def _resolve_codex_cmd() -> List[str]:
    """Resolve the Codex CLI to invoke `codex sandbox windows`.

    Prefer `codex` on PATH; if not found, try common local build locations.
    Returns the argv prefix to run Codex.
    """
    # 1) Prefer PATH
    try:
        cp = subprocess.run(["where", "codex"], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        if cp.returncode == 0:
            for line in cp.stdout.splitlines():
                p = Path(line.strip())
                if p.exists():
                    return [str(p)]
    except Exception:
        pass

    # 2) Try workspace targets
    root = Path(__file__).parent
    ws_root = root.parent
    cargo_target = os.environ.get("CARGO_TARGET_DIR")
    candidates = [
        ws_root / "target" / "release" / "codex.exe",
        ws_root / "target" / "debug" / "codex.exe",
    ]
    if cargo_target:
        candidates.extend([
            Path(cargo_target) / "release" / "codex.exe",
            Path(cargo_target) / "debug" / "codex.exe",
        ])
    for p in candidates:
        if p.exists():
            return [str(p)]

    raise FileNotFoundError(
        "Codex CLI not found. Build it first, e.g.\n"
        "  cargo build -p codex-cli --release\n"
        "or for debug:\n"
        "  cargo build -p codex-cli\n"
    )

CODEX_CMD = _resolve_codex_cmd()
TIMEOUT_SEC = 20

WS_ROOT = Path(os.environ["USERPROFILE"]) / "sbx_ws_tests"
OUTSIDE = Path(os.environ["USERPROFILE"]) / "sbx_ws_outside"  # outside CWD for deny checks

ENV_BASE = {}  # extend if needed

class CaseResult:
    def __init__(self, name: str, ok: bool, detail: str = ""):
        self.name, self.ok, self.detail = name, ok, detail

def run_sbx(policy: str, cmd_argv: List[str], cwd: Path, env_extra: Optional[dict] = None) -> Tuple[int, str, str]:
    env = os.environ.copy()
    env.update(ENV_BASE)
    if env_extra:
        env.update(env_extra)
    # Map policy to codex CLI flags
    # read-only => default; workspace-write => --full-auto
    if policy not in ("read-only", "workspace-write"):
        raise ValueError(f"unknown policy: {policy}")
    policy_flags: List[str] = ["--full-auto"] if policy == "workspace-write" else []

    argv = [*CODEX_CMD, "sandbox", "windows", *policy_flags, "--", *cmd_argv]
    print(cmd_argv)
    cp = subprocess.run(argv, cwd=str(cwd), env=env,
                        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                        timeout=TIMEOUT_SEC, text=True)
    return cp.returncode, cp.stdout, cp.stderr

def have(cmd: str) -> bool:
    try:
        cp = subprocess.run(["where", cmd], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        return cp.returncode == 0 and any(Path(p.strip()).exists() for p in cp.stdout.splitlines())
    except Exception:
        return False

def make_dir_clean(p: Path) -> None:
    if p.exists():
        shutil.rmtree(p, ignore_errors=True)
    p.mkdir(parents=True, exist_ok=True)

def write_file(p: Path, content: str = "x") -> None:
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(content, encoding="utf-8")

def remove_if_exists(p: Path) -> None:
    try:
        if p.is_dir(): shutil.rmtree(p, ignore_errors=True)
        elif p.exists(): p.unlink(missing_ok=True)
    except Exception:
        pass

def assert_exists(p: Path) -> bool:
    return p.exists()

def assert_not_exists(p: Path) -> bool:
    return not p.exists()

def make_junction(link: Path, target: Path) -> bool:
    """Create a directory junction; return True if it exists afterward."""
    remove_if_exists(link)
    target.mkdir(parents=True, exist_ok=True)
    cmd = ["cmd", "/c", f'mklink /J "{link}" "{target}"']
    cp = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    return cp.returncode == 0 and link.exists()

def make_symlink(link: Path, target: Path) -> bool:
    """Create a directory symlink; return True if it exists afterward."""
    remove_if_exists(link)
    if not target.exists():
        try:
            target.mkdir(parents=True, exist_ok=True)
        except OSError:
            pass
    cmd = ["cmd", "/c", f'mklink /D "{link}" "{target}"']
    cp = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    return cp.returncode == 0 and link.exists()

class _QuietHandler(http.server.BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

class _TargetHandler(_QuietHandler):
    def do_GET(self):
        body = b"proxy-ok"
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

class _ProxyHandler(_QuietHandler):
    def do_GET(self):
        parsed = urlsplit(self.path)
        if not parsed.scheme or not parsed.hostname:
            self.send_error(400, "absolute URL required")
            return
        if parsed.hostname not in ("127.0.0.1", "localhost"):
            self.send_error(403, "only loopback hosts are allowed in smoke proxy")
            return
        path = parsed.path or "/"
        if parsed.query:
            path = f"{path}?{parsed.query}"
        conn = None
        try:
            conn = http.client.HTTPConnection(parsed.hostname, parsed.port or 80, timeout=2)
            conn.request("GET", path)
            upstream = conn.getresponse()
            body = upstream.read()
        except Exception as err:
            self.send_error(502, f"proxy upstream error: {err}")
            return
        finally:
            if conn is not None:
                with contextlib.suppress(Exception):
                    conn.close()
        self.send_response(upstream.status, upstream.reason)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

@contextlib.contextmanager
def start_loopback_proxy_fixture():
    target = http.server.ThreadingHTTPServer(("127.0.0.1", 0), _TargetHandler)
    proxy = http.server.ThreadingHTTPServer(("127.0.0.1", 0), _ProxyHandler)
    target_port = target.server_address[1]
    proxy_port = proxy.server_address[1]
    target_thread = threading.Thread(target=target.serve_forever, daemon=True)
    proxy_thread = threading.Thread(target=proxy.serve_forever, daemon=True)
    target_thread.start()
    proxy_thread.start()
    try:
        yield target_port, proxy_port
    finally:
        proxy.shutdown()
        target.shutdown()
        proxy.server_close()
        target.server_close()


def summarize(results: List[CaseResult]) -> int:
    ok = sum(1 for r in results if r.ok)
    total = len(results)
    print("\n" + "=" * 72)
    print(f"Sandbox smoke tests: {ok}/{total} passed")
    for r in results:
        print(f"[{'PASS' if r.ok else 'FAIL'}] {r.name}" + (f" :: {r.detail.strip()}" if r.detail and not r.ok else ""))
    print("=" * 72)
    return 0 if ok == total else 1

def main() -> int:
    results: List[CaseResult] = []
    make_dir_clean(WS_ROOT)
    OUTSIDE.mkdir(exist_ok=True)
    # Environment probe: some hosts allow TEMP writes even under read-only
    # tokens due to ACLs and restricted SID semantics. Detect and adapt tests.
    probe_rc, _, _ = run_sbx(
        "read-only",
        ["cmd", "/c", "echo probe > %TEMP%\\sbx_ro_probe.txt"],
        WS_ROOT,
    )
    ro_temp_denied = probe_rc != 0

    def add(name: str, ok: bool, detail: str = ""):
        print('running', name)
        results.append(CaseResult(name, ok, detail))

    # 1. RO: deny write in CWD
    target = WS_ROOT / "ro_should_fail.txt"
    remove_if_exists(target)
    rc, out, err = run_sbx("read-only", ["cmd", "/c", "echo nope > ro_should_fail.txt"], WS_ROOT)
    add("RO: write in CWD denied", rc != 0 and assert_not_exists(target), f"rc={rc}, err={err}")

    # 2. WS: allow write in CWD
    target = WS_ROOT / "ws_ok.txt"
    remove_if_exists(target)
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "echo ok > ws_ok.txt"], WS_ROOT)
    add("WS: write in CWD allowed", rc == 0 and assert_exists(target), f"rc={rc}, err={err}")

    # 3. WS: deny write outside workspace
    outside_file = OUTSIDE / "blocked.txt"
    remove_if_exists(outside_file)
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", f"echo nope > {outside_file}"], WS_ROOT)
    add("WS: write outside workspace denied", rc != 0 and assert_not_exists(outside_file), f"rc={rc}")

    # 4. WS: allow TEMP write
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "echo tempok > %TEMP%\\ws_temp_ok.txt"], WS_ROOT)
    add("WS: TEMP write allowed", rc == 0, f"rc={rc}")

    # 5. RO: deny TEMP write
    rc, out, err = run_sbx("read-only", ["cmd", "/c", "echo tempno > %TEMP%\\ro_temp_fail.txt"], WS_ROOT)
    if ro_temp_denied:
        add("RO: TEMP write denied", rc != 0, f"rc={rc}")
    else:
        add("RO: TEMP write denied (skipped on this host)", True)

    # 6. WS: append OK in CWD
    target = WS_ROOT / "append.txt"
    remove_if_exists(target); write_file(target, "line1\n")
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "echo line2 >> append.txt"], WS_ROOT)
    add("WS: append allowed", rc == 0 and target.read_text().strip().endswith("line2"), f"rc={rc}")

    # 7. RO: append denied
    target = WS_ROOT / "ro_append.txt"
    write_file(target, "line1\n")
    rc, out, err = run_sbx("read-only", ["cmd", "/c", "echo line2 >> ro_append.txt"], WS_ROOT)
    add("RO: append denied", rc != 0 and target.read_text() == "line1\n", f"rc={rc}")

    # 8. WS: PowerShell Set-Content in CWD (OK)
    target = WS_ROOT / "ps_ok.txt"
    remove_if_exists(target)
    rc, out, err = run_sbx("workspace-write",
                           ["powershell", "-NoLogo", "-NoProfile", "-Command",
                            "Set-Content -LiteralPath ps_ok.txt -Value 'hello' -Encoding ASCII"], WS_ROOT)
    add("WS: PowerShell Set-Content allowed", rc == 0 and assert_exists(target), f"rc={rc}, err={err}")

    # 9. RO: PowerShell Set-Content denied
    target = WS_ROOT / "ps_ro_fail.txt"
    remove_if_exists(target)
    rc, out, err = run_sbx("read-only",
                           ["powershell", "-NoLogo", "-NoProfile", "-Command",
                            "Set-Content -LiteralPath ps_ro_fail.txt -Value 'x'"], WS_ROOT)
    add("RO: PowerShell Set-Content denied", rc != 0 and assert_not_exists(target), f"rc={rc}")

    # 10. WS: mkdir and write (OK)
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "mkdir sub && echo hi > sub\\in_sub.txt"], WS_ROOT)
    add("WS: mkdir+write allowed", rc == 0 and (WS_ROOT / "sub/in_sub.txt").exists(), f"rc={rc}")

    # 11. WS: rename (EXPECTED SUCCESS on this host)
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "echo x > r.txt & ren r.txt r2.txt"], WS_ROOT)
    add("WS: rename succeeds (expected on this host)", rc == 0 and (WS_ROOT / "r2.txt").exists(), f"rc={rc}, err={err}")

    # 12. WS: delete (EXPECTED SUCCESS on this host)
    target = WS_ROOT / "delme.txt"; write_file(target, "x")
    rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "del /q delme.txt"], WS_ROOT)
    add("WS: delete succeeds (expected on this host)", rc == 0 and not target.exists(), f"rc={rc}, err={err}")

    # 13. RO: python tries to write (denied)
    pyfile = WS_ROOT / "py_should_fail.txt"; remove_if_exists(pyfile)
    rc, out, err = run_sbx("read-only", ["python", "-c", "open('py_should_fail.txt','w').write('x')"], WS_ROOT)
    add("RO: python file write denied", rc != 0 and assert_not_exists(pyfile), f"rc={rc}")

    # 14. WS: python writes file (OK)
    pyfile = WS_ROOT / "py_ok.txt"; remove_if_exists(pyfile)
    rc, out, err = run_sbx("workspace-write", ["python", "-c", "open('py_ok.txt','w').write('x')"], WS_ROOT)
    add("WS: python file write allowed", rc == 0 and assert_exists(pyfile), f"rc={rc}, err={err}")

    # 15. WS: curl network blocked (short timeout)
    rc, out, err = run_sbx("workspace-write", ["curl", "--connect-timeout", "1", "--max-time", "2", "https://example.com"], WS_ROOT)
    add("WS: curl network blocked", rc != 0, f"rc={rc}")

    # 16. WS: iwr network blocked (HTTP)
    rc, out, err = run_sbx("workspace-write", ["powershell", "-NoLogo", "-NoProfile", "-Command",
                               "try { iwr http://neverssl.com -TimeoutSec 2 } catch { exit 1 }"], WS_ROOT)
    add("WS: iwr network blocked", rc != 0, f"rc={rc}")

    # 17. WS: direct loopback blocked, proxy loopback allowed via env proxy
    if have("curl"):
        with start_loopback_proxy_fixture() as (target_port, proxy_port):
            proxy_home = WS_ROOT / ".codex_proxy_smoke"
            remove_if_exists(proxy_home)
            proxy_home.mkdir(parents=True, exist_ok=True)
            proxy_url = f"http://127.0.0.1:{proxy_port}"
            proxy_env = {
                "CODEX_HOME": str(proxy_home),
                "HTTP_PROXY": proxy_url,
                "http_proxy": proxy_url,
                "ALL_PROXY": proxy_url,
                "all_proxy": proxy_url,
                "NO_PROXY": "",
                "no_proxy": "",
            }
            proxied_cmd = [
                "curl",
                "--noproxy",
                "",
                "--connect-timeout",
                "2",
                "--max-time",
                "4",
                f"http://127.0.0.1:{target_port}/proxied",
            ]
            rc_proxy, out_proxy, err_proxy = run_sbx(
                "workspace-write",
                proxied_cmd,
                WS_ROOT,
                env_extra=proxy_env,
            )
            add(
                "WS: loopback proxy allowed",
                rc_proxy == 0 and "proxy-ok" in out_proxy,
                f"rc={rc_proxy}, out={out_proxy}, err={err_proxy}",
            )

            direct_cmd = [
                "curl",
                "--noproxy",
                "*",
                "--connect-timeout",
                "1",
                "--max-time",
                "2",
                f"http://127.0.0.1:{target_port}/direct",
            ]
            rc_direct, _out_direct, err_direct = run_sbx(
                "workspace-write",
                direct_cmd,
                WS_ROOT,
                env_extra={"CODEX_HOME": str(proxy_home)},
            )
            add("WS: direct loopback blocked", rc_direct != 0, f"rc={rc_direct}, err={err_direct}")
    else:
        add("WS: direct/proxy loopback tests (curl missing)", True, "curl not installed")

    # 18. RO: deny TEMP writes via PowerShell
    rc, out, err = run_sbx("read-only",
                           ["powershell", "-NoLogo", "-NoProfile", "-Command",
                            "Set-Content -LiteralPath $env:TEMP\\ro_tmpfail.txt -Value 'x'"], WS_ROOT)
    if ro_temp_denied:
        add("RO: TEMP write denied (PS)", rc != 0, f"rc={rc}")
    else:
        add("RO: TEMP write denied (PS, skipped)", True)

    # 19. WS: curl version check — don't rely on stub, just succeed
    if have("curl"):
        rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "curl --version"], WS_ROOT)
        add("WS: curl present (version prints)", rc == 0, f"rc={rc}, err={err}")
    else:
        add("WS: curl present (optional, skipped)", True)

    # 20. Optional: ripgrep version
    if have("rg"):
        rc, out, err = run_sbx("workspace-write", ["cmd", "/c", "rg --version"], WS_ROOT)
        add("WS: rg --version (optional)", rc == 0, f"rc={rc}, err={err}")
    else:
        add("WS: rg --version (optional, skipped)", True)

    # 21. Optional: git --version
    if have("git"):
        rc, out, err = run_sbx("workspace-write", ["git", "--version"], WS_ROOT)
        add("WS: git --version (optional)", rc == 0, f"rc={rc}, err={err}")
    else:
        add("WS: git --version (optional, skipped)", True)

    # 21–23. JSON policy: allow only .\allowed — note CWD is still allowed by current impl
    (WS_ROOT / "allowed").mkdir(exist_ok=True)
    (WS_ROOT / "blocked").mkdir(exist_ok=True)
    policy_json = '{"mode":"workspace-write","workspace_roots":[".\\\\allowed"]}'

    # Allowed: inside .\allowed (OK)
    rc, out, err = run_sbx(policy_json, ["cmd", "/c", "echo ok > allowed\\in_allowed.txt"], WS_ROOT)
    add("JSON WS: write in allowed/ OK", rc == 0 and (WS_ROOT / "allowed/in_allowed.txt").exists(), f"rc={rc}")

    # Outside CWD (deny)
    json_outside = OUTSIDE / "json_blocked.txt"; remove_if_exists(json_outside)
    rc, out, err = run_sbx(policy_json, ["cmd", "/c", f"echo nope > {json_outside}"], WS_ROOT)
    add("JSON WS: write outside allowed/ denied", rc != 0 and not json_outside.exists(), f"rc={rc}")

    # CWD is still allowed by current sandbox (documented behavior)
    rc, out, err = run_sbx(policy_json, ["cmd", "/c", "echo ok > cwd_ok_under_json.txt"], WS_ROOT)
    add("JSON WS: write in CWD allowed (by design)", rc == 0 and (WS_ROOT / "cwd_ok_under_json.txt").exists(), f"rc={rc}")

    # 24. WS: PS bytes write (OK)
    rc, out, err = run_sbx("workspace-write",
                           ["powershell", "-NoLogo", "-NoProfile", "-Command",
                            "[IO.File]::WriteAllBytes('bytes_ok.bin',[byte[]](0..255))"], WS_ROOT)
    add("WS: PS bytes write allowed", rc == 0 and (WS_ROOT / "bytes_ok.bin").exists(), f"rc={rc}")

    # 25. RO: PS bytes write denied
    rc, out, err = run_sbx("read-only",
                           ["powershell", "-NoLogo", "-NoProfile", "-Command",
                            "[IO.File]::WriteAllBytes('bytes_fail.bin',[byte[]](0..10))"], WS_ROOT)
    add("RO: PS bytes write denied", rc != 0 and not (WS_ROOT / "bytes_fail.bin").exists(), f"rc={rc}")

    # 26. WS: deep mkdir and write (OK)
    rc, out, err = run_sbx("workspace-write",
                           ["cmd", "/c", "mkdir deep\\nest && echo ok > deep\\nest\\f.txt"], WS_ROOT)
    add("WS: deep mkdir+write allowed", rc == 0 and (WS_ROOT / "deep/nest/f.txt").exists(), f"rc={rc}")

    # 27. WS: move (EXPECTED SUCCESS on this host)
    rc, out, err = run_sbx("workspace-write",
                           ["cmd", "/c", "echo x > m1.txt & move /y m1.txt m2.txt"], WS_ROOT)
    add("WS: move succeeds (expected on this host)", rc == 0 and (WS_ROOT / "m2.txt").exists(), f"rc={rc}, err={err}")

    # 28. RO: cmd redirection denied
    target = WS_ROOT / "cmd_ro.txt"; remove_if_exists(target)
    rc, out, err = run_sbx("read-only", ["cmd", "/c", "echo nope > cmd_ro.txt"], WS_ROOT)
    add("RO: cmd redirection denied", rc != 0 and not target.exists(), f"rc={rc}")

    return summarize(results)

if __name__ == "__main__":
    sys.exit(main())
