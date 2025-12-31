use anyhow::{Context, Result};
use codex_core::chrome::{ChromeNlRequest, ChromeOrigin, parse_nl_command};
use codex_deep_research::{DeepResearcher, DeepResearcherConfig, ResearchStrategy};
use codex_web_search::WebSearchProvider;
use serde_json;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn handle_deep_research(
    query: String,
    depth: Option<u8>,
    breadth: Option<u8>,
) -> Result<serde_json::Value> {
    let config = DeepResearcherConfig {
        max_depth: depth.unwrap_or(3) as u8,
        max_sources: breadth.unwrap_or(10) as u8,
        strategy: ResearchStrategy::Comprehensive,
    };

    let provider = Arc::new(WebSearchProvider::new(3, 30));
    let researcher = DeepResearcher::new(config, provider);

    let report = researcher
        .research(&query)
        .await
        .context("Failed to conduct research")?;

    let out_path = PathBuf::from("artifacts").join(format!("research_{}.md", uuid::Uuid::new_v4()));
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let markdown = format!(
        "# Research Report: {}\n\n## Summary\n\n{}\n\n## Sources\n\n",
        report.query, report.summary
    );

    let sources_text = report
        .sources
        .iter()
        .enumerate()
        .map(|(i, source)| format!("{}. [{}]({})", i + 1, source.title, source.url))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(&out_path, format!("{}{}", markdown, sources_text))?;

    Ok(serde_json::json!({
        "summary": report.summary,
        "sources": report.sources.iter().map(|s| serde_json::json!({
            "title": s.title,
            "url": s.url,
            "snippet": s.snippet
        })).collect::<Vec<_>>(),
        "report_path": out_path.to_string_lossy().to_string(),
        "depth_reached": report.depth_reached,
        "diversity_score": report.diversity_score
    }))
}

pub fn handle_nl_command(
    utterance: String,
    origin: Option<ChromeOrigin>,
) -> Result<serde_json::Value> {
    let request = ChromeNlRequest {
        utterance,
        origin,
        constraints: None,
    };

    let response = parse_nl_command(request)?;

    Ok(serde_json::json!({
        "intent": {
            "intent": response.intent.intent,
            "args": response.intent.args,
            "risk": response.intent.risk,
            "requires_confirmation": response.intent.requires_confirmation
        },
        "warnings": response.warnings
    }))
}
