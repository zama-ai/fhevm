//! sqlx PgPool wiring, mirroring relayer/src/store.

pub mod repositories;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::settings::DatabaseConfig;

/// Connects a pooled Postgres handle and verifies it with a ping.
pub async fn connect(config: &DatabaseConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await?;
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await?;
    Ok(pool)
}
