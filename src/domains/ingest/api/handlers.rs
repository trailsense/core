use crate::common::app_state::AppState;
use crate::common::dto::RestApiResponse;
use crate::common::error::AppError;
use crate::domains::ingest::dto::ingest_dto::IngestDto;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
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
    if payload.is_empty() {
        return Err(AppError::ValidationError(
            "Payload must contain at least one measurement".to_string(),
        ));
    }

    for (index, ingest) in payload.iter().enumerate() {
        ingest.validate().map_err(|err| {
            tracing::error!("Validation error at payload[{index}]: {}", err);
            AppError::ValidationError(format!("Invalid input at payload[{index}]: {}", err))
        })?;
    }

    state.ingest_service.create_measurements(payload).await?;

    Ok(RestApiResponse::<()>::success_message_only(
        "Ingest accepted",
    ))
}
