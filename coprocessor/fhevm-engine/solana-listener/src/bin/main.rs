use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = solana_listener::cmd::Args::parse();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();

    solana_listener::cmd::main(args).await
}
