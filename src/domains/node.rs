pub mod api {
    mod handlers;
    pub mod routes;
}

pub mod dto {
    pub mod node_dto;
}

pub mod model;
pub mod repository;
pub mod service;

pub use api::routes::node_routes;
pub use service::NodeService;
