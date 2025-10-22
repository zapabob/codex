# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.48.0-zapabob.1] - 2025-10-23

### Added (zapabob独自機能)

#### 🔌 Gemini CLI MCP統合
- OAuth 2.0認証によるGoogle Gemini AI統合（APIキー不要）
- Google Search Grounding機能の活用
- 自動フォールバック: gemini-2.5-pro → gemini-2.5-flash
- MCP（Model Context Protocol）サーバー実装
- Windows互換性対応（`cmd /c` コマンド実行）
- レート制限ハンドリングと自動フォールバック

**実装ファイル**:
- `codex-rs/gemini-cli-mcp-server/` - 新規MCPサーバー
- `codex-rs/deep-research/src/gemini_search_provider.rs` - Gemini CLI統合
- `codex-rs/deep-research/src/web_search_provider.rs` - マルチプロバイダー統合

#### 🔔 Marisa音声通知システム
- タスク完了時の音声通知機能
- Marisaキャラクター音声（marisa_owattaze.wav）
- クロスプラットフォーム対応（Windows/macOS/Linux）
- config.tomlでのフック統合

**実装ファイル**:
- `zapabob/scripts/play-completion-sound.ps1` - 音声再生スクリプト
- `.codex/marisa_owattaze.wav` - 音声ファイル
- `config.toml` - フック設定

#### 🧠 自律オーケストレーション強化
- TaskAnalyzerによる自動複雑度判定（閾値0.7）
- AutoOrchestratorによる専門サブエージェントの自動委譲
- 並列エージェント実行による3倍高速化
- 7つの専門サブエージェント統合

**機能**:
- Code Expert: コード品質分析
- Security Expert: セキュリティレビュー
- Testing Expert: テスト生成
- Deep Researcher: 深層研究
- Docs Expert: ドキュメント生成
- Debug Expert: デバッグ支援
- Performance Expert: パフォーマンス最適化

#### 🔍 マルチソース研究エンジン
- 5つの検索プロバイダー統合
  - DuckDuckGo
  - Brave Search
  - Google Custom Search
  - Bing Search
  - Gemini CLI（新規）
- 引用管理システム
- 矛盾検出アルゴリズム
- 信頼性スコアリング（1-5段階）
- 設定可能な研究深度

**実装ファイル**:
- `codex-rs/deep-research/src/web_search_provider.rs` - マルチプロバイダー統合
- `codex-rs/deep-research/src/gemini_search_provider.rs` - Gemini CLI統合

#### 📄 GitHub PR Review自動化
- Codex CLI + Gemini CLI フォールバックワークフロー
- セキュリティレビュー専用ジョブ
- 自動設定スクリプト（PowerShell/Bash）
- pnpm高速インストール（4-5秒 vs 20-25秒）

**実装ファイル**:
- `.github/workflows/pr-review.yml` - Codex CLIワークフロー
- `.github/workflows/pr-review-gemini.yml` - Gemini CLIワークフロー
- `scripts/setup-pr-review.ps1` - PowerShell自動設定
- `scripts/setup-pr-review.sh` - Bash自動設定
- `scripts/README.md` - 設定ガイド

#### 📊 アーキテクチャドキュメント
- Mermaid形式の包括的アーキテクチャ図
- 8つの主要レイヤー、70+コンポーネント
- 複数サイズ対応（4K、SNS、Twitter、Instagram）

**実装ファイル**:
- `zapabob/docs/codex-architecture.mmd` - Mermaidソース
- `zapabob/docs/codex-v0.48.0-architecture.svg` - SVG版
- `zapabob/docs/codex-v0.48.0-architecture.png` - 4K PNG版
- `zapabob/docs/codex-v0.48.0-architecture-sns.png` - SNS用
- `zapabob/docs/codex-v0.48.0-architecture-twitter.png` - Twitter用
- `zapabob/docs/codex-v0.48.0-architecture-instagram.png` - Instagram用

#### 📝 包括的ドキュメント
- 日英両言語での完全ドキュメント
- 実装ログとベストプラクティス
- 設定ガイドとトラブルシューティング

**実装ファイル**:
- `README.md` - メインドキュメント（日英併記）
- `_docs/公式リポジトリとの整合性管理.md` - 整合性管理ガイド
- `_docs/GitHub_PR_Review_設定ガイド.md` - PR Review設定
- `_docs/MCP設定ファイル同期管理ガイド.md` - MCP設定同期

### Changed

#### ライセンス統一
- Apache 2.0への完全統一（デュアルライセンスから変更）
- 全ファイルへのライセンスヘッダー追加
- README.mdライセンスセクション更新

**更新ファイル**:
- `LICENSE` - Apache 2.0全文
- `README.md` - ライセンスセクション更新
- `scripts/setup-pr-review.ps1` - ライセンスヘッダー追加
- `scripts/setup-pr-review.sh` - ライセンスヘッダー追加
- `.github/workflows/pr-review.yml` - ライセンスヘッダー追加
- `.github/workflows/pr-review-gemini.yml` - ライセンスヘッダー追加

#### README.md大幅改訂
- アーキテクチャ概要セクション追加
- 主要アーキテクチャ特徴説明追加
- 日英両言語での完全対応
- ライセンス情報更新

### Fixed

#### Gemini CLI統合
- Windows互換性問題の修正（`cmd /c` 使用）
- レート制限エラーハンドリング改善
- 空結果時のフォールバック実装
- MCP統合時の型エラー修正

#### Deep Research Engine
- Gemini CLI優先順位の実装
- フォールバック機能の強化
- エラーログの改善

### Security

- GitHub PR Review自動化によるセキュリティチェック強化
- セキュリティレビュー専用ジョブの追加
- 自動脆弱性スキャン実装

### Documentation

- 包括的な実装ログ作成
- 日英両言語ドキュメント整備
- トラブルシューティングガイド追加
- アーキテクチャ図の作成

### Performance

- 並列エージェント実行による3倍高速化
- pnpm使用による高速インストール
- Gemini CLI統合による検索高速化

## From Upstream (OpenAI/codex)

### Based on OpenAI/codex v0.48.0
- コア機能の継承
- CLIインターフェース
- サブエージェントシステム
- 基本的な研究機能

## [Unreleased]

### Planned Features

#### 短期（1-3ヶ月）
- [ ] 公式v0.49.0同期
- [ ] 追加LLMプロバイダー統合
- [ ] Web UI改善

#### 中期（3-6ヶ月）
- [ ] VSCode拡張機能開発
- [ ] クラウド版提供
- [ ] エンタープライズ機能

#### 長期（6-12ヶ月）
- [ ] プラグインシステム
- [ ] マルチテナント対応
- [ ] AI エージェントマーケットプレイス

---

## Version Format

```
[MAJOR.MINOR.PATCH-zapabob.BUILD]

MAJOR.MINOR.PATCH: 公式ベースバージョン
zapabob.BUILD: zapabob独自ビルド番号
```

### Example
- `0.48.0`: 公式ベースバージョン
- `0.48.0-zapabob.1`: zapabob独自ビルド1

---

**Maintained by**: [zapabob](https://github.com/zapabob)  
**Based on**: [OpenAI/codex](https://github.com/openai/codex)  
**License**: Apache 2.0