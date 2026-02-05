use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct MeasurementDto {
    pub node_id: Uuid,
    pub count: i32,
    pub created_at: Option<DateTime<Utc>>,
}
