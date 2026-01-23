# ClaudeCowork統合ガイド

## 概要

Codex Extended v2.11.0では、ClaudeCoworkと同等の機能を完全統合しました。このガイドでは、統合機能の使用方法、設定方法、ベストプラクティスを説明します。

## 前提条件

### 必要なソフトウェア

- Python 3.x
- Playwright（自動インストール）
- Tesseract OCR（OCR機能を使用する場合）

### 依存ライブラリのインストール

```bash
# すべての依存ライブラリをインストール
python scripts/install_cowork_dependencies.py
```

このスクリプトは以下をインストールします：
- Playwrightとブラウザ
- ドキュメント生成ライブラリ（openpyxl, python-docx, python-pptx）
- データ分析ライブラリ（pandas, numpy, matplotlib, seaborn）
- 外部サービス統合ライブラリ（aiohttp, requests）
- Tesseract OCRのインストール状態確認

## 主要機能

### 1. ブラウザ自動化

視覚的理解、マルチタブ操作、UI要素自動認識を備えた高度なブラウザ自動化機能。

#### 使用方法

```python
from scripts.cowork_browser_automation import EnhancedBrowserAutomationEngine

engine = EnhancedBrowserAutomationEngine(headless=False)
await engine.initialize()

# タブグループ作成
group = await engine.create_tab_group("my_group")

# タブ追加
page = await engine.add_tab_to_group("my_group", "https://example.com")

# 視覚要素分析
elements = await engine.analyze_visual_elements(page)

# ワークフロー実行
workflow = [
    {"type": "navigate", "data": {"url": "https://example.com"}},
    {"type": "click", "data": {"selector": "button.submit"}},
    {"type": "fill_form", "data": {"form_data": {"name": "Test", "email": "test@example.com"}}}
]
result = await engine.execute_workflow(page, workflow)

await engine.close()
```

### 2. ドキュメント生成

Excel（数式、グラフ）、Word（スタイル、目次）、PowerPoint（テンプレート）の生成機能。

#### 使用方法

```python
from scripts.cowork_document_generator import DocumentGenerationEngine

engine = DocumentGenerationEngine()

# Excel生成
excel_data = {
    "sheets": [{
        "name": "Sales",
        "rows": [["Product", "Q1", "Q2"], ["A", 100, 120]],
        "formulas": {"B3": "=SUM(B2:B3)"},
        "styles": {"header": {"row": 1, "bg_color": "366092"}},
        "charts": [{"type": "bar", "data_range": "A1:B2", "title": "Sales Chart"}]
    }]
}
result = engine.generate_excel("output.xlsx", excel_data)

# Word生成
word_content = {
    "title": "レポート",
    "sections": [{
        "heading": "概要",
        "paragraphs": ["これはテストレポートです"],
        "style": {"bold": True}
    }]
}
result = engine.generate_word("output.docx", word_content)

# PowerPoint生成
ppt_data = {
    "title": "プレゼンテーション",
    "slides": [{
        "title": "スライド1",
        "content": "コンテンツ",
        "paragraphs": ["ポイント1", "ポイント2"]
    }]
}
result = engine.generate_powerpoint("output.pptx", ppt_data)
```

### 3. 外部サービス統合

Asana、Notion、PayPal/Stripe、Canvaなどの外部サービスとの統合。

#### 使用方法

```python
from scripts.cowork_connectors.asana_connector import AsanaConnector
from scripts.cowork_connectors.notion_connector import NotionConnector

# Asana統合
asana = AsanaConnector(api_key="your_api_key")
await asana.connect()

# タスク作成
result = await asana.create_task(
    workspace_id="workspace_id",
    name="新規タスク",
    notes="タスクの説明",
    assignee="user_id"
)

# Notion統合
notion = NotionConnector(api_key="your_api_key")
await notion.connect()

# ページ作成
result = await notion.create_page(
    parent_database_id="database_id",
    properties={"Name": {"title": [{"text": {"content": "新規ページ"}}]}}
)
```

### 4. セッション管理

セッションの作成・管理、ファイルプレビュー、タスク履歴管理。

#### 使用方法

```python
from scripts.cowork_session_manager import SessionManager

manager = SessionManager()

# セッション作成
session = manager.create_session("プロジェクトA", {"description": "説明"})

# タスク追加
task = {"name": "タスク1", "status": "pending"}
manager.add_task(session.id, task)

# ファイル追加
manager.add_file(session.id, "path/to/file.txt")

# ファイルプレビュー
preview = manager.preview_file("path/to/file.txt")

# セッション一覧
sessions = manager.list_sessions()
```

### 5. Rust-Python統合ブリッジ

Codexコア（Rust）からPythonスクリプトを呼び出す統合ブリッジ。

#### Rust側の使用方法

```rust
use codex_core::cowork_integration::{CoworkIntegrationManager, CoworkIntegrationConfig, BrowserAutomationTask};

let config = CoworkIntegrationConfig::default();
let manager = CoworkIntegrationManager::new(config);

// ブラウザ自動化実行
let task = BrowserAutomationTask {
    action: "navigate".to_string(),
    url: Some("https://example.com".to_string()),
    selector: None,
    form_data: None,
    workflow: None,
};
let result = manager.execute_browser_automation(task).await?;
```

## セキュリティ

### プロンプトインジェクション対策

すべてのCowork機能には、多層プロンプトインジェクション対策が組み込まれています。

- **入力検証**: ユーザー入力を検証し、悪意のあるパターンを検出
- **入力サニタイズ**: 危険なパターンを安全な形式に変換
- **セキュリティレベル**: MINIMAL、STANDARD、STRICT、MAXIMUMから選択可能

### セキュアサンドボックス

- 承認されたフォルダのみアクセス
- ファイル操作の監査ログ
- リソース制限付き実行環境

## パフォーマンス最適化

### キャッシュシステム

- インテリジェントキャッシュ（LRU方式）
- TTL（Time To Live）設定可能
- 自動キャッシュ無効化

### 並列処理

- 同時実行数制限
- リソース管理
- パフォーマンス監視

## GUI統合

### Apple風デザインGUI

```bash
# GUI起動
python scripts/cowork_apple_gui.py
```

**機能**:
- 機能検索とインテリジェント実行
- デスクトップショートカット自動作成
- タスクトレイ常駐機能
- ダークモード対応

### デスクトップショートカット

GUI起動時に自動的にデスクトップショートカットが作成されます。

## テスト

### 統合テスト実行

```bash
# 全テスト実行
python scripts/test_cowork_integration.py
```

**テスト項目**:
- セッション管理
- ドキュメント生成
- ブラウザ自動化
- 外部サービスコネクター

## トラブルシューティング

### Tesseract OCRのインストール

ブラウザ自動化のOCR機能を使用するには、Tesseract OCRのインストールが必要です。

#### 自動インストール（推奨）

```powershell
# PowerShellで実行
.\scripts\install_tesseract.ps1
```

#### 手動インストール

**方法1: Chocolatey**
```powershell
choco install tesseract
```

**方法2: winget**
```powershell
winget install UB-Mannheim.TesseractOCR
```

**方法3: 手動ダウンロード**
1. https://github.com/UB-Mannheim/tesseract/wiki からインストーラーをダウンロード
2. インストーラーを実行
3. 環境変数PATHに `C:\Program Files\Tesseract-OCR` を追加
4. PowerShellを再起動して確認: `tesseract --version`

#### インストール確認

```powershell
# インストール状態を確認
.\scripts\install_tesseract.ps1 -CheckOnly
```

### Avast誤検知対策

```powershell
# 管理者権限で実行
.\scripts\setup_avast_exclusions.ps1 -AddExclusions
```

### パフォーマンス問題

- キャッシュサイズの調整
- 同時実行数の制限
- リソース使用量の監視

## ベストプラクティス

1. **セキュリティ**: 常にSTRICT以上のセキュリティレベルを使用
2. **パフォーマンス**: キャッシュを活用してAPI呼び出しを最小化
3. **エラーハンドリング**: すべての操作で適切なエラーハンドリングを実装
4. **ログ記録**: 重要な操作はログに記録
5. **リソース管理**: 同時実行数を適切に制限

## 関連ドキュメント

- [アーキテクチャ設計書](./architecture/claudecowork-integration.md)
- [ユーザーガイド](./user-guide.md)
- [APIリファレンス](./api-reference.md)
