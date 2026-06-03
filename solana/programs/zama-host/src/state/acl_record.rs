//! On-chain account data for `AclRecord`.

use super::*;

/// Canonical ACL state for one host handle.
///
/// The account address is independent from the opaque handle:
/// `PDA("acl-record", nonce_key, nonce_sequence)`. The handle is stored in the
/// account body so computed handles can be bound after transaction accounts are
/// declared.
#[account]
pub struct AclRecord {
    /// Opaque FHEVM handle controlled by this ACL record.
    pub handle: [u8; 32],
    /// Domain-separated nonce key derived from app account metadata.
    pub nonce_key: [u8; 32],
    /// App-maintained sequence for this nonce key.
    pub nonce_sequence: u64,
    /// App-level ACL domain, such as a confidential token mint.
    pub acl_domain_key: Pubkey,
    /// App-owned account whose encrypted field is represented by this handle.
    pub app_account: Pubkey,
    /// Domain-separated encrypted field label inside `app_account`.
    pub encrypted_value_label: [u8; 32],
    /// Inline subjects for the common case.
    pub subjects: [Pubkey; MAX_ACL_SUBJECTS],
    /// Role flags parallel to [`AclRecord::subjects`].
    pub subject_roles: [u8; MAX_ACL_SUBJECTS],
    /// Number of valid entries in the inline subject arrays.
    pub subject_count: u8,
    /// Number of overflow subjects represented by [`AclPermission`] PDAs.
    pub overflow_subject_count: u32,
    /// Durable public-decrypt flag checked by KMS-style verification.
    pub public_decrypt: bool,
    /// Canonical material commitment PDA sealed to this ACL record.
    pub material_commitment: Pubkey,
    /// Canonical material commitment hash sealed to this ACL record.
    pub material_commitment_hash: [u8; 32],
    /// Material key identifier sealed to this ACL record.
    pub material_key_id: [u8; 32],
    /// Slot in which this ACL record was first bound.
    pub created_slot: u64,
    /// PDA bump for this record.
    pub bump: u8,
}

impl AclRecord {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32
        + 32
        + 8
        + 32
        + 32
        + 32
        + (32 * MAX_ACL_SUBJECTS)
        + MAX_ACL_SUBJECTS
        + 1
        + 4
        + 1
        + 32
        + 32
        + 32
        + 8
        + 1;

    /// Returns the inline subject index if the subject is embedded in the ACL record.
    pub fn inline_subject_index(&self, subject: Pubkey) -> Option<usize> {
        let subject_count = (self.subject_count as usize).min(MAX_ACL_SUBJECTS);
        self.subjects[..subject_count]
            .iter()
            .position(|candidate| *candidate == subject)
    }

    /// Returns true when an inline subject has every flag in `role`.
    pub fn inline_subject_has_role(&self, subject: Pubkey, role: u8) -> bool {
        self.inline_subject_index(subject)
            .map(|index| subject_has_role(self.subject_roles[index], role))
            .unwrap_or(false)
    }
}
