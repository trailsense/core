use sqlx::PgPool;
use std::sync::Arc;
use crate::common::app_state::AppState;
use crate::common::config::Config;
use crate::domains::ingest::IngestService;
use crate::domains::node::NodeService;

pub fn build_app_state(pool: PgPool, config: Config) -> AppState {
    let ingest_service = Arc::new(IngestService::new(pool.clone()));
    let node_service = Arc::new(NodeService::new(pool));
    AppState {
        config,
        ingest_service,
        node_service,
    }
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}
