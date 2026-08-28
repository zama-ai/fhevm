use fhevm_engine_common::database::{connect_pool_with_options, resolve_database_url_from_option};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = resolve_database_url_from_option(None)?;
    let (pool, _refresh) =
        connect_pool_with_options(&database_url, PgPoolOptions::new().max_connections(1), None)
            .await?;

    fhevm_engine_common::bootstrap_versioning::bootstrap_versioning(&pool).await
}
