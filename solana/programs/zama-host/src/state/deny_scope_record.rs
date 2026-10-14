//! On-chain account data for `DenyScopeRecord`.

use super::*;

/// Optional deny-list record for one application `(program, scope)`. A denied application can
/// neither compute nor allow: `fhe_execute` and `make_handle_public` fail closed on it, the way
/// the EVM ACL blocks a denied `msg.sender` on `allow`.
#[account]
#[derive(InitSpace)]
pub struct DenyScopeRecord {
    /// The application program this record governs.
    pub program: Pubkey,
    /// The scope this record governs: an account of `program`.
    pub scope: Pubkey,
    /// Whether `(program, scope)` is currently denied.
    pub denied: bool,
    /// PDA bump for `PDA("deny-scope", program, scope)`.
    pub bump: u8,
}
