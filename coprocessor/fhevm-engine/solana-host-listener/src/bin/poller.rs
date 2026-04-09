use clap::Parser;
use solana_host_listener::{config::PollerConfig, poller::run_from_cli};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .compact()
        .init();
    let config = PollerConfig::parse();
    run_from_cli(config).await
}
