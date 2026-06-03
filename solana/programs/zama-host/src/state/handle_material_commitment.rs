//! On-chain account data for `HandleMaterialCommitment`.

use super::*;

/// Host-owned commitment proving ciphertext material is available for a handle.
///
/// ACL birth and material readiness are deliberately separate. KMS-style
/// decryption checks must verify both the canonical ACL record and this
/// material commitment before authorizing a decryptable handle.
#[account]
pub struct HandleMaterialCommitment {
    /// ACL record whose handle has committed material.
    pub acl_record: Pubkey,
    /// Handle copied from the ACL record at commitment time.
    pub handle: [u8; 32],
    /// Release/key identifier for the ciphertext material.
    pub key_id: [u8; 32],
    /// Digest of the ciphertext material.
    pub ciphertext_digest: [u8; 32],
    /// Digest of the SNS ciphertext material.
    pub sns_ciphertext_digest: [u8; 32],
    /// Release-pinned coprocessor-set digest.
    pub coprocessor_set_digest: [u8; 32],
    /// Canonical commitment hash over the material witness fields.
    pub material_commitment_hash: [u8; 32],
    /// Slot in which the commitment was recorded.
    pub created_slot: u64,
    /// Commitment state. Native-v0 decryptability requires committed.
    pub state: u8,
    /// PDA bump for this material commitment.
    pub bump: u8,
}

impl HandleMaterialCommitment {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + (32 * 5) + 8 + 1 + 1;
}
