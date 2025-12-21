---
name: Fix TUI CLI input and slash command issues
overview: TUIのCLIで通常の文字入力が表示されない問題と、スラッシュコマンドが消える問題を修正します。paste burst処理が通常のタイピングを誤検出して入力をバッファリングしているため、入力が反映されない問題を解決します。
todos:
  - id: fix_paste_burst
    content: "paste burst処理を改善: スラッシュコマンド入力中は無効化し、通常の入力が即座に反映されるようにする"
    status: completed
  - id: fix_slash_detection
    content: "スラッシュコマンド検出の順序を変更: set_text(\"\")の前に検出を行い、認識されなかった場合に完全に復元する"
    status: completed
  - id: fix_input_restoration
    content: "元の入力の完全な復元: trim()による空白削除を避け、カーソル位置も正しく設定する"
    status: completed
  - id: test_normal_input
    content: 通常の文字入力が即座に表示されることをテスト
    status: completed
  - id: test_slash_commands
    content: スラッシュコマンド入力中に消えないことをテスト
    status: completed
  - id: test_paste
    content: ペースト操作が正常に動作することをテスト
    status: completed
---

# TUI CLI入力とスラッシュコマンド問題の修正計画

## 問題の分析

1. **通常の文字入力が表示されない問題**

- `handle_input_basic`関数（`codex-rs/tui/src/bottom_pane/chat_composer.rs`）で、paste burst処理が通常のタイピングを誤検出している
- `CharDecision::RetainFirstChar`、`BufferAppend`、`BeginBuffer`が返された場合、`textarea.input(input)`が呼ばれずに早期リターンしている（行1128-1162）
- その結果、入力がバッファに保存されるが、即座にtextareaに反映されない

2. **スラッシュコマンドが消える問題**

- Enterキーを押したときに`self.textarea.set_text("")`が呼ばれている（行994）
- その後、スラッシュコマンドが認識されなかった場合に元に戻す処理があるが、`text.trim().to_string()`（行1006）により先頭の空白が削除される可能性がある

## 修正方針

### 1. paste burst処理の改善

**ファイル**: `codex-rs/tui/src/bottom_pane/chat_composer.rs`

- `handle_input_basic`関数（行1091-1238）を修正
- paste burst処理で早期リターンする前に、`flush_if_due`を呼び出して保留中の入力をフラッシュする
- `CharDecision::RetainFirstChar`の場合でも、一定時間経過後は通常の入力として処理する
- スラッシュコマンド入力中（`/`で始まる行）はpaste burst処理を無効化する

**具体的な変更**:

- 行1093-1097: `flush_if_due`を呼び出して保留中の入力をフラッシュ
- 行1110-1163: paste burst処理の条件を改善し、スラッシュコマンド入力中は無効化
- 行1184: `textarea.input(input)`が確実に呼ばれるようにする

### 2. スラッシュコマンド処理の改善

**ファイル**: `codex-rs/tui/src/bottom_pane/chat_composer.rs`

- Enterキー処理（行923-1057）を修正
- `self.textarea.set_text("")`を呼ぶ前に、スラッシュコマンドの検出を先に行う
- スラッシュコマンドが認識されなかった場合、元の入力を完全に復元する（`trim()`による空白削除を避ける）

**具体的な変更**:

- 行991-994: スラッシュコマンド検出を`set_text("")`の前に移動
- 行1006: `text.trim()`の代わりに、先頭の空白を保持したまま処理
- 行1029-1030: 元の入力を完全に復元する際、カーソル位置も正しく設定

### 3. スラッシュコマンド入力中のpaste burst無効化

**ファイル**: `codex-rs/tui/src/bottom_pane/chat_composer.rs`

- `handle_input_basic`関数内で、現在のテキストがスラッシュコマンドで始まる場合、paste burst処理をスキップする
- `in_slash_context`の判定ロジックを改善（行948-955のロジックを再利用）

**具体的な変更**:

- 行1110-1163: `in_slash_context`チェックを追加して、スラッシュコマンド入力中はpaste burst処理をスキップ

## 実装の詳細

### 変更1: paste burst処理の改善

```rust
// codex-rs/tui/src/bottom_pane/chat_composer.rs の handle_input_basic 関数内

// スラッシュコマンド入力中かどうかを判定
let in_slash_context = self
    .textarea
    .text()
    .lines()
    .next()
    .unwrap_or("")
    .starts_with('/');

// paste burst処理をスキップする条件を追加
if !self.disable_paste_burst && !in_slash_context {
    // 既存のpaste burst処理
}
```



### 変更2: スラッシュコマンド検出の順序変更

```rust
// codex-rs/tui/src/bottom_pane/chat_composer.rs の Enterキー処理内

let mut text = self.textarea.text().to_string();
let original_input = text.clone();
let input_starts_with_space = original_input.starts_with(' ');

// スラッシュコマンド検出を先に行う（set_text("")の前）
if let Some((name, _rest)) = parse_slash_name(&text) {
    // スラッシュコマンド処理
}

// その後、set_text("")を呼ぶ
self.textarea.set_text("");
```



### 変更3: 元の入力の完全な復元

```rust
// スラッシュコマンドが認識されなかった場合
self.textarea.set_text(&original_input);  // trim()を使わない
self.textarea.set_cursor(original_input.len());
```



## テスト

- 通常の文字入力が即座に表示されることを確認
- スラッシュコマンド（`/model`、`/diff`など）を入力中に消えないことを確認
- ペースト操作が正常に動作することを確認（paste burst処理が正しく機能する）
- Enterキーを押したときにスラッシュコマンドが正しく実行されることを確認

## 注意事項

- paste burst処理は、ペーストされたテキストを検出するための重要な機能なので、完全に無効化せず、スラッシュコマンド入力中のみ無効化する
- `trim()`による空白削除は、ユーザーが意図的に先頭に空白を入力した場合に問題となる可能性があるため、スラッシュコマンド検出時のみ`trim()`を使用する