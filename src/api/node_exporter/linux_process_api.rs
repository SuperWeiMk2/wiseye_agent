use std::io;
use axum::{Json, Router};
use axum::routing::get;
use crate::node_exporter::proc_utils::process::ProcessStatus;


pub async fn process_api() -> Router {
    Router::new()
        .route("/proc-status", get(processes_handler))
}

async fn processes_handler() -> Result<Json<Vec<ProcessStatus>>, axum::Error> {
    let processes = ProcessStatus::processes()?;

    Ok(Json(processes))
}