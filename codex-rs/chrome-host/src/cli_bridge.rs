use anyhow::{Context, Result};
use codex_core::chrome::{ChromeNlRequest, ChromeNlResponse, ChromeOrigin, parse_nl_command};
use codex_deep_research::DeepResearcher;
use codex_deep_research::DeepResearcherConfig;
use codex_deep_research::ResearchPlanner;
use codex_deep_research::ResearchStrategy;
use codex_deep_research::provider::ResearchProvider;
use codex_web_search::WebSearchProvider;
use std::sync::Arc;

/// Handle deep research request from Chrome extension
pub async fn handle_deep_research(
    query: String,
    depth: Option<u8>,
    breadth: Option<u8>,
) -> Result<serde_json::Value> {
    let depth = depth.unwrap_or(3);
    let breadth = breadth.unwrap_or(10);

    let plan = ResearchPlanner::generate_plan(&query, depth, breadth as usize)
        .context("Failed to generate research plan")?;

    // Use WebSearchProvider as default (no API key required)
    let provider: Arc<dyn ResearchProvider + Send + Sync> = Arc::new(WebSearchProvider::new(3, 30));

    let config = DeepResearcherConfig {
        max_depth: plan.stop_conditions.max_depth,
        max_sources: plan.stop_conditions.max_sources as u8,
        strategy: ResearchStrategy::Comprehensive,
    };

    let researcher = DeepResearcher::new(config, provider);
    let report = researcher
        .research(&query)
        .await
        .context("Failed to conduct research")?;

    // Convert report to JSON
    Ok(serde_json::json!({
        "query": report.query,
        "summary": report.summary,
        "sources": report.sources.iter().map(|s| serde_json::json!({
            "title": s.title,
            "url": s.url,
            "snippet": s.snippet,
            "relevance_score": s.relevance_score,
        })).collect::<Vec<_>>(),
        "findings": report.findings.iter().map(|f| serde_json::json!({
            "content": f.content,
            "confidence": f.confidence,
        })).collect::<Vec<_>>(),
        "depth_reached": report.depth_reached,
        "diversity_score": report.diversity_score,
        "confidence_level": format!("{:?}", report.confidence_level),
        "strategy": format!("{:?}", report.strategy),
        "contradictions": report.contradictions.as_ref().map(|c| serde_json::json!({
            "contradiction_count": c.contradiction_count,
            "contradictions": c.contradictions.iter().map(|cont| serde_json::json!({
                "description": cont.description,
            })).collect::<Vec<_>>(),
        })),
    }))
}

/// Handle natural language command parsing from Chrome extension
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
        "success": true,
        "intent": {
            "intent": response.intent.intent,
            "args": response.intent.args,
            "risk": format!("{:?}", response.intent.risk),
            "requires_confirmation": response.intent.requires_confirmation,
        },
        "warnings": response.warnings,
    }))
}
