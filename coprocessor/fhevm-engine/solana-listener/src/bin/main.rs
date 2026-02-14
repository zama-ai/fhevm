use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = solana_listener::cmd::Args::parse();

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

    solana_listener::cmd::main(args).await
}
