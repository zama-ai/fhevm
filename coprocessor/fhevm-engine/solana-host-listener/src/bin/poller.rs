use clap::Parser;
use solana_host_listener::{config::PollerConfig, poller::run_from_cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = PollerConfig::parse();
    run_from_cli(config).await
}
