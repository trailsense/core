use crate::common::app_state::AppState;
use crate::common::error::{AppError, ErrorResponse};
use crate::domains::measurements::dto::measurements_dto::{TimeseriesPointDto, TimeseriesQueryDto};
use axum::Json;
use axum::extract::{Query, State};
use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/timeseries",
    params(TimeseriesQueryDto),
    responses(
        (status = 200, description = "Measurement timeseries for node", body = [TimeseriesPointDto]),
        (status = 400, description = "Invalid query parameters or range limits exceeded", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Measurements"
)]
pub async fn measurement_timeseries(
    State(state): State<AppState>,
    Query(query): Query<TimeseriesQueryDto>,
) -> Result<impl IntoResponse, AppError> {
    let points = state.measurement_service.timeseries(query).await?;
    Ok(Json(points))
}
