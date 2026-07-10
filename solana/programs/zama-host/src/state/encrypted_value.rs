//! On-chain account data for `EncryptedValue` (RFC-024).
//!
//! Replaces the keyed-nonce ACL model: one account per encrypted-value
//! lineage, reused across every handle update, carrying a compact MMR history
//! instead of a fresh PDA per birth. Field order follows
//! `zama_solana_acl::EncryptedValue`, so the shared crate's discriminator,
//! size formula, and MMR helpers apply directly.

use super::*;

/// Canonical ACL + history state for one encrypted-value lineage.
///
/// PDA: `[ENCRYPTED_VALUE_SEED, value_key]` where `value_key =
/// zama_solana_acl::derive_value_key(acl_domain_key, app_account, encrypted_value_label)`.
/// The account name must stay exactly `EncryptedValue` — Anchor derives the
/// discriminator from the type name, and it must match
/// `zama_solana_acl::encrypted_value_discriminator()`.
#[account]
pub struct EncryptedValue {
    /// App-level ACL domain, such as a confidential token mint.
    pub acl_domain_key: Pubkey,
    /// App-owned account whose encrypted field this lineage represents.
    pub app_account: Pubkey,
    /// Domain-separated encrypted field label inside `app_account`.
    pub encrypted_value_label: [u8; 32],
    /// Current encrypted value identifier (the live handle).
    pub current_handle: [u8; 32],
    /// Current durable subjects. Membership in this set is the whole ACL.
    pub subjects: Vec<Pubkey>,
    /// Number of MMR leaves appended; `0` means no history.
    pub leaf_count: u64,
    /// MMR peaks, oldest mountain first (`popcount(leaf_count)` entries).
    pub peaks: Vec<[u8; 32]>,
    /// PDA bump.
    pub bump: u8,
}

impl EncryptedValue {
    /// Anchor account body size (excludes the 8-byte discriminator), for a
    /// lineage with `subjects_len` subjects and `peaks_len` peaks.
    pub fn space(subjects_len: usize, peaks_len: usize) -> usize {
        zama_solana_acl::EncryptedValue::account_size(subjects_len, peaks_len) - 8
    }

    /// The lineage's value key — its PDA seed. Derived, never stored.
    pub fn value_key(&self) -> [u8; 32] {
        zama_solana_acl::derive_value_key(
            self.acl_domain_key.to_bytes(),
            self.app_account.to_bytes(),
            self.encrypted_value_label,
        )
    }

    /// Returns the subject's index, if it is a current member.
    pub fn subject_index(&self, subject: Pubkey) -> Option<usize> {
        self.subjects
            .iter()
            .position(|candidate| *candidate == subject)
    }

    /// Returns true when `subject` is a current allowed member.
    pub fn has_subject(&self, subject: Pubkey) -> bool {
        self.subject_index(subject).is_some()
    }

    /// Converts to the shared crate's wire type for MMR/authorization helpers.
    pub fn to_shared(&self) -> zama_solana_acl::EncryptedValue {
        zama_solana_acl::EncryptedValue {
            acl_domain_key: self.acl_domain_key.to_bytes(),
            app_account: self.app_account.to_bytes(),
            encrypted_value_label: self.encrypted_value_label,
            current_handle: self.current_handle,
            subjects: self.subjects.iter().map(|p| p.to_bytes()).collect(),
            leaf_count: self.leaf_count,
            peaks: self.peaks.clone(),
            bump: self.bump,
        }
    }
}

/// Returns the canonical `EncryptedValue` PDA address for a value key.
pub fn encrypted_value_address(value_key: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[zama_solana_acl::ENCRYPTED_VALUE_SEED, value_key.as_ref()],
        &crate::ID,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::Discriminator;

    /// The Anchor-derived discriminator for `EncryptedValue` must match the
    /// shared crate's `sha256("account:EncryptedValue")[..8]`, since the
    /// off-chain KMS/relayer decode account data with the shared crate alone.
    #[test]
    fn discriminator_matches_shared_crate() {
        assert_eq!(
            EncryptedValue::DISCRIMINATOR,
            zama_solana_acl::encrypted_value_discriminator()
        );
    }
}
