use clap::Parser;
use fhevm_engine_common::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = host_listener::cmd::Args::parse();

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );

    host_listener::cmd::main(args).await
}
