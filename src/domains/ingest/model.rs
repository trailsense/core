use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Measurement {
    pub node_id: Uuid,
    pub count: i32,
    pub created_at: Option<DateTime<Utc>>
}
