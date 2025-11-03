use std::time::Duration;

use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::{primitives::Address, transports::http::reqwest::Url};
use clap::Parser;
use fhevm_engine_common::{metrics_server, telemetry};
use gw_listener::aws_s3::AwsS3Client;
use gw_listener::chain_id_from_env;
use gw_listener::gw_listener::GatewayListener;
use gw_listener::http_server::HttpServer;
use gw_listener::ConfigSettings;
use humantime::parse_duration;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Conf {
    #[arg(long)]
    database_url: Option<String>,

    #[arg(long, default_value_t = 16)]
    database_pool_size: u32,

    #[arg(long, default_value = "event_zkpok_new_work")]
    verify_proof_req_database_channel: String,

    #[arg(long)]
    gw_url: Url,

    #[arg(short, long)]
    input_verification_address: Address,

    #[arg(long)]
    kms_generation_address: Address,

    #[arg(long, default_value_t = 1)]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value_t = 10)]
    error_sleep_max_secs: u16,

    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    metrics_addr: Option<String>,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    health_check_timeout: Duration,

    #[arg(long, default_value_t = u32::MAX)]
    provider_max_retries: u32,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long)]
    host_chain_id: Option<u64>,

    #[arg(long, default_value = "1s", value_parser = parse_duration)]
    get_logs_poll_interval: Duration,

    #[arg(long, default_value_t = 100)]
    get_logs_block_batch_size: u64,

    /// gw-listener service name in OTLP traces
    #[arg(long, default_value = "gw-listener")]
    pub service_name: String,

    #[arg(long, default_value = None, help = "Can be negative from last processed block", allow_hyphen_values = true)]
    pub catchup_kms_generation_from_block: Option<i64>,
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
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let conf = Conf::parse();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(conf.log_level)
        .init();

    info!(conf = ?conf, "Starting gw_listener");

    if !conf.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&conf.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let database_url = conf
        .database_url
        .clone()
        .unwrap_or_else(|| std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"));

    let provider = loop {
        match ProviderBuilder::new()
            .connect_ws(
                WsConnect::new(conf.gw_url.clone())
                    .with_max_retries(conf.provider_max_retries)
                    .with_retry_interval(conf.provider_retry_interval),
            )
            .await
        {
            Ok(provider) => {
                info!(gateway_url = %conf.gw_url, "Connected to Gateway");
                break provider;
            }
            Err(e) => {
                error!(
                    gateway_url = %conf.gw_url,
                    error = %e,
                    provider_retry_interval = ?conf.provider_retry_interval,
                    "Failed to connect to Gateway"
                );
                tokio::time::sleep(conf.provider_retry_interval).await;
            }
        }
    };

    let aws_s3_client = AwsS3Client {};

    let cancel_token = CancellationToken::new();

    let Some(host_chain_id) = conf.host_chain_id.or_else(chain_id_from_env) else {
        anyhow::bail!("--host-chain-id or CHAIN_ID env var is missing.")
    };
    let config = ConfigSettings {
        host_chain_id,
        database_url,
        database_pool_size: conf.database_pool_size,
        verify_proof_req_db_channel: conf.verify_proof_req_database_channel,
        gw_url: conf.gw_url,
        error_sleep_initial_secs: conf.error_sleep_initial_secs,
        error_sleep_max_secs: conf.error_sleep_max_secs,
        health_check_port: conf.health_check_port,
        health_check_timeout: conf.health_check_timeout,
        get_logs_poll_interval: conf.get_logs_poll_interval,
        get_logs_block_batch_size: conf.get_logs_block_batch_size,
        catchup_kms_generation_from_block: conf.catchup_kms_generation_from_block,
    };

    let gw_listener = GatewayListener::new(
        conf.input_verification_address,
        conf.kms_generation_address,
        config.clone(),
        cancel_token.clone(),
        provider.clone(),
        aws_s3_client.clone(),
    );

    // Wrap the GatewayListener in an Arc
    let gw_listener = std::sync::Arc::new(gw_listener);

    let http_server = HttpServer::new(
        gw_listener.clone(),
        conf.health_check_port,
        cancel_token.clone(),
    );

    install_signal_handlers(cancel_token.clone())?;

    info!(
        health_check_port = conf.health_check_port,
        "Starting HTTP health check server"
    );

    // Run both services in parallel. Here we assume that if gw listener stops without an error, HTTP server should also stop.
    let gw_listener_fut = tokio::spawn(async move { gw_listener.run().await });
    let http_server_fut = tokio::spawn(async move { http_server.start().await });

    // Start the metrics server.
    metrics_server::spawn(conf.metrics_addr.clone(), cancel_token.child_token());

    let gw_listener_res = gw_listener_fut.await;
    let http_server_res = http_server_fut.await;

    info!(
        gw_listener_res = ?gw_listener_res,
        http_server_res = ?http_server_res,
        "Gateway listener and HTTP health check server tasks have stopped"
    );

    gw_listener_res??;
    http_server_res??;

    info!("Gateway listener and HTTP health check server stopped gracefully");

    Ok(())
}
