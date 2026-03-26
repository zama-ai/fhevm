use relayer_migrate::{
    config::config,
    sql::{client::PgClient, migration::run_migrations},
};
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env().add_directive(Level::INFO.into()),
        )
        .init();
    let db_url = &config().database_url;
    let max_attempts = config().max_attempts;
    let pool = PgClient::new(db_url.clone(), 10, max_attempts).await;
    run_migrations(&pool, max_attempts).await?;
    info!("Migrations executed");
    Ok(())
}
