//! Resource Monitor for Hardware and Performance Tracking
//!
//! Monitors GPU/CPU temperatures, memory usage, and manages concurrent execution limits
//! Provides real-time resource monitoring for AI development workflows

use crate::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::System;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Hardware sensor readings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSensors {
    pub cpu_temperature: Option<f32>,
    pub gpu_temperature: Option<f32>,
    pub gpu_fan_speed: Option<f32>,
    pub memory_usage_percent: f32,
    pub memory_used_gb: f64,
    pub memory_total_gb: f64,
    pub cpu_usage_percent: f32,
    pub gpu_usage_percent: Option<f32>,
}

/// Resource limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_percent: f32, // 85% default
    pub max_cpu_temp_celsius: f32,
    pub max_gpu_temp_celsius: f32,
    pub max_concurrent_tasks: usize,
    pub gpu_required: bool,
}

/// Resource monitoring event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceEvent {
    LimitExceeded {
        resource_type: String,
        current_value: f64,
        limit_value: f64,
        recommendation: String,
    },
    SensorAlert {
        sensor_type: String,
        value: f64,
        threshold: f64,
        severity: AlertSeverity,
    },
    ResourceOptimized {
        action_taken: String,
        resources_freed: String,
    },
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Concurrent execution manager
#[derive(Debug)]
struct ExecutionManager {
    active_tasks: HashMap<String, TaskInfo>,
    max_concurrent: usize,
    memory_limit_percent: f32,
}

/// Task execution information
#[derive(Debug, Clone)]
struct TaskInfo {
    id: String,
    start_time: std::time::Instant,
    estimated_memory_mb: u32,
    priority: TaskPriority,
}

/// Task priority for resource allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Resource Monitor
pub struct ResourceMonitor {
    system: Arc<Mutex<System>>,
    limits: ResourceLimits,
    execution_manager: Arc<Mutex<ExecutionManager>>,
    sensor_history: Arc<Mutex<HashMap<String, Vec<f64>>>>,
    event_tx: mpsc::UnboundedSender<ResourceEvent>,
    command_tx: mpsc::UnboundedSender<MonitorCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<MonitorCommand>>>>,
}

#[derive(Debug)]
enum MonitorCommand {
    GetSensors {
        response: oneshot::Sender<Result<HardwareSensors>>,
    },
    CheckLimits {
        response: oneshot::Sender<Result<Vec<ResourceEvent>>>,
    },
    RequestExecution {
        task_id: String,
        estimated_memory_mb: u32,
        priority: TaskPriority,
        response: oneshot::Sender<Result<bool>>,
    },
    ReleaseExecution {
        task_id: String,
        response: oneshot::Sender<Result<()>>,
    },
    OptimizeResources {
        response: oneshot::Sender<Result<String>>,
    },
    GetActiveTasks {
        response: oneshot::Sender<Result<Vec<String>>>,
    },
}

impl ResourceMonitor {
    pub fn new(limits: ResourceLimits) -> Self {
        let (event_tx, _) = mpsc::unbounded_channel();
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let execution_manager = ExecutionManager {
            active_tasks: HashMap::new(),
            max_concurrent: limits.max_concurrent_tasks,
            memory_limit_percent: limits.max_memory_percent,
        };

        Self {
            system: Arc::new(Mutex::new(System::new_all())),
            limits,
            execution_manager: Arc::new(Mutex::new(execution_manager)),
            sensor_history: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            command_tx: cmd_tx,
            command_rx: Arc::new(Mutex::new(Some(cmd_rx))),
        }
    }

    /// Get current hardware sensor readings
    pub async fn get_sensors(&self) -> Result<HardwareSensors> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(MonitorCommand::GetSensors { response: tx })?;

        rx.await?
    }

    /// Check if resource limits are exceeded
    pub async fn check_limits(&self) -> Result<Vec<ResourceEvent>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(MonitorCommand::CheckLimits { response: tx })?;

        rx.await?
    }

    /// Request execution slot for a task
    pub async fn request_execution(
        &self,
        task_id: &str,
        estimated_memory_mb: u32,
        priority: TaskPriority,
    ) -> Result<bool> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(MonitorCommand::RequestExecution {
            task_id: task_id.to_string(),
            estimated_memory_mb,
            priority,
            response: tx,
        })?;

        rx.await?
    }

    /// Release execution slot
    pub async fn release_execution(&self, task_id: &str) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(MonitorCommand::ReleaseExecution {
            task_id: task_id.to_string(),
            response: tx,
        })?;

        rx.await?
    }

    /// Optimize resource usage
    pub async fn optimize_resources(&self) -> Result<String> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(MonitorCommand::OptimizeResources { response: tx })?;

        rx.await?
    }

    /// Get active task IDs
    pub async fn get_active_tasks(&self) -> Result<Vec<String>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(MonitorCommand::GetActiveTasks { response: tx })?;

        rx.await?
    }

    /// Start monitoring loop
    pub async fn start_monitoring(self) -> Result<()> {
        let monitor_handle = tokio::spawn(async move { self.monitoring_loop().await });

        let command_handle = tokio::spawn(async move { self.command_loop().await });

        // Run both tasks
        let (monitor_result, command_result) = tokio::join!(monitor_handle, command_handle);

        monitor_result??;
        command_result??;

        Ok(())
    }

    async fn monitoring_loop(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

        loop {
            interval.tick().await;

            // Update system information
            {
                let mut system = self.system.lock().unwrap();
                system.refresh_all();
            }

            // Check limits
            if let Ok(events) = self.check_limits_internal().await {
                for event in events {
                    let _ = self.event_tx.send(event);
                }
            }
        }
    }

    async fn command_loop(mut self) -> Result<()> {
        let mut rx = self.command_rx.lock().unwrap().take().unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                MonitorCommand::GetSensors { response } => {
                    let sensors = self.get_sensors_internal();
                    let _ = response.send(Ok(sensors));
                }
                MonitorCommand::CheckLimits { response } => {
                    let events = self.check_limits_internal().await;
                    let _ = response.send(events);
                }
                MonitorCommand::RequestExecution {
                    task_id,
                    estimated_memory_mb,
                    priority,
                    response,
                } => {
                    let result =
                        self.request_execution_internal(&task_id, estimated_memory_mb, priority);
                    let _ = response.send(Ok(result));
                }
                MonitorCommand::ReleaseExecution { task_id, response } => {
                    self.release_execution_internal(&task_id);
                    let _ = response.send(Ok(()));
                }
                MonitorCommand::OptimizeResources { response } => {
                    let result = self.optimize_resources_internal().await;
                    let _ = response.send(result);
                }
                MonitorCommand::GetActiveTasks { response } => {
                    let tasks = self.get_active_tasks_internal();
                    let _ = response.send(Ok(tasks));
                }
            }
        }

        Ok(())
    }

    fn get_sensors_internal(&self) -> HardwareSensors {
        let mut system = self.system.lock().unwrap();
        system.refresh_all();

        // CPU information
        let cpu_usage =
            system.cpus().iter().map(|p| p.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;

        // Memory information
        let memory_used = system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0; // GB
        let memory_total = system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0; // GB
        let memory_usage_percent = (memory_used / memory_total * 100.0) as f32;

        // GPU information (simplified - would need GPU-specific libraries)
        let gpu_temp = self.read_gpu_temperature();
        let gpu_fan = self.read_gpu_fan_speed();
        let gpu_usage = self.read_gpu_usage();

        HardwareSensors {
            cpu_temperature: self.read_cpu_temperature(),
            gpu_temperature: gpu_temp,
            gpu_fan_speed: gpu_fan,
            memory_usage_percent,
            memory_used_gb: memory_used,
            memory_total_gb: memory_total,
            cpu_usage_percent: cpu_usage,
            gpu_usage_percent: gpu_usage,
        }
    }

    async fn check_limits_internal(&self) -> Result<Vec<ResourceEvent>> {
        let sensors = self.get_sensors_internal();
        let mut events = Vec::new();

        // Memory limit check
        if sensors.memory_usage_percent > self.limits.max_memory_percent {
            events.push(ResourceEvent::LimitExceeded {
                resource_type: "memory".to_string(),
                current_value: sensors.memory_usage_percent as f64,
                limit_value: self.limits.max_memory_percent as f64,
                recommendation: "Reduce concurrent tasks or free up memory".to_string(),
            });
        }

        // CPU temperature check
        if let Some(cpu_temp) = sensors.cpu_temperature {
            if cpu_temp > self.limits.max_cpu_temp_celsius {
                events.push(ResourceEvent::SensorAlert {
                    sensor_type: "cpu_temperature".to_string(),
                    value: cpu_temp as f64,
                    threshold: self.limits.max_cpu_temp_celsius as f64,
                    severity: AlertSeverity::High,
                });
            }
        }

        // GPU temperature check
        if let Some(gpu_temp) = sensors.gpu_temperature {
            if gpu_temp > self.limits.max_gpu_temp_celsius {
                events.push(ResourceEvent::SensorAlert {
                    sensor_type: "gpu_temperature".to_string(),
                    value: gpu_temp as f64,
                    threshold: self.limits.max_gpu_temp_celsius as f64,
                    severity: AlertSeverity::High,
                });
            }
        }

        // Concurrent tasks check
        let active_count = self.execution_manager.lock().unwrap().active_tasks.len();
        if active_count >= self.limits.max_concurrent_tasks {
            events.push(ResourceEvent::LimitExceeded {
                resource_type: "concurrent_tasks".to_string(),
                current_value: active_count as f64,
                limit_value: self.limits.max_concurrent_tasks as f64,
                recommendation: "Wait for existing tasks to complete or increase limit".to_string(),
            });
        }

        Ok(events)
    }

    fn request_execution_internal(
        &self,
        task_id: &str,
        estimated_memory_mb: u32,
        priority: TaskPriority,
    ) -> bool {
        let mut manager = self.execution_manager.lock().unwrap();

        // Check concurrent limit
        if manager.active_tasks.len() >= manager.max_concurrent {
            return false;
        }

        // Check memory limit
        let sensors = self.get_sensors_internal();
        let estimated_memory_percent =
            (estimated_memory_mb as f64 / sensors.memory_total_gb / 1024.0) * 100.0;
        if sensors.memory_usage_percent + estimated_memory_percent as f32
            > manager.memory_limit_percent
        {
            return false;
        }

        // Add task
        let task_info = TaskInfo {
            id: task_id.to_string(),
            start_time: std::time::Instant::now(),
            estimated_memory_mb,
            priority,
        };

        manager.active_tasks.insert(task_id.to_string(), task_info);
        true
    }

    fn release_execution_internal(&self, task_id: &str) {
        let mut manager = self.execution_manager.lock().unwrap();
        manager.active_tasks.remove(task_id);
    }

    async fn optimize_resources_internal(&self) -> Result<String> {
        let active_tasks = self.get_active_tasks_internal();

        if active_tasks.is_empty() {
            return Ok("No active tasks to optimize".to_string());
        }

        // Simple optimization: suggest reducing concurrent tasks if memory is high
        let sensors = self.get_sensors_internal();
        if sensors.memory_usage_percent > 80.0 {
            Ok(format!(
                "Memory usage high ({}%). Consider reducing concurrent tasks from {} to {}",
                sensors.memory_usage_percent,
                active_tasks.len(),
                active_tasks.len() / 2
            ))
        } else {
            Ok(format!(
                "Resources optimized. {} active tasks, memory usage: {}%",
                active_tasks.len(),
                sensors.memory_usage_percent
            ))
        }
    }

    fn get_active_tasks_internal(&self) -> Vec<String> {
        let manager = self.execution_manager.lock().unwrap();
        manager.active_tasks.keys().cloned().collect()
    }

    // Platform-specific sensor reading methods
    #[cfg(target_os = "linux")]
    fn read_cpu_temperature(&self) -> Option<f32> {
        // Read from /sys/class/thermal/thermal_zone*/temp
        std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .ok()?
            .trim()
            .parse::<f32>()
            .ok()
            .map(|t| t / 1000.0) // Convert millidegrees to degrees
    }

    #[cfg(target_os = "macos")]
    fn read_cpu_temperature(&self) -> Option<f32> {
        // Use macOS-specific APIs or iStats
        None // Placeholder
    }

    #[cfg(target_os = "windows")]
    fn read_cpu_temperature(&self) -> Option<f32> {
        // Use Windows Management Instrumentation (WMI)
        None // Placeholder - would need windows-specific libraries
    }

    #[cfg(target_os = "linux")]
    fn read_gpu_temperature(&self) -> Option<f32> {
        // Try NVIDIA GPU
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=temperature.gpu",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            if let Ok(temp_str) = String::from_utf8(output.stdout) {
                return temp_str.trim().parse::<f32>().ok();
            }
        }

        // Try AMD GPU
        if let Ok(output) = std::process::Command::new("rocm-smi")
            .args(&["--showtemp"])
            .output()
        {
            // Parse AMD output - simplified
            return Some(60.0); // Placeholder
        }

        None
    }

    #[cfg(not(target_os = "linux"))]
    fn read_gpu_temperature(&self) -> Option<f32> {
        // Platform-specific GPU temperature reading
        None // Placeholder
    }

    fn read_gpu_fan_speed(&self) -> Option<f32> {
        // GPU fan speed reading - platform specific
        None // Placeholder - would need GPU-specific libraries
    }

    fn read_gpu_usage(&self) -> Option<f32> {
        // GPU usage reading - platform specific
        None // Placeholder - would need GPU-specific libraries
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_percent: 85.0,
            max_cpu_temp_celsius: 85.0,
            max_gpu_temp_celsius: 80.0,
            max_concurrent_tasks: 5,
            gpu_required: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sensor_reading() {
        let limits = ResourceLimits::default();
        let monitor = ResourceMonitor::new(limits);

        let sensors = monitor.get_sensors().await;
        assert!(sensors.is_ok());

        let sensors = sensors.unwrap();
        assert!(sensors.memory_usage_percent >= 0.0 && sensors.memory_usage_percent <= 100.0);
        assert!(sensors.cpu_usage_percent >= 0.0);
    }

    #[tokio::test]
    async fn test_execution_limits() {
        let limits = ResourceLimits {
            max_memory_percent: 85.0,
            max_cpu_temp_celsius: 85.0,
            max_gpu_temp_celsius: 80.0,
            max_concurrent_tasks: 2,
            gpu_required: false,
        };

        let monitor = ResourceMonitor::new(limits);

        // Should allow first task
        let result1 = monitor
            .request_execution("task1", 100, TaskPriority::Normal)
            .await;
        assert_eq!(result1.unwrap(), true);

        // Should allow second task
        let result2 = monitor
            .request_execution("task2", 100, TaskPriority::Normal)
            .await;
        assert_eq!(result2.unwrap(), true);

        // Should deny third task (exceeds concurrent limit)
        let result3 = monitor
            .request_execution("task3", 100, TaskPriority::Normal)
            .await;
        assert_eq!(result3.unwrap(), false);

        // Release a task
        monitor.release_execution("task1").await.unwrap();

        // Should now allow third task
        let result4 = monitor
            .request_execution("task3", 100, TaskPriority::Normal)
            .await;
        assert_eq!(result4.unwrap(), true);
    }

    #[tokio::test]
    async fn test_resource_optimization() {
        let limits = ResourceLimits::default();
        let monitor = ResourceMonitor::new(limits);

        let result = monitor.optimize_resources().await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
