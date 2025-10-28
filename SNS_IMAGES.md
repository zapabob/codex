# SNS用画像ファイル - Codex v0.51.0 Architecture

## 📊 生成されたアーキテクチャ図

### 🎨 ファイル一覧

| 用途 | ファイル | 形式 | サイズ | 解像度 |
|------|---------|------|--------|--------|
| GitHub README | `codex-v0.51.0-architecture.svg` | SVG | - | ベクター |
| X (Twitter) | `codex-v0.51.0-architecture.png` | PNG | - | 2400x1800 |
| LinkedIn | `codex-v0.51.0-architecture.png` | PNG | - | 2400x1800 |

---

## 📍 ファイル配置

```
zapabob/docs/
├── codex-v0.51.0-architecture.mmd   # Mermaidソースファイル
├── codex-v0.51.0-architecture.svg   # GitHub用ベクター画像
└── codex-v0.51.0-architecture.png   # SNS用高解像度PNG
```

---

## 🎯 アーキテクチャ図の構成

### 9つの主要レイヤー

1. **🖥️ User Interface Layer**
   - CLI (12 subcommands)
   - TUI (Interactive Terminal)
   - Cursor IDE (MCP Integration)
   - Natural Language CLI

2. **🧠 Core Layer** - codex-core v0.51.0
   - Codex (Main orchestrator)
   - ConversationManager
   - AuthManager (Keyring/OAuth 2.0)
   - Config (TOML parser)

3. **🎯 Orchestration Layer** - rmcp 0.8.3
   - TaskAnalyzer (Complexity detection)
   - AutoOrchestrator (Strategy selection)
   - CollaborationStore (Message passing)
   - ConflictResolver (3 merge strategies)

4. **🤖 Sub-Agent System** - 8 Agents
   - AgentRuntime (Lifecycle, Token budget)
   - code-reviewer, sec-audit, test-gen
   - researcher, python-reviewer, ts-reviewer
   - unity-reviewer, Custom Agents

5. **🔍 Deep Research Engine** - v0.51.0
   - Research Pipeline
   - Search Provider (Cache TTL: 1h)
   - **Gemini Search Grounding** (default) ✨
   - DuckDuckGo, Google, Bing (fallbacks)
   - Citation Manager
   - Contradiction Checker

6. **🔗 MCP Integration** - 15 Servers
   - codex, codex-research, codex-agent (NEW!)
   - codex-gemini-mcp, serena (21 tools)
   - arxiv, chrome-devtools, playwright
   - sequential-thinking (NEW!)
   - + 6 more servers

7. **🛠️ Tools & Execution**
   - ToolRouter (Dynamic dispatch)
   - ToolCallRuntime (Parallel 3x speedup)
   - ExecEngine (Sandboxed execution)
   - ApplyPatch (Git-style merging)

8. **🌐 External Integrations**
   - GitHub API (PR/Issue webhooks)
   - Slack (Channel notifications)
   - Audio System (marisa_owattaze.wav)
   - Hook System (lifecycle events)

9. **💾 Storage & State**
   - Session Manager (Resume capability)
   - Audit Logger (JSON, Token tracking)
   - Cache System (Search/MCP responses)

---

## 🎨 カラーコーディング

| レイヤー | カラー | 意味 |
|---------|--------|------|
| UI Layer | 🔵 Blue | ユーザー接点 |
| Core Layer | 🟡 Yellow | コア機能 |
| Orchestration | 🟣 Purple | 自動調整 |
| Sub-Agents | 🟢 Green | AI処理 |
| Deep Research | 🔵 Light Blue | 情報収集 |
| MCP Integration | 🟠 Orange | 外部統合 |
| Tools & Execution | 🔴 Pink | 実行系 |
| External | 🟢 Lime | 外部API |
| Storage | ⚫ Gray | データ永続化 |

---

## 📐 技術仕様

### Mermaid生成コマンド

```bash
# SVG生成（GitHub用）
mmdc -i codex-v0.51.0-architecture.mmd \
     -o codex-v0.51.0-architecture.svg \
     -t dark -b transparent

# PNG生成（SNS用、高解像度）
mmdc -i codex-v0.51.0-architecture.mmd \
     -o codex-v0.51.0-architecture.png \
     -t dark -b transparent \
     -w 2400 -H 1800
```

### パラメータ説明
- `-t dark`: ダークテーマ（見やすい）
- `-b transparent`: 透過背景
- `-w 2400 -H 1800`: 高解像度（SNS最適）

---

## 📱 SNS投稿での使用方法

### X (Twitter)
1. `codex-v0.51.0-architecture.png` を添付
2. ツイート文は `X_TWEET_WITH_URL.md` の Version 3 を使用
3. 画像が自動的にプレビュー表示される

### LinkedIn
1. `codex-v0.51.0-architecture.png` を添付
2. 投稿文は `SNS_POST.md` の LinkedIn英語版を使用
3. 技術的詳細を強調

### GitHub README.md
1. SVGファイルをMarkdownで埋め込み済み
2. ダークモード対応
3. スケーラブル（ベクター形式）

---

## ✅ v0.51.0の主要ハイライト

### 🆕 NEW in v0.51.0
- **Gemini Search Grounding**: デフォルト検索バックエンド
- **codex-research MCP**: Deep Research専用サーバー
- **codex-agent MCP**: 自然言語CLI専用サーバー
- **sequential-thinking MCP**: 段階的思考サーバー
- **15 MCP Servers**: 14個から増加

### 🔄 Updated
- **OpenAI upstream**: commit 4a42c4e1統合
- **Auth System**: Keyring対応
- **Rust Edition**: 2024互換性

---

**アーキテクチャ図完成や！SVG（GitHub用）とPNG（SNS用）の両方が生成されたで！🎊**

