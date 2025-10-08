# Cursor Rules for Codex Repository

## Meta Prompt for Codex Development

### 1. 役割分担 (Role Assignment)

#### メインエージェント
- コードレビューと意思決定を担当
- アーキテクチャ判断とタスク分解
- 最終的な実装判断とマージ決定

#### サブエージェント (`codex-supervisor`)
- Rust・Node.js・ドキュメントなど専門領域ごとに分担
- 並列タスク実行による高速化
- 専門知識を活かした詳細実装

#### Deep Research (`codex-deep-research`)
- 広範な調査が必要な場合に起動
- 実装に必要な裏付け資料を収集
- 技術選定の根拠資料を提供

---

### 2. 開発プロセス (Development Process)

#### タスク開始時
1. **AGENTS.md のガイドラインを確認**
   ```bash
   cat AGENTS.md
   ```

2. **Multi-Agent/Deep Research 戦略を確認**
   - タスクの複雑度を評価
   - 並列化可能な部分を特定
   - Deep Research が必要な領域を判断

3. **自動化を優先**
   - 繰り返す手順は `just` ターゲットへ落とし込む
   - 再利用可能なプロンプトを作成
   - スクリプト化できる作業は必ずスクリプト化

#### 実行中
- サブエージェント同士は supervisor 経由で進捗共有
- Deep Research の知見を即座に実装へ反映
- ブロッカーは早期に識別してエスカレーション

---

### 3. コーディング規約 (Coding Standards)

#### Rust
- **フォーマット**: `rustfmt` 準拠（`just fmt` で自動適用）
- **命名**: crates 名は `codex-` プレフィックス
- **スタイル**: Stylize ユーティリティ使用（TUI は `tui/styles.md` 参照）
- **モジュール**: snake_case、`codex-` プレフィックス
- **テスト**: `pretty_assertions::assert_eq` 使用、ファイル名は `<feature>_tests.rs`

```rust
// Good
use codex_core::SecurityProfile;
assert_eq!(actual, expected);

// Bad
use core::security_profile;  // プレフィックスなし
assert_eq!(actual, expected, "message");  // pretty_assertions推奨
```

#### TypeScript/Node.js
- **フォーマット**: ESLint + Prettier
- **インデント**: 2スペース
- **命名**: camelCase (runtime), PascalCase (types)
- **テスト**: Jest、ファイル名は `<feature>.test.ts`
- **場所**: `tests/` 下に `src/` の構造をミラー

```typescript
// Good
export function processTask(input: TaskInput): TaskResult {
  return { status: "success" };
}

// Bad
export function ProcessTask(input: TaskInput): TaskResult {  // PascalCaseは型のみ
  return {status:"success"};  // スペース必須
}
```

#### 設定ファイル
- **基本**: ASCII のみ（特定ロケールが必要な場合を除く）
- **フォーマット**: TOML (Rust), JSON (Node.js)

---

### 4. テスト & CI/CD

#### ローカルテスト（必須）

##### Rust
```bash
# フォーマット確認
just fmt

# Lint修正
just fix -p <crate>

# 全機能テスト
cargo test --all-features

# 特定クレートテスト
cargo test -p codex-core
cargo test -p codex-supervisor
cargo test -p codex-deep-research
```

##### TypeScript/Node.js
```bash
# SDK テスト
pnpm --filter @openai/codex-sdk test

# カバレッジ
pnpm --filter @openai/codex-sdk run coverage

# Lint & Format
pnpm --filter @openai/codex-sdk run lint
pnpm --filter @openai/codex-sdk run format
```

#### CI/CD 必須チェック

##### Supervisor & Deep Research
```bash
# Supervisor テスト
cargo test -p codex-supervisor

# Deep Research テスト
cargo test -p codex-deep-research

# CLI実証テスト
cargo test -p codex-cli supervisor_deepresearch
```

##### スナップショットテスト（TUI）
```bash
# スナップショット更新時
cargo test -p codex-tui
cargo insta pending-snapshots -p codex-tui
# レビュー後に accept
```

#### API差分への追従
- コア API が変更された場合、関連テストを更新
- 失敗時は差分を確認して再試行
- 互換性が失われる変更は CHANGELOG に記録

---

### 5. PR づくり (Pull Request Guidelines)

#### コミットメッセージ
**Conventional Commit 形式**
```
feat(supervisor): マルチエージェント並列実行を実装
fix(deep-research): 検索結果の重複除去を修正
docs(readme): インストール手順を更新
test(security): 脱獄E2Eテストを追加
chore(ci): security-tests ワークフローを追加
```

#### PR の構成
1. **タイトル**: Conventional Commit 形式
2. **説明**: 
   - 変更の背景・目的
   - 影響を受ける Crate/Package
   - Deep Research の要点（該当する場合）
3. **チェックリスト**:
   ```markdown
   - [ ] ローカルで全テスト通過
   - [ ] Lint/Format 適用済み
   - [ ] ドキュメント更新済み
   - [ ] CI 期待値を記載
   - [ ] スナップショット変更をレビュー済み
   ```

#### PR の焦点
- **単一論点に集中**: 1 PR = 1 機能/修正
- **小さく保つ**: 大きな変更は複数 PR に分割
- **関連 Issue リンク**: `Closes #123` など

#### ロールアウト手順
- 手動作業が必要な場合は `just` ターゲット化
- スクリプトを `scripts/` に配置
- README に実行手順を記載

```bash
# Good: just ターゲット化
just deploy-supervisor

# Bad: 手動手順の羅列
# 1. cd codex-rs
# 2. cargo build --release
# 3. ...
```

---

### 6. ハンドオフチェック (Handoff Checklist)

PR 作成前に以下を確認：

#### ✅ コード品質
- [ ] `just fmt` でフォーマット済み
- [ ] `just fix -p <crate>` で Lint エラーなし
- [ ] `cargo test --all-features` 全パス
- [ ] `pnpm --filter @openai/codex-sdk test` 全パス（該当する場合）

#### ✅ ドキュメント
- [ ] README 更新済み（新機能の場合）
- [ ] CHANGELOG 更新済み（破壊的変更の場合）
- [ ] コード内ドキュメント（`///` コメント）記載

#### ✅ Multi-Agent & Deep Research
- [ ] サブエージェント実行手順を記述
- [ ] Deep Research の結果を PR 本文へ記録
- [ ] 調査資料へのリンク（該当する場合）

#### ✅ CI/CD
- [ ] CI 期待値（成果物・スナップショット等）を再現
- [ ] スクリーンショット添付（UI 変更の場合）
- [ ] ベンチマーク結果添付（パフォーマンス変更の場合）

#### ✅ 自動化
- [ ] 手動作業を `just` ターゲット化
- [ ] スクリプトに実行権限付与
- [ ] レビュアーが再現可能

---

### 7. トラブルシューティング

#### ビルドエラー
```bash
# 依存関係の再取得
cargo clean
cargo fetch

# キャッシュクリア
rm -rf target/
cargo build
```

#### テスト失敗
```bash
# 詳細ログ表示
cargo test -- --nocapture

# 特定テストのみ実行
cargo test test_name -- --exact

# スナップショット差分確認
cargo insta pending-snapshots
```

#### フォーマット・Lint
```bash
# 自動修正
just fix -p <crate>

# 確認のみ
cargo clippy --all-features -- -D warnings
```

---

### 8. ベストプラクティス

#### セキュリティ
- デフォルトは最も安全な状態（fail-safe）
- エラー時は拒否（deny by default）
- ユーザー入力は必ずサニタイズ

#### パフォーマンス
- ベンチマーク基盤を活用（`benches/`）
- 定量目標を設定（例: cold start <80ms）
- リグレッション防止

#### 保守性
- モジュール境界を明確に
- 依存関係は最小限に
- 再利用可能なコンポーネント設計

#### テスタビリティ
- モック可能な設計（trait 活用）
- 外部依存を注入可能に
- テストユーティリティの共有

---

### 9. リソース

#### ドキュメント
- [AGENTS.md](../AGENTS.md) - エージェント戦略
- [CONTRIBUTING.md](../docs/contributing.md) - コントリビューションガイド
- [Platform Sandboxing](../docs/platform-sandboxing.md) - セキュリティ
- [TUI Styles](../codex-rs/tui/styles.md) - TUI スタイルガイド

#### ツール
- `just` - タスクランナー（`just --list` で一覧）
- `cargo nextest` - 高速テストランナー
- `cargo insta` - スナップショットテスト
- `cargo deny` - 依存関係チェック

#### コミュニティ
- GitHub Issues: バグ報告・機能要望
- GitHub Discussions: 質問・議論
- Pull Requests: コード貢献

---

## 実装例

### Good Example: Multi-Agent タスク

```rust
use codex_supervisor::{AgentManager, AgentType, TaskType, MergeStrategy};

async fn process_complex_task() -> Result<String> {
    let mut manager = AgentManager::new();
    
    // 並列実行
    manager.create_agent(AgentType::CodeExpert);
    manager.create_agent(AgentType::Researcher);
    manager.create_agent(AgentType::Tester);
    
    let task = TaskType::Parallel {
        agent_types: vec![
            AgentType::CodeExpert,
            AgentType::Researcher,
            AgentType::Tester,
        ],
        task: "Implement feature X".to_string(),
        merge_strategy: MergeStrategy::Voting,
    };
    
    let result = manager.execute_task(task).await?;
    Ok(result.output)
}
```

### Good Example: Deep Research 統合

```rust
use codex_deep_research::{ResearchConfig, ResearchEngine};

async fn research_based_implementation() -> Result<()> {
    // 1. Deep Research で調査
    let config = ResearchConfig {
        query: "Best practices for Rust async error handling".to_string(),
        max_sources: 10,
        strategy: "comprehensive".to_string(),
    };
    
    let research = ResearchEngine::new(config);
    let findings = research.execute().await?;
    
    // 2. 調査結果を実装に反映
    // findings を基に実装...
    
    // 3. 結果を PR 本文に記録
    // findings.summary() をドキュメント化
    
    Ok(())
}
```

---

**Last Updated**: 2025-10-08  
**Version**: 1.0.0  
**Status**: Active

