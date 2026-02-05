use crate::common::error::AppError;
use axum::http::HeaderValue;
use std::env;

/// Config is a struct that holds the configuration for the application.
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,

    pub service_host: String,
    pub service_port: String,
    pub cors_allowed_origins: Vec<HeaderValue>,
}

/// from_env reads the environment variables and returns a Config struct.
/// It uses the dotenvy crate to load environment variables from a .env file if it exists.
/// It returns a Result with the Config struct or an error if any of the environment variables are missing.
impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let _ = dotenvy::dotenv();

        let cors_allowed_origins = parse_cors_allowed_origins(env::var("CORS_ALLOWED_ORIGINS"));

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,

            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(5))
                .unwrap_or(5),
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(1))
                .unwrap_or(1),

            service_host: env::var("SERVICE_HOST")?,
            service_port: env::var("SERVICE_PORT")?,
            cors_allowed_origins,
        })
        .map_err(|err| AppError::InvalidEnvFile(err))
    }
}

fn parse_cors_allowed_origins(raw_origins: Result<String, env::VarError>) -> Vec<HeaderValue> {
    raw_origins
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect::<Vec<_>>()
}
