use std::time::Duration;

use clap::Parser;
use coproc_mngr::{run, ConfigSettings};
use fhevm_engine_common::utils::DatabaseURL;
use humantime::parse_duration;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};

#[derive(Parser, Debug, Clone)]
#[command(version, about = "FHEVM coprocessor upgrade orchestrator")]
struct Conf {
    /// GCS Postgres URL.
    #[arg(long, env = "DATABASE_URL")]
    database_url: DatabaseURL,

    #[arg(long, default_value_t = 8)]
    database_pool_size: u32,

    /// BCS Postgres URL. Currently only logged; reserved for the
    /// SNAPSHOTTING phase in a later iteration.
    #[arg(long, env = "BCS_DATABASE_URL")]
    bcs_database_url: Option<DatabaseURL>,

    /// Inbound NOTIFY channel coproc-mngr LISTENs on. Must match the
    /// trigger fired by `upgrade_events_notify_trigger` (default
    /// `event_upgrade`).
    #[arg(long, default_value = "event_upgrade")]
    upgrade_event_channel: String,

    /// Polling fallback interval if no NOTIFY arrives.
    #[arg(long, default_value = "10s", value_parser = parse_duration)]
    poll_interval: Duration,

    /// How long to wait for "fully settled" before giving up on a proposal.
    #[arg(long, default_value = "30m", value_parser = parse_duration)]
    readiness_timeout: Duration,

    /// Backoff between readiness checks.
    #[arg(long, default_value = "5s", value_parser = parse_duration)]
    readiness_poll_interval: Duration,

    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    #[arg(long, default_value = "0.0.0.0:9100")]
    metrics_addr: Option<String>,

    #[arg(long, value_parser = clap::value_parser!(Level), default_value_t = Level::INFO)]
    log_level: Level,

    /// OTel service name.
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "coproc-mngr")]
    service_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Conf::parse();
    init_tracing(args.log_level);

    let cancel = CancellationToken::new();
    spawn_signal_handler(cancel.clone());

    let conf = ConfigSettings {
        database_url: args.database_url,
        database_pool_size: args.database_pool_size,
        bcs_database_url: args.bcs_database_url,
        upgrade_event_channel: args.upgrade_event_channel,
        poll_interval: args.poll_interval,
        readiness_timeout: args.readiness_timeout,
        readiness_poll_interval: args.readiness_poll_interval,
        health_check_port: args.health_check_port,
        metrics_addr: args.metrics_addr,
    };

    info!(service = %args.service_name, "coproc-mngr starting up");

    if let Err(err) = run(conf, cancel).await {
        error!(error = %err, "coproc-mngr exited with error");
        return Err(err);
    }
    Ok(())
}

fn init_tracing(level: Level) {
    let _ = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(true)
        .try_init();
}

fn spawn_signal_handler(cancel: CancellationToken) {
    tokio::spawn(async move {
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                error!(error = %e, "failed to install SIGTERM handler");
                return;
            }
        };
        let mut sigint = match signal(SignalKind::interrupt()) {
            Ok(s) => s,
            Err(e) => {
                error!(error = %e, "failed to install SIGINT handler");
                return;
            }
        };
        tokio::select! {
            _ = sigterm.recv() => info!("SIGTERM"),
            _ = sigint.recv()  => info!("SIGINT"),
        }
        cancel.cancel();
    });
}
