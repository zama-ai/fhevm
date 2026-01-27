use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::fillers::{
        BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, WalletFiller,
    },
    providers::{Identity, ProviderBuilder, RootProvider, WsConnect},
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
    config::DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT, get_chain_id, http_server::HttpServer,
    make_abstract_signer, metrics::spawn_gauge_update_routine, AbstractSigner, ConfigSettings,
    FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

use fhevm_engine_common::{
    metrics_server,
    telemetry::{self, MetricsConfig},
    utils::DatabaseURL,
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

    #[arg(long)]
    host_chain_url: Url,

    #[arg(short, long, value_enum, default_value = "private-key")]
    signer_type: SignerType,

    #[arg(short, long)]
    private_key: Option<String>,

    /// An optional DB URL.
    ///
    /// If not provided, falls back to the DATABASE_URL env var, if it is set.
    ///
    /// If not provided and DATABASE_URL is not set, then defaults to a local Postgres URL.
    #[arg(short, long)]
    database_url: Option<DatabaseURL>,

    #[arg(long, default_value = "10")]
    database_pool_size: u32,

    #[arg(long, default_value = "1")]
    database_polling_interval_secs: u16,

    #[arg(long, default_value = "event_zkpok_computed")]
    verify_proof_resp_database_channel: String,

    #[arg(long, default_value = "event_ciphertexts_uploaded")]
    add_ciphertexts_database_channel: String,

    #[arg(long, default_value = "event_allowed_handle")]
    allow_handle_database_channel: String,

    #[arg(long, default_value_t = 128)]
    verify_proof_resp_batch_limit: u32,

    #[arg(long, default_value_t = 6)]
    verify_proof_resp_max_retries: u32,

    #[arg(long, default_value_t = true)]
    verify_proof_remove_after_max_retries: bool,

    #[arg(long, default_value_t = 10)]
    add_ciphertexts_batch_limit: u32,

    #[arg(long, default_value_t = 10)]
    allow_handle_batch_limit: u32,

    // For now, use i32 as that's what we have in the DB as integer type.
    #[arg(long, default_value_t = i32::MAX, value_parser = clap::value_parser!(i32).range(0..))]
    allow_handle_max_retries: i32,

    // For now, use i32 as that's what we have in the DB as integer type.
    #[arg(long, default_value_t = i32::MAX, value_parser = clap::value_parser!(i32).range(0..))]
    add_ciphertexts_max_retries: i32,

    #[arg(long, default_value_t = 1)]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value_t = 4)]
    error_sleep_max_secs: u16,

    #[arg(long, default_value_t = 4, alias = "txn-receipt-timeout-secs")]
    send_txn_sync_timeout_secs: u16,

    #[deprecated(note = "no longer used and will be removed in future versions")]
    #[arg(long, default_value_t = 0, hide = true)]
    required_txn_confirmations: u16,

    #[arg(long, default_value_t = 30)]
    review_after_unlimited_retries: u16,

    #[arg(long, default_value_t = u32::MAX)]
    provider_max_retries: u32,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,

    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    metrics_addr: Option<String>,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    health_check_timeout: Duration,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value_t = DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT, value_parser = clap::value_parser!(u32).range(100..))]
    gas_limit_overprovision_percent: u32,

    #[arg(long, default_value = "8s", value_parser = parse_duration)]
    graceful_shutdown_timeout: Duration,

    /// service name in OTLP traces
    #[arg(long, default_value = "txn-sender")]
    pub service_name: String,

    /// Prometheus metrics: coprocessor_host_txn_latency_seconds
    #[arg(long, default_value = "0.1:60.0:0.1", value_parser = clap::value_parser!(MetricsConfig))]
    pub metric_host_txn_latency: MetricsConfig,

    /// Prometheus metrics: coprocessor_zkproof_txn_latency_seconds
    #[arg(long, default_value = "0.1:60.0:0.1", value_parser = clap::value_parser!(MetricsConfig))]
    pub metric_zkproof_txn_latency: MetricsConfig,

    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..))]
    pub gauge_update_interval_secs: u64,

    #[arg(
        long,
        default_value = "10",
        help = "Delay for transmitting delegation to gateway in block number (to avoid most reorg)"
    )]
    pub delegation_block_delay: u64,

    #[arg(
        long,
        default_value = "648000", // 3 months assuming 12s block time on host chain
        help = "Clear delegation entries after N blocks (deault to 3 months)"
    )]
    pub delegation_clear_after_n_blocks: u64,

    #[arg(
        long,
        default_value = "30",
        help = "Delay for inspecting delegations without notification (in seconds)"
    )]
    pub delegation_fallback_polling: u64,

    #[arg(
        long,
        default_value = "100000",
        help = "Number of total retry before definitively stopping (retry are progressively intermittent and less frequent)"
    )]
    pub delegation_max_retry: u64,

    #[deprecated(note = "no longer used and will be removed in future versions")]
    #[arg(
        long,
        default_value_t = 0,
        help = "Number of immediate retries when concurrent transactions failed due to nonce errors (0 to disable)",
        hide = true
    )]
    pub retry_immediately_on_nonce_error: u64,
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

fn parse_args() -> Conf {
    let args = Conf::parse();
    // Set global configs from args
    let _ = telemetry::HOST_TXN_LATENCY_CONFIG.set(args.metric_host_txn_latency);
    let _ = telemetry::ZKPROOF_TXN_LATENCY_CONFIG.set(args.metric_zkproof_txn_latency);
    args
}

type Provider = FillProvider<
    JoinFill<
        JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

async fn get_provider(
    conf: &Conf,
    url: &Url,
    name: &str,
    wallet: EthereumWallet,
    cancel_token: &CancellationToken,
) -> anyhow::Result<Provider> {
    loop {
        if cancel_token.is_cancelled() {
            info!(
                "Cancellation requested before provider ({}) was created on startup, exiting",
                name
            );
            anyhow::bail!(
                "Cancellation requested before provider ({}) was created on startup, exiting",
                name
            );
        }
        match ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
            .connect_ws(
                // Note here that max_retries and retry_interval apply to sending requests, not to initial connection.
                // We assume they are set to big values such that when they are reached, the following `BackendGone` error
                // means we can't move on and we would exit the whole sender.
                WsConnect::new(url.clone())
                    .with_max_retries(conf.provider_max_retries)
                    .with_retry_interval(conf.provider_retry_interval),
            )
            .await
        {
            Ok(provider) => {
                info!(name, "Connected to chain");
                return Ok(provider);
            }
            Err(e) => {
                error!(
                    name,
                    error = %e,
                    retry_interval = ?conf.provider_retry_interval,
                    "Failed to connect to chain on startup, retrying"
                );
                tokio::time::sleep(conf.provider_retry_interval).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let conf = parse_args();

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
            let Some(private_key) = &conf.private_key else {
                error!("Private key is required for PrivateKey signer");
                return Err(anyhow::anyhow!(
                    "Private key is required for PrivateKey signer"
                ));
            };
            let mut signer = PrivateKeySigner::from_str(private_key.trim())?;
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

    let Ok(gateway_provider) = get_provider(
        &conf,
        &conf.gateway_url,
        "Gateway",
        wallet.clone(),
        &cancel_token,
    )
    .await
    else {
        info!(
            "Cancellation requested before gateway chain provider was created on startup, exiting"
        );
        return Ok(());
    };
    let gateway_provider =
        NonceManagedProvider::new(gateway_provider, Some(wallet.default_signer().address()));
    let Ok(host_chain_provider) = get_provider(
        &conf,
        &conf.host_chain_url,
        "HostChain",
        wallet.clone(),
        &cancel_token,
    )
    .await
    else {
        info!("Cancellation requested before host chain provider was created on startup, exiting");
        return Ok(());
    };

    let config = ConfigSettings {
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
        send_txn_sync_timeout_secs: conf.send_txn_sync_timeout_secs,
        review_after_unlimited_retries: conf.review_after_unlimited_retries,
        health_check_port: conf.health_check_port,
        health_check_timeout: conf.health_check_timeout,
        gas_limit_overprovision_percent: conf.gas_limit_overprovision_percent,
        graceful_shutdown_timeout: conf.graceful_shutdown_timeout,
        delegation_block_delay: conf.delegation_block_delay,
        delegation_clear_after_n_blocks: conf.delegation_clear_after_n_blocks,
        delegation_fallback_polling: conf.delegation_fallback_polling,
        delegation_max_retry: conf.delegation_max_retry,
    };

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.database_pool_size)
        .connect(conf.database_url.unwrap_or_default().as_str())
        .await?;

    let transaction_sender = std::sync::Arc::new(
        TransactionSender::new(
            db_pool.clone(),
            conf.input_verification_address,
            conf.ciphertext_commits_address,
            conf.multichain_acl_address,
            abstract_signer,
            gateway_provider,
            host_chain_provider,
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

    info!(
        health_check_port = conf.health_check_port,
        conf = ?config,
        "Transaction sender and HTTP health check server starting"
    );

    // Run both services in parallel. Here we assume that if transaction sender stops without an error, HTTP server should also stop.
    let transaction_sender_fut = tokio::spawn(async move { transaction_sender.run().await });
    let http_server_fut = tokio::spawn(async move { http_server.start().await });

    // Start metrics server.
    metrics_server::spawn(conf.metrics_addr.clone(), cancel_token.child_token());

    // Start gauge update routine.
    spawn_gauge_update_routine(
        Duration::from_secs(conf.gauge_update_interval_secs),
        db_pool.clone(),
    );

    let transaction_sender_res = transaction_sender_fut.await;
    let http_server_res = http_server_fut.await;

    info!(
        transaction_sender_res = ?transaction_sender_res,
        http_server_res = ?http_server_res,
        "Transaction sender and HTTP health check server tasks have stopped"
    );

    transaction_sender_res??;
    http_server_res??;

    info!("Transaction sender and HTTP health check server stopped gracefully");

    Ok(())
}
