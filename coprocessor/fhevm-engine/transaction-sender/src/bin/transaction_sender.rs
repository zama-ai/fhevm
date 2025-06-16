use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{ProviderBuilder, WsConnect},
    signers::{aws::AwsSigner, local::PrivateKeySigner, Signer},
    transports::http::reqwest::Url,
};
use anyhow::Context;
use aws_config::BehaviorVersion;
use clap::{Parser, ValueEnum};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};
use transaction_sender::{
    get_chain_id, http_server::HttpServer, make_abstract_signer, AbstractSigner, ConfigSettings,
    FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

use humantime::parse_duration;

#[derive(Parser, Debug, Clone, ValueEnum)]
enum SignerType {
    PrivateKey,
    AwsKms,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Conf {
    #[arg(short, long)]
    input_verification_address: Address,

    #[arg(short, long)]
    ciphertext_commits_address: Address,

    #[arg(short, long)]
    multichain_acl_address: Address,

    #[arg(short, long)]
    gateway_url: Url,

    #[arg(short, long, value_enum, default_value = "private-key")]
    signer_type: SignerType,

    #[arg(short, long)]
    private_key: Option<String>,

    #[arg(short, long)]
    database_url: Option<String>,

    #[arg(long, default_value = "10")]
    database_pool_size: u32,

    #[arg(long, default_value = "5")]
    database_polling_interval_secs: u16,

    #[arg(long, default_value = "verify_proof_responses")]
    verify_proof_resp_database_channel: String,

    #[arg(long, default_value = "add_ciphertexts")]
    add_ciphertexts_database_channel: String,

    #[arg(long, default_value = "event_allowed_handle")]
    allow_handle_database_channel: String,

    #[arg(long, default_value = "128")]
    verify_proof_resp_batch_limit: u32,

    #[arg(long, default_value = "3")]
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

    #[arg(long, default_value = "30")]
    review_after_unlimited_retries: u16,

    #[arg(long, default_value = "1000000")]
    provider_max_retries: u32,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    health_check_timeout: Duration,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    log_level: Level,
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
    let conf = Conf::parse();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(conf.log_level)
        .init();

    let chain_id = get_chain_id(conf.gateway_url.clone(), conf.provider_retry_interval).await;
    let abstract_signer: AbstractSigner;
    match conf.signer_type {
        SignerType::PrivateKey => {
            if conf.private_key.is_none() {
                error!("Private key is required for PrivateKey signer");
                return Err(anyhow::anyhow!(
                    "Private key is required for PrivateKey signer"
                ));
            }
            let mut signer = PrivateKeySigner::from_str(conf.private_key.unwrap().trim())?;
            signer.set_chain_id(Some(chain_id));
            abstract_signer = make_abstract_signer(signer);
        }
        SignerType::AwsKms => {
            let key_id = std::env::var("AWS_KEY_ID")
                .context("AWS_KEY_ID environment variable is required for AwsKms signer")?;
            let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
            let aws_kms_client = aws_sdk_kms::Client::new(&aws_conf);
            let signer = AwsSigner::new(aws_kms_client, key_id, Some(chain_id)).await?;
            abstract_signer = make_abstract_signer(signer);
        }
    }
    let wallet = EthereumWallet::new(abstract_signer.clone());
    let database_url = match conf.database_url.clone() {
        Some(url) => url,
        None => std::env::var("DATABASE_URL").context("DATABASE_URL is undefined")?,
    };
    let cancel_token = CancellationToken::new();

    let provider = loop {
        match ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
            .connect_ws(
                WsConnect::new(conf.gateway_url.clone())
                    .with_max_retries(conf.provider_max_retries)
                    .with_retry_interval(conf.provider_retry_interval),
            )
            .await
        {
            Ok(inner_provider) => {
                info!("Connected to Gateway at {}", conf.gateway_url);
                break NonceManagedProvider::new(
                    inner_provider,
                    Some(wallet.default_signer().address()),
                );
            }
            Err(e) => {
                error!(
                    "Failed to connect to Gateway at {} on startup: {}, retrying in {:?}",
                    conf.gateway_url, e, conf.provider_retry_interval
                );
                tokio::time::sleep(conf.provider_retry_interval).await;
            }
        }
    };

    let config = ConfigSettings {
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
        review_after_unlimited_retries: conf.review_after_unlimited_retries,
        health_check_port: conf.health_check_port,
        health_check_timeout: conf.health_check_timeout,
    };

    let transaction_sender = std::sync::Arc::new(
        TransactionSender::new(
            conf.input_verification_address,
            conf.ciphertext_commits_address,
            conf.multichain_acl_address,
            abstract_signer,
            provider,
            cancel_token.clone(),
            config.clone(),
            None,
        )
        .await?,
    );

    let http_server = HttpServer::new(
        transaction_sender.clone(),
        conf.health_check_port,
        cancel_token.clone(),
    );

    install_signal_handlers(cancel_token.clone())?;

    info!(
        health_check_port = conf.health_check_port,
        conf = ?config,
        "Transaction sender and HTTP health check server starting"
    );

    // Run both services concurrently
    let (sender_result, http_result) = tokio::join!(transaction_sender.run(), http_server.start());

    // Check results
    if let Err(e) = sender_result {
        error!("Transaction sender error: {}", e);
        return Err(e);
    }

    if let Err(e) = http_result {
        error!("HTTP server error: {}", e);
        return Err(e);
    }

    info!("Transaction sender and HTTP server stopped gracefully");
    Ok(())
}
