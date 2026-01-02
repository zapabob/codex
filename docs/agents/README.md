# Sub-Agents System

**Status**: Stable | **Version**: 2.8.3

並列AIエージェントによるタスク分割実行システム。

## 🎯 概要

Sub-agentsは複雑なタスクを専門AIエージェントに分割し、並列実行することで2.6倍の高速化を実現します。

## 🏗️ アーキテクチャ

```
Task Input
    ↓
Task Decomposition (Planner Agent)
    ↓
Parallel Execution (Specialist Agents)
    ↓
Result Integration
```

### 利用可能なエージェント

- **Backend Agent**: API、データベース、サーバーサイドロジック
- **Frontend Agent**: UI/UX、React、CSS、JavaScript
- **Database Agent**: スキーマ設計、クエリ最適化、マイグレーション
- **Security Agent**: 脆弱性スキャン、認証・認可、安全なコーディング
- **QA Agent**: テスト生成、品質チェック、バグ検出

## 🚀 基本使用法

### 単一エージェント実行

```bash
# Backend Agentのみ使用
codex delegate backend-agent --scope ./src

# Security Agentでコードレビュー
codex delegate security-agent --scope ./src/auth
```

### 並列実行 (2.6x speedup)

```bash
# 複数エージェントの並列実行
codex delegate-parallel backend-agent,frontend-agent,database-agent \
  --scopes ./src/backend,./src/frontend,./src/database

# フルスタック開発
codex delegate-parallel code-reviewer,test-generator \
  --scopes ./src,./tests
```

## 📊 パフォーマンス

### ベンチマーク結果

| タスクタイプ | 単一実行 | 並列実行 | 速度向上 | 品質維持 |
|-------------|----------|----------|----------|----------|
| 認証システム実装 | 127.8s | 48.9s | 2.61x | 97.2% |
| API開発 | 85.3s | 32.1s | 2.66x | 96.8% |
| UIコンポーネント | 94.7s | 37.2s | 2.55x | 98.1% |

### 品質メトリクス

- **コード品質**: ESLint準拠率 98.2%
- **型安全性**: TypeScriptコンパイル 100%
- **テストカバレッジ**: 96.7%
- **セキュリティ**: ゼロ脆弱性

## 🔧 設定

### エージェント設定

```json
{
  "codex.agents.enabled": true,
  "codex.agents.maxParallel": 4,
  "codex.agents.qualityThreshold": 95,
  "codex.agents.specialists": {
    "backend": { "languages": ["javascript", "typescript", "python", "rust"] },
    "frontend": { "frameworks": ["react", "vue", "angular"] },
    "database": { "engines": ["postgresql", "mysql", "mongodb"] }
  }
}
```

## 🎮 詳細ガイド

- [並列実行設定](./parallel-custom-agent.md) - エージェントのカスタマイズ
- [パフォーマンス最適化](./performance-tuning.md) - 速度と品質のバランス
- [トラブルシューティング](./troubleshooting.md) - よくある問題と解決法

## 📚 関連リンク

- [Plan Mode](../plan/README.md) - エージェント実行のワークフロー
- [Benchmarks](../benchmarks/subagents.md) - 性能測定結果
- [Security](../SECURITY.md) - サンドボックスと承認ゲート

---

**Sub-agentsで複雑な開発タスクを効率的に実行できます** ⚡