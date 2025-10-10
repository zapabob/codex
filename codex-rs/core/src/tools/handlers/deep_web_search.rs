// DeepResearch機能を統合した拡張Web検索ハンドラー
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::function_tool::FunctionCallError;
use crate::tools::context::{HandlerOutput, ToolHandler, ToolInvocation};

use codex_deep_research::{DeepResearcher, ResearchStrategy};

/// DeepWebSearch パラメータ
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepWebSearchParams {
    /// 検索クエリ
    pub query: String,
    
    /// 検索深度（1-10、デフォルト: 2）
    #[serde(default = "default_depth")]
    pub depth: u8,
    
    /// 最大ソース数（1-100、デフォルト: 10）
    #[serde(default = "default_max_sources")]
    pub max_sources: usize,
    
    /// リサーチ戦略（comprehensive, focused, exploratory）
    #[serde(default = "default_strategy")]
    pub strategy: String,
    
    /// 結果のフォーマット（summary, detailed, json）
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_depth() -> u8 {
    2
}

fn default_max_sources() -> usize {
    10
}

fn default_strategy() -> String {
    "comprehensive".to_string()
}

fn default_format() -> String {
    "summary".to_string()
}

/// DeepWebSearch ハンドラー
pub struct DeepWebSearchHandler;

#[async_trait]
impl ToolHandler for DeepWebSearchHandler {
    async fn call(&self, invocation: ToolInvocation<'_>) -> Result<HandlerOutput, FunctionCallError> {
        let params: DeepWebSearchParams = serde_json::from_value(invocation.arguments.clone())
            .map_err(|e| FunctionCallError::Fatal(format!("Invalid parameters: {}", e)))?;

        // パラメータ検証
        let depth = params.depth.clamp(1, 10);
        let max_sources = params.max_sources.clamp(1, 100);

        // リサーチ戦略をパース
        let strategy = match params.strategy.to_lowercase().as_str() {
            "focused" => ResearchStrategy::Focused,
            "exploratory" => ResearchStrategy::Exploratory,
            _ => ResearchStrategy::Comprehensive,
        };

        // DeepResearcherを初期化
        let config = codex_deep_research::types::DeepResearcherConfig {
            max_depth: depth,
            max_sources,
            strategy,
        };

        // MockProviderを使用（実際のプロダクションでは実際のプロバイダーを使用）
        let provider = Arc::new(codex_deep_research::MockProvider);
        let researcher = DeepResearcher::new(config, provider);

        // リサーチ実行
        let report = researcher
            .research(&params.query)
            .await
            .map_err(|e| FunctionCallError::Fatal(format!("Research failed: {}", e)))?;

        // 結果をフォーマット
        let output = match params.format.to_lowercase().as_str() {
            "json" => {
                // JSON形式で返す
                serde_json::to_string_pretty(&report)
                    .map_err(|e| FunctionCallError::Fatal(format!("JSON serialization failed: {}", e)))?
            }
            "detailed" => {
                // 詳細形式
                format_detailed_report(&report)
            }
            _ => {
                // サマリー形式（デフォルト）
                format_summary_report(&report)
            }
        };

        Ok(HandlerOutput::Success(output))
    }
}

/// サマリー形式でレポートをフォーマット
fn format_summary_report(report: &codex_deep_research::types::ResearchReport) -> String {
    let mut output = String::new();

    output.push_str(&format!("# Deep Web Search Results\n\n"));
    output.push_str(&format!("**Query**: {}\n", report.query));
    output.push_str(&format!("**Strategy**: {:?}\n", report.strategy));
    output.push_str(&format!("**Depth Reached**: {}\n", report.depth_reached));
    output.push_str(&format!("**Sources Found**: {}\n\n", report.sources.len()));

    output.push_str("## Summary\n\n");
    output.push_str(&report.summary);
    output.push_str("\n\n");

    output.push_str("## Key Findings\n\n");
    for (i, finding) in report.findings.iter().take(5).enumerate() {
        output.push_str(&format!("{}. {}\n", i + 1, finding));
    }

    if report.findings.len() > 5 {
        output.push_str(&format!("\n... and {} more findings\n", report.findings.len() - 5));
    }

    output.push_str(&format!("\n\n## Top Sources ({})\n\n", report.sources.len().min(10)));
    for (i, source) in report.sources.iter().take(10).enumerate() {
        output.push_str(&format!(
            "{}. [{}]({}) - Relevance: {:.1}%\n",
            i + 1,
            source.title,
            source.url,
            source.relevance * 100.0
        ));
    }

    if report.sources.len() > 10 {
        output.push_str(&format!("\n... and {} more sources\n", report.sources.len() - 10));
    }

    output
}

/// 詳細形式でレポートをフォーマット
fn format_detailed_report(report: &codex_deep_research::types::ResearchReport) -> String {
    let mut output = String::new();

    output.push_str(&format!("# Deep Web Search Results (Detailed)\n\n"));
    output.push_str(&format!("**Query**: {}\n", report.query));
    output.push_str(&format!("**Strategy**: {:?}\n", report.strategy));
    output.push_str(&format!("**Depth Reached**: {}\n", report.depth_reached));
    output.push_str(&format!("**Total Sources**: {}\n", report.sources.len()));
    output.push_str(&format!("**Total Findings**: {}\n\n", report.findings.len()));

    output.push_str("## Executive Summary\n\n");
    output.push_str(&report.summary);
    output.push_str("\n\n");

    output.push_str("## All Findings\n\n");
    for (i, finding) in report.findings.iter().enumerate() {
        output.push_str(&format!("{}. {}\n", i + 1, finding));
    }

    output.push_str("\n\n## All Sources\n\n");
    for (i, source) in report.sources.iter().enumerate() {
        output.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            source.title
        ));
        output.push_str(&format!("- **URL**: {}\n", source.url));
        output.push_str(&format!("- **Relevance**: {:.1}%\n", source.relevance * 100.0));
        
        if let Some(snippet) = &source.snippet {
            output.push_str(&format!("- **Snippet**: {}\n", snippet));
        }
        
        if let Some(published) = &source.published_date {
            output.push_str(&format!("- **Published**: {}\n", published));
        }
        
        output.push_str("\n");
    }

    output
}

/// DeepWebSearchツールの仕様を作成
pub fn create_deep_web_search_tool() -> crate::client_common::tools::ResponsesApiTool {
    use crate::openai_tools::{JsonSchema, JsonSchemaType};
    use std::collections::HashMap;

    let mut properties = HashMap::new();

    properties.insert(
        "query".to_string(),
        JsonSchema {
            r#type: Some(JsonSchemaType::String),
            description: Some("Search query to research".to_string()),
            ..Default::default()
        },
    );

    properties.insert(
        "depth".to_string(),
        JsonSchema {
            r#type: Some(JsonSchemaType::Number),
            description: Some(
                "Research depth (1-10). Higher values enable more thorough multi-level exploration. Default: 2"
                    .to_string(),
            ),
            ..Default::default()
        },
    );

    properties.insert(
        "max_sources".to_string(),
        JsonSchema {
            r#type: Some(JsonSchemaType::Number),
            description: Some(
                "Maximum number of sources to collect (1-100). Default: 10".to_string(),
            ),
            ..Default::default()
        },
    );

    properties.insert(
        "strategy".to_string(),
        JsonSchema {
            r#type: Some(JsonSchemaType::String),
            description: Some(
                "Research strategy: 'comprehensive' (thorough), 'focused' (high relevance), 'exploratory' (broad). Default: comprehensive"
                    .to_string(),
            ),
            enum_values: Some(vec![
                "comprehensive".to_string(),
                "focused".to_string(),
                "exploratory".to_string(),
            ]),
            ..Default::default()
        },
    );

    properties.insert(
        "format".to_string(),
        JsonSchema {
            r#type: Some(JsonSchemaType::String),
            description: Some(
                "Output format: 'summary' (concise), 'detailed' (full), 'json' (raw). Default: summary"
                    .to_string(),
            ),
            enum_values: Some(vec![
                "summary".to_string(),
                "detailed".to_string(),
                "json".to_string(),
            ]),
            ..Default::default()
        },
    );

    crate::client_common::tools::ResponsesApiTool {
        name: "deep_web_search".to_string(),
        description: "Conduct deep multi-level web research on a topic. This tool combines web search with iterative exploration to gather comprehensive information across multiple sources and depth levels. Use this for complex research tasks that require thorough investigation.".to_string(),
        parameters: JsonSchema {
            r#type: Some(JsonSchemaType::Object),
            properties: Some(properties),
            required: Some(vec!["query".to_string()]),
            additional_properties: Some(Box::new(JsonSchema {
                r#type: Some(JsonSchemaType::Boolean),
                ..Default::default()
            })),
            ..Default::default()
        },
        strict: Some(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deep_web_search_tool_creation() {
        let tool = create_deep_web_search_tool();
        assert_eq!(tool.name, "deep_web_search");
        assert!(tool.description.contains("deep multi-level"));
        assert!(tool.parameters.required.is_some());
        assert_eq!(tool.parameters.required.unwrap(), vec!["query"]);
    }

    #[test]
    fn test_params_defaults() {
        let json = r#"{"query": "test"}"#;
        let params: DeepWebSearchParams = serde_json::from_str(json).unwrap();
        
        assert_eq!(params.query, "test");
        assert_eq!(params.depth, 2);
        assert_eq!(params.max_sources, 10);
        assert_eq!(params.strategy, "comprehensive");
        assert_eq!(params.format, "summary");
    }

    #[test]
    fn test_params_custom() {
        let json = r#"{
            "query": "Rust async",
            "depth": 5,
            "max_sources": 20,
            "strategy": "focused",
            "format": "detailed"
        }"#;
        let params: DeepWebSearchParams = serde_json::from_str(json).unwrap();
        
        assert_eq!(params.query, "Rust async");
        assert_eq!(params.depth, 5);
        assert_eq!(params.max_sources, 20);
        assert_eq!(params.strategy, "focused");
        assert_eq!(params.format, "detailed");
    }
}

