# MCP Token Optimization Guide

## Overview

The MCP token optimization system helps reduce token consumption by:

1. Tracking tool usage frequency
2. Automatically unloading unused tools
3. Compressing tool descriptions
4. Selectively loading tools based on task requirements

## Configuration

### Enable Token Optimization

In `config.toml`:

```toml
[mcp_token_optimization]
enabled = true
auto_unload_enabled = true
auto_unload_threshold_secs = 3600  # 1 hour
min_usage_count = 0
compress_descriptions = true
max_tools_per_prompt = 50
token_budget_per_prompt = 10000  # Optional: limit tokens per prompt
```

## Features

### 1. Usage Tracking

Tool usage is automatically tracked when tools are called. Statistics include:
- Usage count
- Last used timestamp
- Total tokens consumed
- Average tokens per call

### 2. Automatic Unloading

Tools that haven't been used for the configured threshold (default: 1 hour) are automatically unloaded.

**Configuration**:
```toml
auto_unload_enabled = true
auto_unload_threshold_secs = 3600  # 1 hour
min_usage_count = 0  # Minimum uses before considering for unload
```

### 3. Description Compression

Long tool descriptions are compressed to reduce token usage (50-70% reduction).

**Configuration**:
```toml
compress_descriptions = true
```

### 4. Selective Loading

Tools are selected based on task requirements:

- **Keyword-based**: Analyzes task description for keywords
- **Relevance-based**: Selects tools relevant to the task
- **Budget-aware**: Respects token budget limits

### 5. Prompt Optimization

When building prompts, only relevant tools are included:

- Maximum tools per prompt (default: 50)
- Token budget per prompt (optional)
- Tool priority-based selection

## Usage

### Task-Based Loading

```rust
let loader = SelectiveToolLoader::new(optimizer, dynamic_loader);
let tools = loader.load_for_task("Search for Rust async best practices").await?;
// Only relevant tools (web-search, deep-research) are loaded
```

### Manual Tool Tracking

```rust
optimizer.track_tool_usage("tool-name", "server-name", tokens_consumed).await;
```

### Get Usage Statistics

```rust
let stats = optimizer.get_usage_stats().await;
for (tool, stat) in stats {
    println!("{}: {} uses, {} tokens", tool, stat.usage_count, stat.total_tokens_consumed);
}
```

## Best Practices

1. **Monitor Usage**: Regularly check usage statistics to identify unused tools
2. **Adjust Thresholds**: Tune `auto_unload_threshold_secs` based on your workflow
3. **Enable Compression**: Use `compress_descriptions = true` for significant token savings
4. **Set Budgets**: Configure `token_budget_per_prompt` to prevent token overflow
5. **Task-Specific Loading**: Use selective loading for task-specific workflows

## Token Savings

Typical token savings:

- **Description Compression**: 50-70% reduction
- **Selective Loading**: 30-50% reduction (depends on tool count)
- **Auto-Unloading**: Prevents accumulation of unused tool descriptions

**Example**: With 100 tools averaging 200 tokens each:
- Without optimization: 20,000 tokens
- With compression: 6,000-10,000 tokens (50-70% reduction)
- With selective loading: 3,000-7,000 tokens (65-85% reduction)

## Monitoring

Check token usage and optimization effectiveness:

```bash
# View usage statistics
codex mcp dynamic stats

# View unused tools
codex mcp dynamic unused
```
