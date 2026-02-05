use crate::common::app_state::AppState;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(
    tags((name = "Ingest"))
)]
pub struct IngestApiDoc;

pub fn ingest_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(IngestApiDoc::openapi())
        .routes(routes!(super::handlers::add_measurement))
    //.route_layer(middleware::from_fn(NODEAUTHENTICATOR) TODO: Add node authentication here, unprotected endpoint for now
}
