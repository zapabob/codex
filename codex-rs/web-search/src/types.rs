use serde::Deserialize;
use serde::Serialize;

/// A source of information found during web search
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Source {
    pub url: String,
    pub title: String,
    pub snippet: String,
    pub relevance_score: f64,
}
