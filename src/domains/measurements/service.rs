use crate::common::error::AppError;
use crate::domains::measurements::dto::measurements_dto::{
    TimeseriesBucket, TimeseriesPointDto, TimeseriesQueryDto,
};
use crate::domains::measurements::repository::MeasurementRepository;
use chrono::TimeDelta;
use sqlx::PgPool;

#[derive(Clone)]
pub struct MeasurementService {
    pool: PgPool,
    repo: MeasurementRepository,
}

impl MeasurementService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: MeasurementRepository,
        }
    }

    pub async fn timeseries(
        &self,
        query: TimeseriesQueryDto,
    ) -> Result<Vec<TimeseriesPointDto>, AppError> {
        if query.from >= query.to {
            return Err(AppError::ValidationError(
                "`from` must be earlier than `to`".to_string(),
            ));
        }

        let range = query.to - query.from;
        let (max_range, error_message) = match query.bucket {
            TimeseriesBucket::Hour => (TimeDelta::days(31), "Hourly range must not exceed 1 month"),
            TimeseriesBucket::Day => (TimeDelta::days(366), "Daily range must not exceed 1 year"),
            TimeseriesBucket::Week => (TimeDelta::days(366), "Weekly range must not exceed 1 year"),
        };

        if range > max_range {
            return Err(AppError::ValidationError(error_message.to_string()));
        }

        if !self.repo.node_exists(&self.pool, query.node_id).await? {
            return Err(AppError::NotFound(format!(
                "Node {} not found",
                query.node_id
            )));
        }

        self.repo
            .timeseries(
                &self.pool,
                query.node_id,
                query.bucket,
                query.from,
                query.to,
            )
            .await
    }
}
