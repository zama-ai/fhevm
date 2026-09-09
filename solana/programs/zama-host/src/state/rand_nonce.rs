//! On-chain account data for `RandNonce`.

use super::*;

/// The host's single random-seed nonce, created next to `HostConfig` by `initialize_host_config`
/// and incremented by every `fhe_execute` that contains a rand step. A monotonic host-owned
/// counter is a consumed ticket by itself: two executions can never see the same value, so a
/// rand seed can never repeat within a slot whatever the rest of the execution looks like.
///
/// Kept out of `HostConfig` on purpose: every execution reads the config, and write-locking it
/// would serialize them all. Only rand executions take this account, so ordinary writes stay
/// parallel and rand executions serialize on one account across applications.
#[account]
pub struct RandNonce {
    /// Value consumed by the next rand execution.
    pub nonce: u64,
    /// PDA bump for `PDA("rand-nonce")`.
    pub bump: u8,
}

impl RandNonce {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 8 + 1;
}
