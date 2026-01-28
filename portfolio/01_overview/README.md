## Overview（何を作って、何が強みか）

### 目的

このリポジトリは、AI開発を「安全に・速く・再現可能に」回すための基盤（CLI/GUI/拡張/ドキュメント/ベンチ）を統合したものです。

### 3つの強み（採用担当者が評価しやすい軸）

- **再現性**: “動く手順”と“測り方”がドキュメント化されている  
  - `docs/plan/README.md` / `docs/benchmarks/README.md`
- **品質**: 安全ゲート（承認）+ 自動化（テスト/型/リンタ）の思想が見える  
  - `docs/plan/README.md`
- **スケール**: 並列サブエージェント、GPU加速、GUIまで射程が広い  
  - `docs/benchmarks/subagents.md` / `docs/benchmarks/cuda.md` / `gui/`

### 公式互換と拡張の整理

` .github/REPOSITORY_STRUCTURE.md ` に、公式互換構造と拡張領域の切り分けがまとまっています。

