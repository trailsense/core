use crate::common::config::Config;
use crate::domains::ingest::IngestService;
use crate::domains::measurements::MeasurementService;
use crate::domains::node::NodeService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub ingest_service: Arc<IngestService>,
    pub measurement_service: Arc<MeasurementService>,
    pub node_service: Arc<NodeService>,
}
