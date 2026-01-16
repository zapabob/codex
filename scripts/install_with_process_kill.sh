#!/bin/bash
# プロセスキル付きバイナリ上書きインストールスクリプト
# クロスプラットフォーム対応（Linux/macOS）

set -euo pipefail

# 設定
KILL_TIMEOUT="${KILL_TIMEOUT:-30}"
FORCE_INSTALL="${FORCE_INSTALL:-true}"
BINARY_NAME="${BINARY_NAME:-codex}"
INSTALL_PATH="${INSTALL_PATH:-$HOME/.cargo/bin}"

# ログ関数
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" >&2
}

error() {
    log "ERROR: $*" >&2
    exit 1
}

warning() {
    log "WARNING: $*" >&2
}

info() {
    log "INFO: $*" >&2
}

# プラットフォーム検知
detect_platform() {
    case "$(uname -s)" in
        Linux*)     PLATFORM="linux";;
        Darwin*)    PLATFORM="macos";;
        *)          error "Unsupported platform: $(uname -s)";;
    esac

    case "$(uname -m)" in
        x86_64)     ARCH="x86_64";;
        aarch64)    ARCH="aarch64";;
        arm64)      ARCH="aarch64";;
        *)          error "Unsupported architecture: $(uname -m)";;
    esac

    info "Platform: $PLATFORM, Architecture: $ARCH"
}

# プロセス検知関数
find_processes() {
    local pattern="$1"
    case "$PLATFORM" in
        linux|macos)
            # pgrepを使ってプロセス検索
            if command -v pgrep >/dev/null 2>&1; then
                pgrep -f "$pattern" || true
            else
                # pgrepがない場合、psとgrepを使用
                ps aux | grep -F "$pattern" | grep -v grep | awk '{print $2}' || true
            fi
            ;;
        *)
            error "Unsupported platform for process detection"
            ;;
    esac
}

# プロセス終了関数
kill_processes() {
    local pids="$1"
    local signal="${2:-TERM}"
    local timeout="${3:-$KILL_TIMEOUT}"

    if [ -z "$pids" ]; then
        info "No processes to kill"
        return 0
    fi

    info "Killing processes with SIG$signal: $pids"

    # シグナル送信
    for pid in $pids; do
        if kill -"$signal" "$pid" 2>/dev/null; then
            info "Sent SIG$signal to process $pid"
        else
            warning "Failed to send SIG$signal to process $pid"
        fi
    done

    # プロセス終了待機
    local start_time=$(date +%s)
    while [ $(($(date +%s) - start_time)) -lt "$timeout" ]; do
        local remaining=""
        for pid in $pids; do
            if kill -0 "$pid" 2>/dev/null; then
                remaining="$remaining $pid"
            fi
        done

        if [ -z "$remaining" ]; then
            info "All processes terminated successfully"
            return 0
        fi

        sleep 0.5
    done

    # タイムアウト後、SIGKILLで強制終了
    warning "Timeout reached, sending SIGKILL to remaining processes: $remaining"
    for pid in $remaining; do
        if kill -KILL "$pid" 2>/dev/null; then
            info "Sent SIGKILL to process $pid"
        fi
    done

    # 最終確認
    sleep 2
    local still_running=""
    for pid in $pids; do
        if kill -0 "$pid" 2>/dev/null; then
            still_running="$still_running $pid"
        fi
    done

    if [ -n "$still_running" ]; then
        error "Failed to kill processes: $still_running"
    else
        info "All processes terminated with SIGKILL"
    fi
}

# 実行中プロセス終了
terminate_running_processes() {
    info "Checking for running $BINARY_NAME processes..."

    local pids
    pids=$(find_processes "$BINARY_NAME")

    if [ -n "$pids" ]; then
        info "Found running processes: $pids"
        kill_processes "$pids" "TERM" "$KILL_TIMEOUT"

        # ダブルチェック
        pids=$(find_processes "$BINARY_NAME")
        if [ -n "$pids" ]; then
            error "Processes still running after termination attempt: $pids"
        fi
    else
        info "No running processes found"
    fi
}

# バックアップ作成
create_backup() {
    local binary_path="$INSTALL_PATH/$BINARY_NAME"

    if [ -f "$binary_path" ]; then
        local backup_path="${binary_path}.backup.$(date +%Y%m%d_%H%M%S)"
        info "Creating backup: $backup_path"
        cp "$binary_path" "$backup_path"

        # 古いバックアップ削除（最新5個のみ保持）
        ls -t "${binary_path}.backup."* 2>/dev/null | tail -n +6 | xargs rm -f 2>/dev/null || true
    fi
}

# インストール実行
perform_installation() {
    local release_flag=""
    if [ "${RELEASE:-false}" = "true" ]; then
        release_flag="--release"
        info "Installing release build"
    else
        info "Installing debug build"
    fi

    local install_cmd="cargo install --path cli $release_flag --force --root ~/.cargo"

    info "Executing: $install_cmd"
    if eval "$install_cmd"; then
        info "Installation completed successfully"
    else
        error "Installation failed with exit code $?"
    fi
}

# インストール検証
verify_installation() {
    local binary_path="$INSTALL_PATH/$BINARY_NAME"

    if [ ! -f "$binary_path" ]; then
        error "Binary not found after installation: $binary_path"
    fi

    if [ ! -x "$binary_path" ]; then
        error "Binary is not executable: $binary_path"
    fi

    # バージョン確認
    if "$binary_path" --version >/dev/null 2>&1; then
        local version
        version=$("$binary_path" --version 2>/dev/null | head -n1)
        info "Installation verified: $version"
    else
        error "Binary execution failed"
    fi
}

# メイン処理
main() {
    info "Starting process-kill installation for $BINARY_NAME"
    info "Kill timeout: ${KILL_TIMEOUT}s"
    info "Force install: $FORCE_INSTALL"
    info "Install path: $INSTALL_PATH"

    # プラットフォーム検知
    detect_platform

    # 実行中プロセス終了
    terminate_running_processes

    # バックアップ作成
    if [ "$FORCE_INSTALL" = "true" ]; then
        create_backup
    fi

    # インストール実行
    perform_installation

    # インストール検証
    verify_installation

    info "Process-kill installation completed successfully! ✅"
}

# スクリプト実行
main "$@"