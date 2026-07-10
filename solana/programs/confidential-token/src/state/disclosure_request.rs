//! On-chain account data for KMS-facing disclosure request witnesses.

use super::*;
use solana_sha256_hasher::hashv;

/// Balance disclosure request mode.
pub const DISCLOSURE_REQUEST_MODE_BALANCE: u8 = 1;
/// Token amount disclosure request mode.
pub const DISCLOSURE_REQUEST_MODE_AMOUNT: u8 = 2;

/// Request is awaiting a KMS response certificate.
pub const REQUEST_STATUS_PENDING: u8 = 1;
/// Request has been consumed by a successful response instruction.
pub const REQUEST_STATUS_CONSUMED: u8 = 2;

/// Account-backed witness for a public disclosure request.
#[account]
#[derive(InitSpace)]
pub struct DisclosureRequest {
    /// Confidential mint whose ACL domain scopes the request.
    pub mint: Pubkey,
    /// User/app authority that requested disclosure.
    pub requester: Pubkey,
    /// Balance token account for balance mode, or default for amount mode.
    pub token_account: Pubkey,
    /// ACL app account stored in the host `EncryptedValue` lineage.
    pub app_account: Pubkey,
    /// Requested handle.
    pub handle: [u8; 32],
    /// `EncryptedValue` lineage for the requested handle.
    pub encrypted_value: Pubkey,
    /// Host config whose chain id and gates were validated.
    pub host_config: Pubkey,
    /// KMS context id pinned at request time; the response cert must verify
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
    /// Disclosure request mode.
    pub mode: u8,
    /// Request lifecycle state.
    pub status: u8,
    /// PDA bump for this request account.
    pub bump: u8,
}

impl DisclosureRequest {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 9) + (8 * 3) + 1 + 1 + 1;
}

/// Returns the canonical PDA for a disclosure request witness.
pub fn disclosure_request_address(
    mint: Pubkey,
    requester: Pubkey,
    handle: [u8; 32],
    request_nonce: [u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"disclosure-request",
            mint.as_ref(),
            requester.as_ref(),
            handle.as_ref(),
            request_nonce.as_ref(),
        ],
        &crate::ID,
    )
}

/// Canonical request hash used by KMS-facing disclosure witnesses.
#[allow(clippy::too_many_arguments)]
pub fn disclosure_request_hash(
    program_id: Pubkey,
    request_account: Pubkey,
    mint: Pubkey,
    requester: Pubkey,
    token_account: Pubkey,
    app_account: Pubkey,
    handle: [u8; 32],
    encrypted_value: Pubkey,
    host_config: Pubkey,
    kms_context_id: u64,
    request_nonce: [u8; 32],
    chain_id: u64,
    expires_slot: u64,
    mode: u8,
) -> [u8; 32] {
    hashv(&[
        b"zama-confidential-token-disclosure-request-v1",
        program_id.as_ref(),
        request_account.as_ref(),
        mint.as_ref(),
        requester.as_ref(),
        token_account.as_ref(),
        app_account.as_ref(),
        handle.as_ref(),
        encrypted_value.as_ref(),
        host_config.as_ref(),
        &kms_context_id.to_le_bytes(),
        request_nonce.as_ref(),
        &chain_id.to_le_bytes(),
        &expires_slot.to_le_bytes(),
        &[mode],
    ])
    .to_bytes()
}
