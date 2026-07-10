//! On-chain account data for KMS-facing burn-redemption request witnesses.

use super::*;
use solana_sha256_hasher::hashv;

/// Account-backed witness for a burned-amount redemption request.
#[account]
#[derive(InitSpace)]
pub struct BurnRedemptionRequest {
    /// Confidential mint whose vault backs the redemption.
    pub mint: Pubkey,
    /// Token owner that requested redemption.
    pub owner: Pubkey,
    /// Token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Underlying SPL mint for the vault and destination.
    pub underlying_mint: Pubkey,
    /// Destination token account owner.
    pub destination_owner: Pubkey,
    /// Destination token account to receive redeemed underlying tokens.
    pub destination_account: Pubkey,
    /// Burned amount handle.
    pub burned_handle: [u8; 32],
    /// `EncryptedValue` lineage for `burned_handle`.
    pub burned_encrypted_value: Pubkey,
    /// Host config whose chain id and gates were validated.
    pub host_config: Pubkey,
    /// KMS context id pinned at request time; the redemption cert must verify
    /// against this context's signer set, not the current one.
    pub kms_context_id: u64,
    /// Caller-supplied nonce that makes the request PDA unique.
    pub request_nonce: [u8; 32],
    /// Canonical hash over this request witness.
    pub request_hash: [u8; 32],
    /// Host chain id copied from the validated host config.
    pub chain_id: u64,
    /// Last slot in which this request can be consumed.
    pub expires_slot: u64,
    /// Request lifecycle state.
    pub status: u8,
    /// PDA bump for this request account.
    pub bump: u8,
}

impl BurnRedemptionRequest {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 11) + (8 * 3) + 1 + 1;
}

/// Returns the canonical PDA for a burn-redemption request witness.
pub fn burn_redemption_request_address(
    mint: Pubkey,
    owner: Pubkey,
    burned_handle: [u8; 32],
    request_nonce: [u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"burn-redemption-request",
            mint.as_ref(),
            owner.as_ref(),
            burned_handle.as_ref(),
            request_nonce.as_ref(),
        ],
        &crate::ID,
    )
}

/// Canonical request hash used by KMS-facing burn-redemption witnesses.
#[allow(clippy::too_many_arguments)]
pub fn burn_redemption_request_hash(
    program_id: Pubkey,
    request_account: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    token_account: Pubkey,
    underlying_mint: Pubkey,
    destination_owner: Pubkey,
    destination_account: Pubkey,
    burned_handle: [u8; 32],
    burned_encrypted_value: Pubkey,
    host_config: Pubkey,
    kms_context_id: u64,
    request_nonce: [u8; 32],
    chain_id: u64,
    expires_slot: u64,
) -> [u8; 32] {
    hashv(&[
        b"zama-confidential-token-burn-redemption-request-v1",
        program_id.as_ref(),
        request_account.as_ref(),
        mint.as_ref(),
        owner.as_ref(),
        token_account.as_ref(),
        underlying_mint.as_ref(),
        destination_owner.as_ref(),
        destination_account.as_ref(),
        burned_handle.as_ref(),
        burned_encrypted_value.as_ref(),
        host_config.as_ref(),
        &kms_context_id.to_le_bytes(),
        request_nonce.as_ref(),
        &chain_id.to_le_bytes(),
        &expires_slot.to_le_bytes(),
    ])
    .to_bytes()
}
