use clap::Parser;
use fhevm_engine_common::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "test-failpoints")]
    if host_listener::consensus_test_control::key_download_cli()? {
        return Ok(());
    }
    fhevm_engine_common::handle_stack_version_flag();
    let args = host_listener::cmd::Args::parse();

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );

    host_listener::cmd::main(args).await
}
