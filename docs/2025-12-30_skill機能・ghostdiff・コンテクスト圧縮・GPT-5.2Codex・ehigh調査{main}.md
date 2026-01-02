# skill機能・ghostdiff・コンテクスト圧縮・GPT-5.2 Codex・ehigh 調査レポート

**作成日時**: 2025-12-30  
**ワークツリー**: main  
**分析手法**: コードベース分析 + DeepResearch  
**対象**: zapabob/codex リポジトリ

---

## 📋 エグゼクティブサマリー

本レポートは、zapabob/codexリポジトリにおける以下の5つの機能・技術について詳細に調査・分析しました：

1. **skill機能** - モジュール化された拡張機能システム
2. **ghostdiff/ghost commit** - Gitリポジトリのスナップショット管理
3. **コンテクスト圧縮** - 会話履歴の効率的な圧縮機能
4. **GPT-5.2 Codex** - OpenAIの最新エージェントモデル
5. **ehigh** - 調査中（情報不足）

---

## 1. skill機能

### 1.1 概要

skill機能は、Codexの能力を拡張するためのモジュール化された自己完結型パッケージシステムです。各skillは特定のドメインやタスクに対する専門知識、ワークフロー、ツール統合を提供します。

### 1.2 アーキテクチャ

#### 実装場所
- **コア実装**: `codex-rs/core/src/skills/`
- **主要モジュール**:
  - `loader.rs` - skillの検出とロード
  - `manager.rs` - skillの管理とキャッシュ
  - `model.rs` - skillメタデータの定義
  - `injection.rs` - skillのコンテキストへの注入
  - `render.rs` - skillセクションのレンダリング

#### Skill構造

```
skill-name/
├── SKILL.md (必須)
│   ├── YAML frontmatter (必須)
│   │   ├── name: (必須)
│   │   └── description: (必須)
│   └── Markdown instructions (必須)
└── Bundled Resources (オプション)
    ├── scripts/          - 実行可能コード
    ├── references/       - ドキュメント（必要に応じてロード）
    └── assets/           - 出力用ファイル（テンプレート、アイコンなど）
```

#### Skillスコープ（優先順位）

1. **Repo** - リポジトリ固有（`.codex/skills/`）
2. **User** - ユーザー固有（`~/.codex/skills/`）
3. **System** - システムキャッシュ（`~/.codex/skills/.system/`）
4. **Admin** - システム全体（`/etc/codex/skills/`、Unixのみ）

### 1.3 主要機能

#### Progressive Disclosure（段階的開示）

skillは3段階のロードシステムでコンテキストを効率的に管理：

1. **メタデータ（name + description）** - 常にコンテキスト内（~100語）
2. **SKILL.md本体** - skillがトリガーされた時のみ（<5k語）
3. **バンドルリソース** - Codexが必要に応じて（無制限、スクリプトは実行可能）

#### Skill検出とトリガー

- **自動検出**: 起動時に複数のローカルソースからskillを発見
- **トリガールール**: 
  - ユーザーがskill名を明示的に言及（`$SkillName`またはプレーンテキスト）
  - タスクがskillの説明と明確に一致
- **複数skill**: 複数の言及がある場合は全て使用

#### Skill作成プロセス

1. **理解**: 具体的な使用例を理解
2. **計画**: 再利用可能なリソース（scripts、references、assets）を計画
3. **初期化**: `init_skill.py`スクリプトでテンプレート生成
4. **編集**: SKILL.mdとリソースを実装
5. **パッケージング**: `package_skill.py`で検証とパッケージ化
6. **反復**: 実際の使用に基づいて改善

### 1.4 実装の詳細

#### SkillLoader (`loader.rs`)

```rust
pub fn load_skills_from_roots<I>(roots: I) -> SkillLoadOutcome
where
    I: IntoIterator<Item = SkillRoot>,
```

- 再帰的にディレクトリを走査して`SKILL.md`を検出
- YAML frontmatterをパースしてメタデータを抽出
- 名前で重複排除（優先順位: repo > user > system > admin）

#### SkillManager (`manager.rs`)

```rust
pub struct SkillsManager {
    codex_home: PathBuf,
    cache_by_cwd: RwLock<HashMap<PathBuf, SkillLoadOutcome>>,
}
```

- CWDごとにskillをキャッシュ
- 強制リロードオプション付き
- スレッドセーフな実装

#### SkillInjection (`injection.rs`)

- 明示的に言及されたskillのみをコンテキストに注入
- skillファイルの読み込みエラーを警告として処理
- `ResponseItem::SkillInstructions`として注入

### 1.5 設計原則

1. **簡潔性**: コンテキストウィンドウは公共財。必要最小限の情報のみ
2. **自由度の設定**: タスクの脆弱性と変動性に応じた適切な自由度
3. **段階的開示**: メタデータ → SKILL.md → リソースの順でロード
4. **再利用性**: スクリプト、参照、アセットの分離

### 1.6 公式リポジトリとの比較

- **OpenAI/codex**: skill機能の実装状況は不明（調査中）
- **zapabob/codex**: 完全な実装、4つのスコープレベル、Progressive Disclosure

---

## 2. ghostdiff / ghost commit

### 2.1 概要

ghost commitは、Gitリポジトリの現在の作業ツリー状態を参照されていないコミットとしてキャプチャする機能です。これにより、`/undo`コマンドで以前の状態に戻ることができます。

### 2.2 アーキテクチャ

#### 実装場所
- **コア実装**: `codex-rs/utils/git/src/ghost_commits.rs`
- **タスク実装**: `codex-rs/core/src/tasks/ghost_snapshot.rs`
- **型定義**: `codex-rs/utils/git/src/lib.rs`

#### 主要構造体

```rust
pub struct GhostCommit {
    id: CommitID,
    parent: Option<CommitID>,
    preexisting_untracked_files: Vec<PathBuf>,
    preexisting_untracked_dirs: Vec<PathBuf>,
}

pub struct CreateGhostCommitOptions<'a> {
    pub repo_path: &'a Path,
    pub message: Option<&'a str>,
    pub force_include: Vec<PathBuf>,
    pub ghost_snapshot: GhostSnapshotConfig,
}
```

### 2.3 主要機能

#### Ghost Commit作成

```rust
pub fn create_ghost_commit(
    options: &CreateGhostCommitOptions
) -> Result<GhostCommit, GitToolingError>
```

- 現在の作業ツリーを一時的なインデックスにステージング
- 参照されていないコミットを作成（`git commit --no-verify`）
- デフォルトメッセージ: `"codex snapshot"`

#### Ghost Commit復元

```rust
pub fn restore_ghost_commit(
    repo_path: &Path,
    ghost: &GhostCommit
) -> Result<(), GitToolingError>
```

- 指定されたghost commitの状態にリポジトリを復元
- 追跡されていないファイルの保護

#### 大規模ファイルの除外

- **デフォルト閾値**:
  - 大規模追跡外ファイル: 10 MiB
  - 大規模追跡外ディレクトリ: 200ファイル
- **自動除外ディレクトリ**: `node_modules`, `.venv`, `venv`, `dist`, `build`, `.cache`, `__pycache__`など

#### 警告システム

- スナップショットが240秒以上かかる場合に警告
- 大規模追跡外ファイル/ディレクトリの除外を通知
- `.gitignore`の更新を推奨

### 2.4 実装の詳細

#### GhostSnapshotTask (`tasks/ghost_snapshot.rs`)

- 非同期タスクとして実行
- ブロッキングプールでGit操作を実行
- キャンセレーション対応
- `ResponseItem::GhostSnapshot`として履歴に記録

#### スナップショットレポート

```rust
pub struct GhostSnapshotReport {
    pub large_untracked_dirs: Vec<LargeUntrackedDir>,
    pub ignored_untracked_files: Vec<IgnoredUntrackedFile>,
}
```

- 除外されたファイルとディレクトリの詳細情報
- ユーザーへの警告メッセージ生成

### 2.5 使用例

```rust
// Ghost commitの作成
let ghost = create_ghost_commit(&CreateGhostCommitOptions::new(repo))?;

// 後で元の状態に戻す
restore_ghost_commit(repo, &ghost)?;
```

### 2.6 公式リポジトリとの比較

- **OpenAI/codex**: ghost commit機能の実装状況は不明（調査中）
- **zapabob/codex**: 完全な実装、警告システム、大規模ファイル除外

---

## 3. コンテクスト圧縮（Context Compression）

### 3.1 概要

コンテクスト圧縮は、長い会話履歴を効率的に圧縮して、コンテキストウィンドウの制限内に収める機能です。2つの実装方式があります：

1. **Inline Compaction** - ローカルで要約を生成
2. **Remote Compaction** - OpenAI APIの`compact_conversation_history`を使用

### 3.2 アーキテクチャ

#### 実装場所
- **Inline実装**: `codex-rs/core/src/codex/compact.rs`
- **Remote実装**: `codex-rs/core/src/compact_remote.rs`
- **テンプレート**: `codex-rs/core/src/templates/compact/`

#### 主要関数

```rust
pub(crate) async fn run_compact_task(
    sess: Arc<Session>,
    turn_context: Arc<TurnContext>,
    input: Vec<UserInput>,
)

pub(crate) async fn run_remote_compact_task(
    sess: Arc<Session>,
    turn_context: Arc<TurnContext>
)
```

### 3.3 実装方式

#### Inline Compaction

1. **要約プロンプト**: `SUMMARIZATION_PROMPT`を使用してLLMに要約を依頼
2. **履歴構築**: 
   - 初期コンテキスト（システムプロンプトなど）
   - ユーザーメッセージ（最大20,000トークン、古いものから選択）
   - 要約テキスト（最後のアシスタントメッセージ）
3. **Ghost Snapshot保持**: 圧縮後も`GhostSnapshot`アイテムを保持（`/undo`のため）

#### Remote Compaction

1. **API呼び出し**: `client.compact_conversation_history(&prompt)`
2. **履歴置換**: APIが返した圧縮済み履歴で置き換え
3. **Ghost Snapshot保持**: Inlineと同様

### 3.4 実装の詳細

#### 圧縮履歴の構築

```rust
pub(crate) fn build_compacted_history(
    initial_context: Vec<ResponseItem>,
    user_messages: &[String],
    summary_text: &str,
) -> Vec<ResponseItem>
```

- ユーザーメッセージを古いものから選択（最大80,000バイト）
- 要約テキストを最後に追加
- Ghost Snapshotを保持

#### コンテキストウィンドウ超過時の処理

- 最古の履歴アイテムを削除
- リトライ（最大リトライ回数まで）
- エラー時は警告を表示

#### 自動圧縮

- コンテキストウィンドウが一定の閾値を超えた場合に自動実行
- `run_inline_auto_compact_task`または`run_inline_remote_auto_compact_task`が呼ばれる

### 3.5 使用例

```rust
// 手動で圧縮を実行
run_compact_task(session, turn_context, vec![]).await;

// Remote圧縮（OpenAI API使用）
if should_use_remote_compact_task(&session, &provider) {
    run_remote_compact_task(session, turn_context).await;
}
```

### 3.6 公式リポジトリとの比較

- **OpenAI/codex**: コンテキスト圧縮機能の実装状況は不明（調査中）
- **zapabob/codex**: InlineとRemoteの両方を実装、自動圧縮対応

### 3.7 GPT-5.2 CodexのNative Context Compaction

OpenAIのGPT-5.2 Codexは「Native Context Compaction」という独自のアーキテクチャ的ブレークスルーを導入：

- **機能**: 履歴セッションデータをトークン効率的な「スナップショット」に圧縮
- **効果**: 24時間以上の単一タスクで自律動作が可能（「忘却」やコンテキストドリフトなし）
- **実装**: プロプライエタリなアーキテクチャ（詳細は非公開）

**zapabob/codexとの比較**:
- zapabob版は標準的な要約ベースの圧縮を実装
- GPT-5.2 CodexのNative Context Compactionはモデル内部の機能
- 将来的にzapabob版でも同様の機能を統合可能

---

## 4. GPT-5.2 Codex

### 4.1 概要

GPT-5.2 Codexは、OpenAIが2025年12月18日にリリースした、GPT-5.2モデルファミリーの専門化された進化版です。コーディングアシスタントから完全自律的なソフトウェアエンジニアリングエージェントへの移行を目的としています。

### 4.2 主要機能

#### 1. Long-Horizon Task Execution（長期タスク実行）

- 複数日にわたるセッションで複雑なリポジトリを管理
- システム全体のリファクタリングを自律的に実行
- セキュリティ脆弱性を自律的に解決

#### 2. Native Context Compaction（ネイティブコンテクスト圧縮）

- 履歴セッションデータをトークン効率的な「スナップショット」に圧縮
- 24時間以上の単一タスクで自律動作が可能
- 「忘却」やコンテキストドリフトを防止

#### 3. Multimodal Vision（マルチモーダル視覚）

- アーキテクチャ図、フローチャート、Figma UIモックアップを直接取り込み
- ReactやNext.jsのプロトタイプに直接変換
- システム設計の構造的論理欠陥を事前に特定

#### 4. Persistent Mental Map（永続的なメンタルマップ）

- 大規模コードベースの「メンタルマップ」を維持
- コードベース全体の構造と関係性を理解

### 4.3 パフォーマンスベンチマーク

#### SWE-Bench Pro

- **スコア**: 56.4%（記録的な精度）
- **テスト内容**: 大規模で不慣れなソフトウェア環境内で実際のGitHubイシューを解決

#### SWE-Bench Verified

- **GPT-5.2 Codex**: 80.0%
- **Claude 4.5 Opus**: 80.9%（わずかにリード）

#### Terminal-Bench 2.0

- **スコア**: 64.0%
- **特徴**: ライブターミナル環境のナビゲーション、コードのコンパイル、サーバー設定の管理に優れる

### 4.4 セキュリティ機能

#### 脆弱性検出

- **CVE-2025-55182**（React2Shell）の発見とパッチ適用
- 3つの追加ゼロデイ脆弱性を発見:
  - CVE-2025-55183（ソースコード露出）
  - CVE-2025-55184
  - CVE-2025-67779（重大なDoS欠陥）

#### Trusted Access Pilot

- 招待制の「Trusted Access Pilot」を開始
- 審査済みセキュリティ専門家に最も許可的な機能へのアクセスを提供
- 攻撃的悪用に対する厳格な監視

### 4.5 市場への影響

#### エンタープライズ統合

- **Cisco**: エンジニアリングパイプラインの加速
- **Duolingo**: 複雑な機能の出荷時間を40%削減
- **Microsoft**: GitHubエコシステムへの統合

#### 競合との比較

- **Google Gemini 3 Pro**: 100万トークン以上のコンテキストウィンドウ
- **Anthropic Claude**: 優れた「推論と設計」能力
- **OpenAIの優位性**: 「エージェント的自律性」に焦点

### 4.6 将来の展望

#### 2026年の予測

- **ビデオベースUIデバッグ**: ユーザーがWebアプリでバグを体験する様子を観察し、スタックを遡って特定のコード行を特定
- **AGI（人工汎用知能）**: ソフトウェアエンジニアリングドメインでのAGI達成

#### 課題

- 安全クリティカルシステムでのAI生成コードの信頼性
- 自律生成時代における著作権とコード所有権の法的複雑さ

### 4.7 zapabob/codexとの関係

- **zapabob/codex**: OpenAI/codexのフォークで、独自の機能を追加
- **GPT-5.2 Codex**: OpenAIの最新モデル（zapabob/codexが使用可能なモデルの一つ）
- **統合可能性**: zapabob/codexはGPT-5.2 Codexをモデルプロバイダーとして使用可能

---

## 5. ehigh

### 5.1 調査結果

**結論**: コードベース内および外部リソースで「ehigh」に関する情報が見つかりませんでした。

### 5.2 可能性のある解釈

1. **タイプミス**: 他の用語の誤字の可能性
2. **内部コード名**: プロジェクト固有の内部コード名
3. **未実装機能**: 計画中または開発中の機能
4. **別の文脈**: 医療情報システム（EHR）関連の可能性（Web検索結果より）

### 5.3 推奨事項

- ユーザーに「ehigh」の正確な意味や文脈を確認
- 関連する機能名や用語の確認
- 追加の調査が必要

---

## 📊 総合比較表

| 機能 | zapabob/codex実装状況 | 公式リポジトリ | 技術的優位性 |
|------|---------------------|--------------|------------|
| **skill機能** | ✅ 完全実装 | ❓ 不明 | Progressive Disclosure、4スコープレベル |
| **ghost commit** | ✅ 完全実装 | ❓ 不明 | 警告システム、大規模ファイル除外 |
| **コンテクスト圧縮** | ✅ Inline + Remote | ❓ 不明 | 2方式対応、自動圧縮 |
| **GPT-5.2 Codex統合** | ✅ 可能 | ✅ 公式 | モデルプロバイダーとして使用可能 |
| **ehigh** | ❌ 情報なし | ❌ 情報なし | 調査が必要 |

---

## 🔍 技術的詳細

### skill機能の実装ファイル

- `codex-rs/core/src/skills/loader.rs` - skill検出とロード
- `codex-rs/core/src/skills/manager.rs` - skill管理とキャッシュ
- `codex-rs/core/src/skills/model.rs` - メタデータ定義
- `codex-rs/core/src/skills/injection.rs` - コンテキスト注入
- `codex-rs/core/src/skills/render.rs` - レンダリング
- `codex-rs/core/src/skills/assets/samples/skill-creator/SKILL.md` - 作成ガイド

### ghost commitの実装ファイル

- `codex-rs/utils/git/src/ghost_commits.rs` - コア実装
- `codex-rs/core/src/tasks/ghost_snapshot.rs` - タスク実装
- `codex-rs/utils/git/src/lib.rs` - 型定義

### コンテクスト圧縮の実装ファイル

- `codex-rs/core/src/codex/compact.rs` - Inline実装
- `codex-rs/core/src/compact_remote.rs` - Remote実装
- `codex-rs/core/src/templates/compact/` - プロンプトテンプレート

---

## 🎯 結論

1. **skill機能**: zapabob/codexに完全に実装されており、Progressive Disclosureと4つのスコープレベルをサポート
2. **ghost commit**: 完全な実装、警告システム、大規模ファイル除外機能
3. **コンテクスト圧縮**: InlineとRemoteの両方式を実装、自動圧縮対応
4. **GPT-5.2 Codex**: OpenAIの最新モデル、zapabob/codexで使用可能
5. **ehigh**: 情報が見つからず、追加調査が必要

---

## 📚 参考文献

- `codex-rs/core/src/skills/` - skill機能の実装
- `codex-rs/utils/git/src/ghost_commits.rs` - ghost commitの実装
- `codex-rs/core/src/codex/compact.rs` - コンテクスト圧縮の実装
- OpenAI GPT-5.2-Codex Launch記事（2025-12-25）
- `_docs/2025-12-30_公式リポジトリとzapabobリポジトリ比較分析{main}.md`

---

**レポート作成者**: Codex AI Agent  
**最終更新**: 2025-12-30
