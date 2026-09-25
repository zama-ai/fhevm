//! Ingestion progress of the Solana host listener. `time() - applied_block_timestamp_seconds` is
//! the lag in seconds, and `confirmed_slot - applied_slot` the lag in slots. Once the checkpoint
//! leaves the provider's replay window the listener catches up from the archive RPC, one
//! `getBlock` per slot and one `getTransaction` per host transaction, with
//! `archive_catch_up_active` at 1; the lag then shows its progress.
//! `failures_since_commit` keeps rising while one slot fails again and again, and a commit resets
//! it. `handle_check_failures_total` counts steps whose emitted handle this listener could not
//! re-derive, which means its software is wrong.

use std::{sync::LazyLock, time::Duration};

use prometheus::{
    register_int_counter_vec, register_int_gauge_vec, IntCounterVec,
    IntGaugeVec,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use tokio_util::sync::CancellationToken;
use tracing::warn;

use super::StartPosition;
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
        "Interrupted gRPC subscriptions and archive catch-ups the listener resumed from its checkpoint",
        &["host_chain_id"]
    )
    .unwrap()
});

static FAILURES_SINCE_COMMIT: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_solana_host_listener_failures_since_commit",
        "Interruptions the listener resumed from its checkpoint since it last committed a block",
        &["host_chain_id"]
    )
    .unwrap()
});

static ARCHIVE_CATCH_UP_ACTIVE: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_solana_host_listener_archive_catch_up_active",
        "1 while the listener rebuilds slots the stream can no longer replay from the archive RPC, else 0",
        &["host_chain_id"]
    )
    .unwrap()
});

static HANDLE_CHECK_FAILURES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_solana_host_listener_handle_check_failures_total",
        "fhe_execute steps whose emitted result handle the listener did not re-derive; each is held back as an errored computation",
        &["host_chain_id"]
    )
    .unwrap()
});

pub(super) fn record_applied(host_chain_id: u64, block: &SealedBlock) {
    let label = host_chain_id.to_string();
    FAILURES_SINCE_COMMIT.with_label_values(&[&label]).set(0);
    APPLIED_SLOT
        .with_label_values(&[&label])
        .set(block.slot as i64);
    if let Some(timestamp) = block.block_time {
        APPLIED_BLOCK_TIMESTAMP
            .with_label_values(&[&label])
            .set(timestamp);
    }
}

/// Exports the counters at zero, so `increase()` counts the first event, and on a resume the
/// committed checkpoint's slot, so the slot lag reads from the first scrape even if the listener
/// never applies another block.
pub(super) fn record_start(host_chain_id: u64, start: &StartPosition) {
    let label = host_chain_id.to_string();
    RECONNECTS.with_label_values(&[&label]);
    FAILURES_SINCE_COMMIT.with_label_values(&[&label]).set(0);
    ARCHIVE_CATCH_UP_ACTIVE.with_label_values(&[&label]).set(0);
    HANDLE_CHECK_FAILURES.with_label_values(&[&label]);
    if let StartPosition::Resume(checkpoint) = start {
        APPLIED_SLOT
            .with_label_values(&[&label])
            .set(checkpoint.slot as i64);
    }
}

pub(super) fn set_archive_catch_up(host_chain_id: u64, active: bool) {
    ARCHIVE_CATCH_UP_ACTIVE
        .with_label_values(&[&host_chain_id.to_string()])
        .set(i64::from(active));
}

pub(super) fn inc_reconnects(host_chain_id: u64) {
    let label = host_chain_id.to_string();
    RECONNECTS.with_label_values(&[&label]).inc();
    FAILURES_SINCE_COMMIT.with_label_values(&[&label]).inc();
}

pub(super) fn add_handle_check_failures(host_chain_id: u64, count: usize) {
    HANDLE_CHECK_FAILURES
        .with_label_values(&[&host_chain_id.to_string()])
        .inc_by(count as u64);
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
                warn!(error = %error, "confirmed-slot poll failed")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Failures accumulate until a block commits, so only a slot that keeps failing raises it.
    #[test]
    fn a_commit_resets_the_failures_since_commit() {
        let chain = 9_000_001;
        let gauge =
            FAILURES_SINCE_COMMIT.with_label_values(&[&chain.to_string()]);
        record_start(chain, &StartPosition::Tip);
        inc_reconnects(chain);
        inc_reconnects(chain);
        assert_eq!(gauge.get(), 2);
        record_applied(
            chain,
            &SealedBlock {
                slot: 7,
                block_hash: [7; 32],
                parent_slot: 6,
                parent_block_hash: [6; 32],
                block_time: None,
                block_height: None,
                executed_transaction_count: 0,
            },
        );
        assert_eq!(gauge.get(), 0);
    }
}
