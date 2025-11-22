// Copyright 2025 zapabob
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::provider::ResearchProvider;
use crate::types::Source;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use tokio::process::Command;
use tracing::info;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

impl From<GeminiSearchResult> for Source {
    fn from(result: GeminiSearchResult) -> Self {
        Source {
            title: result.title,
            url: result.url,
            snippet: result.snippet,
            relevance_score: 0.8, // Default relevance score
        }
    }
}

pub struct GeminiSearchProvider {
    pub model: String,
    /// Use Google Web Search Grounding for enhanced search capabilities
    pub use_grounding: bool,
}

impl GeminiSearchProvider {
    pub fn new(model: String) -> Self {
        Self {
            model,
            use_grounding: true, // Enable Google Web Search Grounding by default
        }
    }

    /// Create with explicit grounding configuration
    pub fn with_grounding(model: String, use_grounding: bool) -> Self {
        Self {
            model,
            use_grounding,
        }
    }

    /// Execute search via Gemini CLI with Google Web Search Grounding
    async fn execute_gemini_search_direct(&self, query: &str) -> Result<Vec<GeminiSearchResult>> {
        info!("🔍 Executing Gemini CLI search with Google Web Search Grounding");

        // Prepare enhanced query with grounding instructions
        let enhanced_query = if self.use_grounding {
            format!(
                "Using Google Web Search Grounding, provide comprehensive research on: {}\n\
                 Include recent sources, verify information accuracy, and provide citations.",
                query
            )
        } else {
            query.to_string()
        };

        // Gemini CLI command with grounding parameters
        let mut cmd = Command::new("gemini");

        if self.use_grounding {
            // Enable Google Web Search Grounding
            cmd.args([
                "--model", &self.model,
                "--grounding", "web",
                "--format", "json",
                &enhanced_query
            ]);
        } else {
            cmd.args([
                "--model", &self.model,
                &enhanced_query
            ]);
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Gemini CLI error: {}", error));
        }

        let response = String::from_utf8(output.stdout)?;
        self.parse_gemini_response(&response)
    }

    /// Parse Gemini CLI response (JSON or text format)
    fn parse_gemini_response(&self, response: &str) -> Result<Vec<GeminiSearchResult>> {
        // Try to parse as JSON first (with grounding)
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(response) {
            if let Some(results) = json_value.get("searchResults").and_then(|r| r.as_array()) {
                let mut parsed_results = Vec::new();
                for result in results {
                    if let (Some(title), Some(url), Some(snippet)) = (
                        result.get("title").and_then(|t| t.as_str()),
                        result.get("url").and_then(|u| u.as_str()),
                        result.get("snippet").and_then(|s| s.as_str()),
                    ) {
                        parsed_results.push(GeminiSearchResult {
                            title: title.to_string(),
                            url: url.to_string(),
                            snippet: snippet.to_string(),
                        });
                    }
                }
                if !parsed_results.is_empty() {
                    return Ok(parsed_results);
                }
            }

            // Try alternative JSON structure
            if let Some(results) = json_value.get("results").and_then(|r| r.as_array()) {
                let mut parsed_results = Vec::new();
                for result in results {
                    if let (Some(title), Some(url), Some(snippet)) = (
                        result.get("title").and_then(|t| t.as_str()),
                        result.get("url").and_then(|u| u.as_str()),
                        result.get("description").or_else(|| result.get("snippet")).and_then(|s| s.as_str()),
                    ) {
                        parsed_results.push(GeminiSearchResult {
                            title: title.to_string(),
                            url: url.to_string(),
                            snippet: snippet.to_string(),
                        });
                    }
                }
                if !parsed_results.is_empty() {
                    return Ok(parsed_results);
                }
            }
        }

        // Fallback to text parsing
        self.parse_text_response(response)
    }

    /// Parse Gemini CLI text response into structured results
    fn parse_text_response(&self, text: &str) -> Result<Vec<GeminiSearchResult>> {
        let mut results = Vec::new();

        // Enhanced parsing logic for Gemini CLI output
        let lines: Vec<&str> = text.lines().collect();
        let mut current_result: Option<GeminiSearchResult> = None;

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Check for URLs
            if line.starts_with("http") && line.contains("://") {
                // Save previous result if exists
                if let Some(result) = current_result.take() {
                    results.push(result);
                }

                // Create new result
                current_result = Some(GeminiSearchResult {
                    title: format!("Search Result"),
                    url: line.to_string(),
                    snippet: String::new(),
                });
            } else if let Some(ref mut result) = current_result {
                // Add to snippet if we have a current result
                if result.snippet.is_empty() {
                    result.title = line.to_string();
                } else if result.snippet.len() < 500 {
                    result.snippet.push_str(" ");
                    result.snippet.push_str(line);
                }
            } else {
                // Try to extract title and URL from combined line
                if let Some(url_start) = line.find("http") {
                    let title_part = &line[..url_start].trim();
                    let url_part = &line[url_start..].trim();

                    results.push(GeminiSearchResult {
                        title: if title_part.is_empty() { "Search Result".to_string() } else { title_part.to_string() },
                        url: url_part.to_string(),
                        snippet: "Gemini CLI search result with grounding".to_string(),
                    });
                }
            }
        }

        // Save final result
        if let Some(result) = current_result {
            results.push(result);
        }

        if results.is_empty() {
            // Fallback: create a generic result with the full text
            results.push(GeminiSearchResult {
                title: "Gemini Search with Grounding".to_string(),
                url: "https://gemini.google.com".to_string(),
                snippet: text.chars().take(500).collect(),
            });
        }

        Ok(results)
    }

}

#[async_trait::async_trait]
impl ResearchProvider for GeminiSearchProvider {
    async fn search(&self, query: &str, max_results: u8) -> Result<Vec<Source>> {
        info!("🔍 Starting Gemini search with Google Web Search Grounding for: {}", query);

        match self.execute_gemini_search_direct(query).await {
            Ok(results) => {
                info!(
                    "✅ Gemini CLI search with grounding successful: {} results",
                    results.len()
                );
                let sources: Vec<Source> = results.into_iter().map(|r| r.into()).collect();
                Ok(sources.into_iter().take(max_results as usize).collect())
            }
            Err(e) => {
                warn!("❌ Gemini CLI search failed: {}", e);
                Err(e)
            }
        }
    }

    async fn retrieve(&self, url: &str) -> Result<String> {
        info!("📥 Retrieving content from: {}", url);

        // Simple HTTP retrieval - can be enhanced
        let response = reqwest::get(url).await?;
        let content = response.text().await?;

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gemini_search_provider() {
        let provider = GeminiSearchProvider::new("gemini-2.5-flash".to_string());

        // Test with a simple query
        let results = provider.search("Rust programming", 5).await;
        assert!(results.is_ok());
    }

    #[test]
    fn test_parse_text_response() {
        let provider = GeminiSearchProvider::new("gemini-2.5-flash".to_string());

        let text = r#"
        Here are some search results:
        
        https://rust-lang.github.io/async-book/
        https://doc.rust-lang.org/book/
        
        These are great resources for Rust programming.
        "#;

        let results = provider.parse_text_response(text);
        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0].title,
            "Search Result for https://rust-lang.github.io/async-book/"
        );
        assert_eq!(results[0].url, "https://rust-lang.github.io/async-book/");
    }
}
