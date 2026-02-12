use clap::Parser;
use fhevm_engine_common::telemetry;
use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = solana_listener::cmd::Args::parse();

    let mut otlp_setup_error: Option<String> = None;
    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&args.service_name) {
            otlp_setup_error = Some(err.to_string());
        }
    }

    let level_filter = tracing_subscriber::filter::LevelFilter::from_level(args.log_level);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_level(true)
        .with_target(false)
        .with_current_span(true)
        .with_span_list(false);
    tracing_subscriber::registry()
        .with(level_filter)
        .with(fmt_layer)
        .init();

    if let Some(err) = otlp_setup_error {
        error!(error = %err, "Failed to setup OTLP");
    }

    solana_listener::cmd::main(args).await
}
