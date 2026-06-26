use std::time::Duration;

use clap::Parser;
use fhevm_engine_common::{
    database::{connect_pool_with_options, resolve_database_url_from_option},
    utils::DatabaseURL,
};
use sqlx::postgres::PgPoolOptions;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};
use upgrade_controller::Config;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Coprocessor upgrade FSM controller", long_about = None)]
struct Args {
    /// Service name (used by the logger and OTLP traces).
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "upgrade-controller")]
    service_name: String,

    /// Postgres database URL. Falls back to DATABASE_URL env var.
    #[arg(long)]
    database_url: Option<DatabaseURL>,

    /// Postgres pool size.
    #[arg(long, default_value_t = 4)]
    database_pool_size: u32,

    /// Fallback poll interval (seconds) used while waiting for notifications.
    #[arg(long, default_value_t = 30)]
    poll_interval_secs: u64,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO,
    )]
    log_level: Level,
}

fn install_signal_handlers(cancel: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => {}
            _ = sigterm.recv() => {}
        }
        cancel.cancel();
    });
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    let database_url = resolve_database_url_from_option(args.database_url.clone())?;

    let gcs_mode = fhevm_engine_common::versioning::resolve_gcs_mode(database_url.as_str()).await?;

    let config = Config {
        service_name: args.service_name.clone(),
        database_url,
        database_pool_size: args.database_pool_size,
        gcs_mode,
        log_level: args.log_level,
        poll_interval: Duration::from_secs(args.poll_interval_secs),
    };

    info!(
        service_name = %config.service_name,
        gcs_mode = config.gcs_mode,
        pool_size = config.database_pool_size,
        "upgrade-controller starting"
    );

    let cancel = CancellationToken::new();
    install_signal_handlers(cancel.clone())?;

    let (pool, _refresh) = connect_pool_with_options(
        &config.database_url,
        PgPoolOptions::new().max_connections(config.database_pool_size),
        Some(&cancel),
    )
    .await?;

    if let Err(e) = upgrade_controller::run(config, pool, cancel.clone()).await {
        error!(error = %e, "upgrade-controller exited with error");
        return Err(e);
    }

    info!("upgrade-controller stopped cleanly");
    Ok(())
}
