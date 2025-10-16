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

use fhevm_engine_common::{telemetry, utils::DatabaseURL};
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
    database_url: Option<DatabaseURL>,

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

    /// HTTP server port
    #[arg(long, alias = "health-check-port", default_value_t = 8080)]
    http_server_port: u16,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    health_check_timeout: Duration,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value = "120", value_parser = clap::value_parser!(u32).range(100..))]
    gas_limit_overprovision_percent: u32,

    #[arg(long, default_value = "8s", value_parser = parse_duration)]
    graceful_shutdown_timeout: Duration,

    /// service name in OTLP traces
    #[arg(long, default_value = "txn-sender")]
    pub service_name: String,
}

fn install_signal_handlers(cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => { info!("received SIGINT"); },
            _ = sigterm.recv() => { info!("received SIGTERM"); },
        }
        cancel_token.cancel();
        info!("Cancellation signal sent over the token");
    });
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let conf = Conf::parse();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(conf.log_level)
        .init();

    let cancel_token = CancellationToken::new();
    install_signal_handlers(cancel_token.clone())?;

    // Try to get the chain ID until cancelled.
    let chain_id = tokio::select! {
        chain_id = get_chain_id(
            conf.gateway_url.clone(),
            conf.graceful_shutdown_timeout,
        ) => chain_id,

        _ = cancel_token.cancelled() => {
            info!("Cancellation requested before getting chain ID during startup, exiting");
            return Ok(());
        }
    };

    if !conf.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&conf.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

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
        None => std::env::var("DATABASE_URL")
            .context("DATABASE_URL is undefined")?
            .into(),
    };

    let provider = loop {
        if cancel_token.is_cancelled() {
            info!("Cancellation requested before provider was created on startup, exiting");
            return Ok(());
        }
        match ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
            .connect_ws(
                // Note here that max_retries and retry_interval apply to sending requests, not to initial connection.
                // We assume they are set to big values such that when they are reached, the following `BackendGone` error
                // means we can't move on and we would exit the whole sender.
                WsConnect::new(conf.gateway_url.clone())
                    .with_max_retries(conf.provider_max_retries)
                    .with_retry_interval(conf.provider_retry_interval),
            )
            .await
        {
            Ok(inner_provider) => {
                info!(
                    gateway_url = %conf.gateway_url,
                    "Connected to Gateway"
                );
                break NonceManagedProvider::new(
                    inner_provider,
                    Some(wallet.default_signer().address()),
                );
            }
            Err(e) => {
                error!(
                    gateway_url = %conf.gateway_url,
                    error = %e,
                    retry_interval = ?conf.provider_retry_interval,
                    "Failed to connect to Gateway on startup, retrying"
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
        http_server_port: conf.http_server_port,
        health_check_timeout: conf.health_check_timeout,
        gas_limit_overprovision_percent: conf.gas_limit_overprovision_percent,
        graceful_shutdown_timeout: conf.graceful_shutdown_timeout,
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
        conf.http_server_port,
        cancel_token.clone(),
    );

    info!(
        http_server_port = conf.http_server_port,
        conf = ?config,
        "Transaction sender and HTTP server starting"
    );

    // Run both services concurrently. Here we assume that if transaction sender stops without an error, HTTP server should also stop.
    let transaction_sender_fut = tokio::spawn(async move { transaction_sender.run().await });
    let http_server_fut = tokio::spawn(async move { http_server.start().await });

    let transaction_sender_res = transaction_sender_fut.await;
    let http_server_res = http_server_fut.await;

    info!(
        transaction_sender_res = ?transaction_sender_res,
        http_server_res = ?http_server_res,
        "Transaction sender and HTTP server tasks have stopped"
    );

    transaction_sender_res??;
    http_server_res??;

    info!("Transaction sender and HTTP server stopped gracefully");

    Ok(())
}
