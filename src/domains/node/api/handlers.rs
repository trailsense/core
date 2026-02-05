use crate::common::app_state::AppState;
use crate::common::dto::RestApiResponse;
use crate::common::error::AppError;
use crate::domains::node::dto::node_dto::NodeDto;
use axum::extract::State;
use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List nodes", body = [NodeDto])
    ),
    tag = "Nodes"
)]
pub async fn list_nodes(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let nodes = state.node_service.list_nodes().await?;
    Ok(RestApiResponse::success(nodes))
}
