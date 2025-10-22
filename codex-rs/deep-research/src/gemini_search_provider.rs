// Gemini CLI Search Provider - Google Search via Gemini API
// Integrates Gemini CLI to use Google Search with Grounding
use crate::provider::ResearchProvider;
use crate::types::Source;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tracing::debug;
use tracing::info;

/// Gemini CLI search provider that uses Google Search via Gemini API Grounding
/// Requires: gemini CLI installed (OAuth 2.0 authentication)
/// Supports both direct CLI calls and MCP-based calls
pub struct GeminiSearchProvider {
    model: String,
    max_retries: u8,
    use_mcp: bool, // true = MCP経由で呼び出し, false = 直接CLI呼び出し
}

impl Default for GeminiSearchProvider {
    fn default() -> Self {
        Self {
            model: "gemini-2.5-pro".to_string(), // Gemini 2.5 Pro (最高品質・レートリミット注意)
            max_retries: 3,
            use_mcp: false, // デフォルトは直接CLI呼び出し
        }
    }
}

impl GeminiSearchProvider {
    pub fn new(model: Option<String>) -> Self {
        Self {
            model: model.unwrap_or_else(|| "gemini-2.5-pro".to_string()),
            max_retries: 3,
            use_mcp: false,
        }
    }

    /// Create a new GeminiSearchProvider with MCP mode enabled
    /// This will use Codex → MCP → Gemini CLI chain
    pub fn new_with_mcp(model: Option<String>) -> Self {
        Self {
            model: model.unwrap_or_else(|| "gemini-2.5-pro".to_string()),
            max_retries: 3,
            use_mcp: true, // MCP経由で呼び出し
        }
    }

    /// Create a Command to run gemini CLI (cross-platform)
    /// Windows: Uses 'cmd /c gemini' because gemini is a .ps1/.cmd script
    /// Unix: Uses 'gemini' directly
    fn create_gemini_command() -> std::process::Command {
        #[cfg(target_os = "windows")]
        {
            let mut cmd = std::process::Command::new("cmd");
            cmd.args(["/c", "gemini"]);
            cmd
        }

        #[cfg(not(target_os = "windows"))]
        {
            std::process::Command::new("gemini")
        }
    }

    /// Execute search using Gemini CLI with Google Search grounding
    async fn execute_gemini_search(&self, query: &str) -> Result<Vec<GeminiSearchResult>> {
        info!(
            "🔍 Executing Gemini CLI (Node.js version) search for: {}",
            query
        );
        eprintln!("🔍 [DEBUG] execute_gemini_search called for: {}", query);
        eprintln!("🔍 [DEBUG] MCP mode: {}", self.use_mcp);

        // MCP経由で呼び出す場合
        if self.use_mcp {
            eprintln!("🔌 [DEBUG] Using MCP mode: Codex → MCP → Gemini CLI");
            return self.execute_gemini_search_via_mcp(query).await;
        }

        // 直接CLI呼び出し
        eprintln!("🔧 [DEBUG] Using direct CLI mode");

        // Check if gemini CLI is installed
        self.check_gemini_cli_installed()?;
        eprintln!("✅ [DEBUG] Gemini CLI installed check passed");

        // シンプルなプロンプトに変更（長すぎるとエラーの可能性）
        let prompt = format!("Search the web for: {query}");
        eprintln!("📝 [DEBUG] Prompt: {}", prompt);

        let mut cmd = Self::create_gemini_command();
        cmd.arg("-p")
            .arg(&prompt)
            .arg("-o")
            .arg("text")
            .arg("-m")
            .arg(&self.model);

        eprintln!("🔧 [DEBUG] Executing gemini CLI with model: {}", self.model);

        let output = cmd
            .output()
            .context("Failed to execute gemini CLI command (Node.js version)")?;

        eprintln!("📊 [DEBUG] Gemini CLI executed");
        eprintln!("📊 [DEBUG] Status: {:?}", output.status);
        eprintln!("📊 [DEBUG] Success: {}", output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        eprintln!("📊 [DEBUG] Stdout length: {} bytes", stdout.len());
        eprintln!("📊 [DEBUG] Stderr length: {} bytes", stderr.len());

        if !stdout.is_empty() {
            eprintln!(
                "📊 [DEBUG] Stdout preview (first 500 chars):\n{}",
                &stdout.chars().take(500).collect::<String>()
            );
        }

        if !stderr.is_empty() {
            eprintln!("⚠️  [DEBUG] Stderr:\n{}", stderr);
        }

        // エラー検出（status失敗 OR stderr にエラーメッセージ）
        let has_error = !output.status.success()
            || stderr.contains("Error when talking to Gemini API")
            || stderr.contains("rate limit")
            || stderr.contains("quota")
            || stderr.contains("429")
            || stderr.contains("RESOURCE_EXHAUSTED");

        if has_error {
            eprintln!("⚠️  [DEBUG] Error detected in Gemini CLI response");
            eprintln!("⚠️  [DEBUG] Status success: {}", output.status.success());
            eprintln!(
                "⚠️  [DEBUG] Stderr contains API error: {}",
                stderr.contains("Error when talking to Gemini API")
            );

            // レートリミットエラーの検出とフォールバック
            tracing::warn!(
                "⚠️  Gemini CLI error (likely rate limit), falling back to gemini-2.5-flash"
            );
            eprintln!("⚠️  [DEBUG] Attempting fallback to gemini-2.5-flash");

            // gemini-2.5-flashへフォールバック
            if self.model != "gemini-2.5-flash" {
                let mut fallback_cmd = Self::create_gemini_command();
                let fallback_output = fallback_cmd
                    .arg("-p")
                    .arg(&prompt)
                    .arg("-o")
                    .arg("text")
                    .arg("-m")
                    .arg("gemini-2.5-flash")
                    .output()
                    .context("Failed to execute gemini CLI with fallback model")?;

                let fallback_stdout = String::from_utf8_lossy(&fallback_output.stdout);
                let fallback_stderr = String::from_utf8_lossy(&fallback_output.stderr);

                eprintln!("📊 [DEBUG] Fallback status: {:?}", fallback_output.status);
                eprintln!(
                    "📊 [DEBUG] Fallback stdout length: {}",
                    fallback_stdout.len()
                );

                // フォールバックも失敗した場合は空の結果を返す（エラーにしない）
                if fallback_output.status.success()
                    && !fallback_stderr.contains("Error when talking to Gemini API")
                {
                    eprintln!("✅ [DEBUG] Fallback successful");
                    debug!("Gemini CLI fallback output: {}", fallback_stdout);
                    let results = self.parse_text_response(&fallback_stdout);
                    eprintln!("✅ [DEBUG] Fallback parsed {} results", results.len());
                    return Ok(results);
                } else {
                    eprintln!("⚠️  [DEBUG] Fallback also failed, returning empty results");
                    tracing::warn!(
                        "Fallback to gemini-2.5-flash also failed, returning empty results"
                    );
                    return Ok(Vec::new()); // 空の結果を返す（エラーにしない）
                }
            }

            // すでにgemini-2.5-flashを使用中の場合は空の結果を返す
            eprintln!("⚠️  [DEBUG] Already using gemini-2.5-flash, returning empty results");
            return Ok(Vec::new());
        }

        debug!("Gemini CLI output: {}", stdout);
        eprintln!("🔍 [DEBUG] Parsing response...");

        // Parse text response (Node.js版のJSON形式は異なるため、テキスト解析を使用)
        let results = self.parse_text_response(&stdout);
        eprintln!("✅ [DEBUG] Parsed {} results", results.len());

        Ok(results)
    }

    /// Check if gemini CLI is installed (Node.js version)
    fn check_gemini_cli_installed(&self) -> Result<()> {
        let mut cmd = Self::create_gemini_command();
        let output = cmd.arg("--version").output().context(
            "gemini CLI not found. Please install it with: npm install -g @google-labs/gemini-cli",
        )?;

        if !output.status.success() {
            anyhow::bail!("gemini CLI is not properly installed");
        }

        // Node.js版の確認
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("node") || stdout.is_empty() {
            info!("✅ Detected Node.js version of gemini CLI");
            Ok(())
        } else {
            tracing::warn!("⚠️  Unknown gemini CLI version, proceeding anyway");
            Ok(())
        }
    }

    /// Parse Gemini CLI JSON response (将来の拡張用・現在は未使用)
    #[allow(dead_code)]
    fn parse_gemini_response(&self, json_str: &str) -> Result<Vec<GeminiSearchResult>> {
        // Try to parse JSON response
        if let Ok(response) = serde_json::from_str::<GeminiApiResponse>(json_str) {
            return Ok(response
                .candidates
                .into_iter()
                .flat_map(|c| c.search_results)
                .collect());
        }

        // Fallback: Parse text response manually
        info!("⚠️  JSON parsing failed, using text fallback");
        Ok(self.parse_text_response(json_str))
    }

    /// Parse text response as fallback (no regex dependencies)
    fn parse_text_response(&self, text: &str) -> Vec<GeminiSearchResult> {
        let mut results = Vec::new();

        // マークダウンリンクをパース: [title](url)
        let mut current_pos = 0;
        let text_bytes = text.as_bytes();

        while current_pos < text.len() {
            if text_bytes.get(current_pos) == Some(&b'[') {
                // [title]を探す
                if let Some(title_end) = text[current_pos + 1..].find(']') {
                    let title_start = current_pos + 1;
                    let title_end_abs = current_pos + 1 + title_end;

                    // その直後に(url)があるか確認
                    if text_bytes.get(title_end_abs + 1) == Some(&b'(') {
                        if let Some(url_end) = text[title_end_abs + 2..].find(')') {
                            let url_start = title_end_abs + 2;
                            let url_end_abs = title_end_abs + 2 + url_end;

                            let title = text[title_start..title_end_abs].to_string();
                            let url = text[url_start..url_end_abs].to_string();

                            results.push(GeminiSearchResult {
                                title: title.clone(),
                                url: url.clone(),
                                snippet: format!("Result from Gemini search: {title}"),
                            });

                            current_pos = url_end_abs + 1;
                            continue;
                        }
                    }
                }
            }
            current_pos += 1;
        }

        // マークダウンリンクが見つからない場合、プレーンURLを探す
        if results.is_empty() {
            for (i, word) in text.split_whitespace().enumerate() {
                if word.starts_with("http://") || word.starts_with("https://") {
                    // URLの終わりを見つける（空白、括弧、クォートなど）
                    let url = word
                        .trim_end_matches(|c: char| {
                            !c.is_alphanumeric()
                                && c != '/'
                                && c != ':'
                                && c != '.'
                                && c != '-'
                                && c != '_'
                                && c != '?'
                                && c != '='
                                && c != '&'
                        })
                        .to_string();

                    results.push(GeminiSearchResult {
                        title: format!("Search Result {}", i + 1),
                        url,
                        snippet: "Result from Gemini search".to_string(),
                    });
                }
            }
        }

        results
    }

    /// Execute search with retry logic
    async fn search_with_retry(
        &self,
        query: &str,
        max_results: usize,
    ) -> Result<Vec<GeminiSearchResult>> {
        let mut last_error = None;

        for attempt in 0..self.max_retries {
            match self.execute_gemini_search(query).await {
                Ok(results) => {
                    info!("✅ Gemini search succeeded on attempt {}", attempt + 1);
                    return Ok(results.into_iter().take(max_results).collect());
                }
                Err(e) => {
                    tracing::warn!("Gemini search attempt {} failed: {}", attempt + 1, e);
                    last_error = Some(e);

                    // Wait before retry
                    if attempt < self.max_retries - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retry attempts failed")))
    }

    /// Execute search via MCP (Codex → MCP → Gemini CLI)
    /// This uses the gemini-cli MCP server defined in config.toml
    async fn execute_gemini_search_via_mcp(&self, query: &str) -> Result<Vec<GeminiSearchResult>> {
        use codex_mcp_client::McpClient;
        use serde_json::json;
        use std::ffi::OsString;

        info!("🔌 Executing Gemini CLI via MCP server");
        eprintln!("🔌 [DEBUG] Creating MCP client for gemini-cli");

        // MCPサーバー設定（~/.codex/config.tomlから）
        let program = OsString::from("codex-gemini-mcp");
        let args = vec![];

        eprintln!("🔌 [DEBUG] Spawning MCP server: codex-gemini-mcp");

        // MCPクライアント作成
        let client = McpClient::new_stdio_client(program, args, None, &[], None)
            .await
            .context("Failed to spawn codex-gemini-mcp server")?;

        eprintln!("✅ [DEBUG] MCP client created");

        // Initialize MCP session
        use mcp_types::ClientCapabilities;
        use mcp_types::Implementation;
        use mcp_types::InitializeRequestParams;
        let init_params = InitializeRequestParams {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ClientCapabilities {
                elicitation: None,
                experimental: None,
                roots: None,
                sampling: None,
            },
            client_info: Implementation {
                name: "codex-deep-research".to_string(),
                title: Some("Codex Deep Research".to_string()),
                version: "0.48.0".to_string(),
                user_agent: Some("codex-deep-research/0.48.0".to_string()),
            },
        };

        client
            .initialize(init_params, None)
            .await
            .context("Failed to initialize MCP session")?;

        eprintln!("✅ [DEBUG] MCP session initialized");

        // googleSearch ツールを呼び出す
        let tool_params = json!({
            "query": query,
            "model": self.model,
        });

        eprintln!("🔌 [DEBUG] Calling googleSearch tool via MCP");
        eprintln!("🔌 [DEBUG] Query: {}", query);
        eprintln!("🔌 [DEBUG] Model: {}", self.model);

        let result = client
            .call_tool("googleSearch".to_string(), Some(tool_params), None)
            .await
            .context("Failed to call googleSearch via MCP")?;

        eprintln!("✅ [DEBUG] MCP tool call successful");
        eprintln!("📊 [DEBUG] Result: {:?}", result);

        // 結果をパース
        // MCP経由の結果は異なる形式の可能性があるため、柔軟にパース
        use mcp_types::ContentBlock;

        let results = if let Some(content_block) = result.content.first() {
            match content_block {
                ContentBlock::TextContent(text_content) => {
                    eprintln!("🔍 [DEBUG] Parsing MCP response text");
                    self.parse_text_response(&text_content.text)
                }
                _ => {
                    eprintln!("⚠️  [DEBUG] MCP response is not text content, returning empty");
                    Vec::new()
                }
            }
        } else {
            eprintln!("⚠️  [DEBUG] No content in MCP response, returning empty");
            Vec::new()
        };

        eprintln!("✅ [DEBUG] Parsed {} results from MCP", results.len());

        Ok(results)
    }
}

#[async_trait]
impl ResearchProvider for GeminiSearchProvider {
    async fn search(&self, query: &str, max_results: u8) -> Result<Vec<Source>> {
        let gemini_results = self.search_with_retry(query, max_results as usize).await?;

        let sources: Vec<Source> = gemini_results
            .into_iter()
            .map(|result| Source {
                url: result.url,
                title: result.title,
                snippet: result.snippet,
                relevance_score: 0.90, // Gemini-grounded results are high quality
            })
            .collect();

        info!(
            "✅ Found {} sources via Gemini CLI for: {}",
            sources.len(),
            query
        );

        Ok(sources)
    }

    async fn retrieve(&self, url: &str) -> Result<String> {
        // Use Gemini to summarize content from URL
        info!("📥 Retrieving content from {} via Gemini (Node.js)", url);

        let prompt = format!(
            "Please summarize the main content from this URL: {url}\n\n\
            Focus on:\n\
            - Key technical concepts\n\
            - Main arguments or findings\n\
            - Code examples if present\n\n\
            Keep it concise (200-300 words)."
        );

        let mut cmd = Self::create_gemini_command();
        let output = cmd
            .arg("-p") // Node.js版では -p または --prompt
            .arg(&prompt)
            .arg("-m") // Model
            .arg(&self.model)
            .output()
            .context("Failed to execute gemini CLI for content retrieval (Node.js)")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Gemini CLI (Node.js) content retrieval failed: {stderr}");
        }

        let content = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(content)
    }
}

/// Gemini API response structure (将来の拡張用・現在は未使用)
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct GeminiApiResponse {
    candidates: Vec<GeminiCandidate>,
}

/// Gemini API candidate structure (将来の拡張用・現在は未使用)
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct GeminiCandidate {
    #[serde(rename = "searchResults")]
    search_results: Vec<GeminiSearchResult>,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiSearchResult {
    title: String,
    url: String,
    snippet: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires gemini CLI and API key
    async fn test_gemini_search() {
        let provider = GeminiSearchProvider::default();
        let sources = provider.search("Rust async programming", 3).await.unwrap();

        assert!(!sources.is_empty());
        assert!(sources[0].relevance_score > 0.8);
    }

    #[test]
    fn test_parse_text_response() {
        let provider = GeminiSearchProvider::default();
        let text = r#"
        Here are some results:
        [Rust Async Book](https://rust-lang.github.io/async-book/)
        [Tokio Documentation](https://tokio.rs)
        "#;

        let results = provider.parse_text_response(text);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Rust Async Book");
        assert_eq!(results[0].url, "https://rust-lang.github.io/async-book/");
    }
}
