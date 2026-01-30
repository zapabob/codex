use anyhow::Context;
use anyhow::Result;
use chrono::Utc;
use codex_deep_research::DeepResearcher;
use codex_deep_research::DeepResearcherConfig;
use codex_deep_research::GeminiSearchProvider;
use codex_deep_research::McpSearchProvider;
use codex_deep_research::ResearchPlanner;
use codex_deep_research::ResearchStrategy;
use codex_deep_research::SearchBackend;
use codex_deep_research::provider::ResearchProvider;
use codex_web_search::WebSearchProvider;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn run_research_command(
    topic: String,
    depth: u8,
    breadth: u8,
    budget: usize,
    _citations: bool,
    mcp_url: Option<String>,
    lightweight_fallback: bool,
    out: Option<PathBuf>,
    use_gemini: bool,
    use_mcp: bool,
) -> Result<()> {
    println!("🔍 Starting deep research on: {}", topic);
    println!("   Depth: {}, Breadth: {}", depth, breadth);
    println!("   Budget: {} tokens", budget);

    let plan = ResearchPlanner::generate_plan(&topic, depth, breadth as usize)
        .context("Failed to generate research plan")?;

    println!("\n📋 Research Plan:");
    println!("   Main topic: {}", plan.main_topic);
    println!("   Sub-queries ({}):", plan.sub_queries.len());
    for (i, query) in plan.sub_queries.iter().enumerate() {
        println!("     {}. {}", i + 1, query);
    }

    let actual_plan = if lightweight_fallback && budget < 30_000 {
        println!("\n⚡ Using lightweight research mode due to budget constraints");
        ResearchPlanner::downgrade_to_lightweight(&plan)
    } else {
        plan
    };

    // Provider selection priority: Gemini CLI -> MCP search -> built-in web search.
    let provider: Arc<dyn ResearchProvider + Send + Sync> = if use_gemini {
        let model =
            std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-2.5-flash".to_string());
        if use_mcp {
            println!("🔌 Using Gemini CLI via MCP (Codex → MCP → Gemini CLI)");
            if let Some(url) = mcp_url.as_deref() {
                println!("   ℹ️ Requested MCP server: {url}");
            }
            println!(
                "   ℹ️ MCP-backed Gemini integration is not fully configured; falling back to direct CLI execution."
            );
        } else {
            println!("🤖 Using Gemini CLI with Google Search (Grounding)");
        }
        println!("   ℹ️ Note: Using OAuth 2.0 authentication (API key not required)");
        Arc::new(GeminiSearchProvider::new(model))
    } else if let Some(url) = mcp_url.as_deref() {
        println!("🔌 Using MCP Search Provider (DuckDuckGo backend)");
        println!("   ℹ️ Server: {url}");
        Arc::new(McpSearchProvider::new(SearchBackend::DuckDuckGo, None))
    } else {
        println!("🌐 Using Web Search Provider with DuckDuckGo integration");
        println!("   Priority: Brave > Google > Bing > DuckDuckGo (no API key required)");

        if std::env::var("BRAVE_API_KEY").is_ok() {
            println!("   ✅ Brave Search API detected");
        } else if std::env::var("GOOGLE_API_KEY").is_ok() && std::env::var("GOOGLE_CSE_ID").is_ok()
        {
            println!("   ✅ Google Custom Search API detected");
        } else if std::env::var("BING_API_KEY").is_ok() {
            println!("   ✅ Bing Web Search API detected");
        } else {
            println!("   🔓 No API keys found, using DuckDuckGo (free, no API key required)");
        }

        Arc::new(WebSearchProvider::new(3, 30))
    };

    let config = DeepResearcherConfig {
        max_depth: actual_plan.stop_conditions.max_depth,
        max_sources: actual_plan.stop_conditions.max_sources as u8,
        strategy: ResearchStrategy::Comprehensive,
    };

    let researcher = DeepResearcher::new(config, provider);
    let report = researcher
        .research(&topic)
        .await
        .context("Failed to conduct research")?;

    println!("\n📊 Research Report:");
    println!("   Query: {}", report.query);
    println!("   Strategy: {:?}", report.strategy);
    println!("   Depth reached: {}", report.depth_reached);
    println!("   Sources found: {}", report.sources.len());
    println!("   Diversity score: {:.2}", report.diversity_score);
    println!("   Confidence: {:?}", report.confidence_level);

    if let Some(ref contradictions) = report.contradictions {
        println!(
            "\n⚠️ Contradictions detected: {}",
            contradictions.contradiction_count
        );
        for (i, contradiction) in contradictions.contradictions.iter().enumerate().take(3) {
            println!("   {}. {}", i + 1, contradiction.description);
        }
    }

    println!("\n📝 Summary:");
    println!("{}", report.summary);

    println!("\n🔗 Sources:");
    for (i, source) in report.sources.iter().enumerate() {
        println!("   [{}] {} - {}", i + 1, source.title, source.url);
    }

    let out_path = out.unwrap_or_else(|| PathBuf::from("artifacts/report.md"));
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let markdown = generate_markdown_report(&report);
    std::fs::write(&out_path, markdown)?;

    println!("\n💾 Report saved to: {}", out_path.display());

    Ok(())
}

fn generate_markdown_report(report: &codex_deep_research::types::ResearchReport) -> String {
    let mut md = String::new();

    // Title with timestamp
    md.push_str(&format!("# {}\n\n", report.query));
    md.push_str(&format!(
        "*Generated on: {}*\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    // Executive Summary
    md.push_str("## Executive Summary\n\n");
    md.push_str(&format!("{}\n\n", report.summary));

    // Key Metrics Table
    md.push_str("## Key Metrics\n\n");
    md.push_str("| Metric | Value |\n");
    md.push_str("|--------|-------|\n");
    md.push_str(&format!("| **Strategy** | {:?} |\n", report.strategy));
    md.push_str(&format!(
        "| **Depth Reached** | {} |\n",
        report.depth_reached
    ));
    md.push_str(&format!(
        "| **Total Sources** | {} |\n",
        report.sources.len()
    ));
    md.push_str(&format!(
        "| **Diversity Score** | {:.2} |\n",
        report.diversity_score
    ));
    md.push_str(&format!(
        "| **Confidence Level** | {:?} |\n\n",
        report.confidence_level
    ));

    // Contradictions Section (Enhanced)
    if let Some(ref contradictions) = report.contradictions {
        if contradictions.contradiction_count > 0 {
            md.push_str("## ⚠️ Contradictions Detected\n\n");
            md.push_str(&format!(
                "**Total**: {} contradiction(s) found\n\n",
                contradictions.contradiction_count
            ));
            md.push_str("| # | Description | Severity |\n");
            md.push_str("|---|-------------|----------|\n");
            for (i, contradiction) in contradictions.contradictions.iter().enumerate() {
                let severity = if contradiction.description.contains("critical")
                    || contradiction.description.contains("major")
                {
                    "High"
                } else if contradiction.description.contains("minor") {
                    "Low"
                } else {
                    "Medium"
                };
                md.push_str(&format!(
                    "| {} | {} | {} |\n",
                    i + 1,
                    contradiction.description,
                    severity
                ));
            }
            md.push_str("\n");
        }
    }

    // Findings Section (Enhanced with categorization)
    md.push_str("## Detailed Findings\n\n");
    if report.findings.is_empty() {
        md.push_str("*No specific findings documented.*\n\n");
    } else {
        for (i, finding) in report.findings.iter().enumerate() {
            md.push_str(&format!(
                "### Finding {}: {}\n\n",
                i + 1,
                finding
                    .content
                    .lines()
                    .next()
                    .unwrap_or("Untitled")
                    .chars()
                    .take(60)
                    .collect::<String>()
            ));
            md.push_str(&format!("{}\n\n", finding.content));

            // Confidence indicator
            let confidence_bar = if finding.confidence >= 0.8 {
                "████████████████████"
            } else if finding.confidence >= 0.6 {
                "████████████░░░░░░░░"
            } else if finding.confidence >= 0.4 {
                "████████░░░░░░░░░░░░"
            } else {
                "████░░░░░░░░░░░░░░░░"
            };
            md.push_str(&format!(
                "**Confidence**: {:.1}% {}\n\n",
                finding.confidence * 100.0,
                confidence_bar
            ));
        }
    }

    // Sources Section (Enhanced with better organization)
    md.push_str("## Sources & Citations\n\n");
    if report.sources.is_empty() {
        md.push_str("*No sources found.*\n\n");
    } else {
        // Group sources by relevance
        let mut high_relevance: Vec<_> = report
            .sources
            .iter()
            .filter(|s| s.relevance_score >= 0.7)
            .collect();
        let mut medium_relevance: Vec<_> = report
            .sources
            .iter()
            .filter(|s| s.relevance_score >= 0.4 && s.relevance_score < 0.7)
            .collect();
        let mut low_relevance: Vec<_> = report
            .sources
            .iter()
            .filter(|s| s.relevance_score < 0.4)
            .collect();

        if !high_relevance.is_empty() {
            md.push_str("### High Relevance Sources\n\n");
            for (i, source) in high_relevance.iter().enumerate() {
                md.push_str(&format!(
                    "{}. **[{}]({})** (Relevance: {:.1}%)\n",
                    i + 1,
                    source.title,
                    source.url,
                    source.relevance_score * 100.0
                ));
                md.push_str(&format!("   > {}\n\n", source.snippet));
            }
        }

        if !medium_relevance.is_empty() {
            md.push_str("### Medium Relevance Sources\n\n");
            for (i, source) in medium_relevance.iter().enumerate() {
                md.push_str(&format!(
                    "{}. [{}]({}) (Relevance: {:.1}%)\n",
                    i + 1,
                    source.title,
                    source.url,
                    source.relevance_score * 100.0
                ));
                md.push_str(&format!("   > {}\n\n", source.snippet));
            }
        }

        if !low_relevance.is_empty() {
            md.push_str("### Additional Sources\n\n");
            for (i, source) in low_relevance.iter().enumerate() {
                md.push_str(&format!(
                    "{}. [{}]({}) (Relevance: {:.1}%)\n\n",
                    i + 1,
                    source.title,
                    source.url,
                    source.relevance_score * 100.0
                ));
            }
        }
    }

    // Research Methodology
    md.push_str("## Research Methodology\n\n");
    md.push_str(&format!("- **Research Strategy**: {:?}\n", report.strategy));
    md.push_str(&format!("- **Maximum Depth**: {}\n", report.depth_reached));
    md.push_str(&format!(
        "- **Source Diversity**: {:.2}\n",
        report.diversity_score
    ));
    md.push_str(&format!(
        "- **Confidence Assessment**: {:?}\n\n",
        report.confidence_level
    ));

    // Footer
    md.push_str("---\n\n");
    md.push_str(&format!(
        "*This report was generated by Codex DeepResearch on {}*\n",
        chrono::Utc::now().format("%Y-%m-%d at %H:%M:%S UTC")
    ));

    md
}
