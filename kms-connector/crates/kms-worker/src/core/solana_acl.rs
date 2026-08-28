//! Solana ACL verification helpers for the KMS connector.
//!
//! RFC-024 replaced the old keyed-nonce ACL and material-commitment on-chain accounts
//! with a single `EncryptedValue` encrypted value account (see
//! [`super::solana_encrypted_value_acl`]): those account types no longer exist on-chain, so the
//! byte-offset decoders that used to read them were deleted from this module along with them —
//! decoding them would read garbage from a nonexistent layout, not merely dead code.
//!
//! Material commitments (ciphertext digest / key id binding) are no longer an on-chain Solana ACL
//! concern at all: that check now lives solely in the gateway's `CiphertextCommits` contract,
//! enforced off-chain in the KMS connector by `ciphertext_attestation::consensus` (see
//! `event_processor::ciphertext::manager::CiphertextManager`), which every decryption request path
//! (EVM and Solana alike) already runs before this ACL check. `HandleMaterialCommitmentWitness`
//! and `verify_material_commitment` are deleted, not reimplemented.
//!
//! What remains here: the delegation witness and its layout decoder (a Solana-specific feature
//! with no `EncryptedValue` equivalent, decoded from the snapshot by [`super::solana::delegation`]),
//! and the shared pubkey/handle byte types + the verifier that [`super::solana_encrypted_value_acl`]
//! uses for the current/historical/public decrypt paths. Delegation freshness is decided against
//! the observed snapshot by [`super::solana::delegation`], not by a standalone verifier here.

use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;
use thiserror::Error;

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

// The record layout, its decoder and the wildcard sentinel live in the shared crate, so every
// off-chain reader (this connector's authoritative check, the relayer's advisory pre-check)
// decodes the same bytes through one implementation.
pub use zama_solana_acl::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY;
pub use zama_solana_acl::delegation::DELEGATION_SEED;

pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
const ANCHOR_DISCRIMINATOR_LEN: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDecryptionDelegationWitness {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub delegator: SolanaPubkeyBytes,
    pub delegate: SolanaPubkeyBytes,
    pub encrypted_value_account_authority: SolanaPubkeyBytes,
    pub expiration_slot: u64,
    pub delegation_counter: u64,
    pub last_update_slot: u64,
    pub revoked: bool,
    pub bump: u8,
}

impl UserDecryptionDelegationWitness {
    /// The shared crate's liveness rule, asked of this witness — the boundary (revocation
    /// wins; the expiration slot itself is inside the life) has exactly one definition, in
    /// `zama-solana-acl`, so this reader cannot drift from the relayer's on it. The witness is
    /// the decoded record plus provenance, so rebuilding the record is a field-for-field copy.
    pub fn is_live_at(&self, slot: u64) -> bool {
        zama_solana_acl::UserDecryptionDelegationRecord {
            delegator: self.delegator,
            delegate: self.delegate,
            encrypted_value_account_authority: self.encrypted_value_account_authority,
            expiration_slot: self.expiration_slot,
            delegation_counter: self.delegation_counter,
            last_update_slot: self.last_update_slot,
            revoked: self.revoked,
            bump: self.bump,
        }
        .is_live_at(slot)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaAclVerifier {
    pub host_program_id: SolanaPubkeyBytes,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaAclVerificationError {
    #[error("ACL account is not owned by the configured ZamaHost program")]
    InvalidAccountOwner,
    #[error("account data length does not match the expected Anchor layout")]
    AccountDataLengthMismatch,
    #[error("account discriminator does not match the expected Anchor account type")]
    AccountDiscriminatorMismatch,
    #[error("account data contains invalid field values")]
    InvalidAccountData,
    #[error("encrypted value account's current_handle does not match the requested handle")]
    EncryptedValueHandleMismatch,
    #[error("subject is not a current member of the encrypted value account")]
    EncryptedValueSubjectMissing,
    #[error("encrypted value account is not the canonical PDA for its encrypted value ID")]
    NonCanonicalEncryptedValueAcl,
    #[error("encrypted value account bump does not match the canonical PDA bump")]
    EncryptedValueAclBumpMismatch,
    #[error("historical-access MMR proof failed to verify against the live peaks")]
    HistoricalAccessProofInvalid,
    #[error("public-decrypt MMR proof failed to verify against the live peaks")]
    PublicDecryptProofInvalid,
    #[error("encrypted value account MMR state (peaks/leaf_count) is internally inconsistent")]
    MmrStateInconsistent,
    #[error("domain is outside the signed authorization scope")]
    DomainNotAllowed,
}

impl SolanaAclVerifier {
    pub fn new(host_program_id: SolanaPubkeyBytes) -> Self {
        Self { host_program_id }
    }
}

pub fn decode_user_decryption_delegation_witness(
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: &[u8],
) -> Result<UserDecryptionDelegationWitness, SolanaAclVerificationError> {
    let record =
        zama_solana_acl::decode_user_decryption_delegation(data).map_err(|error| match error {
            zama_solana_acl::AclError::BadDiscriminator => {
                SolanaAclVerificationError::AccountDiscriminatorMismatch
            }
            zama_solana_acl::AclError::BadAccountData => {
                SolanaAclVerificationError::InvalidAccountData
            }
            // The remaining variants belong to the encrypted value account's authorization
            // rules; the delegation decoder cannot produce them, and enumerating them keeps a
            // new one from landing here silently.
            zama_solana_acl::AclError::MmrInconsistent
            | zama_solana_acl::AclError::MmrPeakCapacityExceeded
            | zama_solana_acl::AclError::SubjectCapacityExceeded
            | zama_solana_acl::AclError::HandleMismatch
            | zama_solana_acl::AclError::SubjectMissing
            | zama_solana_acl::AclError::HistoricalProofInvalid
            | zama_solana_acl::AclError::PublicDecryptProofInvalid => {
                SolanaAclVerificationError::InvalidAccountData
            }
        })?;
    Ok(UserDecryptionDelegationWitness {
        account_key,
        owner,
        delegator: record.delegator,
        delegate: record.delegate,
        encrypted_value_account_authority: record.encrypted_value_account_authority,
        expiration_slot: record.expiration_slot,
        delegation_counter: record.delegation_counter,
        last_update_slot: record.last_update_slot,
        revoked: record.revoked,
        bump: record.bump,
    })
}

pub fn host_config_address(host_program_id: SolanaPubkeyBytes) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(&[HOST_CONFIG_SEED], &host_program_id);
    (address.to_bytes(), bump)
}

pub fn user_decryption_delegation_address(
    host_program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    encrypted_value_account_authority: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            encrypted_value_account_authority.as_ref(),
        ],
        &host_program_id,
    );
    (address.to_bytes(), bump)
}

pub fn anchor_account_discriminator(account_name: &str) -> [u8; ANCHOR_DISCRIMINATOR_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(b"account:");
    hasher.update(account_name.as_bytes());
    let digest = hasher.finalize();
    let mut discriminator = [0; ANCHOR_DISCRIMINATOR_LEN];
    discriminator.copy_from_slice(&digest[..ANCHOR_DISCRIMINATOR_LEN]);
    discriminator
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const AUTHORITY: SolanaPubkeyBytes = [2; 32];
    const DELEGATE: SolanaPubkeyBytes = [5; 32];
    const OBSERVED_SLOT: u64 = 500;

    fn delegation_for_authority(
        encrypted_value_account_authority: SolanaPubkeyBytes,
    ) -> UserDecryptionDelegationWitness {
        let (account_key, bump) = user_decryption_delegation_address(
            HOST_PROGRAM_ID,
            OWNER,
            DELEGATE,
            encrypted_value_account_authority,
        );
        UserDecryptionDelegationWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            delegator: OWNER,
            delegate: DELEGATE,
            encrypted_value_account_authority,
            expiration_slot: OBSERVED_SLOT + 20,
            delegation_counter: 9,
            last_update_slot: OBSERVED_SLOT - 1,
            revoked: false,
            bump,
        }
    }

    fn delegation() -> UserDecryptionDelegationWitness {
        delegation_for_authority(AUTHORITY)
    }

    fn encode_delegation(delegation: &UserDecryptionDelegationWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("UserDecryptionDelegation").to_vec();
        data.extend_from_slice(&delegation.delegator);
        data.extend_from_slice(&delegation.delegate);
        data.extend_from_slice(&delegation.encrypted_value_account_authority);
        data.extend_from_slice(&delegation.expiration_slot.to_le_bytes());
        data.extend_from_slice(&delegation.delegation_counter.to_le_bytes());
        data.extend_from_slice(&delegation.last_update_slot.to_le_bytes());
        data.push(delegation.revoked as u8);
        data.push(delegation.bump);
        data
    }

    #[test]
    fn decodes_anchor_delegation_account_data() {
        let delegation = delegation();
        let decoded = decode_user_decryption_delegation_witness(
            delegation.account_key,
            HOST_PROGRAM_ID,
            &encode_delegation(&delegation),
        )
        .expect("delegation decodes");
        assert_eq!(decoded, delegation);
    }

    #[test]
    fn rejects_invalid_anchor_account_data() {
        let mut invalid_bool = encode_delegation(&delegation());
        let revoked_offset = invalid_bool.len() - 2;
        invalid_bool[revoked_offset] = 2;
        assert_eq!(
            decode_user_decryption_delegation_witness([0; 32], HOST_PROGRAM_ID, &invalid_bool),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );
    }
}
