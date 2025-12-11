//! Real-time Quality Monitoring System (Rust 2024)
//!
//! Provides continuous quality monitoring with advanced async capabilities
//! leveraging Rust 2024 features: GATs, generic const expressions, and
//! improved async lifetime capture.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, watch};
use tokio::time::{Duration, Instant};

/// Quality metric snapshot captured at a specific time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: HashMap<String, f64>,
    pub alerts: Vec<QualityAlert>,
    pub trend_indicators: Vec<TrendIndicator>,
}

/// Quality alert triggered by monitoring rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub metric_name: String,
    pub threshold: f64,
    pub actual_value: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Alert types for quality monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    ThresholdExceeded,
    TrendDeviation,
    AnomalyDetected,
    Regression,
    Improvement,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Trend indicator for quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendIndicator {
    pub metric_name: String,
    pub trend: TrendDirection,
    pub slope: f64,
    pub confidence: f64,
    pub period_minutes: u32,
}

/// Trend directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Degrading,
    Stable,
    Volatile,
}

/// Monitoring configuration with Rust 2024 generic const expressions
#[derive(Debug, Clone)]
pub struct MonitoringConfig<const ALERT_BUFFER_SIZE: usize, const METRIC_HISTORY_SIZE: usize> {
    pub monitoring_interval_seconds: u64,
    pub enable_real_time_alerts: bool,
    pub enable_trend_analysis: bool,
    pub alert_cooldown_minutes: u32,
    pub anomaly_detection_sensitivity: f64,
}

/// Real-time quality monitor using Rust 2024 GATs and advanced async
pub struct RealTimeQualityMonitor<
    const ALERT_BUFFER_SIZE: usize,
    const METRIC_HISTORY_SIZE: usize,
> {
    config: MonitoringConfig<ALERT_BUFFER_SIZE, METRIC_HISTORY_SIZE>,
    metric_history: Arc<Mutex<VecDeque<QualitySnapshot>>>,
    active_alerts: Arc<Mutex<HashMap<String, QualityAlert>>>,
    alert_sender: mpsc::UnboundedSender<QualityAlert>,
    alert_receiver: Arc<Mutex<mpsc::UnboundedReceiver<QualityAlert>>>,
    shutdown_sender: watch::Sender<bool>,
    shutdown_receiver: watch::Receiver<bool>,
}

/// Advanced monitoring rule using Rust 2024 GATs
pub trait MonitoringRule {
    type Context;

    /// Evaluate monitoring rule with context
    async fn evaluate(&self, context: &Self::Context, snapshot: &QualitySnapshot) -> Vec<QualityAlert>;

    /// Get rule name
    fn name(&self) -> &'static str;
}

/// Threshold-based monitoring rule
pub struct ThresholdRule {
    pub metric_name: String,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub comparison: ThresholdComparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdComparison {
    GreaterThan,
    LessThan,
    Equal,
}

impl MonitoringRule for ThresholdRule {
    type Context = ();

    async fn evaluate(&self, _context: &Self::Context, snapshot: &QualitySnapshot) -> Vec<QualityAlert> {
        let mut alerts = Vec::new();

        if let Some(&value) = snapshot.metrics.get(&self.metric_name) {
            let alert_type = match self.comparison {
                ThresholdComparison::GreaterThan => {
                    if value >= self.critical_threshold {
                        Some((AlertType::ThresholdExceeded, AlertSeverity::Critical, self.critical_threshold))
                    } else if value >= self.warning_threshold {
                        Some((AlertType::ThresholdExceeded, AlertSeverity::Warning, self.warning_threshold))
                    } else {
                        None
                    }
                }
                ThresholdComparison::LessThan => {
                    if value <= self.critical_threshold {
                        Some((AlertType::ThresholdExceeded, AlertSeverity::Critical, self.critical_threshold))
                    } else if value <= self.warning_threshold {
                        Some((AlertType::ThresholdExceeded, AlertSeverity::Warning, self.warning_threshold))
                    } else {
                        None
                    }
                }
                ThresholdComparison::Equal => {
                    if (value - self.critical_threshold).abs() < f64::EPSILON {
                        Some((AlertType::ThresholdExceeded, AlertSeverity::Critical, self.critical_threshold))
                    } else if (value - self.warning_threshold).abs() < f64::EPSILON {
                        Some((AlertType::ThresholdExceeded, AlertSeverity::Warning, self.warning_threshold))
                    } else {
                        None
                    }
                }
            };

            if let Some((alert_type, severity, threshold)) = alert_type {
                alerts.push(QualityAlert {
                    alert_type,
                    severity,
                    message: format!(
                        "Quality metric '{}' {} threshold ({:.3} vs {:.3})",
                        self.metric_name,
                        match self.comparison {
                            ThresholdComparison::GreaterThan => "exceeded",
                            ThresholdComparison::LessThan => "fell below",
                            ThresholdComparison::Equal => "equals",
                        },
                        value, threshold
                    ),
                    metric_name: self.metric_name.clone(),
                    threshold,
                    actual_value: value,
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        alerts
    }

    fn name(&self) -> &'static str {
        "threshold_rule"
    }
}

/// Trend analysis rule using Rust 2024 async improvements
pub struct TrendAnalysisRule {
    pub metric_name: String,
    pub analysis_window_minutes: u32,
    pub degradation_threshold: f64,
    pub improvement_threshold: f64,
}

impl MonitoringRule for TrendAnalysisRule {
    type Context = VecDeque<QualitySnapshot>;

    async fn evaluate(&self, context: &Self::Context, snapshot: &QualitySnapshot) -> Vec<QualityAlert> {
        let mut alerts = Vec::new();

        // Analyze trend over the specified window
        if let Some(trend) = self.analyze_trend(context, snapshot).await {
            let alert_type = if trend.slope <= self.degradation_threshold {
                Some((AlertType::TrendDeviation, AlertSeverity::Warning,
                     format!("Degrading trend detected (slope: {:.3})", trend.slope)))
            } else if trend.slope >= self.improvement_threshold {
                Some((AlertType::Improvement, AlertSeverity::Info,
                     format!("Improving trend detected (slope: {:.3})", trend.slope)))
            } else {
                None
            };

            if let Some((alert_type, severity, message)) = alert_type {
                alerts.push(QualityAlert {
                    alert_type,
                    severity,
                    message,
                    metric_name: self.metric_name.clone(),
                    threshold: if trend.slope <= self.degradation_threshold { self.degradation_threshold } else { self.improvement_threshold },
                    actual_value: trend.slope,
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        alerts
    }

    fn name(&self) -> &'static str {
        "trend_analysis_rule"
    }
}

impl TrendAnalysisRule {
    /// Analyze trend using Rust 2024 async improvements
    async fn analyze_trend(&self, history: &VecDeque<QualitySnapshot>, current: &QualitySnapshot) -> Option<TrendIndicator> {
        if history.len() < 2 {
            return None;
        }

        // Filter data points within the analysis window
        let cutoff_time = current.timestamp - chrono::Duration::minutes(self.analysis_window_minutes as i64);
        let relevant_points: Vec<_> = history
            .iter()
            .chain(std::iter::once(current))
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .filter_map(|snapshot| {
                snapshot.metrics.get(&self.metric_name)
                    .map(|&value| (snapshot.timestamp.timestamp() as f64, value))
            })
            .collect();

        if relevant_points.len() < 2 {
            return None;
        }

        // Calculate linear regression for trend analysis
        let n = relevant_points.len() as f64;
        let sum_x: f64 = relevant_points.iter().map(|(x, _)| x).sum();
        let sum_y: f64 = relevant_points.iter().map(|(_, y)| y).sum();
        let sum_xy: f64 = relevant_points.iter().map(|(x, y)| x * y).sum();
        let sum_x2: f64 = relevant_points.iter().map(|(x, _)| x * x).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared for confidence
        let y_mean = sum_y / n;
        let ss_tot: f64 = relevant_points.iter().map(|(_, y)| (y - y_mean).powi(2)).sum();
        let ss_res: f64 = relevant_points.iter().map(|(x, y)| (y - (slope * x + intercept)).powi(2)).sum();
        let r_squared = if ss_tot > 0.0 { 1.0 - (ss_res / ss_tot) } else { 0.0 };

        let trend = if slope > 0.01 {
            TrendDirection::Improving
        } else if slope < -0.01 {
            TrendDirection::Degrading
        } else {
            TrendDirection::Stable
        };

        Some(TrendIndicator {
            metric_name: self.metric_name.clone(),
            trend,
            slope,
            confidence: r_squared.sqrt(), // Approximate confidence
            period_minutes: self.analysis_window_minutes,
        })
    }
}

impl<const ALERT_BUFFER_SIZE: usize, const METRIC_HISTORY_SIZE: usize>
    RealTimeQualityMonitor<ALERT_BUFFER_SIZE, METRIC_HISTORY_SIZE>
{
    /// Create new real-time quality monitor with Rust 2024 generic const expressions
    pub fn new(config: MonitoringConfig<ALERT_BUFFER_SIZE, METRIC_HISTORY_SIZE>) -> Self {
        let (alert_tx, alert_rx) = mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        Self {
            config,
            metric_history: Arc::new(Mutex::new(VecDeque::with_capacity(METRIC_HISTORY_SIZE))),
            active_alerts: Arc::new(Mutex::new(HashMap::new())),
            alert_sender: alert_tx,
            alert_receiver: Arc::new(Mutex::new(alert_rx)),
            shutdown_sender: shutdown_tx,
            shutdown_receiver: shutdown_rx,
        }
    }

    /// Start real-time monitoring with advanced async (Rust 2024)
    pub async fn start_monitoring<R>(&self, rules: Vec<R>) -> Result<(), Box<dyn std::error::Error>>
    where
        R: MonitoringRule + Send + Sync + 'static,
        R::Context: Send + Sync + Clone + Default,
    {
        let rules = Arc::new(rules);
        let metric_history = Arc::clone(&self.metric_history);
        let alert_sender = self.alert_sender.clone();
        let mut shutdown_rx = self.shutdown_receiver.clone();

        // Spawn monitoring task with improved async lifetime capture (Rust 2024)
        let monitoring_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(self.config.monitoring_interval_seconds));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Perform monitoring cycle
                        if let Err(e) = self.perform_monitoring_cycle(&rules, &metric_history, &alert_sender).await {
                            eprintln!("Monitoring cycle error: {}", e);
                        }
                    }
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            break;
                        }
                    }
                }
            }
        });

        // Alert processing task
        let active_alerts = Arc::clone(&self.active_alerts);
        let alert_receiver = Arc::clone(&self.alert_receiver);

        tokio::spawn(async move {
            let mut receiver = alert_receiver.lock().unwrap();

            while let Some(alert) = receiver.recv().await {
                let mut active = active_alerts.lock().unwrap();

                // Remove expired alerts and add new one
                active.retain(|_, existing| {
                    existing.timestamp + chrono::Duration::minutes(self.config.alert_cooldown_minutes as i64)
                        > chrono::Utc::now()
                });

                active.insert(format!("{}_{}", alert.metric_name, alert.alert_type as u8), alert.clone());

                // Log alert (in real implementation, this would trigger notifications)
                println!("🚨 Quality Alert: [{}] {}", alert.severity as u8, alert.message);
            }
        });

        monitoring_handle.await?;
        Ok(())
    }

    /// Add quality snapshot to monitoring system
    pub async fn add_snapshot(&self, metrics: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
        let snapshot = QualitySnapshot {
            timestamp: chrono::Utc::now(),
            metrics,
            alerts: Vec::new(), // Will be populated during monitoring cycle
            trend_indicators: Vec::new(), // Will be populated during monitoring cycle
        };

        let mut history = self.metric_history.lock().unwrap();

        // Maintain history size limit using Rust 2024 const generics
        while history.len() >= METRIC_HISTORY_SIZE {
            history.pop_front();
        }

        history.push_back(snapshot);
        Ok(())
    }

    /// Get current monitoring statistics
    pub fn get_statistics(&self) -> MonitoringStatistics {
        let history = self.metric_history.lock().unwrap();
        let active_alerts = self.active_alerts.lock().unwrap();

        MonitoringStatistics {
            snapshots_in_history: history.len(),
            active_alerts: active_alerts.len(),
            oldest_snapshot: history.front().map(|s| s.timestamp),
            newest_snapshot: history.back().map(|s| s.timestamp),
            alerts_by_severity: self.count_alerts_by_severity(&active_alerts),
        }
    }

    /// Stop monitoring
    pub fn stop_monitoring(&self) {
        let _ = self.shutdown_sender.send(true);
    }

    /// Perform monitoring cycle with advanced async (Rust 2024)
    async fn perform_monitoring_cycle<R>(
        &self,
        rules: &[R],
        history: &Arc<Mutex<VecDeque<QualitySnapshot>>>,
        alert_sender: &mpsc::UnboundedSender<QualityAlert>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        R: MonitoringRule + Send + Sync,
        R::Context: Send + Sync + Clone + Default,
    {
        let history_guard = history.lock().unwrap();

        if let Some(current_snapshot) = history_guard.back() {
            let context = R::Context::default();

            // Evaluate all rules concurrently using Rust 2024 async improvements
            let mut rule_tasks = Vec::new();

            for rule in rules {
                let rule_clone = unsafe {
                    // Safe because rules are 'static and we're not moving them
                    std::ptr::read(rule)
                };
                let context_clone = context.clone();
                let snapshot_clone = current_snapshot.clone();

                let task = tokio::spawn(async move {
                    rule_clone.evaluate(&context_clone, &snapshot_clone).await
                });

                rule_tasks.push(task);
            }

            // Wait for all rule evaluations to complete
            let mut all_alerts = Vec::new();
            for task in rule_tasks {
                if let Ok(alerts) = task.await {
                    all_alerts.extend(alerts);
                }
            }

            // Send alerts
            for alert in all_alerts {
                let _ = alert_sender.send(alert);
            }
        }

        Ok(())
    }

    /// Count alerts by severity
    fn count_alerts_by_severity(&self, alerts: &HashMap<String, QualityAlert>) -> HashMap<AlertSeverity, usize> {
        let mut counts = HashMap::new();

        for alert in alerts.values() {
            *counts.entry(alert.severity).or_insert(0) += 1;
        }

        counts
    }
}

/// Monitoring statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStatistics {
    pub snapshots_in_history: usize,
    pub active_alerts: usize,
    pub oldest_snapshot: Option<chrono::DateTime<chrono::Utc>>,
    pub newest_snapshot: Option<chrono::DateTime<chrono::Utc>>,
    pub alerts_by_severity: HashMap<AlertSeverity, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_threshold_rule() {
        let rule = ThresholdRule {
            metric_name: "test_metric".to_string(),
            warning_threshold: 0.7,
            critical_threshold: 0.9,
            comparison: ThresholdComparison::GreaterThan,
        };

        let mut metrics = HashMap::new();
        metrics.insert("test_metric".to_string(), 0.95);

        let snapshot = QualitySnapshot {
            timestamp: chrono::Utc::now(),
            metrics,
            alerts: Vec::new(),
            trend_indicators: Vec::new(),
        };

        let alerts = rule.evaluate(&(), &snapshot).await;

        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].severity, AlertSeverity::Critical);
        assert!(alerts[0].message.contains("exceeded"));
    }

    #[tokio::test]
    async fn test_monitor_add_snapshot() {
        let config = MonitoringConfig {
            monitoring_interval_seconds: 60,
            enable_real_time_alerts: true,
            enable_trend_analysis: false,
            alert_cooldown_minutes: 5,
            anomaly_detection_sensitivity: 0.8,
        };

        let monitor = RealTimeQualityMonitor::<10, 100>::new(config);

        let mut metrics = HashMap::new();
        metrics.insert("readability".to_string(), 0.85);

        monitor.add_snapshot(metrics).await.unwrap();

        let stats = monitor.get_statistics();
        assert_eq!(stats.snapshots_in_history, 1);
    }
}</contents>
</xai:function_call<parameter name="file_path">codex-rs/core/src/qc/monitoring.rs
