use crate::common::error::AppError;
use crate::db_tx;
use crate::domains::ingest::dto::ingest_dto::IngestDto;
use crate::domains::ingest::repository::IngestRepository;
use crate::domains::measurements::dto::measurements_dto::MeasurementDto;
use chrono::{TimeDelta, Utc};
use sqlx::PgPool;
use std::collections::HashSet;

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
        let node_ids: HashSet<_> = payload.iter().map(|ingest| ingest.node_id).collect();
        let existing_ids = self.repo.existing_node_ids(&self.pool, &node_ids).await?;

        if existing_ids.len() != node_ids.len() {
            let mut missing: Vec<_> = node_ids
                .difference(&existing_ids)
                .map(|id| id.to_string())
                .collect();
            missing.sort();

            return Err(AppError::ValidationError(format!(
                "Unknown node_id values: {}",
                missing.join(", ")
            )));
        }

        let repo = self.repo.clone();
        db_tx!(&self.pool, |tx| {
            for ingest in payload {
                let delta = TimeDelta::seconds(i64::from(ingest.age_in_seconds));
                let created_at = Utc::now() - delta;
                let measurement = MeasurementDto {
                    node_id: ingest.node_id,
                    created_at,
                    count: i64::from(ingest.count),
                };

                repo.create(tx, measurement).await?;
            }
            Ok::<(), sqlx::Error>(())
        })
        .await?;

        Ok("Ingest accepted".to_string())
    }
}
