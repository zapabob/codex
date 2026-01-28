## System（どこを見ると実装が追えるか）

### 俯瞰図

- `ARCHITECTURE.md`

### 主要コンポーネントの入口

- **Rustコア**: `codex-rs/`  
  - CLI/TUIやコア機能、拡張基盤の中心
- **Node.js CLI**: `codex-cli/`  
  - npm配布やCLIまわりの実装
- **GUI**: `gui/`  
  - UI/可視化、フロント実装
- **IDE拡張**: `extensions/`  
  - VS Codeなどの拡張
- **CI/CD**: `.github/workflows/`

### “設計と実装の距離”が短い証拠

アーキテクチャ/機能説明が、具体的な再現手順（Quickstart）とベンチ（測定）に直結しています。

- `docs/plan/README.md`
- `docs/benchmarks/README.md`

