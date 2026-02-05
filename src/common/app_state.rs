use std::sync::Arc;
use crate::common::config::Config;
use crate::domains::ingest::IngestService;
use crate::domains::node::NodeService;

#[derive(Clone)]
pub struct AppState {
  pub config: Config,
  pub ingest_service: Arc<IngestService>,
  pub node_service: Arc<NodeService>,
}
