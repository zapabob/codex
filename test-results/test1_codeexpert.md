# テスト1: CodeExpert - 基本関数生成

## 📋 テスト情報
- **実施日時**: 2025-10-08 03:25 JST
- **エージェント**: CodeExpert
- **タスク**: Fibonacci数計算関数の生成

## 🎯 テスト目的
CodeExpertエージェントの基本的なコード生成能力を確認

## 📝 実行コマンド
```powershell
codex exec "Create a Rust function to calculate the nth Fibonacci number with:
- Input validation (n must be non-negative)
- Error handling using Result type
- Documentation comments
- Unit tests with edge cases
- Performance consideration for large n"
```

## 💡 期待される出力

### 生成されるべき関数
```rust
/// Calculates the nth Fibonacci number.
///
/// # Arguments
/// * `n` - The position in the Fibonacci sequence (0-indexed)
///
/// # Returns
/// * `Ok(u64)` - The nth Fibonacci number
/// * `Err(String)` - Error if n is too large or causes overflow
///
/// # Examples
/// ```
/// assert_eq!(fibonacci(0).unwrap(), 0);
/// assert_eq!(fibonacci(1).unwrap(), 1);
/// assert_eq!(fibonacci(10).unwrap(), 55);
/// ```
pub fn fibonacci(n: u32) -> Result<u64, String> {
    match n {
        0 => Ok(0),
        1 => Ok(1),
        _ => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            
            for _ in 2..=n {
                let next = a.checked_add(b)
                    .ok_or_else(|| format!("Overflow calculating fibonacci({})", n))?;
                a = b;
                b = next;
            }
            
            Ok(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0).unwrap(), 0);
        assert_eq!(fibonacci(1).unwrap(), 1);
    }

    #[test]
    fn test_fibonacci_small_numbers() {
        assert_eq!(fibonacci(2).unwrap(), 1);
        assert_eq!(fibonacci(3).unwrap(), 2);
        assert_eq!(fibonacci(5).unwrap(), 5);
        assert_eq!(fibonacci(10).unwrap(), 55);
    }

    #[test]
    fn test_fibonacci_larger_numbers() {
        assert_eq!(fibonacci(20).unwrap(), 6765);
        assert_eq!(fibonacci(30).unwrap(), 832040);
    }

    #[test]
    fn test_fibonacci_overflow() {
        // fibonacci(93) = 12200160415121876738 (最大のu64に収まる値)
        // fibonacci(94)はオーバーフローする
        assert!(fibonacci(93).is_ok());
        assert!(fibonacci(94).is_err());
    }
}
```

## ✅ 評価基準

### 機能性 (5点満点)
- [ ] 基本的なFibonacci計算が正しい: __/1点
- [ ] エッジケース（0, 1）の処理: __/1点
- [ ] エラーハンドリング（Result型）: __/1点
- [ ] オーバーフロー対策: __/1点
- [ ] パフォーマンス考慮: __/1点

### コード品質 (5点満点)
- [ ] 型安全な実装: __/1点
- [ ] ドキュメントコメント: __/1点
- [ ] 命名規則の遵守: __/1点
- [ ] コードの可読性: __/1点
- [ ] Rustのベストプラクティス: __/1点

### テストの質 (5点満点)
- [ ] 基本ケースのテスト: __/1点
- [ ] エッジケースのテスト: __/1点
- [ ] エラーケースのテスト: __/1点
- [ ] テストの網羅性: __/1点
- [ ] テストの可読性: __/1点

## 📊 実行結果

### 生成されたコード
```
[実際に生成されたコードをここに記録]
```

### テスト実行結果
```bash
cargo test
```

```
[テスト実行結果をここに記録]
```

## 🎯 総合評価

- **機能性**: __/5点
- **コード品質**: __/5点
- **テストの質**: __/5点
- **総合**: __/15点

### 合格基準
- 12点以上: 優秀
- 9-11点: 良好
- 6-8点: 合格
- 5点以下: 要改善

## 📝 備考・気づいた点

### 良かった点
- 

### 改善が必要な点
- 

### その他
- 

---

**テスト実施者**: AI Assistant  
**所要時間**: __分__秒  
**ステータス**: [ ] 成功 / [ ] 失敗 / [ ] 部分成功

