use anyhow::Result;
use assert_cmd::Command;

/// Helper to create a codex command
fn codex_command() -> Result<Command> {
    let cmd = Command::cargo_bin("codex")?;
    Ok(cmd)
}

#[test]
fn supervisor_basic_execution() -> Result<()> {
    let mut cmd = codex_command()?;
    cmd.args(["supervisor", "Test goal"]).assert().success();
    Ok(())
}

#[test]
fn supervisor_with_agents() -> Result<()> {
    let mut cmd = codex_command()?;
    cmd.args([
        "supervisor",
        "Implement secure auth",
        "--agents",
        "Security,Backend",
    ])
    .assert()
    .success();
    Ok(())
}

#[test]
fn supervisor_with_strategy() -> Result<()> {
    let mut cmd = codex_command()?;
    cmd.args(["supervisor", "Test goal", "--strategy", "sequential"])
        .assert()
        .success();

    let mut cmd = codex_command()?;
    cmd.args(["supervisor", "Test goal", "--strategy", "parallel"])
        .assert()
        .success();

    let mut cmd = codex_command()?;
    cmd.args(["supervisor", "Test goal", "--strategy", "hybrid"])
        .assert()
        .success();

    Ok(())
}

#[test]
fn supervisor_json_output() -> Result<()> {
    let mut cmd = codex_command()?;
    let output = cmd
        .args(["supervisor", "Test goal", "--format", "json"])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout)?;
    assert!(json.get("goal").is_some());
    assert!(json.get("plan").is_some());
    assert!(json.get("assignments").is_some());

    Ok(())
}

#[test]
fn deep_research_basic_execution() -> Result<()> {
    let mut cmd = codex_command()?;
    cmd.args(["deep-research", "Test query"])
        .assert()
        .success();
    Ok(())
}

#[test]
fn deep_research_with_params() -> Result<()> {
    let mut cmd = codex_command()?;
    cmd.args([
        "deep-research",
        "Rust async patterns",
        "--levels",
        "2",
        "--sources",
        "5",
    ])
    .assert()
    .success();
    Ok(())
}

#[test]
fn deep_research_with_strategy() -> Result<()> {
    let mut cmd = codex_command()?;
    cmd.args([
        "deep-research",
        "Test query",
        "--strategy",
        "comprehensive",
    ])
    .assert()
    .success();

    let mut cmd = codex_command()?;
    cmd.args(["deep-research", "Test query", "--strategy", "focused"])
        .assert()
        .success();

    let mut cmd = codex_command()?;
    cmd.args(["deep-research", "Test query", "--strategy", "exploratory"])
        .assert()
        .success();

    Ok(())
}

#[test]
fn deep_research_json_output() -> Result<()> {
    let mut cmd = codex_command()?;
    let output = cmd
        .args(["deep-research", "Test query", "--format", "json"])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout)?;
    assert!(json.get("query").is_some());
    assert!(json.get("sources").is_some());
    assert!(json.get("findings").is_some());
    assert!(json.get("summary").is_some());

    Ok(())
}
