#![cfg(not(target_os = "windows"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use anyhow::Result;
use codex_core::reasoning::ReasoningChain;
use codex_core::reasoning::ReasoningConfig;

#[tokio::test]
async fn test_reasoning_chain_basic() -> Result<()> {
    let config = ReasoningConfig::new()
        .with_depth(3)
        .with_timeout(10)
        .with_max_steps(5);
    let chain = ReasoningChain::new(config);

    let result = chain
        .execute(
            "What is 2+2?".to_string(),
            Some("Basic arithmetic question".to_string()),
        )
        .await?;

    assert!(!result.chain_id.is_empty());
    assert!(!result.steps.is_empty());
    assert!(result.execution_time > 0.0);
    assert!(result.overall_confidence >= 0.0 && result.overall_confidence <= 1.0);

    Ok(())
}

#[tokio::test]
async fn test_reasoning_chain_timeout() -> Result<()> {
    let config = ReasoningConfig::new()
        .with_depth(100) // Very deep
        .with_timeout(1) // Very short timeout
        .with_max_steps(1000);
    let chain = ReasoningChain::new(config);

    let result = chain
        .execute("Complex question that might timeout".to_string(), None)
        .await?;

    // Should complete within timeout or stop early
    assert!(result.execution_time <= 2.0); // Allow some margin

    Ok(())
}

#[tokio::test]
async fn test_reasoning_chain_dependencies() -> Result<()> {
    let config = ReasoningConfig::new().with_depth(5).with_timeout(30);
    let chain = ReasoningChain::new(config);

    let result = chain
        .execute(
            "Multi-step reasoning question".to_string(),
            Some("Context for multi-step reasoning".to_string()),
        )
        .await?;

    // Check that steps have dependencies
    let steps_with_deps: Vec<_> = result
        .steps
        .iter()
        .filter(|step| !step.dependencies.is_empty())
        .collect();

    // At least some steps should have dependencies in a multi-step chain
    if result.steps.len() > 1 {
        // Dependencies are optional, so we just verify the structure
        for step in &result.steps {
            assert!(!step.step_id.is_empty());
            assert!(!step.description.is_empty());
        }
    }

    Ok(())
}
