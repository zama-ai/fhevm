//! On-chain account data for `RandNonce`.

use super::*;

/// An application's random-seed nonce, incremented by every `fhe_execute` of that application
/// that contains a rand step. A monotonic host-owned counter is a consumed ticket by itself: two
/// executions of one application never see the same value, and the seed also binds the
/// application, so a rand seed can never repeat within a slot whatever the rest of the
/// execution looks like.
///
/// One per `(program, scope)`, at `PDA("rand-nonce", program, scope)`: rand executions of
/// different applications do not serialize on a shared account, and those of one application do
/// (DD-057). Created by the application's first rand execution, which pays its rent. Only the
/// `admin-sweep` wipe of preview environments closes it, so in production the counter cannot
/// restart.
#[account]
pub struct RandNonce {
    /// Value consumed by the application's next rand execution.
    pub nonce: u64,
}

impl RandNonce {
    /// Serialized size of the account body, excluding the Anchor discriminator.
    pub const SPACE: usize = 8;
}
