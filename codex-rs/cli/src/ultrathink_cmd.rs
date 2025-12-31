//! Ultrathink Mode command for deep reasoning chains

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use codex_core::reasoning::ReasoningChain;
use codex_core::reasoning::ReasoningConfig;
use std::io::Write;
use std::io::{self};
use tracing::info;

/// Ultrathink Mode command for deep reasoning
#[derive(Debug, Parser)]
pub struct UltrathinkCli {
    /// Question or problem to reason about
    #[clap(value_name = "QUESTION")]
    pub question: String,

    /// Maximum depth of reasoning chain
    #[clap(long, default_value = "10")]
    pub max_depth: usize,

    /// Timeout in seconds
    #[clap(long, default_value = "300")]
    pub timeout: u64,

    /// Initial context or background information
    #[clap(long)]
    pub context: Option<String>,

    /// Output format (text, json)
    #[clap(long, default_value = "text")]
    pub format: String,
}

/// Run the ultrathink command
pub async fn run_ultrathink(cli: UltrathinkCli) -> Result<()> {
    info!("Starting Ultrathink Mode reasoning chain");

    let config = ReasoningConfig::new()
        .with_depth(cli.max_depth)
        .with_timeout(cli.timeout);

    let chain = ReasoningChain::new(config);

    // Execute reasoning chain
    let result = chain
        .execute(cli.question.clone(), cli.context.clone())
        .await
        .context("Failed to execute reasoning chain")?;

    // Output results
    match cli.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&result)?;
            println!("{}", json);
        }
        _ => {
            output_text_format(&result)?;
        }
    }

    Ok(())
}

/// Output results in text format
fn output_text_format(result: &codex_core::reasoning::ReasoningResult) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "=== Ultrathink Mode Reasoning Chain ===")?;
    writeln!(handle, "Chain ID: {}", result.chain_id)?;
    writeln!(handle, "Execution Time: {:.2}s", result.execution_time)?;
    writeln!(
        handle,
        "Overall Confidence: {:.1}%",
        result.overall_confidence * 100.0
    )?;
    writeln!(
        handle,
        "Verification Passed: {}",
        result.verification_passed
    )?;
    writeln!(handle)?;

    if let Some(ref counter_evidence) = result.counter_evidence_summary {
        writeln!(handle, "Counter-Evidence: {}", counter_evidence)?;
        writeln!(handle)?;
    }

    writeln!(handle, "=== Reasoning Steps ===")?;
    for (i, step) in result.steps.iter().enumerate() {
        writeln!(handle, "\nStep {}: {}", i + 1, step.description)?;
        writeln!(handle, "  Reasoning: {}", step.reasoning)?;
        if let Some(ref result) = step.result {
            writeln!(handle, "  Result: {}", result)?;
        }
        writeln!(handle, "  Confidence: {:.1}%", step.confidence * 100.0)?;
        writeln!(handle, "  Verified: {}", step.verified)?;
        if let Some(ref counter) = step.counter_evidence {
            writeln!(handle, "  Counter-Evidence: {}", counter)?;
        }
    }

    writeln!(handle)?;
    writeln!(handle, "=== Final Conclusion ===")?;
    writeln!(handle, "{}", result.conclusion)?;

    Ok(())
}
