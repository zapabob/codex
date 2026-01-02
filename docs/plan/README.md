# Plan Mode Quickstart - 5分で再現可能

**Status**: Stable | **Proof**: このガイド通りに動く

---

## 🎯 5分で試せるPlan Mode

Plan Modeは**タスクを計画→承認→実行**の3段階で安全にAI開発を進める機能です。

### Step 1: インストール確認 (30秒)

```bash
npm install -g @zapabob/codex
codex --version
# codex-cli 2.8.3
```

### Step 2: Plan Mode有効化 (30秒)

```bash
codex /Plan on
# Plan mode enabled
```

### Step 3: 簡単なタスク計画 (1分)

```bash
codex /Plan "Add a simple logging function to utils.js"
```

### Step 4: 計画内容を確認 (1分)

```bash
codex /Plan export bp-123 --format=md
# docs/Plans/bp-123.md が生成される
```

### Step 5: 承認して実行 (1分)

```bash
codex /approve bp-123
codex execute bp-123
```

### Step 6: 結果確認 (1分)

```bash
# utils.js に logging 関数が追加されているはず
cat utils.js
```

---

## 📋 動作原理

**Plan Modeの3段階フロー**:

1. **Planning**: AIがタスクを分析・計画（副作用なし）
2. **Approval**: 人間が計画を承認（安全ゲート）
3. **Execution**: 承認済み計画を実行（実際の変更）

**安全保障**:
- 承認前はファイル変更なし
- 予算超過で自動停止
- 構造化ログで監査可能

---

## 🚀 応用パターン

### 並列サブエージェント使用

```bash
codex /Plan "Implement user authentication with tests" --mode=orchestrated
```

### パフォーマンス競争実行

```bash
codex /Plan "Optimize slow database query" --mode=competition
```

### リサーチ統合

```bash
codex /Plan "Add React error boundaries" --research-depth=2
```

---

## 📚 詳細ドキュメント

- [実行モード詳細](./execution-modes.md) - Orchestrated/Competitionモードの違い
- [スラッシュコマンド](./slash-commands.md) - 全コマンドリファレンス
- [GUI操作](./gui-controls.md) - VS Code拡張での操作
- [Webhook設定](./webhooks.md) - CI/CD統合

---

## ✅ 証明: このガイドで実際に動く

**実行環境**: Windows 11 + Node.js 18+
**所要時間**: 5分
**必要なファイル**: utils.js (空ファイルでOK)
**成功条件**: utils.jsにlogging関数が追加される

**実際に試してみてください！** 🎯

---

## Slash Commands

### `/Plan on|off`

Toggle plan mode.

```bash
codex /Plan on   # Enter plan mode
codex /Plan off  # Exit plan mode
```

### `/Plan "<title>" [options]`

Create a new Plan.

**Options**:
- `--mode=single|orchestrated|competition` (default: orchestrated)
- `--budget.tokens=<number>` (default: 100000)
- `--budget.time=<minutes>` (default: 30)

**Examples**:
```bash
# Simple feature
codex /Plan "Add logging middleware" --mode=single

# Orchestrated refactor
codex /Plan "Refactor auth system" --mode=orchestrated --budget.tokens=150000

# Performance competition
codex /Plan "Optimize DB query" --mode=competition
```

### `/approve <bp-id>`

Approve a Plan for execution.

```bash
codex /approve bp-2025-11-02T12:00:00Z_add-logging
```

### `/reject <bp-id> --reason="..."`

Reject a Plan with reason.

```bash
codex /reject bp-123 --reason="Scope too broad, split into smaller tasks"
```

### `/Plan export <bp-id> [options]`

Export Plan to file.

**Options**:
- `--format=md|json|both` (default: both)
- `--path=<directory>` (default: docs/Plans)

**Examples**:
```bash
# Export both formats
codex /Plan export bp-123

# Markdown only
codex /Plan export bp-123 --format=md

# Custom path
codex /Plan export bp-123 --path=./my-Plans
```

### `/mode <single|orchestrated|competition>`

Set execution mode.

```bash
codex /mode orchestrated
codex /mode competition
```

### `/deepresearch "<query>" [options]`

Conduct deep research (requires approval).

**Options**:
- `--depth=1|2|3` (default: 2)
- `--policy=focused|comprehensive|exploratory` (default: focused)

**Examples**:
```bash
# Quick research
codex /deepresearch "React Server Components best practices"

# Deep dive
codex /deepresearch "Rust async error handling" --depth=3 --policy=comprehensive
```

---

## GUI Controls

### Status Bar

- **Inactive**: "$(edit) Enter plan mode"
- **Drafting**: "$(edit) Plan: drafting"
- **Pending**: "$(clock) Plan: pending" (amber background)
- **Approved**: "$(check) Plan: approved" (green)
- **Rejected**: "$(x) Plan: rejected" (red background)

Click to toggle plan mode.

### Toolbar Buttons

Located in Plan panel:

1. **Enter Plan** - Toggle mode
2. **Approve** - Approve current Plan
3. **Reject** - Reject with reason
4. **Export** - Export MD/JSON
5. **Mode Selector** - Switch execution strategy

### Keybindings

- `Shift+Tab` - Toggle plan mode (editorTextFocus)
- `Ctrl+Shift+D` - Delegate task to agent
- `Ctrl+Shift+R` - Deep research
- `Ctrl+Shift+C` - Review selected code

---

## Execution Modes

### Single Mode

**Use Case**: Simple, single-file changes

**Behavior**:
- No sub-agents
- Single LLM context
- Fast execution

**Example**:
```bash
codex /Plan "Add docstring to function" --mode=single
```

### Orchestrated Mode (Default)

**Use Case**: Complex, multi-file changes requiring coordination

**Behavior**:
- Central planner generates task DAG
- Specialist sub-agents (Backend/Frontend/DB/Security/QA)
- Integrator merges deterministic diffs
- Tests/linters run before PR

**Example**:
```bash
codex /Plan "Refactor auth system to JWT" --mode=orchestrated
```

**Agents Used**:
- Backend Agent: Core logic
- Database Agent: Schema changes
- Security Agent: Vulnerability review
- QA Agent: Test generation

### Competition Mode

**Use Case**: Performance optimization, algorithm selection

**Behavior**:
- Spawns 2-5 git worktrees (variants A/B/C)
- Executes identical task in parallel
- Runs tests/benchmarks/linters in each
- Auto-scores: Tests (50%) + Perf (30%) + Simplicity (20%)
- Merges winner, archives losers

**Example**:
```bash
codex /Plan "Optimize slow DB query" --mode=competition
```

**Scoring**:
```
| Variant | Tests | Performance | Simplicity | Total | Winner |
|---------|-------|-------------|------------|-------|--------|
| A       | 100.0 | 95.2        | 92.0       | 95.6  | ✅     |
| B       | 100.0 | 98.5        | 75.0       | 92.2  |        |
| C       | 100.0 | 88.0        | 95.0       | 92.6  |        |
```

---

## Deep Research Integration

### Approval Dialog

When you request deep research, an approval dialog shows:

- **Query**: "React Server Components best practices"
- **Depth**: 2
- **Domains**: duckduckgo.com, github.com, docs.rs
- **Token Budget**: ~25,000 tokens
- **Time Budget**: ~3 minutes
- **Data Retention**: 30 days, then auto-deleted

Click **Approve** or **Reject**.

### Research Block

Results appended to Plan:

```markdown
## Research Results

**Query**: React Server Components best practices
**Depth**: 2
**Strategy**: focused
**Confidence**: 0.89

### Sources

- [Next.js Docs](https://nextjs.org/docs/app)
  - Date: 2024-10-15
  - Finding: Use async components for data fetching
  - Confidence: 0.95

### Synthesis

React Server Components enable zero-bundle-size server-side rendering...
```

---

## Webhook Notifications

### GitHub Integration

Sends commit status to GitHub:

```json
{
  "context": "codex/Plan",
  "state": "success",
  "description": "Plan bp-123 approved",
  "target_url": "https://github.com/zapabob/codex/Plans/bp-123"
}
```

**Configuration**:
```json
{
  "codex.webhooks.github.enabled": true
}
```

### Slack Integration

Posts to Slack channel:

> ✅ **Auth System Refactor**
> Plan approved by reviewer!
> 
> **Artifacts**: docs/Plans/2025-11-02_refactor-auth.md

**Configuration**:
```json
{
  "codex.webhooks.slack.enabled": true
}
```

### HTTP Generic

Posts JSON to any endpoint with HMAC signature:

**Headers**:
- `X-Codex-Signature: sha256=abc123...`
- `X-Codex-Event: Plan.approved`

---

## Examples

See `docs/Plans/samples/` for complete examples:

1. **simple-feature.md** - Add logging middleware (single mode)
2. **orchestrated-refactor.md** - JWT auth migration (orchestrated)
3. **competition-optimization.md** - DB query optimization (competition)

---

## FAQ

### Q: Can I modify an approved Plan?

**A**: No. Approved Plans are locked. To make changes, reject it, modify, and re-approve. Or create a new Plan that supersedes the old one.

### Q: What happens if I exceed the budget?

**A**: Execution stops immediately. You'll see a budget exceeded error with current usage stats.

### Q: Can I disable telemetry?

**A**: Yes. Set `codex.telemetry.enabled: false` in VS Code settings. All telemetry is opt-out.

### Q: How do I verify webhook signatures?

**A**: Use HMAC-SHA256 with your webhook secret:

```python
import hmac
import hashlib

def verify_signature(body, signature, secret):
    expected = hmac.new(
        secret.encode(),
        body.encode(),
        hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(f"sha256={expected}", signature)
```

---

## Troubleshooting

### Plan stuck in "pending"

**Solution**: Approve or reject explicitly with `/approve` or `/reject` commands.

### "Approval required" error

**Solution**: Check `codex.research.requireApproval` setting. Network operations require Maintainer role or higher.

### Competition variant merge conflicts

**Solution**: Competition auto-resolves conflicts. If manual intervention needed, check `.codex/worktrees/` for variant branches.

---

## Next Steps

- Read [Execution Modes Guide](./execution-modes.md) for strategy details
- Check [Slash Commands Reference](./slash-commands.md) for full command list
- See [Webhook Setup Guide](./webhooks.md) for integration instructions

---

**Made with ❤️ by zapabob**

