use fhevm_engine_common::database::{connect_pool_with_options, resolve_database_url_from_option};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The migration script checks this first; the binary checks it again so the
    // versions can never be written by a release that was not told to create the database.
    anyhow::ensure!(
        std::env::var("ALLOW_DB_BOOTSTRAP").is_ok_and(|v| v == "true"),
        "ALLOW_DB_BOOTSTRAP is not true: this release must not create a database"
    );
    let database_url = resolve_database_url_from_option(None)?;
    let (pool, _refresh) =
        connect_pool_with_options(&database_url, PgPoolOptions::new().max_connections(1), None)
            .await?;

    fhevm_engine_common::bootstrap_versioning::bootstrap_versioning(&pool).await
}
