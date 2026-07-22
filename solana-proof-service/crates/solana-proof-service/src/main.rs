//! Solana proof service binary: config → migrate → validate → ingest + HTTP.

use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use solana_proof_source::{YellowstoneBlockSource, YellowstoneSourceConfig};
use solana_proof_store::{run_sequential_ingest, IngestHooks, SqlProofStore};
use sqlx::postgres::PgPoolOptions;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

use solana_proof_service::chain::RpcChainFetcher;
use solana_proof_service::config::ServiceConfig;
use solana_proof_service::http::{router, AppState};
use solana_proof_service::ingest_health::IngestHealth;
use solana_proof_service::lifecycle::{
    supervise_http_and_writer, wait_for_shutdown, INGEST_SHUTDOWN_DEADLINE,
};
use solana_proof_service::metrics;
use solana_proof_service::startup_validation::validate_startup_dependencies;

/// Bound pool checkout so readiness/proof probes cannot hang forever on acquire.
const DB_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(2);

#[tokio::main]
async fn main() -> Result<()> {
    metrics::init_metrics();
    init_tracing();

    let config = ServiceConfig::load().context("failed to load service config")?;
    let program_id = config.program_id_bytes()?;

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(DB_ACQUIRE_TIMEOUT)
        .connect(&config.database.connection_string)
        .await
        .context("failed to connect to postgres")?;
    let store = SqlProofStore::new(pool, program_id);
    store.migrate().await.context("failed to run migrations")?;

    validate_startup_dependencies(&store)
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

    let ingest_handle =
        spawn_ingest_writer(source, store.clone(), Arc::clone(&ingest), cancel.clone());

    let fetcher = Arc::new(RpcChainFetcher::new(
        config.solana.rpc_url.clone(),
        program_id,
    ));
    let state = Arc::new(AppState {
        store,
        fetcher,
        ingest: Arc::clone(&ingest),
    });
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(config.server.bind_address)
        .await
        .with_context(|| format!("failed to bind {}", config.server.bind_address))?;
    tracing::info!(%config.server.bind_address, "HTTP listening");

    // One token: OS signal, unexpected writer exit, and Axum graceful shutdown.
    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        shutdown_signal().await;
        signal_cancel.cancel();
    });

    let shutdown = cancel.clone();
    let server = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            wait_for_shutdown(shutdown).await;
        })
        .into_future();

    supervise_http_and_writer(
        server,
        ingest_handle,
        &ingest,
        cancel,
        INGEST_SHUTDOWN_DEADLINE,
    )
    .await
}

fn spawn_ingest_writer(
    source: YellowstoneBlockSource,
    store: SqlProofStore,
    ingest_health: Arc<IngestHealth>,
    cancel: CancellationToken,
) -> JoinHandle<()> {
    let on_progress: Arc<dyn Fn(u64) + Send + Sync> = {
        let health = Arc::clone(&ingest_health);
        Arc::new(move |slot: u64| health.mark_progress(slot))
    };
    let on_disconnected: Arc<dyn Fn() + Send + Sync> = {
        let health = Arc::clone(&ingest_health);
        Arc::new(move || health.mark_disconnected())
    };

    tokio::spawn(async move {
        ingest_health.mark_started();
        let result = run_sequential_ingest(
            &source,
            &store,
            cancel,
            IngestHooks {
                on_progress: Some(on_progress.as_ref()),
                on_disconnected: Some(on_disconnected.as_ref()),
            },
        )
        .await;
        if let Err(err) = &result {
            tracing::error!(%err, "ingest task stopped");
        }
        ingest_health.mark_finished(result);
    })
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
