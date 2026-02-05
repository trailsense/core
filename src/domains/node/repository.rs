use crate::common::error::AppError;
use crate::domains::node::dto::node_dto::NodeDto;
use crate::domains::node::model::NodeStatus;
use sqlx::{PgPool, Row};

#[derive(Clone, Default)]
pub struct NodeRepository;

impl NodeRepository {
    pub async fn list(&self, pool: &PgPool) -> Result<Vec<NodeDto>, AppError> {
        sqlx::query(
            r#"
            SELECT
                id,
                name,
                status,
                ST_Y(location::geometry) AS latitude,
                ST_X(location::geometry) AS longitude,
                send_frequency_seconds,
                created_at
            FROM nodes
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)
        .and_then(|rows| {
            let mut nodes = Vec::with_capacity(rows.len());
            for row in rows {
                let status_str: String = row.get("status");
                let status = NodeStatus::try_from(status_str).map_err(|err| {
                    tracing::error!("Invalid node status in database: {err}");
                    AppError::InternalError
                })?;
                nodes.push(NodeDto {
                    id: row.get("id"),
                    name: row.get("name"),
                    status,
                    latitude: row.get("latitude"),
                    longitude: row.get("longitude"),
                    send_frequency_seconds: row.get("send_frequency_seconds"),
                    created_at: row.get("created_at"),
                });
            }
            Ok(nodes)
        })
    }
}
