use std::time::Duration;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, warn};
use crate::common::config::Config;

/// setup_database initializes the database connection pool.
pub async fn setup_database(config: &Config) -> Result<PgPool, sqlx::Error> {
    // Attempt to connect repeatedly, with a small delay, until success (or a max number of tries)
    let mut attempts = 0;
    let pool = loop {
        attempts += 1;
        match PgPoolOptions::new()
            .max_connections(config.database_max_connections)
            .min_connections(config.database_min_connections)
            .connect(&config.database_url)
            .await
        {
            Ok(pool) => break pool,
            Err(err) => {
                if attempts >= 3 {
                    return Err(err);
                }
                warn!(
                    "Postgres not ready yet ({:?}), retrying in 3s… (attempt {}/{})",
                    err, attempts, 3
                );
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    };

    info!("Postgres connection successful");

    Ok(pool)
}
