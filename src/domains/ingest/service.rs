use crate::common::error::AppError;
use crate::db_tx;
use crate::domains::ingest::dto::ingest_dto::{IngestDto, MeasurementDto};
use crate::domains::ingest::repository::IngestRepository;
use chrono::{TimeDelta, Utc};
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
            repo: IngestRepository,
        }
    }

    pub async fn create_measurements(&self, payload: Vec<IngestDto>) -> Result<String, AppError> {
        let repo = self.repo.clone();
        db_tx!(&self.pool, |tx| {
            for ingest in payload {
                let delta = TimeDelta::seconds(i64::from(ingest.age_in_seconds));
                let created_at = Utc::now() - delta;
                let measurement = MeasurementDto {
                    node_id: ingest.node_id,
                    created_at,
                    count: ingest.count,
                };

                repo.create(tx, measurement).await?;
            }
            Ok::<(), sqlx::Error>(())
        })
        .await?;

        Ok("Ingest accepted".to_string())
    }
}
