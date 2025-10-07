// テスト用サンプルコード
// このコードにはセキュリティ上の問題があります

use std::collections::HashMap;

// 脆弱性1: SQL Injection の可能性
fn get_user_by_id(id: String) -> String {
    // ❌ 危険: 文字列連結でSQLクエリを構築
    format!("SELECT * FROM users WHERE id = '{}'", id)
}

// 脆弱性2: パスワードのハードコード
const API_KEY: &str = "sk-1234567890abcdef";  // ❌ 危険

// 脆弱性3: エラーハンドリングなし
fn read_file(path: String) -> String {
    std::fs::read_to_string(path).unwrap()  // ❌ unwrap は本番環境では危険
}

// 正しい実装例（比較用）
fn get_user_by_id_safe(id: i32) -> String {
    // ✅ 安全: プレースホルダーを使用（擬似コード）
    format!("SELECT * FROM users WHERE id = ?", /* id をバインド */)
}

fn main() {
    println!("This is a test code for SecurityExpert review");
}

