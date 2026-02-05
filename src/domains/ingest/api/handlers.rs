use crate::common::app_state::AppState;
use crate::common::dto::RestApiResponse;
use crate::common::error::AppError;
use crate::domains::ingest::dto::ingest_dto::MeasurementDto;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/",
    request_body = MeasurementDto,
    responses(
        (status = 200, description = "Ingest accepted", body = String)
    ),
    tag = "Ingest"
)]
pub async fn add_measurement(
    State(state): State<AppState>,
    Json(payload): Json<MeasurementDto>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|err| {
        tracing::error!("Validation error: {}", err);
        AppError::ValidationError(format!("Invalid input: {}", err))
    })?;

    state.ingest_service.create_measurement(payload).await?;

    Ok(RestApiResponse::<()>::success_message_only(
        "Ingest accepted",
    ))
}
