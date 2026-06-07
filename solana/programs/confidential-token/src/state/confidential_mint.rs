//! On-chain account data for `ConfidentialMint`.

use anchor_lang::prelude::*;

/// Confidential mint state for the token PoC.
#[account]
pub struct ConfidentialMint {
    /// Admin/authority that created the mint.
    pub authority: Pubkey,
    /// ACL domain key, currently equal to the mint pubkey.
    pub acl_domain_key: Pubkey,
    /// Program-controlled compute signer PDA.
    pub compute_signer: Pubkey,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Pubkey,
    /// Active threshold verifier set accepted for KMS disclosure certificates.
    pub disclosure_verifier_set: Pubkey,
    /// Active threshold verifier set accepted for KMS burn-redemption certificates.
    pub redemption_verifier_set: Pubkey,
    /// Decimal precision inherited from the underlying mint.
    pub decimals: u8,
    /// Current encrypted total-supply handle.
    pub total_supply_handle: [u8; 32],
    /// Current ZamaHost ACL record for `total_supply_handle`.
    pub total_supply_acl_record: Pubkey,
    /// Next nonce sequence to use for a total-supply ACL record.
    pub next_total_supply_nonce_sequence: u64,
}

impl ConfidentialMint {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 32 + 1 + 32 + 32 + 8;
}

/// Legacy mint layout used before token disclosure/redemption verifier sets split.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct LegacyConfidentialMintV1 {
    /// Admin/authority that created the mint.
    pub authority: Pubkey,
    /// ACL domain key, currently equal to the mint pubkey.
    pub acl_domain_key: Pubkey,
    /// Program-controlled compute signer PDA.
    pub compute_signer: Pubkey,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Pubkey,
    /// Legacy Ed25519 authority accepted for KMS disclosure response certificates.
    pub kms_verifier_authority: Pubkey,
    /// Decimal precision inherited from the underlying mint.
    pub decimals: u8,
    /// Current encrypted total-supply handle.
    pub total_supply_handle: [u8; 32],
    /// Current ZamaHost ACL record for `total_supply_handle`.
    pub total_supply_acl_record: Pubkey,
    /// Next nonce sequence to use for a total-supply ACL record.
    pub next_total_supply_nonce_sequence: u64,
}

impl LegacyConfidentialMintV1 {
    /// Serialized size of the legacy account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 1 + 32 + 32 + 8;
}
