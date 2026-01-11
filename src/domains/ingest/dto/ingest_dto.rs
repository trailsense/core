use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct MeasurementDto {
    pub wifi: i32,
    pub bluetooth: i32,
    pub created_at: Option<DateTime<Utc>>
}