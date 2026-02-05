use crate::common::app_state::AppState;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(
    tags((name = "Nodes"))
)]
pub struct NodeApiDoc;

pub fn node_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(NodeApiDoc::openapi()).routes(routes!(super::handlers::list_nodes))
}
