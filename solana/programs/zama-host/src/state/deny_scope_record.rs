//! On-chain account data for `DenyScopeRecord`.

use super::*;

/// Optional deny-list record for one application `(program, scope)`. A denied application can
/// neither compute nor allow: `fhe_execute` and `make_handle_public` fail closed on it, the way
/// the EVM ACL blocks a denied `msg.sender` on `allow`.
#[account]
pub struct DenyScopeRecord {
    /// The application program this record governs.
    pub program: Pubkey,
    /// The program-declared scope this record governs.
    pub scope: [u8; 32],
    /// Whether `(program, scope)` is currently denied.
    pub denied: bool,
    /// PDA bump for `PDA("deny-scope", program, scope)`.
    pub bump: u8,
}

impl DenyScopeRecord {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 1 + 1;
}
