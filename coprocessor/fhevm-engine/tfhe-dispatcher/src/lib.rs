use ::tracing::{error, info};
use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};
use fhevm_engine_common::telemetry;
use tokio_util::sync::CancellationToken;

use std::sync::Once;
use tokio::task::JoinSet;

pub mod cli;

pub mod dispatcher;
pub mod scheduler;
pub mod tfhe_dispatcher;

// separate function for testing
pub fn start_runtime(args: cli::Args, close_recv: Option<tokio::sync::watch::Receiver<bool>>) {
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

pub async fn async_main(args: cli::Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(args.log_level)
            .init();
    });

    info!(target: "async_main", args = ?args, "Starting runtime with args");
    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp("dispatcher") {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    tfhe_dispatcher::run_tfhe_dispatcher(args.clone()).await?;

    Ok(())
}
