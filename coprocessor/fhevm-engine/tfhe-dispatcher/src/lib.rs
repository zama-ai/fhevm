use ::tracing::{error, info};
use fhevm_engine_common::telemetry;
use std::sync::Once;
use tokio_util::sync::CancellationToken;
pub mod cli;

pub mod dispatcher;
pub mod scheduler;
pub mod tfhe_dispatcher;

// Used for testing as we would call `async_main()` multiple times.
static TRACING_INIT: Once = Once::new();

pub async fn async_main(
    args: cli::Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    tfhe_dispatcher::run_tfhe_dispatcher(args, cancel_token).await?;

    Ok(())
}
