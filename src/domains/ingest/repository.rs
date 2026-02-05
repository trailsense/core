use crate::domains::ingest::dto::ingest_dto::MeasurementDto;
use sqlx::{Postgres, Transaction};

#[derive(Clone, Default)]
pub struct IngestRepository;

impl IngestRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        measurement: MeasurementDto,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO measurements (node_id, count, created_at)
        VALUES ($1, $2, COALESCE($3, NOW()))
        "#,
            measurement.node_id,
            i64::from(measurement.count),
            measurement.created_at
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
