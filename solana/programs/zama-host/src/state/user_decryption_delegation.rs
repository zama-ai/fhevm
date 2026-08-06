//! On-chain account data for `UserDecryptionDelegation`.

use super::*;

/// PoC user-decryption delegation witness.
///
/// Gateway/KMS payloads do not yet carry these records, but the account shape is
/// present so the final witness format has a concrete Solana state target.
///
/// Freshness contract for the future KMS consumer (stated here so the integration
/// doesn't have to reverse-engineer it): `delegation_counter` is a strictly monotonic
/// version — of two snapshots of the same record, the higher counter is authoritative —
/// and `last_update_slot` lets a reader require the witness be at least as fresh as a
/// slot it has observed. Writes require `last_update_slot < current slot`, so a record
/// mutates at most once per slot and every `(delegation_counter, last_update_slot)`
/// pair is unambiguous.
#[account]
pub struct UserDecryptionDelegation {
    /// User granting delegated decrypt rights.
    pub delegator: Pubkey,
    /// Delegate allowed to request user decryption.
    pub delegate: Pubkey,
    /// The encrypted value account authority the delegation is scoped over. A delegation covers
    /// every value of that authority in every domain: the domain is not one of the PDA's seeds.
    pub encrypted_value_account_authority: Pubkey,
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
