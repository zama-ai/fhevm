//! Service composition: connect the pool, build shared state, and supervise the
//! Carbon pipeline, the axum HTTP server, and the Prometheus metrics server under
//! one `CancellationToken` — mirroring relayer/src/startup.rs.

use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::config::settings::Settings;
use crate::http::{self, AppState};
use crate::metrics::{self, Metrics};
use crate::pipeline;
use crate::rpc::SolanaRpc;
use crate::store::{self, repositories::lineage_repo::LineageRepo};

/// Runs the indexer until `shutdown` fires or a supervised task fails.
pub async fn run(settings: Settings, shutdown: CancellationToken) -> anyhow::Result<()> {
    info!("starting solana rotation-leaf indexer");

    let pool = store::connect(&settings.database).await?;
    let repo = LineageRepo::new(pool);
    let metrics = Metrics::new();

    let commitment = settings.solana.commitment_config();
    let rpc = Some(SolanaRpc::new(settings.solana.rpc_url.clone(), commitment));

    let state = Arc::new(AppState {
        repo: repo.clone(),
        rpc,
        metrics: metrics.clone(),
    });

    // Bind both servers up front so the bound addresses are known before serving.
    let (http_listener, http_addr) = http::server::bind(&settings.http.endpoint).await?;
    let (metrics_listener, metrics_addr) =
        metrics::server::bind(&settings.metrics.endpoint).await?;
    info!(%http_addr, %metrics_addr, "servers bound");

    let mut tasks = tokio::task::JoinSet::new();

    // HTTP server.
    {
        let state = state.clone();
        let token = shutdown.clone();
        tasks.spawn(async move {
            tokio::select! {
                res = http::server::serve(http_listener, state) => {
                    if let Err(e) = res { error!(error = %e, "http server exited"); }
                }
                _ = token.cancelled() => info!("http server shutting down"),
            }
        });
    }

    // Metrics server.
    {
        let registry = metrics.registry.clone();
        let token = shutdown.clone();
        tasks.spawn(async move {
            tokio::select! {
                res = metrics::server::serve(metrics_listener, registry) => {
                    if let Err(e) = res { error!(error = %e, "metrics server exited"); }
                }
                _ = token.cancelled() => info!("metrics server shutting down"),
            }
        });
    }

    // Carbon pipeline. On exit (error or completion) trip the token so the whole
    // service shuts down together.
    {
        let solana = settings.solana.clone();
        let token = shutdown.clone();
        tasks.spawn(async move {
            if let Err(e) = pipeline::run(&solana, repo, metrics, token.clone()).await {
                error!(error = %e, "pipeline exited with error");
            }
            token.cancel();
        });
    }

    // Supervise: if any task ends, cancel the rest and drain.
    while let Some(joined) = tasks.join_next().await {
        if let Err(e) = joined {
            error!(error = %e, "supervised task panicked");
        }
        shutdown.cancel();
    }

    info!("indexer shutdown complete");
    Ok(())
}
