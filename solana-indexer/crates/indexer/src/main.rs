//! Indexer binary entrypoint: parse args, load config, init tracing, run under a
//! CancellationToken wired to Ctrl-C.

use clap::Parser;
use indexer::config::settings::Settings;
use indexer::tracing::init_tracing;
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
#[command(name = "indexer", about = "Solana rotation-leaf MMR-proof indexer")]
struct Args {
    /// Directory holding `default.toml` (overridable by APP_… env vars).
    #[arg(long, default_value = "config", env = "APP_CONFIG_DIR")]
    config_dir: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let settings = Settings::load(&args.config_dir)?;
    init_tracing(&settings.log);

    let shutdown = CancellationToken::new();
    {
        let shutdown = shutdown.clone();
        tokio::spawn(async move {
            let _ = tokio::signal::ctrl_c().await;
            tracing::info!("ctrl-c received, shutting down");
            shutdown.cancel();
        });
    }

    indexer::run(settings, shutdown).await
}
