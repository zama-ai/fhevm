use alloy::network::EthereumWallet;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use alloy::{primitives::Address, transports::http::reqwest::Url};
use clap::Parser;
use gw_listener::gw_listener::GatewayListener;
use gw_listener::ConfigSettings;
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

    #[arg(long)]
    ws_endpoint_url: String,
}

#[tokio::main]
async fn main() {
    let conf = Conf::parse();

    let database_url = conf
        .database_url
        .clone()
        .unwrap_or_else(|| std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"));

    let signer: PrivateKeySigner = PrivateKeySigner::random(); // TODO:
    let wallet: EthereumWallet = signer.clone().into();

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_ws(WsConnect::new(conf.ws_endpoint_url))
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
        provider.clone(),
    );

    // Run gw_listener thread
    let _ = gw_listener.run().await;
}
