use crate::common::error::{handle_error, AppError};
use axum::error_handling::HandleErrorLayer;
use axum::http::{header, Method, StatusCode};
use axum::{Router};
use std::time::Duration;
use axum::response::IntoResponse;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use crate::{
    domains::{
        ingest::{ingest_routes}
    }
};
use crate::common::openapi::ApiDoc;

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT]);

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(10))
        .layer(cors);

    let openapi_router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/ingest", ingest_routes());

    let (router, openapi) = openapi_router.split_for_parts();

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/docs/openapi.json", axum::routing::get({
            let openapi = openapi.clone();
            move || async { axum::Json(openapi) }
        }))
        .merge(router)
        .fallback(fallback)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            "request completed: status = {status}, latency = {latency:?}",
                            status = response.status(),
                            latency = latency
                        );
                    },
                ),
        )
        .layer(middleware_stack)
        //.with_state(state)
}

async fn health_check() -> &'static str {
    "OK\n"
}

/// Fallback handler for unmatched routes
/// This function returns a 404 Not Found response with a message.
pub async fn fallback() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_FOUND, "Not Found"))
}