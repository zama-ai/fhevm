//! On-chain account data for `RandNonce`.

use super::*;

/// An application's random-seed nonce, incremented by every `fhe_execute` of that application
/// that contains a rand step. A monotonic host-owned counter is a consumed ticket by itself: two
/// executions of one application never see the same value, and the seed also binds the
/// application, so a rand seed can never repeat within a slot whatever the rest of the
/// execution looks like.
///
/// One per `(program, scope)`, so rand executions of different applications do not serialize on
/// a shared account; those of one application already share its Stores. Lazy-created by the first
/// rand execution, which pays its rent, and never closed, so the counter cannot restart.
#[account]
pub struct RandNonce {
    /// Value consumed by the application's next rand execution.
    pub nonce: u64,
    /// PDA bump for `PDA("rand-nonce", program, scope)`.
    pub bump: u8,
}

impl RandNonce {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 8 + 1;
}
