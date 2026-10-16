#[cfg(feature = "test-failpoints")]
mod test_failpoints;
use ::tracing::{error, info};
use fhevm_engine_common::database::{connect_pool_with_options, resolve_database_url_from_option};
use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};
use fhevm_engine_common::{drift_revert, healthz_server, metrics_server, telemetry};
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;

use std::sync::{Once, OnceLock};
use tokio::task::JoinSet;

#[cfg(feature = "bench")]
pub mod benchmark_exact_tuples;
pub mod bridge;
pub mod daemon_cli;
pub mod dependence_chain;
pub mod health_check;

#[cfg(test)]
mod tests;
pub mod tfhe_worker;
pub mod types;

// separate function for testing
pub fn start_runtime(
    args: daemon_cli::Args,
    close_recv: Option<tokio::sync::watch::Receiver<bool>>,
) {
    start_runtime_inner(args, close_recv, tfhe_worker::NO_READINESS);
}

/// Starts the worker and signals readiness once the worker is installed.
/// Benchmark harness only.
#[cfg(feature = "bench")]
pub fn start_runtime_with_readiness(
    args: daemon_cli::Args,
    close_recv: Option<tokio::sync::watch::Receiver<bool>>,
    readiness: Option<tfhe_worker::ReadinessSignal>,
) {
    start_runtime_inner(args, close_recv, readiness);
}

fn start_runtime_inner(
    args: daemon_cli::Args,
    close_recv: Option<tokio::sync::watch::Receiver<bool>>,
    readiness: tfhe_worker::Readiness,
) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        // not using tokio main to specify max blocking threads
        .max_blocking_threads(args.coprocessor_fhe_threads)
        .enable_all()
        .build()
        .unwrap();
    let fatal = runtime.block_on(async {
        let shutdown = CancellationToken::new();
        if let Some(mut close_recv) = close_recv {
            // A close request takes the same shutdown path as SIGTERM.
            let main = async_main_inner(args, readiness, shutdown.clone());
            tokio::pin!(main);
            let result = tokio::select! {
                result = &mut main => result,
                _ = close_recv.changed() => {
                    info!(target: "main_wchannel", "Service stopped voluntarily");
                    shutdown.cancel();
                    main.await
                }
            };
            if let Err(e) = result {
                error!(target: "main_wchannel", { error = e }, "Runtime error");
            }
            // Callers that pass `close_recv` run the worker in-process (the
            // unit tests and the benchmark harness) and own the process
            // themselves, so a failure here must not take them down.
            false
        } else if let Err(e) = install_signal_handlers(shutdown.clone()) {
            error!(target: "main", { error = %e }, "Failed to install signal handlers");
            true
        } else if let Err(e) = async_main_inner(args, readiness, shutdown).await {
            error!(target: "main", { error = e }, "Runtime error");
            // Export the fatal event while its async runtime is still alive.
            telemetry::flush();
            true
        } else {
            false
        }
    });
    // An abandoned batch keeps its FHE threads busy, and waiting for them would
    // run past the termination grace period. Its writes are already rolled back.
    runtime.shutdown_timeout(std::time::Duration::from_secs(1));

    // Supervisors using restart-on-failure need a nonzero status. The runtime
    // has been dropped here; embedded callers retain ownership of their process.
    if fatal {
        std::process::exit(1);
    }
}

// Used for testing as we would call `async_main()` multiple times.
static TRACING_INIT: Once = Once::new();
static OTEL_GUARD: OnceLock<Option<telemetry::TracerProviderGuard>> = OnceLock::new();

pub async fn async_main(
    args: daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    async_main_inner(args, tfhe_worker::NO_READINESS, CancellationToken::new()).await
}

/// SIGTERM (Kubernetes scale-in, rollout) and SIGINT stop the worker: it
/// abandons its open batch, hands its chains back and exits.
fn install_signal_handlers(shutdown: CancellationToken) -> std::io::Result<()> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => info!(target: "main", "SIGINT received; shutting down"),
            _ = sigterm.recv() => info!(target: "main", "SIGTERM received; shutting down"),
        }
        shutdown.cancel();
    });
    Ok(())
}

// `readiness` is the cfg-erased unit type without `bench`.
#[cfg_attr(not(feature = "bench"), allow(unused_variables))]
async fn async_main_inner(
    args: daemon_cli::Args,
    readiness: tfhe_worker::Readiness,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    TRACING_INIT.call_once(|| {
        let otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
            args.log_level,
            &args.service_name,
            "otlp-layer",
        );
        let _ = OTEL_GUARD.set(otel_guard);
    });

    #[cfg(feature = "gpu")]
    if let Err(err) = fhevm_engine_common::gpu_arch::ensure_matching_visible_devices() {
        error!(error = %err, "GPU runtime architecture does not match image target");
        telemetry::flush();
        std::process::exit(1);
    }

    info!(target: "async_main", args = ?args, "Starting runtime with args");

    let database_url = resolve_database_url_from_option(args.database_url.clone())?;

    let health_check = health_check::HealthCheck::new(
        database_url.clone(),
        std::time::Duration::from_secs(args.max_batch_ttl_secs),
    );

    let mut set = JoinSet::new();
    let metrics_addr = args.metrics_addr.clone();
    if let Some(fut) = metrics_server::metrics_future(metrics_addr, cancel_token.child_token()) {
        set.spawn(async {
            fut.await;
            Ok(())
        });
    }

    info!(target: "async_main", "Start health check server");
    let health_check_cancel_token = cancel_token.child_token();
    let health_check_server = healthz_server::HttpServer::new(
        std::sync::Arc::new(health_check.clone()),
        args.health_check_port,
        health_check_cancel_token,
    );
    set.spawn(async move {
        if let Err(e) = health_check_server.start().await {
            error!(target: "async_main", error = %e, "Health check server failed");
        }
        Ok(())
    });

    let (drift_revert_pool, _pool_refresh_handle) = connect_pool_with_options(
        &database_url,
        PgPoolOptions::new().max_connections(1),
        Some(&cancel_token),
    )
    .await?;
    drift_revert::init(
        drift_revert_pool,
        cancel_token.clone(),
        None,
        args.drift_revert_watcher_timeouts,
    )
    .await?;

    if args.run_bg_worker {
        let gpu_enabled = fhevm_engine_common::utils::log_backend();
        info!(target: "async_main", gpu_enabled,  "Initializing background worker");

        #[cfg(feature = "bench")]
        set.spawn(tfhe_worker::run_tfhe_worker_with_readiness(
            args.clone(),
            health_check.clone(),
            readiness,
            cancel_token.child_token(),
        ));
        #[cfg(not(feature = "bench"))]
        set.spawn(tfhe_worker::run_tfhe_worker(
            args.clone(),
            health_check.clone(),
            cancel_token.child_token(),
        ));

        set.spawn(bridge::run_confidential_bridge(
            args.clone(),
            cancel_token.child_token(),
        ));
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(())) => {}
            Ok(Err(err)) => {
                panic!("Background worker exited with error: {err:?}");
            }
            Err(join_err) => {
                panic!("Background worker task failed: {join_err:?}");
            }
        }
    }

    Ok(())
}

pub fn generate_dump_fhe_keys() {
    let keys = FhevmKeys::new();
    let ser_keys: SerializedFhevmKeys = keys.into();
    ser_keys.save_to_disk();
}
