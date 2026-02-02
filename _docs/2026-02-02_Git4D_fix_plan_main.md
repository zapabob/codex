# Git4D 可視化起動対応メモ（main）

- 日付: 2026-02-02
- ブランチ/ワークツリー: main

## 実施内容
- Skill ローダー互換性向上: 22 個の SKILL.md に YAML フロントマターを自動付与。
- Playwright MCP を 
px @playwright/mcp@latest 起動型に変更し、
px --help でバイナリ取得を確認。
- logs/automation-log.md に作業ログ追記。
- codex-gui 起動トラブルシュート: cargo run -p codex-gui 実行→ CUDA/Git4D まわりの型制約不足でビルド失敗。
- 既存バイナリ codex-gui.exe --port 8787 も起動試行したがポート未リッスン。

## 現状のブロッカー（要修正）
1. core/src/git4d_accelerated.rs
   - RenderParameters 生成時に ranch_filter_count 欠落。
   - ranch_filter は [u32; 32] へ Vec 経由で 	ry_into する必要あり。
2. core/src/cuda_accelerator.rs
   - GitCommitVertex, RenderParameters, [[f32;4];4], [f32;3] 等に DeviceRepr / ValidAsZeroBits 未実装。
   - CudaDevice::launch 呼び出しは cudarc の unction().launch(...) 形への修正が必要。
3. core/src/qc/mathematical.rs
   - Result<Self> → Result<Self, E> などジェネリック指定不足。

## 次回アクション候補
- 上記 3 点を修正後、cargo run -p codex-gui を再試行し、http://localhost:8787/api/health を確認。
- 成功後、Git4D 可視化コマンドの再実行とログ取得。

## コミット
- ix: add skill frontmatter and update playwright mcp (3badf045e)
- chore: log codex-gui bringup attempt (de97e7a6c)

