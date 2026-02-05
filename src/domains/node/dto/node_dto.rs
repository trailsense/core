use crate::domains::node::model::NodeStatus;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct NodeDto {
    pub id: Uuid,
    pub name: String,
    pub status: NodeStatus,
    pub latitude: f64,
    pub longitude: f64,
    #[schema(minimum = 1, maximum = 86_400)]
    pub send_frequency_seconds: i64,
    pub created_at: DateTime<Utc>,
}
