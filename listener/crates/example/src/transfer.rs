//! ERC-20 `Transfer` event binding.
//!
//! The `sol!` macro generates a typed `Transfer` struct *and* the
//! `topic0` keccak hash from the JSON ABI signature — so we never hand-write
//! the topic constant. Downstream code matches logs on
//! `log.topics[0] == TRANSFER_TOPIC0` and decodes the payload via
//! [`decode_transfer`].

use alloy::sol;
use alloy::sol_types::SolEvent;
use alloy_primitives::{B256, Log, LogData};
use primitives::event::IndexedLog;

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
