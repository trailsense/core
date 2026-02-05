use crate::common::app_state::AppState;
use crate::common::error::{AppError, ErrorResponse};
use crate::domains::node::dto::node_dto::NodeDto;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List nodes", body = [NodeDto]),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Nodes"
)]
pub async fn list_nodes(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let nodes = state.node_service.list_nodes().await?;
    Ok(Json(nodes))
}
