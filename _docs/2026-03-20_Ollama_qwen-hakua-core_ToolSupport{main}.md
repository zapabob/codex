# Ollama qwen-hakua-core ツール呼び出し対応実装ログ

**日時**: 2026-03-20
**ブランチ**: main
**担当**: zapabob + Claude Code

---

## 概要

OllamaでカスタムインポートしたGGUFモデル `qwen-hakua-core` がツール呼び出し（function calling）API使用時に `400 Bad Request` エラーを返す問題を解消した。

---

## 問題

```
Ollama API error 400: {"error":"registry.ollama.ai/library/qwen-hakua-core:latest does not support tools"}
```

**原因**: カスタムGGUFをOllamaにインポートした際、デフォルトのModelfileにはツール対応テンプレートが含まれない。

---

## モデル情報

| 項目 | 内容 |
|------|------|
| モデル名 | qwen-hakua-core |
| ベースモデル | Qwen3.5-9B-Uncensored-HauhauCS-Aggressive |
| 量子化 | Q8_0 |
| ファイルサイズ | 9.5 GB |
| 入手元 | HuggingFace |
| GGUFファイル | Qwen3.5-9B-Uncensored-HauhauCS-Aggressive-Q8_0.gguf |

---

## 対処

### 1. 既存Modelfileの確認

```bash
ollama show qwen-hakua-core --modelfile
# → 出力なし（カスタムインポートのためModelfile情報が空）
```

### 2. ツール対応Modelfileを作成

Qwen3系のchat templateに合わせて以下の要素を追加:

- `{{- if .Tools }}` ブロック: ツール一覧をシステムプロンプトとして注入
- `<tool_call>` フォーマット: モデルがツール呼び出しを出力する形式
- `{{ if .Thinking }}` ブロック: Qwen3の思考モード（`<think>`タグ）対応
- PARAMETER stop: `<|im_end|>`, `<|im_start|>`, `<tool_call>`

ファイル: `bin/ollama/Modelfile.qwen-hakua-core`

### 3. Ollamaへの登録

```bash
ollama create qwen-hakua-core -f "C:\Users\downl\Downloads\Modelfile"
# gathering model components
# copying file sha256:99e7f2201c... 100%
# parsing GGUF
# using existing layer sha256:99e7f2201c...
# creating new layer sha256:8559575bdb...
# creating new layer sha256:e25d7e51a8...
# writing manifest
# success
```

---

## 検証結果

### テストコマンド

```bash
curl http://localhost:11434/api/chat -d '{
  "model": "qwen-hakua-core",
  "messages": [{"role": "user", "content": "hello"}],
  "tools": [{
    "type": "function",
    "function": {
      "name": "get_weather",
      "description": "Get weather",
      "parameters": {
        "type": "object",
        "properties": {"location": {"type": "string"}},
        "required": ["location"]
      }
    }
  }]
}'
```

### レスポンス（抜粋）

```json
{"model":"qwen-hakua-core","message":{"role":"assistant","content":"<think>"},"done":false}
{"model":"qwen-hakua-core","message":{"role":"assistant","content":"\n\n"},"done":false}
{"model":"qwen-hakua-core","message":{"role":"assistant","content":"</think>"},"done":false}
{"model":"qwen-hakua-core","message":{"role":"assistant","content":"Hello! How can I help you today? You can ask me about the weather, or anything else you'd like to discuss."},"done":false}
{"done":true,"done_reason":"stop","total_duration":211652623800,"load_duration":210091402800,"prompt_eval_count":99,"eval_count":31}
```

### 結果

| 確認項目 | 結果 |
|----------|------|
| 400エラー解消 | ✅ |
| ツールAPI受け付け | ✅ |
| `<think>`思考モード動作 | ✅ |
| ツール認識（weather言及） | ✅ |
| 初回ロード時間 | 約3分31秒（9.5GB Q8_0） |

---

## 技術メモ

- **GGUFとツールサポート**: GGUFフォーマット自体の問題ではなく、OllamaのModelfileテンプレートにツール対応記述がないことが原因
- **abliterated（無検閲）モデル**: 安全フィルター除去のみのため、ツール呼び出し能力は保持される
- **Q8_0量子化**: 高精度量子化のためツール性能への影響は最小限
- **Qwen3テンプレート形式**: `<|im_start|>` / `<|im_end|>` トークンを使用するChatml形式

---

## 関連ファイル

- `bin/ollama/Modelfile.qwen-hakua-core` — 再利用可能なModelfileテンプレート（FROMパスは環境依存）
