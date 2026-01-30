use crate::types::SystemMetrics;
use axum::Json;

// System Metrics handler
pub async fn get_system_metrics() -> Json<SystemMetrics> {
    use sysinfo::System;

    let mut sys = System::new_all();

    // Refresh system information
    sys.refresh_all();

    // CPU usage
    let cpu_usage =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    // Memory usage
    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let memory_usage = if total_memory > 0.0 {
        used_memory / total_memory * 100.0
    } else {
        0.0
    };

    // Disk usage (simplified)
    let disk_usage = 50.0; // Placeholder - disk monitoring would require additional setup

    // Active processes
    let active_processes = sys.processes().len() as u32;

    // Uptime (simplified - in production, get from system)
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let metrics = SystemMetrics {
        cpu_usage: cpu_usage as f64,
        memory_usage,
        disk_usage,
        network_usage: None,
        active_processes,
        uptime,
    };

    Json(metrics)
}
