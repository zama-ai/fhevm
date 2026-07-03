//! On-chain account data for `ConfidentialMint`.

use anchor_lang::prelude::*;

/// Confidential mint state for the token PoC.
///
/// The total supply's current handle and access lineage live in the
/// `EncryptedValue` account at `total_supply_encrypted_value`.
#[account]
#[derive(InitSpace)]
pub struct ConfidentialMint {
    /// Admin/authority that created the mint.
    pub authority: Pubkey,
    /// ACL domain key, currently equal to the mint pubkey.
    pub acl_domain_key: Pubkey,
    /// Program-controlled compute signer PDA.
    pub compute_signer: Pubkey,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Pubkey,
    /// Decimal precision inherited from the underlying mint.
    pub decimals: u8,
    /// `EncryptedValue` lineage PDA holding the current total-supply handle.
    pub total_supply_encrypted_value: Pubkey,
}

impl ConfidentialMint {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 1 + 32;
}
