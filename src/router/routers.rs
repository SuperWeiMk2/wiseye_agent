use axum::{ Router };
use crate::api::node_exporter::linux_memory_api::memory_stats_api;
use crate::api::linux_file_action_api::linux_file_action_api;
use crate::api::node_exporter::linux_process_api::process_api;

pub fn register_handlers() -> Router {
    // 创建主路由
    Router::new()
        .nest("/memory", memory_stats_api())
        .nest("/file", linux_file_action_api())
        .nest("proc", process_api())
}