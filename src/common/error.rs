use crate::common::dto::RestApiResponse;
use axum::{
    BoxError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::env;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid .env file")]
    InvalidEnvFile(#[from] env::VarError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        RestApiResponse::<()>::failure(status.as_u16(), self.to_string()).into_response()
    }
}

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    let status = if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let message = error.to_string();
    error!(?status, %message, "Request failed");

    let body = RestApiResponse::<()>::failure(status.as_u16(), message);

    (status, body)
}
