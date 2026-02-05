use crate::domains::measurements::dto::measurements_dto::MeasurementDto;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct IngestRepository;

impl IngestRepository {
    pub async fn existing_node_ids(
        &self,
        pool: &PgPool,
        node_ids: &HashSet<Uuid>,
    ) -> Result<HashSet<Uuid>, sqlx::Error> {
        let rows = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT id
            FROM nodes
            WHERE id = ANY($1)
            "#,
        )
        .bind(node_ids.iter().copied().collect::<Vec<_>>())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().collect())
    }

    pub async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        measurement: MeasurementDto,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO measurements (node_id, count, created_at)
        VALUES ($1, $2, $3)
        "#,
            measurement.node_id,
            measurement.count,
            measurement.created_at
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
