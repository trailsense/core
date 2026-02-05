use crate::common::error::AppError;
use crate::domains::measurements::dto::measurements_dto::{TimeseriesBucket, TimeseriesPointDto};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct MeasurementRepository;

impl MeasurementRepository {
    pub async fn timeseries(
        &self,
        pool: &PgPool,
        node_id: Uuid,
        bucket: TimeseriesBucket,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<TimeseriesPointDto>, AppError> {
        let bucket_width = match bucket {
            TimeseriesBucket::Hour => "1 hour",
            TimeseriesBucket::Day => "1 day",
        };

        sqlx::query_as::<_, TimeseriesPointDto>(
            r#"
            SELECT
                time_bucket($1::interval, created_at) AS bucket_start,
                SUM(count)::bigint AS total_count
            FROM measurements
            WHERE node_id = $2
              AND created_at >= $3
              AND created_at < $4
            GROUP BY bucket_start
            ORDER BY bucket_start ASC
            "#,
        )
        .bind(bucket_width)
        .bind(node_id)
        .bind(from)
        .bind(to)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)
    }
}
