use crate::common::config::Config;
use crate::common::error::AppError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgPool, Postgres, Transaction};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tracing::{info, warn};

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

    // Run schema migrations on startup so all environments stay in sync.
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Database migrations applied");

    Ok(pool)
}

pub type TransactionFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send + 'a>>;

pub async fn run_in_transaction<T, F>(pool: &PgPool, f: F) -> Result<T, AppError>
where
    T: Send,
    F: for<'c> FnOnce(&'c mut Transaction<'_, Postgres>) -> TransactionFuture<'c, T> + Send + Sync,
{
    let mut conn = pool.acquire().await?;
    conn.transaction(f).await.map_err(|err| {
        tracing::error!("Transaction failed: {err}");
        AppError::DatabaseError(err)
    })
}
