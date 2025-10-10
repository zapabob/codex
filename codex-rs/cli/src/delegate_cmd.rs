use anyhow::Context;
use anyhow::Result;
use codex_core::agents::AgentRuntime;
use std::collections::HashMap;
use std::path::PathBuf;

pub async fn run_delegate_command(
    agent: String,
    goal: Option<String>,
    scope: Option<PathBuf>,
    budget: Option<usize>,
    deadline: Option<u64>,
    out: Option<PathBuf>,
) -> Result<()> {
    let workspace_dir = std::env::current_dir()?;
    let total_budget = budget.unwrap_or(40000);

    let runtime = AgentRuntime::new(workspace_dir.clone(), total_budget);

    // デフォルトのゴールを設定
    let goal_str = goal.unwrap_or_else(|| {
        if let Some(ref scope) = scope {
            format!("Process files in {}", scope.display())
        } else {
            "Complete delegated task".to_string()
        }
    });

    // 入力パラメータを準備
    let mut inputs = HashMap::new();
    if let Some(scope) = scope {
        inputs.insert("scope".to_string(), scope.display().to_string());
    }

    println!("🤖 Delegating to agent '{}'...", agent);
    println!("   Goal: {}", goal_str);
    if let Some(budget) = budget {
        println!("   Budget: {} tokens", budget);
    }
    if let Some(deadline) = deadline {
        println!("   Deadline: {} minutes", deadline);
    }

    // エージェント実行
    let result = runtime
        .delegate(&agent, &goal_str, inputs, budget, deadline)
        .await
        .with_context(|| format!("Failed to delegate to agent '{}'", agent))?;

    // 結果を出力
    println!("\n✅ Agent '{}' completed!", agent);
    println!("   Status: {:?}", result.status);
    println!("   Tokens used: {}", result.tokens_used);
    println!("   Duration: {:.2}s", result.duration_secs);

    if !result.artifacts.is_empty() {
        println!("\n📄 Generated artifacts:");
        for artifact in &result.artifacts {
            println!("   - {}", artifact);
        }
    }

    if let Some(error) = &result.error {
        println!("\n❌ Error: {}", error);
    }

    // 結果をファイルに保存
    if let Some(out_path) = out {
        let report = serde_json::to_string_pretty(&result)?;
        std::fs::write(&out_path, report)?;
        println!("\n💾 Result saved to: {}", out_path.display());
    }

    Ok(())
}
