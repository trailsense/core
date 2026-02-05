pub mod api {
    mod handlers;
    pub mod routes;
}

pub mod dto {
    pub mod measurements_dto;
}

pub mod model;
pub mod repository;
pub mod service;

pub use api::routes::measurement_routes;
pub use service::MeasurementService;
