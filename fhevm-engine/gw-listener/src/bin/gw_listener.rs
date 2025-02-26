use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::{primitives::Address, transports::http::reqwest::Url};
use clap::Parser;
use gw_listener::gw_listener::GatewayListener;
use gw_listener::ConfigSettings;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Conf {
    #[arg(long)]
    database_url: Option<String>,

    #[arg(long, default_value = "16")]
    database_pool_size: u32,

    #[arg(long, default_value = "verify_proof_requests")]
    verify_proof_req_database_channel: String,

    #[arg(long)]
    gw_url: Url,

    #[arg(short, long)]
    zkpok_manager_address: Address,

    #[arg(long, default_value = "1")]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value = "10")]
    error_sleep_max_secs: u16,
}

fn install_signal_handlers(cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => (),
            _ = sigterm.recv() => ()
        }
        cancel_token.cancel();
    });
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().json().with_level(true).init();

    let conf = Conf::parse();

    let database_url = conf
        .database_url
        .clone()
        .unwrap_or_else(|| std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"));

    let provider = ProviderBuilder::new()
        .on_ws(WsConnect::new(conf.gw_url.clone()))
        .await
        .expect("should have valid provider");

    let cancel_token = CancellationToken::new();
    let gw_listener = GatewayListener::new(
        conf.zkpok_manager_address,
        ConfigSettings {
            database_url,
            database_pool_size: conf.database_pool_size,
            verify_proof_req_db_channel: conf.verify_proof_req_database_channel,
            gw_url: conf.gw_url,
            error_sleep_initial_secs: conf.error_sleep_initial_secs,
            error_sleep_max_secs: conf.error_sleep_max_secs,
        },
        cancel_token.clone(),
        provider,
    );

    // Run gw_listener thread
    install_signal_handlers(cancel_token)?;
    gw_listener.run().await
}
