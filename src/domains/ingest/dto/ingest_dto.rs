use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct MeasurementDto {
    pub node_id: Uuid,
    pub count: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct IngestDto {
    pub node_id: Uuid,
    pub count: u32,
    pub age_in_seconds: u32,
}
