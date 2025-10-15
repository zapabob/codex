//! Natural language interpreter for agent invocation.
//!
//! Parses natural language commands and translates them into agent
//! names and parameters for execution.

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsed agent invocation from natural language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInvocation {
    /// Agent name to invoke
    pub agent_name: String,
    /// Goal/task description for the agent
    pub goal: String,
    /// Additional parameters parsed from the input
    pub parameters: HashMap<String, String>,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Natural language interpreter for agent commands.
pub struct AgentInterpreter {
    /// Precompiled patterns for matching agent intents
    patterns: Vec<Pattern>,
}

#[derive(Clone)]
struct Pattern {
    /// Regular expression to match
    regex: Regex,
    /// Agent name to invoke
    agent_name: String,
    /// Parameter extractors
    param_extractors: Vec<ParamExtractor>,
    /// Base confidence for this pattern
    confidence: f64,
}

#[derive(Clone)]
struct ParamExtractor {
    /// Parameter name
    name: String,
    /// Capture group index in regex
    group_index: usize,
    /// Default value if not captured
    default: Option<String>,
}

impl AgentInterpreter {
    /// Create a new agent interpreter with default patterns.
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }

    /// Parse natural language input into an agent invocation.
    pub fn parse(&self, input: &str) -> Result<AgentInvocation> {
        let input_lower = input.to_lowercase();

        // Try each pattern in order of confidence
        for pattern in &self.patterns {
            if let Some(captures) = pattern.regex.captures(&input_lower) {
                let mut parameters = HashMap::new();

                // Extract parameters from capture groups
                for extractor in &pattern.param_extractors {
                    let value = captures
                        .get(extractor.group_index)
                        .map(|m| m.as_str().to_string())
                        .or_else(|| extractor.default.clone());

                    if let Some(v) = value {
                        parameters.insert(extractor.name.clone(), v);
                    }
                }

                return Ok(AgentInvocation {
                    agent_name: pattern.agent_name.clone(),
                    goal: input.to_string(),
                    parameters,
                    confidence: pattern.confidence,
                });
            }
        }

        // Fallback: use code-reviewer for general tasks
        Ok(AgentInvocation {
            agent_name: "code-reviewer".to_string(),
            goal: input.to_string(),
            parameters: HashMap::new(),
            confidence: 0.3,
        })
    }

    /// Get default patterns for common agent invocations.
    fn default_patterns() -> Vec<Pattern> {
        vec![
            // Security patterns
            Pattern {
                regex: Regex::new(
                    r"(?i)(security|sec|audit|vulnerability|vuln|exploit|cve|oauth|auth|jwt|token)",
                )
                .unwrap(),
                agent_name: "sec-audit".to_string(),
                param_extractors: vec![],
                confidence: 0.95,
            },
            // Test patterns
            Pattern {
                regex: Regex::new(r"(?i)(test|unit test|integration test|e2e|spec|jest|pytest)")
                    .unwrap(),
                agent_name: "test-gen".to_string(),
                param_extractors: vec![],
                confidence: 0.9,
            },
            // Review patterns
            Pattern {
                regex: Regex::new(r"(?i)(review|check|inspect|analyze|examine|lint)(?:\s+(.+))?")
                    .unwrap(),
                agent_name: "code-reviewer".to_string(),
                param_extractors: vec![ParamExtractor {
                    name: "scope".to_string(),
                    group_index: 2,
                    default: Some(".".to_string()),
                }],
                confidence: 0.85,
            },
            // Research patterns
            Pattern {
                regex: Regex::new(
                    r"(?i)(research|investigate|learn|study|find out|explore)(?:\s+(.+))?",
                )
                .unwrap(),
                agent_name: "researcher".to_string(),
                param_extractors: vec![ParamExtractor {
                    name: "query".to_string(),
                    group_index: 2,
                    default: None,
                }],
                confidence: 0.8,
            },
            // TypeScript specific
            Pattern {
                regex: Regex::new(r"(?i)(typescript|ts|tsx|react)").unwrap(),
                agent_name: "ts-reviewer".to_string(),
                param_extractors: vec![],
                confidence: 0.75,
            },
            // Python specific
            Pattern {
                regex: Regex::new(r"(?i)(python|py|pytest|django|flask)").unwrap(),
                agent_name: "python-reviewer".to_string(),
                param_extractors: vec![],
                confidence: 0.75,
            },
            // Unity specific
            Pattern {
                regex: Regex::new(r"(?i)(unity|c#|csharp|game|gameobject)").unwrap(),
                agent_name: "unity-reviewer".to_string(),
                param_extractors: vec![],
                confidence: 0.75,
            },
        ]
    }

    /// Add a custom pattern to the interpreter.
    pub fn add_pattern(
        &mut self,
        regex_str: &str,
        agent_name: String,
        confidence: f64,
    ) -> Result<()> {
        let regex = Regex::new(regex_str)
            .with_context(|| format!("Invalid regex pattern: {regex_str}"))?;

        self.patterns.push(Pattern {
            regex,
            agent_name,
            param_extractors: vec![],
            confidence,
        });

        // Re-sort patterns by confidence (highest first)
        self.patterns.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(())
    }

    /// Get all available agent names from patterns.
    pub fn available_agents(&self) -> Vec<String> {
        let mut agents: Vec<String> = self.patterns.iter().map(|p| p.agent_name.clone()).collect();
        agents.sort();
        agents.dedup();
        agents
    }
}

impl Default for AgentInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_pattern() {
        let interpreter = AgentInterpreter::new();

        let result = interpreter
            .parse("Security audit this authentication module")
            .unwrap();
        assert_eq!(result.agent_name, "sec-audit");
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_test_pattern() {
        let interpreter = AgentInterpreter::new();

        let result = interpreter
            .parse("Generate unit tests for this component")
            .unwrap();
        assert_eq!(result.agent_name, "test-gen");
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_review_pattern() {
        let interpreter = AgentInterpreter::new();

        let result = interpreter
            .parse("Review this file with security focus")
            .unwrap();
        assert_eq!(result.agent_name, "code-reviewer");
    }

    #[test]
    fn test_research_pattern() {
        let interpreter = AgentInterpreter::new();

        let result = interpreter
            .parse("Research React Server Components best practices")
            .unwrap();
        assert_eq!(result.agent_name, "researcher");
    }

    #[test]
    fn test_typescript_pattern() {
        let interpreter = AgentInterpreter::new();

        let result = interpreter.parse("Review this TypeScript code").unwrap();
        assert_eq!(result.agent_name, "ts-reviewer");
    }

    #[test]
    fn test_fallback() {
        let interpreter = AgentInterpreter::new();

        let result = interpreter.parse("Do something generic").unwrap();
        assert_eq!(result.agent_name, "code-reviewer");
        assert!(result.confidence < 0.5);
    }

    #[test]
    fn test_custom_pattern() {
        let mut interpreter = AgentInterpreter::new();
        interpreter
            .add_pattern(r"(?i)refactor", "code-reviewer".to_string(), 0.85)
            .unwrap();

        let result = interpreter.parse("Refactor this module").unwrap();
        assert_eq!(result.agent_name, "code-reviewer");
    }
}
