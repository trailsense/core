use crate::common::app_state::AppState;
use crate::domains::measurements::dto::measurements_dto::{TimeseriesBucket, TimeseriesPointDto};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(
    tags((name = "Measurements")),
    components(schemas(TimeseriesBucket, TimeseriesPointDto))
)]
pub struct MeasurementApiDoc;

pub fn measurement_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(MeasurementApiDoc::openapi())
        .routes(routes!(super::handlers::measurement_timeseries))
}
