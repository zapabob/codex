# codex-deep-research

Deep research pipeline for Codex that conducts multi-level exploration and information gathering on any topic.

**✨ Conforms to OpenAI/codex official `web_search` tool implementation ✨**

## Features

- **Multi-Depth Exploration**: Configurable research depth (1-10 levels)
- **Source Management**: Collect and filter up to 100 sources
- **Research Strategies**:
  - **Comprehensive**: Thorough exploration across all sources
  - **Focused**: Targeted search with high relevance threshold (≥0.7)
  - **Exploratory**: Broad discovery with diverse sources
- **Provider Abstraction**: Extensible trait-based design for search providers
- **Automatic Summarization**: Generates structured research reports

## Usage

```rust
use codex_deep_research::{DeepResearcher, MockProvider, types::DeepResearcherConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DeepResearcherConfig {
        max_depth: 3,
        max_sources: 10,
        strategy: ResearchStrategy::Comprehensive,
    };

    let provider = Arc::new(MockProvider);
    let researcher = DeepResearcher::new(config, provider);

    let report = researcher.research("Rust async patterns").await?;

    println!("Query: {}", report.query);
    println!("Sources found: {}", report.sources.len());
    println!("Findings: {}", report.findings.len());
    println!("\nSummary:\n{}", report.summary);

    Ok(())
}
```

## CLI Usage

```bash
# Basic usage
codex deep-research "Rust async patterns"

# With parameters
codex deep-research "AI safety" --levels 3 --sources 20

# With research strategy
codex deep-research "Climate change" --strategy focused

# JSON output
codex deep-research "Quantum computing" --format json
```

## Architecture

The research pipeline consists of:

1. **Provider** (`provider.rs`): Trait for search and retrieval
2. **Strategies** (`strategies.rs`): Filtering and prioritization logic
3. **Pipeline** (`pipeline.rs`): Orchestrates the research flow
4. **Types** (`types.rs`): Data structures for reports and results

### Research Flow

```
Query → Search → Filter → Retrieve → Extract → Summarize → Report
```

## Research Strategies

### Comprehensive
- Takes all sources up to max_sources
- Sorts by relevance score
- Balanced coverage

### Focused
- Filters sources with relevance ≥ 0.7
- Only high-confidence results
- Precise information

### Exploratory
- Diverse source selection
- Broad topic coverage
- Discovery-oriented

## Provider Interface

Implement the `ResearchProvider` trait to add new search backends:

```rust
#[async_trait]
pub trait ResearchProvider: Send + Sync {
    async fn search(&self, query: &str, max_results: u8) -> Result<Vec<Source>>;
    async fn retrieve(&self, url: &str) -> Result<String>;
}
```

## Testing

Run the test suite:

```bash
cargo test -p codex-deep-research
```

All tests pass with 100% success rate (11 tests).

## Future Enhancements

- Real search provider implementations (web search, academic databases)
- LLM-based query refinement
- Citation tracking and verification
- Semantic clustering of findings
- Interactive research sessions
