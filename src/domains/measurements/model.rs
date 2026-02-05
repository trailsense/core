use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Measurement {
    pub node_id: Uuid,
    pub count: i64,
    pub created_at: DateTime<Utc>,
}
