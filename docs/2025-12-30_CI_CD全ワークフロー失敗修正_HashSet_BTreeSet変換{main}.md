# CI/CD全ワークフロー失敗修正 - HashSet/BTreeSet変換

**日時**: 2025-12-30
**ワークツリー**: main
**タスク**: CI/CDワークフローの失敗を修正するため、HashSetをBTreeSetに変換

---

## 問題の分析

CI/CDワークフローの失敗原因：
1. **Clippyの`disallowed_types`ルール違反**: `clippy.toml`で`HashSet`の使用が禁止されているが、コードベース全体で193箇所の`HashSet`使用が存在
2. **`ghost_commits.rs`**: 既にローカルで`BTreeSet`に修正済みだが、CI環境でエラーが発生
3. **その他のファイル**: 多数のファイルで`HashSet`が使用されている

---

## 修正内容

### 1. utils/readiness/src/lib.rs

**修正箇所**:
- Line 3: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 15: `Token`構造体に`PartialOrd, Ord`を追加
- Line 49: `tokens: Mutex<HashSet<Token>>` → `tokens: Mutex<BTreeSet<Token>>`
- Line 61: `Mutex::new(HashSet::new())` → `Mutex::new(BTreeSet::new())`
- Line 68: `f: impl FnOnce(&mut HashSet<Token>)` → `f: impl FnOnce(&mut BTreeSet<Token>)`

### 2. core/src/orchestration/auto_orchestrator.rs

**修正箇所**:
- Line 22: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 427: `let mut seen_agents = HashSet::new();` → `let mut seen_agents = BTreeSet::new();`
- Line 900: `let mut selected_agents = HashSet::new();` → `let mut selected_agents = BTreeSet::new();`

### 3. core/src/orchestration/task_analyzer.rs

**修正箇所**:
- Line 7: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 182: `let mut detected_domains = HashSet::new();` → `let mut detected_domains = BTreeSet::new();`
- Line 266: `let mut tags = HashSet::new();` → `let mut tags = BTreeSet::new();`
- Line 397: `let mut agents = HashSet::new();` → `let mut agents = BTreeSet::new();`

### 4. core/src/mcp_connection_manager.rs

**修正箇所**:
- Line 10: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 85: `let mut used_names = HashSet::new();` → `let mut used_names = BTreeSet::new();`
- Line 676-677: `ToolFilter`構造体のフィールド型を変更
  - `enabled: Option<HashSet<String>>` → `enabled: Option<BTreeSet<String>>`
  - `disabled: HashSet<String>` → `disabled: BTreeSet<String>`
- Line 685, 689: `collect::<HashSet<_>>()` → `collect::<BTreeSet<_>>()`
- Line 953: テストコード内の`use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- テストコード内の`HashSet::from()` → `BTreeSet::from()` (複数箇所)
- テストコード内の`HashSet::new()` → `BTreeSet::new()` (複数箇所)

### 5. core/src/ai_orchestrator.rs

**修正箇所**:
- Line 239: `let mut used_agents = std::collections::HashSet::new();` → `let mut used_agents = std::collections::BTreeSet::new();`

### 6. supervisor/src/multi_agent_evaluator.rs

**修正箇所**:
- Line 13: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 297: `let mut seen = HashSet::new();` → `let mut seen = BTreeSet::new();`
- Line 315: `let mut seen = HashSet::new();` → `let mut seen = BTreeSet::new();`

### 7. supervisor/src/executor.rs

**修正箇所**:
- Line 8: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 39: `let mut all_step_ids = HashSet::new();` → `let mut all_step_ids = BTreeSet::new();`
- Line 46: `HashMap<String, HashSet<String>>` → `HashMap<String, BTreeSet<String>>`
- Line 50: `let filtered_dependencies: HashSet<String>` → `let filtered_dependencies: BTreeSet<String>`
- Line 68: `let mut active_domains: HashSet<String> = HashSet::new();` → `let mut active_domains: BTreeSet<String> = BTreeSet::new();`
- Line 138: `active_domains: &HashSet<String>` → `active_domains: &BTreeSet<String>`

### 8. supervisor/src/autonomous_orchestrator.rs

**修正箇所**:
- Line 17: `use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- Line 125: `active_agents: HashSet<AgentType>` → `active_agents: BTreeSet<AgentType>`
- Line 176: `active_agents: HashSet::new()` → `active_agents: BTreeSet::new()`
- Line 369: `let mut noted_busy: HashSet<AgentType> = HashSet::new();` → `let mut noted_busy: BTreeSet<AgentType> = BTreeSet::new();`

### 9. supervisor/src/subagent.rs

**修正箇所**:
- Line 10: `AgentType` enumに`PartialOrd, Ord`を追加
  - `#[derive(Debug, Clone, PartialEq, Eq, Hash, ...)]` → `#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ...)]`

### 10. supervisor/src/autonomous_dispatcher.rs

**修正箇所**:
- Line 280: `.collect::<std::collections::HashSet<_>>()` → `.collect::<std::collections::BTreeSet<_>>()`

---

## 技術的な注意事項

- `HashSet`から`BTreeSet`への変更により、順序が保証されるようになった
- `BTreeSet`は`HashSet`より挿入・検索が遅いが、Clippyの`disallowed_types`ルールに準拠するため必要
- `Token`と`AgentType`に`Ord`トレイトを追加して`BTreeSet`で使用可能にした
- テストコード内の`HashSet`使用もすべて`BTreeSet`に変更

---

## 完了したタスク

1. ✅ `utils/readiness/src/lib.rs`の`HashSet`を`BTreeSet`に変更
2. ✅ `core/src/orchestration/auto_orchestrator.rs`の`HashSet`を`BTreeSet`に変更
3. ✅ `core/src/orchestration/task_analyzer.rs`の`HashSet`を`BTreeSet`に変更
4. ✅ `core/src/mcp_connection_manager.rs`の`HashSet`を`BTreeSet`に変更（構造体フィールド含む）
5. ✅ `core/src/ai_orchestrator.rs`の`HashSet`を`BTreeSet`に変更
6. ✅ `supervisor/src/multi_agent_evaluator.rs`の`HashSet`を`BTreeSet`に変更
7. ✅ `supervisor/src/executor.rs`の`HashSet`を`BTreeSet`に変更
8. ✅ `supervisor/src/autonomous_orchestrator.rs`の`HashSet`を`BTreeSet`に変更
9. ✅ `supervisor/src/subagent.rs`の`AgentType`に`Ord`トレイトを追加
10. ✅ `supervisor/src/autonomous_dispatcher.rs`の`HashSet`を`BTreeSet`に変更

---

## 次のステップ

- ローカルで`cargo clippy`を実行して残りのエラーを確認
- `HashMap`エラーも多数存在するが、計画では`HashSet`を優先
- 他のCI/CDワークフローのエラー（cargo-deny、ci、Sub-Agent CI、Codespell、AI Kernel Modules CI、sdk）を確認して修正

---

完了！
