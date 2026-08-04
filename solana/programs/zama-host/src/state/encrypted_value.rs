//! On-chain account data for `EncryptedValue` (RFC-024).
//!
//! Replaces the keyed-nonce ACL model: one encrypted value account per encrypted
//! value, reused across every handle update, carrying a compact MMR history
//! instead of a fresh PDA per creation. Field order follows
//! `zama_solana_acl::EncryptedValue`, so the shared crate's discriminator,
//! size formula, and MMR helpers apply directly.

use super::*;

/// Canonical ACL + history state for one encrypted value account.
///
/// PDA: `[ENCRYPTED_VALUE_SEED, encrypted_value_id]` where `encrypted_value_id =
/// zama_solana_acl::derive_encrypted_value_id(domain, encrypted_value_account_authority, label)`.
/// The account name must stay exactly `EncryptedValue` — Anchor derives the
/// discriminator from the type name, and it must match
/// `zama_solana_acl::encrypted_value_discriminator()`.
#[account]
pub struct EncryptedValue {
    /// App-level ACL domain, such as a confidential token mint.
    pub domain: Pubkey,
    /// The account that controls this encrypted value: it must sign to create it, update its
    /// handle, or replace its subject list. Enforced by address rather than by comparing this
    /// field — the signer must equal the authority declared in the execution
    /// (`assert_output_acl_metadata`) and the account written to must be the PDA rederived from
    /// that declared triple, which is what ties the signer to the stored value on update. For a
    /// token balance this is the token account itself. It is not the sole controller of the
    /// audience — any current subject may also add or remove subjects through `allow_subjects` /
    /// `remove_subject`.
    pub encrypted_value_account_authority: Pubkey,
    /// The encrypted value label: the third component of the encrypted value ID, naming which
    /// encrypted value of the authority this is.
    pub label: [u8; 32],
    /// Current encrypted value identifier (the live handle).
    pub current_handle: [u8; 32],
    /// Current persistent subjects. Membership in this set is the whole ACL.
    pub subjects: Vec<Pubkey>,
    /// Number of MMR leaves appended; `0` means no history.
    pub leaf_count: u64,
    /// MMR peaks, oldest mountain first (`popcount(leaf_count)` entries).
    pub peaks: Vec<[u8; 32]>,
    /// PDA bump.
    pub bump: u8,
}

impl EncryptedValue {
    /// Anchor account body size (excludes the 8-byte discriminator), for an
    /// encrypted value account with `subjects_len` subjects and `peaks_len` peaks.
    pub fn space(subjects_len: usize, peaks_len: usize) -> usize {
        zama_solana_acl::EncryptedValue::account_size(subjects_len, peaks_len) - 8
    }

    /// The encrypted value account's encrypted value ID — its PDA seed. Derived, never stored.
    pub fn encrypted_value_id(&self) -> [u8; 32] {
        zama_solana_acl::derive_encrypted_value_id(
            self.domain.to_bytes(),
            self.encrypted_value_account_authority.to_bytes(),
            self.label,
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
            domain: self.domain.to_bytes(),
            encrypted_value_account_authority: self.encrypted_value_account_authority.to_bytes(),
            label: self.label,
            current_handle: self.current_handle,
            subjects: self.subjects.iter().map(|p| p.to_bytes()).collect(),
            leaf_count: self.leaf_count,
            peaks: self.peaks.clone(),
            bump: self.bump,
        }
    }
}

/// Returns the canonical `EncryptedValue` PDA address for an encrypted value ID.
pub fn encrypted_value_address(encrypted_value_id: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            zama_solana_acl::ENCRYPTED_VALUE_SEED,
            encrypted_value_id.as_ref(),
        ],
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
