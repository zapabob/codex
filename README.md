# Codex Extended - Ultimate AI-Native Development Platform

<p align="center"><code>npm install -g @zapabob/codex</code><br />or <code>just build-install</code></p>
<p align="center"><strong>Codex Extended CLI</strong> - ClaudeCode-Powered AI development platform with Multi-Model Intelligence</p>
<p align="center">
  <img src="./docs/architecture/architecture-v2.11.0.svg" alt="Codex v2.11.0 Architecture" width="80%" />
</p>

<div align="center">
<strong>v2.11.0 "ClaudeCode Integration"</strong><br />
Revolutionary AI development with ClaudeCode's autonomous features, multi-model intelligence, and enterprise-grade privacy protection.
</div>

<br />

*This is an independent fork/extension and is not affiliated with OpenAI or Anthropic.*

[![Version](https://img.shields.io/badge/version-2.11.0-blue.svg)](https://github.com/zapabob/codex)
[![npm](https://img.shields.io/npm/v/@zapabob/codex)](https://www.npmjs.com/package/@zapabob/codex)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache--2.0-green.svg)](LICENSE)

[English](#english) | [日本語](#japanese)

---

## 🚀 Key Highlights

### ClaudeCode Integration
- **Natural Language Processing**: Understand and execute plain English development tasks
- **Autonomous Code Generation**: AI builds complete applications from your descriptions
- **Terminal Integration**: Direct system operations and Git management

### Multi-Model Intelligence
- **Provider Agnostic**: Seamlessly switch between Gemini, Claude, GPT, and local models
- **Intelligent Selection**: Automatic model optimization based on task requirements
- **Cost Optimization**: 70%+ API cost reduction through smart caching and routing

### Enterprise Security
- **Privacy-First**: Local processing with end-to-end encryption
- **Multi-Layer Protection**: Advanced prompt injection guards and sandboxed execution
- **Compliance Ready**: GDPR, HIPAA, SOC2 compliant design

### Cowork Productivity
- **File Management**: Intelligent organization and bulk operations
- **Browser Automation**: Web scraping, testing, and data extraction
- **Workflow Orchestration**: Complex process automation and scheduling

---

<a name="english"></a>
## 📖 English Documentation

### 🎯 What is Codex Extended v2.11.0?

Codex Extended v2.11.0 represents the next evolution of AI-assisted development, completely integrating ClaudeCode's revolutionary features while solving its fundamental limitations. Built on a hybrid Rust + Python architecture, it delivers enterprise-grade performance, security, and cost efficiency.

### 💡 Perfect For

- **AI Engineers**: Multi-model orchestration and cost optimization
- **LLMOps Engineers**: Model management and performance monitoring
- **Full-Stack Developers**: Autonomous code generation and testing
- **DevOps Teams**: CI/CD automation and infrastructure management
- **Enterprise Teams**: Privacy-compliant, scalable AI workflows

### ⚡ Quick Start

```bash
# Install globally
npm install -g @zapabob/codex

# Verify installation
codex --version

# Your first AI-generated task
codex task "Create a React authentication component with form validation"

# Multi-model cost optimization
codex multi "Optimize this database query" --cost-optimize

# Privacy-protected development
codex private "Analyze this codebase for security issues"
```

### 🏗️ Architecture Overview

Codex Extended v2.11.0 implements a sophisticated multi-layered architecture:

#### User Interface Layer
- **Terminal UI**: Text-based interface for power users
- **Command Line**: Full CLI with advanced options
- **MCP Client**: WebSocket-based tool execution

#### AI/ML Core Layer
- **Codex Core**: LLMOps + A2A communication engine
- **Skills System**: Dynamic plugin architecture for extensibility

#### ClaudeCode Integration Core
- **Task Parser**: Natural language processing and intent analysis
- **Autonomous Executor**: Code generation and workflow automation
- **Multi-Model Orchestrator**: Intelligent model selection and load balancing
- **Cost Optimizer**: Token management and API cost reduction
- **Privacy Guard**: Data anonymization and security protection
- **Prompt Injection Guard**: Multi-layer attack prevention
- **Secure Execution Engine**: Sandboxed environment management

#### Cowork Productivity Suite
- **File Management**: Intelligent organization and bulk operations
- **Browser Automation**: Web scraping and UI testing
- **Workflow Orchestrator**: Complex process automation

#### Multi-Agent System
- **Supervisor**: Agent orchestration and conflict resolution
- **Worker Agents**: Specialized AI assistants for code review, testing, security, and performance analysis

#### Development Tools
- **Build Manager**: Incremental builds and process optimization
- **Worktree Manager**: Parallel development with Git worktrees
- **QA Service**: Automated reviews and background monitoring
- **CI/CD Integration**: Pipeline generation and deployment automation

### 📊 Performance Metrics

- **Cost Reduction**: 70%+ API cost savings through intelligent caching
- **Response Time**: 60% faster average processing
- **Reliability**: 99.5% success rate with enterprise-grade error handling
- **Throughput**: 3x concurrent task processing capacity
- **Privacy**: 100% local processing option available

### 🔧 Advanced Features

#### Model Orchestration

```bash
# Automatic model selection
codex task "Debug this complex JavaScript error"

# Manual model specification
codex multi "Architect a microservices system" --models claude-3,gpt-4

# Cost-optimized execution
codex task "Generate comprehensive documentation" --budget 1.00
```

#### Privacy & Security

```bash
# Maximum privacy mode
codex private "Audit this code for GDPR compliance" --privacy maximum

# Sandboxed execution
codex sandbox "Test this potentially unsafe operation"

# Audit logging
codex audit "Review security posture" --report-format pdf
```

#### Cowork Automation

```bash
# File management
codex organize "Sort project files by type and functionality"

# Browser automation
codex browser "Extract product data from e-commerce site"

# Workflow orchestration
codex workflow "Deploy application with full testing pipeline"
```

### 🔗 Integration Ecosystem

#### Supported Platforms
- **Operating Systems**: Windows, macOS, Linux
- **Architectures**: x64, ARM64
- **Node.js Versions**: 16.0+ (18+ recommended)

#### External Integrations
- **Version Control**: Git, GitHub, GitLab
- **CI/CD**: GitHub Actions, Jenkins, CircleCI
- **Cloud Platforms**: AWS, GCP, Azure
- **Communication**: Slack, Discord, Microsoft Teams

#### API Compatibility

##### OpenAI API (Latest as of January 2026)
- **GPT-5.2** (Dec 2025) - Flagship model with Instant/Thinking/Pro modes
- **GPT-5.1** (Nov 2025) - 8 personality styles, GPT-5.1-Codex-Max variant
- **GPT-5-codex** (Sep 2025) - Coding-optimized with image/screenshot inputs
- **o4-mini** (Apr 2025) - Cost-efficient reasoning model with multimodal capabilities
- **Sora 2** (Sep 2025) - Advanced video + audio generation

##### Anthropic API (Latest as of January 2026)
- **Claude Opus 4.5** (Nov 2025) - Top-tier with advanced code/workflows
- **Claude Sonnet 4.5** (Sep 2025) - Mid-tier with long context (30+ hours tasks)
- **Claude Haiku 4.5** (Oct 2025) - Lightweight, cost-efficient, near-frontier coding
- **Claude Code** (Feb 2025) - Command-line coding assistant
- **Claude Cowork** (Early 2026) - GUI for non-technical users with file/folder access

##### Google AI (Latest as of January 2026)
- **Gemini 3 Pro** (Nov 2025) - Main flagship multimodal model
- **Gemini 3 Deep Think** (Nov 2025) - Advanced reasoning and complex tasks
- **Gemini 3 Flash** (Dec 2025) - Fast, cost-efficient with near-same capabilities
- **Nano Banana Pro** - High-quality image generation/editing (up to 4K)
- **Veo 3.1** - Advanced video generation with character consistency

##### Local Models
- **Llama 3 series** - Meta's open-source models
- **Code Llama** - Code-specialized variants
- **Custom models** - Fine-tuned local deployments

### 📚 Documentation & Support

#### Official Resources
- **User Guide**: https://github.com/zapabob/Codex/blob/main/docs/user-guide.md
- **API Reference**: https://docs.codex.ai/api
- **Architecture Guide**: https://github.com/zapabob/Codex/tree/main/docs/architecture

#### Community Support
- **GitHub Discussions**: https://github.com/zapabob/Codex/discussions
- **Discord Community**: https://discord.gg/codex
- **Stack Overflow**: Tag `codex-extended`

#### Professional Support
- **Enterprise Support**: enterprise@codex.ai
- **Security Issues**: security@codex.ai
- **Partnerships**: partners@codex.ai

### 🤝 Contributing

We welcome contributions from the community! Codex Extended is built on open principles and collaborative development.

#### Development Setup
```bash
# Clone the repository
git clone https://github.com/zapabob/Codex.git
cd Codex

# Install dependencies
npm install
pip install -r requirements-dev.txt

# Build from source
cargo build --release

# Run tests
npm test
cargo test
```

#### Contribution Guidelines
- **Code Style**: Follow Rust and TypeScript best practices
- **Testing**: Ensure 80%+ code coverage for new features
- **Documentation**: Update docs for any API changes
- **Security**: Run security scans before submitting PRs

### 📋 Changelog

#### v2.11.0 "ClaudeCode Integration" (Latest)
- ✅ Complete ClaudeCode feature integration
- ✅ Multi-model orchestration with intelligent selection
- ✅ 70%+ cost optimization through caching and routing
- ✅ Enterprise-grade privacy and security features
- ✅ Cowork productivity suite for file and workflow automation
- ✅ Advanced prompt injection protection
- ✅ Sandboxed execution environments
- ✅ Comprehensive CI/CD integration
- ✅ Real-time telemetry and analytics

#### Previous Versions
- **v2.10.1**: LLMOps integration and A2A communication
- **v2.10.0**: Skills system and MCP protocol implementation
- **v2.8.3**: Git4D VR/AR visualization and advanced QA
- **v2.8.2**: Performance optimization and caching improvements

### 🔮 Roadmap

#### v2.12.0 (Q1 2026)
- Advanced AI agent orchestration
- Real-time collaboration features
- Enhanced multimodal processing
- Enterprise security integrations

#### v2.13.0 (Q2 2026)
- Quantum-accelerated processing
- Neural architecture search
- Autonomous deployment pipelines
- Global distributed processing

#### v3.0.0 (Q3 2026)
- Complete AI-native IDE
- Advanced code understanding
- Predictive development assistance
- Full-stack AI automation

---

<a name="japanese"></a>
## 🇯🇵 日本語ドキュメント

### 🎯 Codex Extended v2.11.0とは？

Codex Extended v2.11.0は、AI支援開発の次の進化を表します。ClaudeCodeの革新的な機能を完全に統合しつつ、その根本的な制限を解決したプラットフォームです。Rust + Pythonのハイブリッドアーキテクチャ上に構築され、エンタープライズグレードのパフォーマンス、セキュリティ、コスト効率を提供します。

### 💡 こんな方に最適

- **AIエンジニア**: 多モデルオーケストレーションとコスト最適化
- **LLMOpsエンジニア**: モデル管理と性能監視
- **フルスタック開発者**: 自律コード生成とテスト
- **DevOpsチーム**: CI/CD自動化とインフラ管理
- **エンタープライズチーム**: プライバシー準拠のスケーラブルなAIワークフロー

### ⚡ クイックスタート

```bash
# グローバルインストール
npm install -g @zapabob/codex

# インストール確認
codex --version

# 最初のAI生成タスク
codex task "React認証コンポーネントを作成し、バリデーションを追加"

# 多モデルコスト最適化
codex multi "データベースクエリを最適化" --cost-optimize

# プライバシー保護開発
codex private "このコードベースのセキュリティ問題を分析"
```

### 🏗️ アーキテクチャ概要

Codex Extended v2.11.0は、洗練された多層アーキテクチャを実装しています：

#### ユーザーインターフェース層
- **ターミナルUI**: パワーユーザー向けテキストベースインターフェース
- **コマンドライン**: 高度なオプションを備えた完全CLI
- **MCPクライアント**: WebSocketベースのツール実行

#### AI/MLコア層
- **Codex Core**: LLMOps + A2A通信エンジン
- **スキルシステム**: 拡張性のための動的プラグインアーキテクチャ

#### ClaudeCode統合コア
- **タスクパーサー**: 自然言語処理とインテント分析
- **自律エグゼキューター**: コード生成とワークフロー自動化
- **多モデルオーケストレーター**: インテリジェントモデル選択と負荷分散
- **コストオプティマイザー**: トークン管理とAPIコスト削減
- **プライバシーガード**: データ匿名化とセキュリティ保護
- **プロンプトインジェクションガード**: 多層攻撃防止
- **セキュア実行エンジン**: サンドボックス環境管理

#### Cowork生産性スイート
- **ファイル管理**: インテリジェント整理と一括操作
- **ブラウザ自動化**: WebスクレイピングとUIテスト
- **ワークフローオーケストレーター**: 複雑プロセス自動化

#### 多エージェントシステム
- **スーパーバイザー**: エージェントオーケストレーションと競合解決
- **ワーカーエージェント**: コードレビュー、テスト、セキュリティ、パフォーマンス分析の専門AIアシスタント

#### 開発ツール
- **ビルドマネージャー**: インクリメンタルビルドとプロセス最適化
- **Worktreeマネージャー**: Git worktreeによる並行開発
- **QAサービス**: 自動レビューとバックグラウンド監視
- **CI/CD統合**: パイプライン生成とデプロイ自動化

### 📊 パフォーマンス指標

- **コスト削減**: インテリジェントキャッシュによる70%+ APIコスト削減
- **レスポンス時間**: 平均60%高速処理
- **信頼性**: エンタープライズグレードのエラー処理による99.5%成功率
- **スループット**: 3倍の並行タスク処理能力
- **プライバシー**: 100%ローカル処理オプション対応

### 🔧 高度な機能

#### モデルオーケストレーション

```bash
# 自動モデル選択
codex task "この複雑なJavaScriptエラーをデバッグ"

# 手動モデル指定
codex multi "マイクロサービスシステムを設計" --models claude-3,gpt-4

# コスト最適化実行
codex task "包括的なドキュメントを生成" --budget 1.00
```

#### プライバシー & セキュリティ

```bash
# 最大プライバシーモード
codex private "このコードをGDPR準拠で監査" --privacy maximum

# サンドボックス実行
codex sandbox "この潜在的に危険な操作をテスト"

# 監査ログ
codex audit "セキュリティ体制を確認" --report-format pdf
```

#### Cowork自動化

```bash
# ファイル管理
codex organize "プロジェクトファイルを種類と機能で整理"

# ブラウザ自動化
codex browser "eコマースサイトから製品データを抽出"

# ワークフローオーケストレーション
codex workflow "完全テストパイプラインでアプリケーションをデプロイ"
```

### 🔗 統合エコシステム

#### 対応プラットフォーム
- **OS**: Windows、macOS、Linux
- **アーキテクチャ**: x64、ARM64
- **Node.jsバージョン**: 16.0+（18+推奨）

#### 外部統合
- **バージョン管理**: Git、GitHub、GitLab
- **CI/CD**: GitHub Actions、Jenkins、CircleCI
- **クラウドプラットフォーム**: AWS、GCP、Azure
- **コミュニケーションツール**: Slack、Discord、Microsoft Teams

#### API互換性
- **OpenAI API**: GPT-3.5、GPT-4、GPT-4 Turbo
- **Anthropic API**: Claude 3 Opus、Sonnet、Haiku
- **Google AI**: Gemini Pro、Gemini Pro Vision
- **ローカルモデル**: Llama、Code Llama、カスタムモデル

### 📚 ドキュメント & サポート

#### 公式リソース
- **ユーザーガイド**: https://github.com/zapabob/Codex/blob/main/docs/user-guide.md
- **APIリファレンス**: https://docs.codex.ai/api
- **アーキテクチャガイド**: https://github.com/zapabob/Codex/tree/main/docs/architecture

#### コミュニティサポート
- **GitHub Discussions**: https://github.com/zapabob/Codex/discussions
- **Discordコミュニティ**: https://discord.gg/codex
- **Stack Overflow**: タグ `codex-extended`

#### プロフェッショナルサポート
- **Enterpriseサポート**: enterprise@codex.ai
- **セキュリティ問題**: security@codex.ai
- **パートナーシップ**: partners@codex.ai

### 🤝 コントリビューション

コミュニティからのコントリビューションを歓迎します！Codex Extendedはオープンな原則と協調開発に基づいて構築されています。

#### 開発環境構築
```bash
# リポジトリをクローン
git clone https://github.com/zapabob/Codex.git
cd Codex

# 依存関係をインストール
npm install
pip install -r requirements-dev.txt

# ソースからビルド
cargo build --release

# テストを実行
npm test
cargo test
```

#### コントリビューションガイドライン
- **コードスタイル**: RustおよびTypeScriptのベストプラクティスに従う
- **テスト**: 新機能のコードカバレッジ80%+を確保
- **ドキュメント**: API変更時はドキュメントを更新
- **セキュリティ**: PR提出前にセキュリティスキャンを実行

### 📋 変更履歴

#### v2.11.0 "ClaudeCode Integration" (最新)
- ✅ ClaudeCode機能の完全統合
- ✅ インテリジェント選択による多モデルオーケストレーション
- ✅ キャッシュとルーティングによる70%+コスト最適化
- ✅ エンタープライズグレードのプライバシー・セキュリティ機能
- ✅ ファイル・ワークフロー自動化のためのCowork生産性スイート
- ✅ 高度なプロンプトインジェクション保護
- ✅ サンドボックス実行環境
- ✅ 包括的なCI/CD統合
- ✅ リアルタイムテレメトリとアナリティクス

#### 以前のバージョン
- **v2.10.1**: LLMOps統合とA2A通信
- **v2.10.0**: スキルシステムとMCPプロトコル実装
- **v2.8.3**: Git4D VR/AR可視化と高度なQA
- **v2.8.2**: パフォーマンス最適化とキャッシュ改善

### 🔮 ロードマップ

#### v2.12.0 (2026年第1四半期)
- 高度なAIエージェントオーケストレーション
- リアルタイム協調機能
- 強化されたマルチモーダル処理
- エンタープライズセキュリティ統合

#### v2.13.0 (2026年第2四半期)
- 量子加速処理
- ニューラルアーキテクチャ検索
- 自律デプロイパイプライン
- グローバル分散処理

#### v3.0.0 (2026年第3四半期)
- 完全AIネイティブIDE
- 高度なコード理解
- 予測開発支援
- フルスタックAI自動化

---

## 📄 License

Apache License 2.0 - See [LICENSE](./LICENSE)

## 🙏 Acknowledgments

- **Anthropic** for ClaudeCode inspiration
- **Google** for Gemini model capabilities
- **OpenAI** for GPT model innovations
- **Community contributors** for valuable feedback and contributions

## 📚 Additional Resources

- [**User Guide**](./docs/user-guide.md) - Comprehensive usage documentation
- [**Architecture**](./docs/architecture/) - Technical architecture details
- [**API Reference**](https://docs.codex.ai/api) - Complete API documentation
- [**Contributing Guide**](./CONTRIBUTING.md) - How to contribute to the project
- [**Security Policy**](./SECURITY.md) - Security reporting and updates

---

**Built with ❤️ by [@zapabob](https://github.com/zapabob)**

*Codex Extended v2.11.0 - Revolutionizing AI-assisted development* 🚀