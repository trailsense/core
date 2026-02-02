use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct Measurement {
    pub node_id: String,
    pub wifi: i32,
    pub bluetooth: i32,
    pub created_at: Option<DateTime<Utc>>
}
