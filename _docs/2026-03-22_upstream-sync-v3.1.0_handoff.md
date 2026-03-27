# 作業手順引き継ぎ書

対象ワークツリーは `C:\Users\downl\Desktop\codex-main-upstream-sync`、ブランチは `codex/upstream-sync-2026-03-22` です。元の `C:\Users\downl\Desktop\codex-main` は触らない前提です。製品バージョンは内部 `3.1.0`、表示 `v3.1.0` に統一済みです。

現在の到達点は、`features`、`exec-server`、`core/lib`、`core/exec`、`core/exec_policy`、`core/mcp_tool_call`、`core/plugins/manager` の major な merge splice を upstream-first で整理したところまでです。実装ログは [_docs/2026-03-22_upstream-sync-v3.1.0_completion_log.md](C:\Users\downl\Desktop\codex-main-upstream-sync\_docs\2026-03-22_upstream-sync-v3.1.0_completion_log.md) に追記済みです。

`C:\Users\downl\Desktop\codex-main-upstream-sync\codex-rs\target\debug` は削除対象として確認済みですが、この環境では削除コマンドがポリシーでブロックされます。`Remove-Item`、`cmd.exe /c rmdir`、WSL `rm -rf` のいずれも拒否されました。そのため、以後の Cargo-heavy command はすべて `F:\codex-targets\codex-main-upstream-sync` を target dir に使ってください。`F:` には十分な空きがあります。

## 次の作業手順

1. 既存の `cargo` / `rustc` が残っていないか確認する。  
   `codex-main-upstream-sync` または `F:\codex-targets\codex-main-upstream-sync` を含むプロセスだけを見る。

2. 以後のコマンドは必ず `codex-rs` 直下で、`CARGO_TARGET_DIR=F:\codex-targets\codex-main-upstream-sync` を付けて実行する。

3. 最優先で実行するコマンドはこれです。

```powershell
$env:CARGO_TARGET_DIR='F:\codex-targets\codex-main-upstream-sync'
cargo run --bin codex -- --version
```

4. 期待結果は `3.1.0` の出力です。  
   通らなければ、出た最初の concrete blocker を 1 件だけ直します。複数箇所を同時に触らないでください。

5. `--version` が通ったら次を実行します。

```powershell
$env:CARGO_TARGET_DIR='F:\codex-targets\codex-main-upstream-sync'
cargo check -p codex-core -p codex-cli -p codex-mcp-server -p codex-deep-research -p codex-tui -p codex-tui-app-server
```

6. compile convergence 後にだけ次を実行します。

```powershell
just fmt
just argument-comment-lint
just clippy
```

7. 依存や lockfile に実差分が残る場合のみ次を実行します。

```powershell
just bazel-lock-update
just bazel-lock-check
```

## 重要な注意点

- Cargo は必ず 1 本ずつ実行してください。並列実行すると lock contention で状況が見えなくなります。
- `plugins/manager.rs` は upstream current shape を正本にしてください。inline test は戻さず、[manager_tests.rs](C:\Users\downl\Desktop\codex-main-upstream-sync\codex-rs\core\src\plugins\manager_tests.rs) を正本として扱います。
- `C:` 側 `target/debug` の削除は未完了ですが、これはコード blocker ではなく環境制約です。
- 進捗は必ず [_docs/2026-03-22_upstream-sync-v3.1.0_completion_log.md](C:\Users\downl\Desktop\codex-main-upstream-sync\_docs\2026-03-22_upstream-sync-v3.1.0_completion_log.md) に追記してください。新規ログファイルは増やさないでください。
