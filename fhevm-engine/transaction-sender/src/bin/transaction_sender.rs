use std::str::FromStr;

use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use clap::Parser;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use transaction_sender::{
    ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Conf {
    #[arg(short, long)]
    zkpok_manager_address: Address,

    #[arg(short, long)]
    ciphertext_manager_address: Address,

    #[arg(short, long)]
    acl_manager_address: Address,

    #[arg(short, long)]
    gateway_url: Url,

    #[arg(short, long)]
    private_key: String,

    #[arg(short, long)]
    database_url: Option<String>,

    #[arg(long, default_value = "10")]
    database_pool_size: u32,

    #[arg(long, default_value = "5")]
    database_polling_interval_secs: u16,

    #[arg(long, default_value = "verify_proof_responses")]
    verify_proof_resp_database_channel: String,

    #[arg(short, long, default_value = "add_ciphertexts")]
    add_ciphertexts_database_channel: String,

    #[arg(short, long, default_value = "event_allowed_handle")]
    allow_handle_database_channel: String,

    #[arg(long, default_value = "128")]
    verify_proof_resp_batch_limit: u32,

    #[arg(long, default_value = "15")]
    verify_proof_resp_max_retries: u32,

    #[arg(long, default_value = "true")]
    verify_proof_remove_after_max_retries: bool,

    #[arg(long, default_value = "10")]
    add_ciphertexts_batch_limit: u32,

    #[arg(long, default_value = "10")]
    allow_handle_batch_limit: u32,

    #[arg(long, default_value = "10")]
    allow_handle_max_retries: u32,

    #[arg(long, default_value = "15")]
    add_ciphertexts_max_retries: u32,

    #[arg(long, default_value = "1")]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value = "16")]
    error_sleep_max_secs: u16,

    #[arg(long, default_value = "10")]
    txn_receipt_timeout_secs: u16,

    #[arg(long, default_value = "0")]
    required_txn_confirmations: u16,
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
    let signer = PrivateKeySigner::from_str(conf.private_key.trim())?;
    let wallet = EthereumWallet::new(signer.clone());
    let database_url = conf
        .database_url
        .clone()
        .unwrap_or_else(|| std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"));
    let cancel_token = CancellationToken::new();
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
            .on_ws(WsConnect::new(conf.gateway_url))
            .await?,
        Some(wallet.default_signer().address()),
    );
    let sender = TransactionSender::new(
        conf.zkpok_manager_address,
        conf.ciphertext_manager_address,
        conf.acl_manager_address,
        signer,
        provider,
        cancel_token.clone(),
        ConfigSettings {
            database_url,
            database_pool_size: conf.database_pool_size,
            verify_proof_resp_db_channel: conf.verify_proof_resp_database_channel,
            add_ciphertexts_db_channel: conf.add_ciphertexts_database_channel,
            allow_handle_db_channel: conf.allow_handle_database_channel,
            verify_proof_resp_batch_limit: conf.verify_proof_resp_batch_limit,
            verify_proof_resp_max_retries: conf.verify_proof_resp_max_retries,
            verify_proof_remove_after_max_retries: conf.verify_proof_remove_after_max_retries,
            add_ciphertexts_batch_limit: conf.add_ciphertexts_batch_limit,
            db_polling_interval_secs: conf.database_polling_interval_secs,
            error_sleep_initial_secs: conf.error_sleep_initial_secs,
            error_sleep_max_secs: conf.error_sleep_max_secs,
            add_ciphertexts_max_retries: conf.add_ciphertexts_max_retries,
            allow_handle_batch_limit: conf.allow_handle_batch_limit,
            allow_handle_max_retries: conf.allow_handle_max_retries,
            txn_receipt_timeout_secs: conf.txn_receipt_timeout_secs,
            required_txn_confirmations: conf.required_txn_confirmations,
        },
        None,
    )
    .await?;
    install_signal_handlers(cancel_token)?;
    sender.run().await
}
