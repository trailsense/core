use crate::domains::node::model::NodeStatus;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct NodeDto {
    pub id: Uuid,
    pub status: NodeStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Utc>,
}
