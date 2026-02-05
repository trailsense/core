use crate::common::error::AppError;
use crate::domains::node::dto::node_dto::NodeDto;
use crate::domains::node::model::NodeStatus;
use sqlx::PgPool;

#[derive(Clone, Default)]
pub struct NodeRepository;

impl NodeRepository {
    pub async fn list(&self, pool: &PgPool) -> Result<Vec<NodeDto>, AppError> {
        sqlx::query!(
            r#"
            SELECT
                id,
                status,
                ST_Y(location::geometry) AS "latitude!",
                ST_X(location::geometry) AS "longitude!",
                created_at
            FROM nodes
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)
        .and_then(|rows| {
            let mut nodes = Vec::with_capacity(rows.len());
            for row in rows {
                let status = NodeStatus::try_from(row.status).map_err(|err| {
                    tracing::error!("Invalid node status in database: {err}");
                    AppError::InternalError
                })?;
                nodes.push(NodeDto {
                    id: row.id,
                    status,
                    latitude: row.latitude,
                    longitude: row.longitude,
                    created_at: row.created_at,
                });
            }
            Ok(nodes)
        })
    }
}
