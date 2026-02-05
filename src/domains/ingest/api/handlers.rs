use crate::common::app_state::AppState;
use crate::common::error::{AppError, ErrorResponse};
use crate::domains::ingest::dto::ingest_dto::IngestDto;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, ToSchema)]
struct IngestAcceptedResponse {
    /// Human-readable success description.
    #[schema(example = "Ingest accepted")]
    message: String,
}

#[utoipa::path(
    post,
    path = "/",
    request_body = Vec<IngestDto>,
    responses(
        (status = 200, description = "Ingest accepted", body = IngestAcceptedResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
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

    Ok(Json(IngestAcceptedResponse {
        message: "Ingest accepted".to_string(),
    }))
}
