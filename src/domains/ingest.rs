mod domain {
    mod model;
}

mod api {
    mod handlers;
    pub mod routes;
}

pub mod dto {
    pub mod ingest_dto;
}

pub use api::routes::{ingest_routes};