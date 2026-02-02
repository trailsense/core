use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Measurement {
    pub node_id: Uuid,
    pub wifi: i32,
    pub bluetooth: i32,
    pub created_at: Option<DateTime<Utc>>
}
