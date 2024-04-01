use std::time::Duration;

use deadpool_diesel::postgres::{Manager, Pool};
use sqlx::{Error, PgPool};
use sqlx::postgres::PgPoolOptions;

use crate::platform::config::Config;

pub fn setup_connection_pool(config: &Config) -> Pool {
    let manager = Manager::new(
        &config.db_url().to_string(),
        deadpool_diesel::Runtime::Tokio1,
    );
    Pool::builder(manager).build().unwrap()
}

pub async fn setup_connection_pool_sqlx(config: &Config) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(60))
        .max_connections(400)
        .connect(&config.db_url().to_string())
        .await?;

    Ok(pool)
}