# Release Notes Archive — v2.x Release Line

This archive preserves the prior root release notes for the 2.x line.
For the current release line, see [`../../RELEASE_NOTES.md`](../../RELEASE_NOTES.md).

---

# v2.13.0 Release Notes

## 🌟 Highlights

This release focuses on **GUI enhancements** and **System Integration**, bridging the gap between the web interface and the underlying specific command-line tools.

- **GUI Dashboard with Real-Time Metrics**: Monitor CPU, RAM, and GPU usage in real-time via the new Node.js backend (`gui/server.js`).
- **Collapsible Sidebar**: Improved screen real estate management with a new collapsible sidebar component.
- **CLI Bridge**: Execute CLI commands directly from the GUI, enabling a seamless workflow between visual and terminal operations.
- **SSR Fixes**: Resolved Next.js Server-Side Rendering issues for a smoother user experience.

## 🇯🇵 日本語リリースノート

本リリースでは、GUIの強化とシステム統合に焦点を当てています。

- **リアルタイムメトリクス**: Node.jsバックエンドにより、CPU/メモリ/GPUの使用率をGUI上でリアルタイム監視可能になりました。
- **サイドバー改善**: 折りたたみ可能なサイドバーを実装し、作業領域を広く使えるようになりました。
- **CLI連携**: GUIから直接CLIコマンドを実行できるブリッジ機能を追加しました。
- **SSR修正**: Next.jsのServer-Side Renderingに関する問題を修正し、安定性を向上させました。

## 🛡️ Security

- **Updated Dependencies**: Bumped `sysinfo`, `ws`, `cors` and other core dependencies.
- **Pre-commit Checks**: Passed rigorous Clippy and Large File checks.

## 📦 Changes

- feat(gui): Implement collapsible sidebar, real metrics server, and CLI integration
- fix(gui): Resolve window is not defined SSR error
- chore: Update workspace versions to v2.13.0
- doc: Rewrite README.md for better recruitment appeal (Bilingual)
