use clap::Parser;
use fhevm_engine_common::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = host_listener::cmd::Args::parse();

    telemetry::setup_otlp("host-listener").unwrap();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();

    host_listener::cmd::main(args).await
}
