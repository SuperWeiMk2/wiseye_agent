use std::error::Error;
use axum::{Json, Router, routing::{get, post}};

use crate::node_exporter::mem_utils::meminfo::MemInfo;

pub fn memory_stats_api() -> Router {
    Router::new()
        .route("/meminfo", get(memory_stats_handler))
        .route("/memoryUsed",get(memory_used_handler))
        .route("/memoryInfoAndUsed", get(memory_info_and_used_handler))
        .route("/clearCache", post("TODO"))
}

struct MemoryStats {
    meminfo: MemInfo,
    used: f64
}

async fn init_memory_info() -> Result<MemInfo, Box<dyn Error>> {
    MemInfo::init()
}

async fn memory_stats_handler() -> Json<Option<MemInfo>> {
    match init_memory_info().await {
        Ok(memory_info) => {
            Json(Some(memory_info))
        },
        Err(e) => {
            eprintln!("Failed to retrieve memory stats: {}", e);
            Json(None)
        }
    }
}

async fn memory_used_handler() -> Json<Option<f64>> {
    match init_memory_info().await {
        Ok(memory_info) => {
            let memory_used = calculate_memory_used(&memory_info);
            Json(Some(memory_used))
        },
        Err(e) => {
            eprintln!("Failed to retrieve memory stats: {}", e);
            Json(None)
        }
    }
}

async fn memory_info_and_used_handler() -> Json<Option<MemoryStats>> {
    match init_memory_info().await {
        Ok(memory_info) => {
            let used = calculate_memory_used(&memory_info);
            let result = MemoryStats {
                meminfo: memory_info,
                used,
            };
            Json(Some(result))
        },
        Err(e) => {
            eprintln!("Failed to retrieve memory stats: {}", e);
            Json(None)
        }
    }
}

fn calculate_memory_used(meminfo: &MemInfo) -> f64 {
    let total = meminfo.total as f64;
    let free = (meminfo.free + meminfo.buffers + meminfo.cached) as f64;
    ((total - free) / total) * 100.0
}