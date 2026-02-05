mod api {
    mod handlers;
    pub mod routes;
}

pub mod dto {
    pub mod ingest_dto;
}

pub mod repository;
pub mod service;

pub use api::routes::ingest_routes;
pub use service::IngestService;
