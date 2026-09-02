//! ERC-20 `Transfer` event binding.
//!
//! The `sol!` macro generates a typed `Transfer` struct *and* the
//! `topic0` keccak hash from the JSON ABI signature — so we never hand-write
//! the topic constant. Downstream code matches logs on
//! `log.topics[0] == TRANSFER_TOPIC0` and decodes the payload via
//! [`decode_transfer`].

use alloy::sol;
use alloy::sol_types::SolEvent;
use alloy_primitives::{Address, B256, Log, LogData};
use consumer::BlockPayload;
use primitives::event::IndexedLog;
use tracing::{info, warn};

sol! {
    /// Subset of the ERC-20 ABI we care about. `sol!` will derive the
    /// `Transfer` struct, the `Transfer::SIGNATURE_HASH` constant
    /// (== keccak256("Transfer(address,address,uint256)")), and decoders.
    #[sol(rpc)]
    #[derive(Debug)]
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
    }
}

/// Topic0 for the ERC-20 `Transfer` event, derived from the ABI at compile time.
pub const TRANSFER_TOPIC0: B256 = IERC20::Transfer::SIGNATURE_HASH;

/// Decode an [`IndexedLog`] into a typed `Transfer` event.
///
/// Returns `None` if the topic count, layout, or data don't match. Caller
/// is expected to have already filtered by `topics[0] == TRANSFER_TOPIC0`,
/// but `decode_log` does that check itself when `validate = true`.
pub fn decode_transfer(log: &IndexedLog) -> Option<IERC20::Transfer> {
    let log = Log {
        address: log.address,
        data: LogData::new_unchecked(log.topics.clone(), log.data.clone()),
    };
    IERC20::Transfer::decode_log(&log).ok().map(|d| d.data)
}

/// Iterate a [`BlockPayload`], keep only logs from `token` where
/// `topics[0] == Transfer`, decode them, and log a human-readable line.
///
/// `tag` is the flow label (`LIVE`, `LIVE-CATCHUP`, `FINAL`,
/// `FINAL-CATCHUP`) — it appears as the `flow` field on every line, which is
/// what keeps the four pipelines visually separated in the output.
pub fn log_transfers(tag: &str, payload: &BlockPayload, token: Address) {
    info!(
        flow = tag,
        block = payload.block_number,
        txs = payload.transactions.len(),
        "block"
    );
    for tx in &payload.transactions {
        for log in &tx.logs {
            if log.address != token {
                continue;
            }
            if log.topics.first() != Some(&TRANSFER_TOPIC0) {
                continue;
            }
            match decode_transfer(log) {
                Some(t) => info!(
                    flow = tag,
                    block = payload.block_number,
                    tx = %tx.hash,
                    log_index = log.log_index,
                    from = %t.from,
                    to = %t.to,
                    value = %t.value,
                    "Transfer"
                ),
                None => warn!(
                    flow = tag,
                    tx = %tx.hash,
                    "log matched topic0 but failed to decode"
                ),
            }
        }
    }
}
