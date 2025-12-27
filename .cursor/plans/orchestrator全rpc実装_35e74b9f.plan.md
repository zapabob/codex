---
name: Orchestrator全RPC実装
overview: Orchestratorサーバの全RPCメソッドを実装し、既存のPlanManager、RepositoryLock、GitLockManagerなどの機能と統合します。
todos:
  - id: lock-status
    content: lock.statusメソッドの完全実装（RepositoryLock::status()を使用）
    status: completed
  - id: lock-acquire
    content: lock.acquireメソッドの実装（RepositoryLock::acquire()を使用、409エラー対応）
    status: completed
  - id: lock-release
    content: lock.releaseメソッドの実装（RepositoryLockのリリース機能を使用）
    status: completed
  - id: fs-read
    content: fs.readメソッドの実装（ファイル読み込み、パス検証）
    status: completed
  - id: fs-write
    content: fs.writeメソッドの実装（preimage SHA256検証、アトミック書き込み）
    status: completed
  - id: fs-patch
    content: fs.patchメソッドの実装（unified diffのパースと適用）
    status: completed
  - id: vcs-diff
    content: vcs.diffメソッドの実装（git2を使用）
    status: completed
  - id: vcs-commit
    content: vcs.commitメソッドの実装（git2を使用）
    status: completed
  - id: vcs-push
    content: vcs.pushメソッドの実装（git2を使用）
    status: completed
  - id: agent-heartbeat
    content: agent.heartbeatメソッドの実装（タイムスタンプ更新、タイムアウト検出）
    status: completed
  - id: task-cancel
    content: task.cancelメソッドの実装（タスクキャンセル処理）
    status: completed
  - id: session-start
    content: session.startメソッドの実装（SessionInfo構造体の追加とセッション管理）
    status: completed
  - id: session-end
    content: session.endメソッドの実装（セッション終了処理）
    status: completed
  - id: pubsub-subscribe
    content: pubsub.subscribeメソッドの実装（トピック購読登録）
    status: completed
  - id: pubsub-unsubscribe
    content: pubsub.unsubscribeメソッドの実装（購読解除）
    status: completed
  - id: blueprint-get
    content: blueprint.getメソッドの完全実装（PlanManager::get_Plan()を使用）
    status: completed
  - id: blueprint-create
    content: blueprint.createメソッドの完全実装（PlanManager::create_Plan()を使用）
    status: completed
  - id: blueprint-update
    content: blueprint.updateメソッドの完全実装（PlanManager::update_Plan()を使用）
    status: completed
  - id: blueprint-approve
    content: blueprint.approveメソッドの完全実装（PlanManager::approve_Plan()を使用）
    status: completed
  - id: blueprint-reject
    content: blueprint.rejectメソッドの完全実装（PlanManager::reject_Plan()を使用）
    status: completed
  - id: blueprint-export
    content: blueprint.exportメソッドの完全実装（PlanManager::export_Plan()を使用）
    status: completed
  - id: blueprint-setmode
    content: blueprint.setModeメソッドの実装（グローバルモード設定）
    status: completed
  - id: blueprint-addresearch
    content: blueprint.addResearchメソッドの完全実装（PlanManager::add_research()を使用）
    status: completed
  - id: queue-size-tracking
    content: queue_sizeの実際のサイズ追跡とstatus.getへの反映
    status: completed
  - id: event-publishing
    content: イベント発行機能の追加（各write操作後の購読者への通知）
    status: completed
---

# Orchestratorサーバ全RPC実装計画

## 現状分析

### 実装済みメソッド

- `status.get` ✅ (queue_sizeの追跡はTODO)
- `agent.list` ✅
- `tokens.getBudget` ✅
- `agent.register` ✅
- `task.submit` ✅
- `tokens.reportUsage` ✅

### 部分的実装（TODOあり）

- `lock.status` - スタブ実装のみ
- `blueprint.get` - スタブ実装のみ
- `blueprint.create` - スタブ実装のみ
- `blueprint.update` - スタブ実装のみ
- `blueprint.approve` - スタブ実装のみ
- `blueprint.reject` - スタブ実装のみ
- `blueprint.export` - スタブ実装のみ
- `blueprint.setMode` - スタブ実装のみ
- `blueprint.addResearch` - スタブ実装のみ

### 未実装メソッド

- `lock.acquire`
- `lock.release`
- `fs.read`
- `fs.write`
- `fs.patch`
- `vcs.diff`
- `vcs.commit`
- `vcs.push`
- `agent.heartbeat`
- `task.cancel`
- `session.start`
- `session.end`
- `pubsub.subscribe`
- `pubsub.unsubscribe`

## 実装計画

### Phase 1: Lock機能の完全実装

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **lock.status の実装**

- `codex-rs/core/src/lock.rs`の`RepositoryLock::status()`を使用
- パス指定のサポート
- ロックホルダー情報の取得

2. **lock.acquire の実装**

- `RepositoryLock::acquire()`を使用
- forceオプションの処理
- 競合時の409エラー返却

3. **lock.release の実装**

- `RepositoryLock`のリリース機能を使用
- ロック所有者の検証

### Phase 2: Filesystem操作の実装

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **fs.read の実装**

- `std::fs::read_to_string()`を使用
- パス検証とエラーハンドリング

2. **fs.write の実装**

- `std::fs::write()`を使用
- preimage SHA256検証（競合検出）
- アトミック書き込み

3. **fs.patch の実装**

- unified diffのパース
- ファイルへの適用
- base_commitの検証

### Phase 3: VCS操作の実装

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **vcs.diff の実装**

- `git2`クレートを使用してdiff取得
- 作業ディレクトリの検出

2. **vcs.commit の実装**

- `git2`を使用してコミット作成
- メッセージの検証

3. **vcs.push の実装**

- `git2`を使用してリモートへプッシュ
- 認証情報の処理

### Phase 4: Agent/Task機能の拡張

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **agent.heartbeat の実装**

- 既存の`active_agents`を更新
- タイムスタンプの更新
- タイムアウト検出

2. **task.cancel の実装**

- タスクのキャンセル処理
- ステータスの更新

### Phase 5: Session管理の実装

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **Session構造体の追加**

- `active_sessions: Arc<RwLock<HashMap<String, SessionInfo>>>`
- `SessionInfo`構造体の定義

2. **session.start の実装**

- セッションの作成と登録
- 作業ディレクトリの設定

3. **session.end の実装**

- セッションの終了処理
- リソースのクリーンアップ

### Phase 6: PubSub機能の実装

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **pubsub.subscribe の実装**

- トピックへの購読登録
- 接続IDとトピックのマッピング

2. **pubsub.unsubscribe の実装**

- 購読の解除

3. **イベント発行機能の追加**

- 各write操作後にイベントを発行
- 購読者への通知

### Phase 7: Blueprint機能の完全実装

**ファイル**: `codex-rs/orchestrator/src/server.rs`

1. **PlanManagerの統合**

- `codex-rs/core/src/plan/manager.rs`の`PlanManager`を使用
- シングルトンまたは共有インスタンスの管理

2. **blueprint.get の実装**

- `PlanManager::get_Plan()`を使用
- JSON形式での返却

3. **blueprint.create の実装**

- `PlanManager::create_Plan()`を使用
- モードと予算の設定

4. **blueprint.update の実装**

- `PlanManager::update_Plan()`を使用
- 部分更新の処理

5. **blueprint.approve の実装**

- `PlanManager::approve_Plan()`を使用
- 承認者の記録

6. **blueprint.reject の実装**

- `PlanManager::reject_Plan()`を使用
- 却下理由の記録

7. **blueprint.export の実装**

- `PlanManager::export_Plan()`を使用
- フォーマット指定の処理

8. **blueprint.setMode の実装**

- グローバルモード設定の実装
- 設定ファイルへの保存

9. **blueprint.addResearch の実装**

- `PlanManager::add_research()`を使用
- ResearchBlockの追加

### Phase 8: 改善と最適化

1. **queue_sizeの追跡**

- `write_queue`の実際のサイズを追跡
- `status.get`レスポンスに反映

2. **エラーハンドリングの強化**

- 適切なエラーコードの返却
- エラーメッセージの改善

3. **テストの追加**

- 各メソッドのユニットテスト
- 統合テスト

## 実装の優先順位

1. **高優先度**: Lock機能、Filesystem操作、VCS操作（基盤機能）
2. **中優先度**: Agent/Task拡張、Session管理
3. **低優先度**: PubSub機能、Blueprint機能（既存PlanManager統合）

## 技術的な考慮事項

- **既存コードの活用**: `codex-rs/core`の既存実装を最大限活用
- **エラーハンドリング**: 適切なRPCエラーコードの返却（409, 429など）
- **非同期処理**: すべての操作は非同期で実装
- **型安全性**: Rustの型システムを活用した安全な実装
- **テスト**: 各メソッドに対してユニットテストを追加

## 依存関係

- `codex-rs/core`: PlanManager, RepositoryLock, GitLockManager
- `git2`: VCS操作
- `serde_json`: JSON処理
- `tokio`: 非同期処理