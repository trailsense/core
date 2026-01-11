use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "TrailSense API", version = "0.1.0"),
    components()
)]
pub struct ApiDoc;