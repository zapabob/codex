mod pipeline;
pub mod provider;
mod strategies;
pub mod types;

use anyhow::Result;
use provider::ResearchProvider;
use std::sync::Arc;
use types::DeepResearcherConfig;
use types::ResearchReport;

pub use provider::MockProvider;
pub use types::ResearchStrategy;

/// Main deep researcher for conducting comprehensive research
pub struct DeepResearcher {
    config: DeepResearcherConfig,
    provider: Arc<dyn ResearchProvider>,
}

impl DeepResearcher {
    /// Create a new deep researcher with the given configuration and provider
    pub fn new(config: DeepResearcherConfig, provider: Arc<dyn ResearchProvider>) -> Self {
        Self { config, provider }
    }

    /// Conduct research on the given query
    pub async fn research(&self, query: &str) -> Result<ResearchReport> {
        pipeline::conduct_research(Arc::clone(&self.provider), &self.config, query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_deep_researcher() {
        let config = DeepResearcherConfig::default();
        let provider = Arc::new(MockProvider);
        let researcher = DeepResearcher::new(config, provider);

        let report = researcher.research("Rust async patterns").await.unwrap();

        assert_eq!(report.query, "Rust async patterns");
        assert!(report.sources.len() > 0);
        assert!(!report.summary.is_empty());
    }

    #[tokio::test]
    async fn test_deep_researcher_with_custom_config() {
        let config = DeepResearcherConfig {
            max_depth: 2,
            max_sources: 5,
            strategy: ResearchStrategy::Focused,
        };
        let provider = Arc::new(MockProvider);
        let researcher = DeepResearcher::new(config, provider);

        let report = researcher.research("test query").await.unwrap();

        assert_eq!(report.strategy, ResearchStrategy::Focused);
        assert!(report.depth_reached <= 2);
    }
}
