use axum::Json;
use axum::response::IntoResponse;
use validator::Validate;
use crate::common::dto::RestApiResponse;
use crate::common::error::AppError;
use crate::domains::ingest::dto::ingest_dto::MeasurementDto;

#[utoipa::path(
    post,
    path = "/{id}",
    request_body = MeasurementDto,
    responses(
        (status = 200, description = "Ingest accepted", body = String)
    ),
    tag = "Ingest"
)]
pub async fn add_measurement(
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<MeasurementDto>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|err| {
        tracing::error!("Validation error: {}", err);
        AppError::ValidationError(format!("Invalid input: {}", err))
    })?;

    let mut payload = payload;
    Ok(RestApiResponse::<()>::success_message_only("Ingest accepted"))
}