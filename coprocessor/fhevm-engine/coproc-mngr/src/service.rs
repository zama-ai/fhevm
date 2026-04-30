//! Main daemon loop. LISTENs on `event_upgrade`, drains unhandled rows from
//! `upgrade_events`, dispatches to per-event handlers.
//!
//! Cancellation: a single tokio CancellationToken stops the inbound LISTEN
//! and lets in-flight handlers finish their current SQL transaction before
//! exiting cleanly.

use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::postgres::{PgListener, PgPoolOptions};
use sqlx::PgPool;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::config::ConfigSettings;
use crate::handlers;
use crate::metrics::{
    INBOUND_EVENT_COUNTER, INBOUND_NOTIFICATION_COUNTER, INBOUND_POLL_COUNTER,
};

const DRAIN_BATCH_SIZE: i64 = 16;

pub async fn run(conf: ConfigSettings, cancel: CancellationToken) -> Result<()> {
    let pool = connect(&conf).await?;
    info!(
        database_url = %conf.database_url.as_str(),
        bcs_database_url = ?conf.bcs_database_url.as_ref().map(|u| u.as_str().to_string()),
        upgrade_event_channel = %conf.upgrade_event_channel,
        "coproc-mngr starting"
    );

    // Drain anything left over from a previous run. Restart-safe.
    drain_unhandled(&pool, &conf).await;

    let mut listener = PgListener::connect_with(&pool)
        .await
        .context("PgListener connect")?;
    listener
        .listen(&conf.upgrade_event_channel)
        .await
        .with_context(|| format!("LISTEN {}", conf.upgrade_event_channel))?;
    info!(channel = %conf.upgrade_event_channel, "Subscribed to upgrade events");

    let mut poll = interval(conf.poll_interval);
    poll.tick().await; // burn the immediate tick

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                info!("Cancellation requested, shutting down");
                return Ok(());
            }
            recv = listener.try_recv() => {
                match recv {
                    Ok(Some(_n)) => {
                        INBOUND_NOTIFICATION_COUNTER.inc();
                        drain_unhandled(&pool, &conf).await;
                    }
                    Ok(None) => {
                        warn!("Listener connection closed; reconnecting");
                        listener = match PgListener::connect_with(&pool).await {
                            Ok(l) => l,
                            Err(e) => {
                                error!(error = %e, "Listener reconnect failed; backing off");
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }
                        };
                        if let Err(e) = listener.listen(&conf.upgrade_event_channel).await {
                            error!(error = %e, "Re-LISTEN failed");
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "try_recv error");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            _ = poll.tick() => {
                INBOUND_POLL_COUNTER.inc();
                drain_unhandled(&pool, &conf).await;
            }
        }
    }
}

async fn drain_unhandled(pool: &PgPool, conf: &ConfigSettings) {
    let evs = match handlers::fetch_unhandled(pool, DRAIN_BATCH_SIZE).await {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e, "fetch_unhandled failed");
            return;
        }
    };
    if evs.is_empty() {
        return;
    }
    info!(count = evs.len(), "Dispatching upgrade events");
    INBOUND_EVENT_COUNTER.inc_by(evs.len() as u64);
    for ev in evs {
        if let Err(e) = handlers::dispatch(pool, conf, ev).await {
            // Errors in handlers are already logged; row is still marked
            // handled to prevent infinite re-dispatch. Operator alerts
            // off the prometheus counter UPGRADE_EVENT_FAIL_COUNTER.
            error!(error = format!("{e:#}").as_str(), "handler dispatch failed");
        }
    }
}

async fn connect(conf: &ConfigSettings) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(conf.database_pool_size)
        .connect(conf.database_url.as_str())
        .await
        .context("PgPool connect")
}
