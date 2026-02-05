use crate::common::error::AppError;
use crate::domains::node::dto::node_dto::NodeDto;
use crate::domains::node::repository::NodeRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct NodeService {
    pool: PgPool,
    repo: NodeRepository,
}

impl NodeService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: NodeRepository,
        }
    }

    pub async fn list_nodes(&self) -> Result<Vec<NodeDto>, AppError> {
        self.repo.list(&self.pool).await
    }
}
