use crate::common::error::AppError;
use crate::domains::ingest::dto::ingest_dto::MeasurementDto;
use crate::domains::ingest::repository::IngestRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct IngestService {
    pool: PgPool,
    repo: IngestRepository,
}

impl IngestService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: IngestRepository::new(),
        }
    }

    pub async fn create_measurement(&self, payload: MeasurementDto) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;
        match self.repo.create(&mut tx, payload).await {
            Ok(_measurement) => {
                tx.commit().await?;
                Ok("Ingest accepted".to_string())
            }
            Err(err) => {
                tracing::error!("Error creating device: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }
}
