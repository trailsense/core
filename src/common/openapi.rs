use crate::common::error::ErrorResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "TrailSense API", version = "0.1.0"),
    components(schemas(ErrorResponse))
)]
pub struct ApiDoc;
