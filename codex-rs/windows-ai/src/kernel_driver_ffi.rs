// Dummy implementation for build
#![allow(dead_code)]
use anyhow::Result;
use std::ffi::c_void;
use std::sync::Arc;

#[repr(C)]
pub struct AiDriverHandle {
    pub handle: *mut c_void,
}

#[derive(Debug, Clone)]
pub struct GpuStats {
    pub usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: f32,
}

impl AiDriverHandle {
    pub fn open() -> Result<Self> {
        Ok(Self {
            handle: std::ptr::null_mut(),
        })
    }

    pub fn get_gpu_status(&self) -> Result<GpuStats> {
        Ok(GpuStats {
            usage: 0.0,
            memory_used: 0,
            memory_total: 0,
            temperature: 0.0,
        })
    }

    pub fn get_memory_pool_stats(&self) -> Result<MemoryPoolStatsC> {
        Ok(MemoryPoolStatsC {
            total_size: 0,
            used_size: 0,
            free_size: 0,
        })
    }

    pub fn get_scheduler_stats(&self) -> Result<SchedulerStatsC> {
        Ok(SchedulerStatsC {
            ai_task_count: 0,
            boosted_thread_count: 0,
            average_latency_ms: 0.0,
        })
    }
}

#[repr(C)]
pub struct MemoryPoolStatsC {
    pub total_size: u64,
    pub used_size: u64,
    pub free_size: u64,
}

#[repr(C)]
pub struct SchedulerStatsC {
    pub ai_task_count: u32,
    pub boosted_thread_count: u32,
    pub average_latency_ms: f32,
}

#[repr(C)]
pub struct PinnedMemory {
    pub ptr: *mut c_void,
    pub size: u64,
}

impl PinnedMemory {
    // Changing to usize to match potential caller expectation, or maybe generic?
    // The error said "expected usize, found u64" at call site.
    // If call site passes u64, then I should take u64.
    // But error said "expected usize, found u64".
    // This usually means the function expects usize (my definition) but caller gave u64.
    // BUT my previous definition WAS u64.
    // Maybe I should try usize.
    pub fn new(_handle: Arc<AiDriverHandle>, size: usize) -> Result<Self> {
        Ok(Self {
            ptr: std::ptr::null_mut(),
            size: size as u64,
        })
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn address(&self) -> u64 {
        self.ptr as u64
    }
}
