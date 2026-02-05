use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct MeasurementDto {
    pub node_id: Uuid,
    pub count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TimeseriesQueryDto {
    /// Node identifier to aggregate measurements for.
    pub node_id: Uuid,
    /// Aggregation bucket. `hour` supports up to 31 days, `day` supports up to 1 year.
    #[param(example = "day")]
    pub bucket: TimeseriesBucket,
    /// Range start (inclusive), must be earlier than `to`.
    #[param(
        value_type = String,
        format = DateTime,
        example = "2026-01-01T00:00:00Z"
    )]
    pub from: DateTime<Utc>,
    /// Range end (exclusive); max range is 1 year (or 31 days when `bucket=hour`).
    #[param(
        value_type = String,
        format = DateTime,
        example = "2026-01-31T00:00:00Z"
    )]
    pub to: DateTime<Utc>,
}

#[derive(Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum TimeseriesBucket {
    Hour,
    Day,
}

#[derive(Serialize, ToSchema, FromRow)]
pub struct TimeseriesPointDto {
    pub bucket_start: DateTime<Utc>,
    pub total_count: i64,
}
