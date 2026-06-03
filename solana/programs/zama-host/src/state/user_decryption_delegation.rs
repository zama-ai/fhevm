//! On-chain account data for `UserDecryptionDelegation`.

use super::*;

/// PoC user-decryption delegation witness.
///
/// Gateway/KMS payloads do not yet carry these records, but the account shape is
/// present so the final witness format has a concrete Solana state target.
#[account]
pub struct UserDecryptionDelegation {
    /// User granting delegated decrypt rights.
    pub delegator: Pubkey,
    /// Delegate allowed to request user decryption.
    pub delegate: Pubkey,
    /// App context for the delegation.
    pub app_account: Pubkey,
    /// Slot after which the delegation is invalid.
    pub expiration_slot: u64,
    /// Monotonic counter incremented on every grant, regrant, and revoke.
    pub delegation_counter: u64,
    /// Slot in which this row was last updated.
    pub last_update_slot: u64,
    /// Whether the delegation has been revoked by the delegator.
    pub revoked: bool,
    /// PDA bump for this delegation account.
    pub bump: u8,
}

impl UserDecryptionDelegation {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1;
}
