//! LSP diagnostics manager for collecting and managing diagnostic information
//!
//! Provides real-time diagnostic collection, caching, and WebSocket distribution

use anyhow::{Context, Result};
use lsp_types::{Diagnostic, PublishDiagnosticsParams, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};

/// Diagnostic information for a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDiagnostics {
    /// Document URI
    pub uri: Url,
    /// List of diagnostics
    pub diagnostics: Vec<Diagnostic>,
    /// Server name that produced these diagnostics
    pub server_name: String,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Diagnostics manager for collecting and managing LSP diagnostics
pub struct DiagnosticsManager {
    /// Document diagnostics cache
    diagnostics: Arc<RwLock<HashMap<Url, Vec<DocumentDiagnostics>>>>,
    /// Broadcast channel for real-time diagnostic updates
    diagnostics_tx: broadcast::Sender<DocumentDiagnostics>,
    /// Maximum number of cached diagnostics per document
    max_cached_per_document: usize,
}

impl DiagnosticsManager {
    /// Create a new diagnostics manager
    pub fn new(max_cached_per_document: usize) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            diagnostics: Arc::new(RwLock::new(HashMap::new())),
            diagnostics_tx: tx,
            max_cached_per_document,
        }
    }

    /// Update diagnostics for a document
    pub async fn update_diagnostics(
        &self,
        params: PublishDiagnosticsParams,
        server_name: String,
    ) -> Result<()> {
        let uri = params.uri.clone();
        let diagnostics = params.diagnostics;

        let doc_diagnostics = DocumentDiagnostics {
            uri: uri.clone(),
            diagnostics: diagnostics.clone(),
            server_name: server_name.clone(),
            updated_at: chrono::Utc::now(),
        };

        // Update cache
        {
            let mut cache = self.diagnostics.write().await;
            let entry = cache.entry(uri.clone()).or_insert_with(Vec::new);

            // Remove old diagnostics from the same server
            entry.retain(|d| d.server_name != server_name);

            // Add new diagnostics
            entry.push(doc_diagnostics.clone());

            // Limit cache size
            if entry.len() > self.max_cached_per_document {
                entry.sort_by_key(|d| d.updated_at);
                entry.drain(0..entry.len() - self.max_cached_per_document);
            }
        }

        // Broadcast update
        let _ = self.diagnostics_tx.send(doc_diagnostics);

        debug!(
            "Updated diagnostics for {} from {}: {} diagnostics",
            uri,
            server_name,
            diagnostics.len()
        );

        Ok(())
    }

    /// Get all diagnostics for a document
    pub async fn get_diagnostics(&self, uri: &Url) -> Vec<DocumentDiagnostics> {
        let cache = self.diagnostics.read().await;
        cache.get(uri).cloned().unwrap_or_default()
    }

    /// Get all diagnostics across all documents
    pub async fn get_all_diagnostics(&self) -> HashMap<Url, Vec<DocumentDiagnostics>> {
        let cache = self.diagnostics.read().await;
        cache.clone()
    }

    /// Get combined diagnostics for a document (from all servers)
    pub async fn get_combined_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let doc_diagnostics = self.get_diagnostics(uri).await;
        doc_diagnostics
            .into_iter()
            .flat_map(|d| d.diagnostics)
            .collect()
    }

    /// Clear diagnostics for a document
    pub async fn clear_diagnostics(&self, uri: &Url) {
        let mut cache = self.diagnostics.write().await;
        cache.remove(uri);
        info!("Cleared diagnostics for {}", uri);
    }

    /// Clear all diagnostics
    pub async fn clear_all(&self) {
        let mut cache = self.diagnostics.write().await;
        cache.clear();
        info!("Cleared all diagnostics");
    }

    /// Subscribe to diagnostic updates
    pub fn subscribe(&self) -> broadcast::Receiver<DocumentDiagnostics> {
        self.diagnostics_tx.subscribe()
    }

    /// Get statistics about diagnostics
    pub async fn get_statistics(&self) -> DiagnosticsStatistics {
        let cache = self.diagnostics.read().await;
        let mut total_diagnostics = 0;
        let total_documents = cache.len();
        let mut diagnostics_by_severity: HashMap<String, usize> = HashMap::new();

        for doc_diagnostics in cache.values().flatten() {
            total_diagnostics += doc_diagnostics.diagnostics.len();
            for diagnostic in &doc_diagnostics.diagnostics {
                let severity = match diagnostic.severity {
                    Some(lsp_types::DiagnosticSeverity::ERROR) => "error",
                    Some(lsp_types::DiagnosticSeverity::WARNING) => "warning",
                    Some(lsp_types::DiagnosticSeverity::INFORMATION) => "information",
                    Some(lsp_types::DiagnosticSeverity::HINT) => "hint",
                    Some(_) => "unknown",
                    None => "unknown",
                };
                *diagnostics_by_severity.entry(severity.to_string()).or_insert(0) += 1;
            }
        }

        DiagnosticsStatistics {
            total_documents,
            total_diagnostics,
            diagnostics_by_severity,
        }
    }
}

/// Statistics about diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsStatistics {
    /// Total number of documents with diagnostics
    pub total_documents: usize,
    /// Total number of diagnostics
    pub total_diagnostics: usize,
    /// Diagnostics grouped by severity
    pub diagnostics_by_severity: HashMap<String, usize>,
}
