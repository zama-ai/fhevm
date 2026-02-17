use clap::Parser;
use fhevm_engine_common::telemetry;
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = host_listener::cmd::Args::parse();

    let mut otlp_setup_error: Option<String> = None;
    let _otel_guard = match telemetry::init_json_subscriber(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    ) {
        Ok(guard) => guard,
        Err(err) => {
            otlp_setup_error = Some(err.to_string());
            None
        }
    };
    if let Some(err) = otlp_setup_error {
        error!(error = %err, "Failed to setup OTLP");
    }

    host_listener::cmd::main(args).await
}
