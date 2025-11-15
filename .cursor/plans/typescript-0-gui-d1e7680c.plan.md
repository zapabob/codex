<!-- d1e7680c-cc20-47da-a63c-d4adaa1434c2 cbb31454-3999-4b3c-9b66-688e7a27cbbc -->
# TypeScript警告0検証とGUI統合テスト実装計画

## 1. TypeScript警告0検証

### 1.1 依存関係のインストール

**ファイル**: `codex-rs/tauri-gui/package.json`

- `npm install`を実行して依存関係をインストール
- 不足している型定義パッケージを確認

### 1.2 ESLint設定の追加

**新規ファイル**: `codex-rs/tauri-gui/.eslintrc.json`

- ESLint + TypeScript ESLintの設定を追加
- React用のルールを有効化
- 未使用変数・インポートの検出を有効化

**依存関係追加**: `codex-rs/tauri-gui/package.json`

```json
{
  "devDependencies": {
    "eslint": "^8.57.0",
    "@typescript-eslint/eslint-plugin": "^6.19.0",
    "@typescript-eslint/parser": "^6.19.0",
    "eslint-plugin-react": "^7.33.2",
    "eslint-plugin-react-hooks": "^4.6.0"
  }
}
```

**スクリプト追加**: `codex-rs/tauri-gui/package.json`

```json
{
  "scripts": {
    "lint": "eslint src --ext .ts,.tsx",
    "lint:fix": "eslint src --ext .ts,.tsx --fix"
  }
}
```

### 1.3 型エラーと警告の修正

**実行コマンド**:

```bash
cd codex-rs/tauri-gui
npm install
npm run type-check
npm run lint
```

**修正対象ファイル**:

- `src/App.tsx` - 未使用変数・インポート、型エラー
- `src/components/security/*.tsx` - 型エラー、未使用変数
- その他のTypeScriptファイル

**修正内容**:

- 未使用変数・インポートの削除
- `any`型の明示的な型指定への変更
- 暗黙的な型エラーの修正
- JSX要素の型エラー修正

## 2. GUI統合テスト実装（Vitest + React Testing Library）

### 2.1 Vitest設定

**依存関係追加**: `codex-rs/tauri-gui/package.json`

```json
{
  "devDependencies": {
    "vitest": "^1.2.0",
    "@vitest/ui": "^1.2.0",
    "@testing-library/react": "^14.1.2",
    "@testing-library/jest-dom": "^6.1.5",
    "@testing-library/user-event": "^14.5.1",
    "jsdom": "^23.0.1"
  }
}
```

**Vitest設定**: `codex-rs/tauri-gui/vite.config.ts`を拡張

- Vitestプラグインの追加
- jsdom環境の設定
- テストファイルのパターン設定

**スクリプト追加**: `codex-rs/tauri-gui/package.json`

```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

### 2.2 テストユーティリティ

**新規ファイル**: `codex-rs/tauri-gui/src/test-utils.tsx`

- Tauriコマンドのモック
- テスト用のラッパーコンポーネント
- 共通のテストヘルパー関数

### 2.3 セキュリティコンポーネントのテスト

**新規ファイル**: `codex-rs/tauri-gui/src/components/security/__tests__/SecurityDashboard.test.tsx`

**テスト内容**:

- コンポーネントのレンダリング
- 各パネル（MalwareDetectionPanel、PasswordManagerPanel、RealtimeMonitoringPanel）の表示確認
- 初期状態の確認

**新規ファイル**: `codex-rs/tauri-gui/src/components/security/__tests__/MalwareDetectionPanel.test.tsx`

**テスト内容**:

- パス選択ダイアログの表示
- スキャンボタンのクリック
- スキャン結果の表示
- 隔離・削除ボタンの動作
- エラーハンドリング

**新規ファイル**: `codex-rs/tauri-gui/src/components/security/__tests__/PasswordManagerPanel.test.tsx`

**テスト内容**:

- マスターパスワード初期化
- パスワードエントリの追加
- パスワード漏洩チェック
- エントリリストの表示

**新規ファイル**: `codex-rs/tauri-gui/src/components/security/__tests__/RealtimeMonitoringPanel.test.tsx`

**テスト内容**:

- セキュリティイベントの表示
- システムステータスの表示
- リアルタイム更新のシミュレーション

### 2.4 Tauriコマンドのモック

**新規ファイル**: `codex-rs/tauri-gui/src/test-utils/mock-tauri.ts`

- `@tauri-apps/api/core`の`invoke`関数のモック
- 各セキュリティコマンドのモック実装
- エラーケースのシミュレーション

## 3. 実装順序

1. 依存関係のインストール（`npm install`）
2. ESLint設定の追加
3. TypeScript警告の修正（`npm run type-check`、`npm run lint`）
4. Vitest設定の追加
5. テストユーティリティの作成
6. 各コンポーネントのテスト実装
7. テストの実行と検証（`npm test`）

## 4. 検証

**実行コマンド**:

```bash
cd codex-rs/tauri-gui
npm run type-check  # 型エラー0を確認
npm run lint        # ESLint警告0を確認
npm test            # すべてのテストがパスすることを確認
```

### To-dos

- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）
- [ ] npm installを実行して依存関係をインストール
- [ ] ESLint設定を追加（.eslintrc.json、package.jsonに依存関係とスクリプト追加）
- [ ] TypeScript警告を修正（type-check、lint実行、未使用変数・型エラー修正）
- [ ] Vitest設定を追加（vite.config.ts拡張、package.jsonに依存関係とスクリプト追加）
- [ ] テストユーティリティを作成（test-utils.tsx、mock-tauri.ts）
- [ ] SecurityDashboard.test.tsxを作成
- [ ] MalwareDetectionPanel.test.tsxを作成
- [ ] PasswordManagerPanel.test.tsxを作成
- [ ] RealtimeMonitoringPanel.test.tsxを作成
- [ ] すべてのテストを実行して検証（npm test）