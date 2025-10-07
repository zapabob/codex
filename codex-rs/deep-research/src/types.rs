use serde::Deserialize;
use serde::Serialize;

/// Strategy for conducting research
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResearchStrategy {
    /// Thorough exploration across multiple sources
    Comprehensive,
    /// Targeted search for specific information
    Focused,
    /// Broad exploration for discovery
    Exploratory,
}

impl Default for ResearchStrategy {
    fn default() -> Self {
        Self::Comprehensive
    }
}

/// A source of information found during research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
    pub title: String,
    pub snippet: String,
    pub relevance_score: f64,
}

/// A finding extracted from research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub content: String,
    pub sources: Vec<String>,
    pub confidence: f64,
}

/// Final research report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    pub query: String,
    pub strategy: ResearchStrategy,
    pub sources: Vec<Source>,
    pub findings: Vec<Finding>,
    pub summary: String,
    pub depth_reached: u8,
}

/// Configuration for the deep researcher
#[derive(Debug, Clone)]
pub struct DeepResearcherConfig {
    pub max_depth: u8,
    pub max_sources: u8,
    pub strategy: ResearchStrategy,
}

impl Default for DeepResearcherConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_sources: 10,
            strategy: ResearchStrategy::default(),
        }
    }
}
