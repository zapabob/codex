# skills API 同期＆GUITUICLI再編成プラン（zapabob@codex）

## 背景
- 公式 Codex skills API の最新仕様に追従し、脆弱性修正とバグフィックスを取り込む。
- zapabob 版で提供している QC エージェント、GUI/TUI/CLI 統合、Windows 11 25H2 カーネル統合、mac 風仮想 OS（CLI からコード生成アプリを組み立て可能・インターネット接続可能）、マルウェア検知/隔離/削除機能を新 API ベースで整理する。
- 目標は **GUITUICLI が公式 API 互換で動作し、独自機能も温存する** こと。

## 方針
1. **skills API トラッキング**
   - `codex` 公式リリースの skills 変更点（スキーマ拡張、権限モデルの更新、セキュリティパッチ）を定期的に diff 取得し、zapabob 派生ブランチへ週次で取り込む。
   - API バージョンを環境変数 `CODEX_SKILLS_API_VERSION` で固定し、GUI/TUI/CLI すべてで共通のクライアントを利用。
2. **依存ライブラリの脆弱性更新**
   - Node: `pnpm audit`/`pnpm up --latest` を月次実行し、skills SDK・GUI コンポーネントを最新版へ。
   - Rust: `cargo update -p codex-*` で codex-rs 系の skills 連携 crate を更新し、`just fix -p codex-tui` でリンター整合性を担保。
3. **独自機能の skills 化**
   - QC エージェント: skills の tool 定義へ分割（解析、レビュー、報告）し、GUI/TUI/CLI から `skills.invoke("qc/*")` で呼び出す。
   - Windows 11 25H2 カーネル統合: VM/コンテナ制御を skills の `system/*` ツールとして公開し、GUI のボタン操作と CLI のコマンドを同一エンドポイントで処理。
   - mac 風仮想 OS & コード生成アプリ: 仮想 OS 制御とコード生成を `workspace/*` スキルへまとめ、CLI からのテンプレート生成コマンドを GUI/TUI のランチャーからも呼べるようにする。
   - マルウェア検知/隔離/削除: スキャン、隔離、削除をそれぞれ `security/scan|quarantine|clean` skills として登録し、全 UI で統一したイベントログを残す。
4. **GUITUICLI の共通クライアント設計**
   - 共通の skills クライアントを `packages/` の SDK レイヤーに配置し、GUI（Electron/Next.js）、TUI（codex-rs/tui）、CLI（codex-cli）で依存。
   - 認証・レートリミット・権限チェックは SDK で吸収し、UI 側では最小限の設定で動作させる。
5. **テレメトリと監査ログ**
   - skills 呼び出し結果を JSONL で永続化し、QC/セキュリティ系 skills は必ず出典付きで記録。
   - GUI/TUI ではステータスバッジ、CLI では `--audit-log` で同一のログを確認できるようにする。
6. **検証フロー**
   - 単体: skills クライアントの契約テスト（mock サーバー）。
   - 統合: `test_gui_cli_integration_fixed.py` を skills API v 最新で実行し、GUI/TUI/CLI の共通動作を確認。
   - 回帰: マルウェア対策 skills で EICAR 互換サンプルを用いた隔離・削除フローを CI で検証。

## 実装ステップ（最短経路）
- **ベースライン同期**: 公式 skills API の最新バージョンとスキーマを `packages/skills-client` に反映し、`CODEX_SKILLS_API_VERSION` を `.env`/CI で明示する。
- **共通クライアント抽象化**: GUI/TUI/CLI で使う薄い Facade（例: `SkillsClientFacade`）を SDK に用意し、認証・リトライ・レートリミットを統一。
- **独自機能の skills 定義化**: QC/セキュリティ/仮想 OS/コード生成の各機能を skill manifest に分解し、`skills.invoke` 経路に揃える。既存の GUI ボタン・CLI コマンドは Facade 経由で再配線。
- **テレメトリ配線**: JSONL 監査ログをクライアント層に実装し、UI 層はイベントを購読するだけにする（GUI: badge 更新、CLI: `--audit-log`）。
- **テストとゲーティング**: 契約テスト → GUI/TUI/CLI 統合テスト → マルウェア回帰（EICAR 互換）を GitHub Actions で段階的に実行し、すべて緑でない限りリリースブランチへマージしない。

## 互換性とリスク低減
- **API 互換層**: skills API 変更で互換性が崩れる場合に備え、Facade 内でリクエスト/レスポンスを正規化し、UI 側の破壊的変更を避ける。
- **フェイルセーフ**: マルウェア関連 skills は失敗時に「隔離優先」でフォールバックし、UI に明示的に通知する。QC skills はタイムアウト時に保留状態を返し、再実行を促す。
- **権限ガード**: Windows 11 25H2 カーネル統合や仮想 OS 制御は sandboxed execution をデフォルトにし、権限昇格は skills のポリシーで明示的にチェックする。
- **ログ整合性**: 監査ログはすべての呼び出しで session ID と request ID を必須にし、UI からも参照できるようにする。

## マイルストーン（例）
1. **M1: 同期基盤整備（週次取り込みと Facade 実装）** — API バージョン固定・Facade 提供・契約テスト通過。
2. **M2: 独自機能を skills 化** — QC/セキュリティ/仮想 OS/コード生成の skill manifest 追加と UI 再配線。
3. **M3: テレメトリ統合と回帰網羅** — JSONL 監査ログを GUI/TUI/CLI で可視化し、E2E とマルウェア回帰を CI 化。
4. **M4: リリースゲート** — 監査ログの整合性チェックと全テスト緑を確認後、リリースブランチへマージ。

## 作業チェックリスト
- [ ] skills API バージョンを `CODEX_SKILLS_API_VERSION` で固定し、全 UI で同一値を使用
- [ ] 共通 skills クライアントを SDK 層に配置（GUI/TUI/CLI で共有）
- [ ] QC/セキュリティ/仮想 OS 関連の独自機能を skills 定義へ分解
- [ ] 脆弱性パッチ適用（pnpm audit fix / cargo update）
- [ ] GUITUICLI 統合テストを skills v 最新で完走
- [ ] 監査ログとテレメトリを skills 呼び出し単位で保存
