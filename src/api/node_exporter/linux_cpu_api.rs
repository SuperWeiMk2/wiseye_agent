use axum::Router;
use axum::routing::get;

fn cpu_stat_api() -> Router {
    Router::new()
        .route("/cpu", get("1"))
}