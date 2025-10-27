use ::tracing::{error, info};
use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};
use fhevm_engine_common::{healthz_server, metrics_server, telemetry};
use tokio_util::sync::CancellationToken;

use std::sync::Once;
use tokio::task::JoinSet;

pub mod daemon_cli;
mod db_queries;
pub mod health_check;
pub mod server;

#[cfg(test)]
mod tests;
pub mod tfhe_worker;
pub mod types;
mod utils;

// separate function for testing
pub fn start_runtime(
    args: daemon_cli::Args,
    close_recv: Option<tokio::sync::watch::Receiver<bool>>,
) {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        // not using tokio main to specify max blocking threads
        .max_blocking_threads(args.coprocessor_fhe_threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Some(mut close_recv) = close_recv {
                tokio::select! {
                    main = async_main(args) => {
                        if let Err(e) = main {
                            error!(target: "main_wchannel", { error = e }, "Runtime error");
                        }
                    }
                    _ = close_recv.changed() => {
                        info!(target: "main_wchannel", "Service stopped voluntarily");
                    }
                }
            } else if let Err(e) = async_main(args).await {
                error!(target: "main", { error = e }, "Runtime error");
            }
        })
}

// Used for testing as we would call `async_main()` multiple times.
static TRACING_INIT: Once = Once::new();

pub async fn async_main(
    args: daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(args.log_level)
            .init();
    });

    let cancel_token = CancellationToken::new();
    info!(target: "async_main", args = ?args, "Starting runtime with args");

    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&args.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let health_check = health_check::HealthCheck::new(
        args.database_url
            .clone()
            .unwrap_or("no_database_url".to_string()),
    );

    let mut set = JoinSet::new();
    if args.run_server {
        info!(target: "async_main", "Initializing api server");
        set.spawn(server::run_server(args.clone()));
    }

    if args.run_bg_worker {
        info!(target: "async_main", "Initializing background worker");
        set.spawn(tfhe_worker::run_tfhe_worker(
            args.clone(),
            health_check.clone(),
        ));
    }

    let metrics_addr = args.metrics_addr.clone();
    if let Some(fut) = metrics_server::metrics_future(metrics_addr, cancel_token.child_token()) {
        set.spawn(async {
            fut.await;
            Ok(())
        });
    }

    if set.is_empty() {
        panic!("No tasks specified to run");
    }

    info!(target: "async_main", "Start health check server");
    let health_check_cancel_token = CancellationToken::new();
    let health_check_server = healthz_server::HttpServer::new(
        std::sync::Arc::new(health_check.clone()),
        args.health_check_port,
        health_check_cancel_token,
    );
    let Ok(()) = health_check_server.start().await else {
        panic!("Failed to start health check server");
    };

    while let Some(res) = set.join_next().await {
        if let Err(e) = res {
            panic!("Error background initializing worker: {:?}", e);
        }
    }

    Ok(())
}

pub fn generate_dump_fhe_keys() {
    let keys = FhevmKeys::new();
    let ser_keys: SerializedFhevmKeys = keys.into();
    ser_keys.save_to_disk();
}
