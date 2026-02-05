use crate::common::app_state::AppState;
use crate::common::dto::RestApiResponse;
use crate::common::error::AppError;
use crate::domains::ingest::dto::ingest_dto::IngestDto;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/",
    request_body = Vec<IngestDto>,
    responses(
        (status = 200, description = "Ingest accepted", body = String)
    ),
    tag = "Ingest"
)]
pub async fn add_measurement(
    State(state): State<AppState>,
    Json(payload): Json<Vec<IngestDto>>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|err| {
        tracing::error!("Validation error: {}", err);
        AppError::ValidationError(format!("Invalid input: {}", err))
    })?;

    state.ingest_service.create_measurements(payload).await?;

    Ok(RestApiResponse::<()>::success_message_only(
        "Ingest accepted",
    ))
}
