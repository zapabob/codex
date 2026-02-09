# Codex - AI Coding Assistant | AIコーディングアシスタント

\*\*

<div align="center">
  <img src="docs/assets/hero-banner.png" alt="Codex AI Banner" width="100%">
</div>

## 📜 Development History / 開発履歴

| Version     | Key Features & Implementation Details (English)                                                    | 実装履歴・主要機能 (日本語)                                                                               |
| :---------- | :------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| **v2.15.1** | **A2A Swarm Intelligence**: Real-time agent coordination. **GUI-X**: Next.js 15/React 19 frontend. | **A2Aスウォーム・インテリジェンス**: エージェント間リアルタイム協調。**GUI-X**: Next.js 15/React 19採用。 |
| **v2.12.0** | **QC Optimization**: LLM output benchmarking. **CUDA 12**: 3.7x faster semantic search.            | **QC最適化**: LLM出力のベンチマーク比較。**CUDA 12**: セマンティック検索の3.7倍高速化。                   |
| **v2.10.0** | **Rust 2024**: Memory safety & high-concurrency core. **MCP 1.0**: Standardized tool-use.          | **Rust 2024**: メモリ安全性と高並列コア。**MCP 1.0**: 標準化されたツール利用プロトコル。                  |
| **v2.5.0**  | **Zero-Trust Sandbox**: Win32 Job Objects isolation. **Git Worktree**: Parallel agent dev.         | **ゼロトラスト・サンドボックス**: Win32 Job Objectsによる隔離。**Git Worktree**: エージェント並列開発。   |

\*\*

<div align="center">

[![Version](https://img.shields.io/badge/version-v2.15.1-blue)](./docs/v2.15.1_Release_Details.md)
[![Rust](https://img.shields.io/badge/rust-2024-orange)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0-blue)](https://www.typescriptlang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)](./INSTALL.md)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](./LICENSE)
[![CUDA](https://img.shields.io/badge/CUDA-12.0+-76B900?logo=nvidia)](./docs/cuda/)
[![MCP](https://img.shields.io/badge/MCP-Protocol-blueviolet)](./docs/mcp/)

**🚀 Next-generation AI coding assistant featuring high-performance parallel execution, industrial-grade security sandboxing, and autonomous multi-agent orchestration.**  
**🚀 次世代AIコーディングアシスタント：高性能並列実行、産業グレードのセキュリティサンドボックス、自律型マルチエージェントオーケストレーションを搭載。**

[🇺🇸 English](#-english) | [🇯🇵 日本語](#-japanese)

</div>

---

## 🇺🇸 English

### 🎯 What is Codex?

**Codex** is an **advanced AI orchestration platform** designed for enterprise-grade autonomous software engineering. It transforms AI from a simple chat interface into a powerful, multi-agent development environment.

Key pillars of this project include:

- **⚡ Rust 2024 Core** - State-of-the-art memory safety and high-concurrency performance.
- **🔒 Zero-Trust Sandboxing** - Secure isolation with strict ACLs and audit trails.
- **🤖 Autonomous Swarms** - Parallel sub-agents (Backend, QA, Frontend) with A2A coordination.
- **🎮 Immersive Repositories** - Git4D 3D visualization with VR/AR support.

### 🌟 Latest in v2.15.1 (Unique & Official Features)

We emphasize **technical excellence** and **innovation** by integrating the best of official features with our unique competitive additions.

- **Agent-to-Agent (A2A) Swarm Intelligence**: Real-time coordination and self-correction during development.
- **Codex GUI-X**: Next-gen frontend built with **Next.js 15**, **React 19**, and **3D visualization**.
- **Git Worktree Orchestration**: Parallel feature development managed directly by AI agents.
- **QC Optimization Competition**: Benchmarking LLM outputs to select the most optimal code path.

### 🚀 Technical Excellence Stack

| Component        | Technology        | Impact                                               |
| ---------------- | ----------------- | ---------------------------------------------------- |
| **Backend**      | Rust 2024 / Tokio | Sub-millisecond latency & thread-safe orchestration. |
| **Frontend**     | Next.js 15 / MUI  | Real-time telemetry & intuitive AI interaction.      |
| **AI Protocol**  | MCP 1.0           | Extensible tool-use across any LLM provider.         |
| **Acceleration** | CUDA 12 / GPU     | 3.7x faster codebase analysis & semantic search.     |
| **Security**     | Win32 Job Objects | Secure, isolated execution of untrusted code.        |

---

## 🇯🇵 日本語

### 🎯 Codexとは？

**Codex**は、エンタープライズレベルの自律型ソフトウェアエンジニアリングのために設計された**高度なAIオーケストレーションプラットフォーム**です。AIを単なるチャットインターフェースから、強力なマルチエージェント開発環境へと進化させます。

本プロジェクトの4つの柱：

- **⚡ Rust 2024 採用** - 最新のメモリ安全性と高並列パフォーマンスを実現。
- **🔒 ゼロトラスト・サンドボックス** - 厳格なACLと監査証跡による安全な分離環境。
- **🤖 自律型エージェント群** - A2A通信を介した並列サブエージェント（Backend, QA, Frontend）の協調。
- **🎮 イマーシブ・リポジトリ** - VR/ARに対応したGit4D 3D可視化。

### 🌟 v2.15.1 最新機能 (独自機能 & 公式統合)

公式の最新機能を完全に取り込みつつ、他にはない独自の**技術的卓越性**と**革新性**を強調しています。

- **Agent-to-Agent (A2A) 群知能**: 開発中のエージェント間リアルタイム協調と自動修正。
- **Codex GUI-X**: **Next.js 15** と **React 19** を採用した、3D可視化機能搭載の次世代フロントエンド。
- **Gitワークツリー・オーケストレーション**: AIエージェントによる直接的な並列開発管理。
- **QC最適化コンペティション**: 複数のLLM出力をベンチマークし、最適なコードパスを自動選択。

### 🚀 テクノロジースタック

| コンポーネント     | 採用技術          | メリット                                         |
| ------------------ | ----------------- | ------------------------------------------------ |
| **バックエンド**   | Rust 2024 / Tokio | ミリ秒単位の低遅延とスレッドセーフなタスク制御。 |
| **フロントエンド** | Next.js 15 / MUI  | リアルタイム・テレメトリと直感的なUI。           |
| **AIプロトコル**   | MCP 1.0           | プロバイダーを問わない拡張可能なツール利用。     |
| **高速化**         | CUDA 12 / GPU     | 3.7倍高速なコード解析とセマンティック検索。      |
| **セキュリティ**   | Win32 Job Objects | 信頼できないコードを安全に分離・実行。           |

---

<div align="center">

### 👨‍💻 For AI & Tech Recruiters | 採用担当者様へ

This repository showcases mastery in **high-performance systems**, **AI orchestration**, and **modern product engineering**. It is built with a focus on **scalability, security, and developer experience**.

本リポジトリは、**高性能システム**、**AIオーケストレーション**、および**モダンなプロダクトエンジニアリング**における熟練度を示しています。**スケーラビリティ、セキュリティ、および開発者体験**に重点を置いて構築されています。

**Highlights | 実証された主要スキル:**

- **Systems Integration**: Low-level Rust & Windows system security.
- **AI Innovations**: Multi-agent swarms & MCP protocol architecture.
- **Graphics/HPC**: 3D Visualization (Three.js) & CUDA acceleration.

**Full Portfolio**: [github.com/zapabob](https://github.com/zapabob)

---

**Built with ❤️ and Rust 2024**  
**Rust 2024と情熱を持って構築**

[Issues](https://github.com/zapabob/Codex/issues) | [Releases](https://github.com/zapabob/Codex/releases) | [Documentation](./docs/v2.15.1_Release_Details.md)

</div>
