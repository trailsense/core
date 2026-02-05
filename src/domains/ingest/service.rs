use crate::common::error::AppError;
use crate::domains::ingest::dto::ingest_dto::{IngestDto, MeasurementDto};
use crate::domains::ingest::repository::IngestRepository;
use chrono::{DateTime, Local, TimeDelta, Utc};
use sqlx::PgPool;
use std::ops::Add;

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

    pub async fn create_measurements(&self, payload: Vec<IngestDto>) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;
        for ingest in payload {
            let delta = TimeDelta::seconds(i64::from(ingest.age_in_seconds));
            let created_at: DateTime<Utc> = DateTime::from(Local::now()).add(-delta);
            let measurement = MeasurementDto {
                node_id: ingest.node_id,
                created_at,
                count: ingest.count,
            };
            if let Err(err) = self.repo.create(&mut tx, measurement).await {
                tracing::error!("Error creating measurement: {err}");
                if let Err(rollback_err) = tx.rollback().await {
                    tracing::error!("Rollback failed after insert error: {rollback_err}");
                }
                return Err(AppError::DatabaseError(err));
            }
        }

        tx.commit().await?;
        Ok("Ingest accepted".to_string())
    }
}
