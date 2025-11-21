# QC Optimizer サブエージェント & 実装ログ自動化ガイド

zapabob/codex の中央集権型 AI オーケストレーションおよび Git worktree 並列開発において、メインマージ直前の品質保証を一貫化するための運用仕様です。

## 1. QC Optimizer サブエージェントの役割

| 項目 | 内容 |
| --- | --- |
| エージェント名 | `qc-optimizer` |
| 主目的 | 単体/ユニット/異常系/包括的テストを実施し、統計・QC 観点でコード品質とアーキテクチャを評価。量子/数理最適化の観点でリスクを最小化し、メインマージ判断を可視化する。 |
| 主要アウトプット | 1) テストスイート実行結果、2) 問題点の可視化レポート、3) QC/統計/量子最適化の結論、4) `_doc/*.md` に残す実装ログ |
| 主なツール | `cargo`, `npm`, `just`, `git`, `scripts/create-implementation-log.js`, Codex MCP ファイル/検索ツール |

## 2. オーケストレーション手順

1. **タスク解析**: Plan/TaskAnalyzer が worktree 単位にタスクを分割し、複雑度 0.7 以上の案件は自動的に `qc-optimizer` を含む並列フローに昇格。
2. **開発サブエージェントの作業**: executor/refactorer などがコード変更を行い、PR 前に `qc-optimizer` に制御を移譲。
3. **テストマトリクス実行**:
   - `cargo test -p <crate>` / `cargo test --all-features`（必要に応じて）
   - `npm test` / `pnpm test`（GUI, website, prism-web 等）
   - `just fmt`/`just fix -p <project>` による Lint 修復
   - シナリオ/異常系テスト、snapshot テスト
4. **統計・量子最適化評価**:
   - 失敗率、カバレッジ、実行時間、リソース消費を統計的に評価。
   - 分岐/コンポーネントごとの欠陥率を重み付き線形計画として評価し、改善優先度を算出。
   - 量子インスパイア手法（QAOA など）を模倣したヒューリスティクスを利用し、リスク閾値を決定。
5. **判断ログ化**: `qc-optimizer` が `scripts/create-implementation-log.js` を使い、テスト結果と統計/最適化判断を `_doc/<date>-<worktree>実装ログ.md` に残す。
6. **可視化レポート**: MCP tools で `docs/` 以下に短い差分サマリを出力し、問題点を赤字でハイライト。
7. **メインマージ判定**: ログに記録した最終判断理由を元に、Budgeter/AutoOrchestrator が merge/push を実行する。

## 3. `_doc` 自動実装ログフロー

### 3.1 ディレクトリ構成

```
/docroot
├── _doc/                     # 自動生成ログ
│   └── YYYY-MM-DD-<worktree>実装ログ.md
├── _docs/                    # 既存の詳細実装ログ
└── scripts/create-implementation-log.js
```

### 3.2 ログに必須の項目

- 日時（UTC とローカル）
- ワークツリー名 / ブランチ名
- 担当エージェント & AI 名
- 機能実装概要 / 動作確認結果
- Git 変更サマリ
- テスト結果（単体、ユニット、異常系、包括テスト）
- QC 結論、統計学的有意差、量子/数理最適化メモ、最終判断理由

### 3.3 生成コマンド例

```bash
npm run implementation-log -- \
  --worktree main \
  --functionality "Deep QC orchestrator + log auto save" \
  --verification "cargo test -p codex-core / npm test (website)" \
  --agent qc-optimizer \
  --ai gpt-5-codex \
  --prompt "AIオーケストレーションQC指示" \
  --tests "cargo test ✅ / npm test ✅" \
  --qc "全テスト成功。エラー率0%" \
  --stats "p<0.05 で回帰差異なし" \
  --optimization "QUBOでテスト優先度算出" \
  --decision "メインマージ許可" \
  --notes "実装ログをworktreeチームに共有"
```

> `git status --short` が自動でログに埋め込まれるため、ワークツリーごとの差分が可視化されます。

## 4. メインマージ前のチェックリスト

1. **実装ログ確認**: 最新の `_doc/*.md` を参照し、連続性と QC 結果を確認。
2. **統計手法の適用**: 変更が閾値を超えた場合は、追加の統計テスト (例: McNemar, t-test) を実行。
3. **量子/数理最適化ノート**: 修正の優先度を QUBO/線形計画の形に落とし込み、ペアリングするタスクを提示。
4. **可視化レポート添付**: `docs/` 内のサマリリンクを PR 説明欄に必ず添付。
5. **Budgeter 承認**: コスト/トークン使用量を Budgeter が承認した後に `qc-optimizer` が `merge` を許可。

## 5. 運用 Tips

- `TZ` を設定すると `scripts/create-implementation-log.js` のローカルタイム出力が各ワークツリー地域に揃います。
- Git worktree 名は自動的にサニタイズされますが、`feature/xxx` のようなパスは `feature-xxx` へ変換される点に注意。
- 追加のメトリクス（コードカバレッジ、SLO 逸脱率など）が必要な場合は、`--notes` に JSON 形式で追記し、Downstream の解析エージェントが機械的に読み取れるようにしてください。
- `_doc/README.md` に本仕様の概要を記載済みです。サブエージェントは実装前に README と最新ログを必ず参照してください。

このガイドと `qc-optimizer` エージェントを組み合わせることで、メインマージ直前の QC 判定と実装ログの自動化が統合され、zapabob/codex 独自の AI オーケストレーションフローに適合します。
