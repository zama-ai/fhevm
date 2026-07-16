//! Solana proof service binary: config → migrate → validate → ingest + HTTP.

use std::sync::Arc;

use anyhow::{Context, Result};
use solana_proof_source::{YellowstoneBlockSource, YellowstoneSourceConfig};
use solana_proof_store::{run_sequential_ingest, SqlProofStore};
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

use solana_proof_service::chain::RpcChainFetcher;
use solana_proof_service::config::ServiceConfig;
use solana_proof_service::http::{router, AppState};
use solana_proof_service::ingest_health::IngestHealth;
use solana_proof_service::metrics;
use solana_proof_service::startup_validation::validate_startup_dependencies;

#[tokio::main]
async fn main() -> Result<()> {
    metrics::init_metrics();
    init_tracing();

    let config = ServiceConfig::load().context("failed to load service config")?;
    let program_id = config.program_id_bytes()?;

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.connection_string)
        .await
        .context("failed to connect to postgres")?;
    let store = SqlProofStore::new(pool, program_id);
    store.migrate().await.context("failed to run migrations")?;

    validate_startup_dependencies(&config, &store)
        .await
        .context("startup dependency validation failed")?;

    let ingest = IngestHealth::new();
    let cancel = CancellationToken::new();

    let source = YellowstoneBlockSource::new(YellowstoneSourceConfig {
        grpc_url: config.yellowstone.grpc_url.clone(),
        x_token: config.yellowstone.x_token.clone(),
        program_id: config.solana.program_id.clone(),
    })
    .context("invalid yellowstone source config")?;

    let ingest_store = store.clone();
    let ingest_health = Arc::clone(&ingest);
    let ingest_cancel = cancel.clone();
    let on_progress: Arc<dyn Fn(u64) + Send + Sync> = {
        let health = Arc::clone(&ingest);
        Arc::new(move |slot: u64| health.mark_progress(slot))
    };
    tokio::spawn(async move {
        ingest_health.mark_started();
        let result = run_sequential_ingest(
            &source,
            &ingest_store,
            ingest_cancel,
            Some(on_progress.as_ref()),
        )
        .await;
        if let Err(err) = &result {
            tracing::error!(%err, "ingest task stopped");
        }
        ingest_health.mark_finished(result);
    });

    let fetcher = Arc::new(RpcChainFetcher::new(config.solana.rpc_url.clone()));
    let state = Arc::new(AppState {
        store,
        fetcher,
        ingest,
        max_ingest_silence: config.readiness.max_ingest_silence(),
    });
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(config.server.bind_address)
        .await
        .with_context(|| format!("failed to bind {}", config.server.bind_address))?;
    tracing::info!(%config.server.bind_address, "HTTP listening");

    let shutdown = cancel.clone();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown_signal().await;
            shutdown.cancel();
        })
        .await
        .context("HTTP server error")?;

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .json()
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received");
}
