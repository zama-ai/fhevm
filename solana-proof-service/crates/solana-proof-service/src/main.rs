//! Solana proof service binary: config → migrate → validate → ingest + HTTP.

use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use solana_proof_source::{YellowstoneBlockSource, YellowstoneSourceConfig};
use solana_proof_store::{run_sequential_ingest, IngestHooks, SqlProofStore};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

use solana_proof_service::chain::RpcChainFetcher;
use solana_proof_service::config::ServiceConfig;
use solana_proof_service::http::{router, AppState};
use solana_proof_service::ingest_health::IngestHealth;
use solana_proof_service::metrics;
use solana_proof_service::startup_validation::validate_startup_dependencies;

/// Bound how long shutdown waits for the ingest writer after cancel.
const INGEST_SHUTDOWN_DEADLINE: Duration = Duration::from_secs(15);

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

    let mut ingest_handle =
        spawn_ingest_writer(source, store.clone(), Arc::clone(&ingest), cancel.clone());

    let fetcher = Arc::new(RpcChainFetcher::new(config.solana.rpc_url.clone()));
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

    let (force_shutdown_tx, force_shutdown_rx) = oneshot::channel::<()>();
    let shutdown_cancel = cancel.clone();
    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        tokio::select! {
            _ = shutdown_signal() => {}
            _ = force_shutdown_rx => {
                tracing::error!("forcing HTTP shutdown after ingest writer exit");
            }
        }
        shutdown_cancel.cancel();
    });
    // axum Serve implements IntoFuture; pin the future for select! polling.
    let mut server = std::pin::pin!(server.into_future());

    // Supervise HTTP and the ingest writer concurrently so a panic/exit during
    // steady state cannot leave readiness Ready indefinitely.
    enum Stopped {
        Server(std::io::Result<()>),
        Ingest(Result<(), tokio::task::JoinError>),
    }

    let stopped = tokio::select! {
        result = &mut server => Stopped::Server(result),
        join = &mut ingest_handle => Stopped::Ingest(join),
    };

    let mut unexpected_ingest_exit = false;
    let server_result = match stopped {
        Stopped::Server(result) => {
            // Always cancel/join the writer, even when the server returned Err.
            cancel.cancel();
            await_ingest_writer(ingest_handle, &ingest).await;
            result
        }
        Stopped::Ingest(join) => {
            unexpected_ingest_exit = !cancel.is_cancelled();
            record_ingest_join(join, &ingest, unexpected_ingest_exit);
            if unexpected_ingest_exit {
                let _ = force_shutdown_tx.send(());
            } else {
                drop(force_shutdown_tx);
            }
            // Writer already joined; wait for HTTP to finish after force/signal.
            server.as_mut().await
        }
    };

    server_result.context("HTTP server error")?;
    if unexpected_ingest_exit {
        anyhow::bail!("ingest writer exited while HTTP server was still running");
    }
    Ok(())
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

fn record_ingest_join(
    join: Result<(), tokio::task::JoinError>,
    ingest: &IngestHealth,
    unexpected: bool,
) {
    match join {
        Ok(()) => {
            if unexpected {
                tracing::error!(
                    terminal = ?ingest.terminal(),
                    "ingest writer exited while HTTP server was still running"
                );
            } else {
                tracing::info!("ingest writer stopped");
            }
        }
        Err(join_err) => {
            let reason = if join_err.is_panic() {
                format!("ingest writer panicked: {join_err}")
            } else {
                format!("ingest writer join failed: {join_err}")
            };
            tracing::error!(%reason, "ingest writer supervision failure");
            if ingest.writer_running() || ingest.terminal().is_none() {
                ingest.mark_crashed(reason);
            }
        }
    }
}

async fn await_ingest_writer(mut handle: JoinHandle<()>, ingest: &IngestHealth) {
    match tokio::time::timeout(INGEST_SHUTDOWN_DEADLINE, &mut handle).await {
        Ok(Ok(())) => {
            tracing::info!("ingest writer stopped");
        }
        Ok(Err(join_err)) => {
            record_ingest_join(Err(join_err), ingest, false);
        }
        Err(_) => {
            tracing::error!(
                deadline_secs = INGEST_SHUTDOWN_DEADLINE.as_secs(),
                "ingest writer did not exit within shutdown deadline; aborting"
            );
            handle.abort();
            match handle.await {
                Ok(()) | Err(_) => {}
            }
            ingest.mark_crashed("ingest writer shutdown deadline exceeded");
        }
    }
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
