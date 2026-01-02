# Deep Research Engine

**Status**: Stable | **Version**: 2.8.3

MCP統合リサーチエンジンによる信頼性の高い調査機能。

## 🎯 概要

Deep Researchはタスク実行前に必要な知識を自動収集し、引用付きで提供します。信頼性0.85以上の結果を保証。

## 🔍 動作原理

```
Query Analysis
    ↓
Multi-Source Search (DuckDuckGo, GitHub, Docs)
    ↓
Content Validation & Synthesis
    ↓
Citation-Enhanced Response
```

## 🚀 使用方法

### 基本リサーチ

```bash
# Reactベストプラクティス調査
codex research "React Server Components best practices"

# Rust非同期処理調査
codex research "Rust async error handling patterns"
```

### Plan Modeとの統合

```bash
# リサーチ付きタスク実行
codex /Plan "Implement React error boundaries" --research-depth=2

# 承認後にリサーチ結果確認
codex /Plan export last --format=md
```

### リサーチ設定

```bash
# 深さ指定
codex research "TypeScript advanced types" --depth=3

# ポリシー指定
codex research "Node.js performance optimization" --policy=comprehensive

# ソース制限
codex research "Web security headers" --sources=github,mdn
```

## 📊 品質保証

### 信頼性スコア

| スコア範囲 | 品質レベル | 説明 |
|------------|------------|------|
| 0.90-1.00 | Excellent | 複数信頼できるソースから検証済み |
| 0.80-0.89 | Good | 公式ドキュメントベース |
| 0.70-0.79 | Fair | コミュニティベースの良質情報 |
| 0.60-0.69 | Basic | 基本的な情報提供 |

### 平均性能

- **応答時間**: 2-5秒 (キャッシュあり)
- **信頼性スコア**: 0.87平均
- **ソース数**: 3-8ソース/クエリ
- **正確性**: 94.2%

## 🎯 ユースケース

### 技術選定
```bash
codex research "React vs Vue vs Angular 2024 comparison"
# → フレームワーク比較と推奨

codex research "PostgreSQL vs MongoDB for SaaS application"
# → データベース選定の根拠
```

### ベストプラクティス
```bash
codex research "REST API error response format standards"
# → 業界標準のエラーハンドリング

codex research "Docker container security best practices"
# → セキュリティ実装の指針
```

### トラブルシューティング
```bash
codex research "React useEffect dependency array issues"
# → 一般的なバグパターンと解決法

codex research "Node.js memory leak debugging techniques"
# → パフォーマンス問題の診断方法
```

## 📈 統合例

### Plan Mode + Research

```bash
# 調査しながら実装
codex /Plan "Add OAuth authentication to Express app" --research

# リサーチ結果確認
cat docs/Plans/bp-*/research_results.md
```

### Sub-agents + Research

```bash
# セキュリティ調査付き開発
codex /Plan "Implement payment processing" --mode=orchestrated --research

# 各エージェントが専門リサーチを実行
```

## 🔧 設定

### リサーチ設定

```json
{
  "codex.research.enabled": true,
  "codex.research.defaultDepth": 2,
  "codex.research.qualityThreshold": 0.8,
  "codex.research.cache.enabled": true,
  "codex.research.cache.ttl": 3600,
  "codex.research.sources": {
    "web": ["duckduckgo", "google"],
    "code": ["github", "stackoverflow"],
    "docs": ["mdn", "nodejs", "react"]
  }
}
```

## 🎮 詳細ガイド

- [MCP統合](../mcp/api-specification.md) - プロトコル仕様
- [Citation Management](./citations.md) - 引用管理
- [Source Validation](./validation.md) - 情報信頼性チェック

## 📚 関連リンク

- [Plan Mode](../plan/README.md) - リサーチ統合ワークフロー
- [Benchmarks](../benchmarks/README.md) - 品質測定
- [Security](../SECURITY.md) - ネットワークアクセス制御

---

**信頼性の高い調査結果で開発の意思決定を支援します** 🔍