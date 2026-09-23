//! Ingestion progress of the Solana host listener. Once its checkpoint leaves the provider's
//! replay window the listener cannot resume, so these exist to alert well before that:
//! `time() - applied_block_timestamp_seconds` is the lag in seconds, and
//! `confirmed_slot - applied_slot` the lag in slots.

use std::{sync::LazyLock, time::Duration};

use prometheus::{
    register_int_counter_vec, register_int_gauge_vec, IntCounterVec,
    IntGaugeVec,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use tokio_util::sync::CancellationToken;
use tracing::warn;

use crate::solana_grpc_source::SealedBlock;

const CONFIRMED_SLOT_POLL_INTERVAL: Duration = Duration::from_secs(10);

static APPLIED_SLOT: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_solana_host_listener_applied_slot",
        "Slot of the last sealed block whose rows, leaves and checkpoint the listener committed",
        &["host_chain_id"]
    )
    .unwrap()
});

static APPLIED_BLOCK_TIMESTAMP: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_solana_host_listener_applied_block_timestamp_seconds",
        "Unix time the cluster assigned to the last committed block",
        &["host_chain_id"]
    )
    .unwrap()
});

static CONFIRMED_SLOT: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_solana_host_listener_confirmed_slot",
        "The cluster's confirmed slot, polled over RPC",
        &["host_chain_id"]
    )
    .unwrap()
});

static RECONNECTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_solana_host_listener_reconnects_total",
        "gRPC subscriptions the listener dropped and reopened from its checkpoint",
        &["host_chain_id"]
    )
    .unwrap()
});

pub(super) fn record_applied(host_chain_id: u64, block: &SealedBlock) {
    let label = host_chain_id.to_string();
    APPLIED_SLOT
        .with_label_values(&[&label])
        .set(block.slot as i64);
    if let Some(timestamp) = block.unix_timestamp() {
        APPLIED_BLOCK_TIMESTAMP
            .with_label_values(&[&label])
            .set(timestamp);
    }
}

/// Exports the reconnect counter at zero, so `increase()` counts the first reconnect.
pub(super) fn init(host_chain_id: u64) {
    RECONNECTS.with_label_values(&[&host_chain_id.to_string()]);
}

pub(super) fn inc_reconnects(host_chain_id: u64) {
    RECONNECTS
        .with_label_values(&[&host_chain_id.to_string()])
        .inc();
}

/// Polls the cluster's confirmed slot until `cancel` fires. It reads RPC, not the gRPC
/// stream, so a stalled or delayed stream cannot hide its own lag.
pub async fn track_confirmed_slot(
    rpc: RpcClient,
    host_chain_id: u64,
    cancel: CancellationToken,
) {
    let gauge = CONFIRMED_SLOT.with_label_values(&[&host_chain_id.to_string()]);
    let mut interval = tokio::time::interval(CONFIRMED_SLOT_POLL_INTERVAL);
    loop {
        tokio::select! {
            _ = cancel.cancelled() => return,
            _ = interval.tick() => {}
        }
        match rpc
            .get_slot_with_commitment(CommitmentConfig::confirmed())
            .await
        {
            Ok(slot) => gauge.set(slot as i64),
            Err(error) => {
                warn!(error = %error, "poll the confirmed Solana slot")
            }
        }
    }
}
