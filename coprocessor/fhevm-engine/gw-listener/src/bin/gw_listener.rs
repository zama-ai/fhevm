use std::time::Duration;

use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::{primitives::Address, transports::http::reqwest::Url};
use clap::Parser;
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

    #[arg(long, default_value = "16")]
    database_pool_size: u32,

    #[arg(long, default_value = "verify_proof_requests")]
    verify_proof_req_database_channel: String,

    #[arg(long)]
    gw_url: Url,

    #[arg(short, long)]
    input_verification_address: Address,

    #[arg(long, default_value = "1")]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value = "10")]
    error_sleep_max_secs: u16,

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    health_check_timeout: Duration,

    #[arg(long, default_value = "1000000")]
    provider_max_retries: u32,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,

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
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let conf = Conf::parse();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(conf.log_level)
        .init();

    info!(conf = ?conf, "Starting gw_listener");

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

    let cancel_token = CancellationToken::new();

    let config = ConfigSettings {
        database_url,
        database_pool_size: conf.database_pool_size,
        verify_proof_req_db_channel: conf.verify_proof_req_database_channel,
        gw_url: conf.gw_url,
        error_sleep_initial_secs: conf.error_sleep_initial_secs,
        error_sleep_max_secs: conf.error_sleep_max_secs,
        health_check_port: conf.health_check_port,
        health_check_timeout: conf.health_check_timeout,
    };

    let gw_listener = GatewayListener::new(
        conf.input_verification_address,
        config.clone(),
        cancel_token.clone(),
        provider.clone(),
    );

    // Wrap the GatewayListener in an Arc
    let gw_listener = std::sync::Arc::new(gw_listener);

    // Create HTTP server with the Arc-wrapped listener
    let http_server = HttpServer::new(
        gw_listener.clone(),
        conf.health_check_port,
        cancel_token.clone(),
    );

    // Install signal handlers
    install_signal_handlers(cancel_token.clone())?;

    info!(
        health_check_port = conf.health_check_port,
        "Starting HTTP health check server"
    );

    // Run both services concurrently - note we now have to deref the Arc for run()
    let (listener_result, http_result) = tokio::join!(gw_listener.run(), http_server.start());

    // Check results
    if let Err(e) = listener_result {
        error!(error = %e, "Gateway listener error");
        return Err(e);
    }

    if let Err(e) = http_result {
        error!(error = %e, "HTTP server error");
        return Err(e);
    }

    info!("Gateway listener and HTTP server stopped gracefully");
    Ok(())
}
