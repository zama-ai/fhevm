//! Solana ACL verification helpers for the KMS connector.
//!
//! RFC-024 replaced the keyed-nonce `AclRecord`/`AclPermission`/`HandleMaterialCommitment`
//! on-chain accounts with a single `EncryptedValue` lineage account (see
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
//! What remains here: the delegation witness (untouched — a Solana-specific feature with no
//! `EncryptedValue` equivalent) and the shared pubkey/handle byte types + verifier used by both
//! the delegation path and the [`super::solana_encrypted_value_acl`] current/historical/public
//! decrypt paths.

use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;
use thiserror::Error;

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
pub const WILDCARD_APP_CONTEXT: SolanaPubkeyBytes = [0xff; 32];
const ANCHOR_DISCRIMINATOR_LEN: usize = 8;
const USER_DECRYPTION_DELEGATION_SPACE: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDecryptionDelegationWitness {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub delegator: SolanaPubkeyBytes,
    pub delegate: SolanaPubkeyBytes,
    pub app_account: SolanaPubkeyBytes,
    pub expiration_slot: u64,
    pub delegation_counter: u64,
    pub last_update_slot: u64,
    pub revoked: bool,
    pub bump: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaAclVerifier {
    pub host_program_id: SolanaPubkeyBytes,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaAclVerificationError {
    #[error("ACL account is not owned by the configured ZamaHost program")]
    InvalidAccountOwner,
    #[error(
        "delegation account is not the canonical PDA for the delegator, delegate, and app account"
    )]
    NonCanonicalDelegation,
    #[error("delegation bump does not match the canonical PDA bump")]
    DelegationBumpMismatch,
    #[error(
        "delegation account data does not match the requested delegator, delegate, or app account"
    )]
    DelegationMismatch,
    #[error("delegation counter does not match the signed request")]
    DelegationCounterMismatch,
    #[error("delegation is revoked, expired, or newer than the observed slot")]
    DelegationNotActive,
    #[error("account data length does not match the expected Anchor layout")]
    AccountDataLengthMismatch,
    #[error("account discriminator does not match the expected Anchor account type")]
    AccountDiscriminatorMismatch,
    #[error("account data contains invalid field values")]
    InvalidAccountData,
    #[error("encrypted-value lineage's current_handle does not match the requested handle")]
    EncryptedValueHandleMismatch,
    #[error("subject is not a current member of the encrypted-value lineage")]
    EncryptedValueSubjectMissing,
    #[error("encrypted-value lineage account is not the canonical PDA for its value key")]
    NonCanonicalEncryptedValueAcl,
    #[error("encrypted-value lineage bump does not match the canonical PDA bump")]
    EncryptedValueAclBumpMismatch,
    #[error("historical-access MMR proof failed to verify against the live peaks")]
    HistoricalAccessProofInvalid,
    #[error("public-decrypt MMR proof failed to verify against the live peaks")]
    PublicDecryptProofInvalid,
    #[error("encrypted-value lineage MMR state (peaks/leaf_count) is internally inconsistent")]
    MmrStateInconsistent,
    #[error("domain is outside the signed authorization scope")]
    DomainNotAllowed,
}

impl SolanaAclVerifier {
    pub fn new(host_program_id: SolanaPubkeyBytes) -> Self {
        Self { host_program_id }
    }

    /// Verifies a user-decryption delegation: `delegator` granted `delegate` standing authority
    /// over `app_account`, active as of `observed_slot`. Untouched by the RFC-024 `EncryptedValue`
    /// migration — delegation is orthogonal to the lineage ACL check, which the caller runs
    /// separately (with `delegator` as the subject) via [`super::solana_encrypted_value_acl`].
    pub fn verify_delegation(
        &self,
        delegation: &UserDecryptionDelegationWitness,
        delegator: SolanaPubkeyBytes,
        delegate: SolanaPubkeyBytes,
        app_account: SolanaPubkeyBytes,
        expected_delegation_counter: u64,
        observed_slot: u64,
    ) -> Result<(), SolanaAclVerificationError> {
        if delegation.owner != self.host_program_id {
            return Err(SolanaAclVerificationError::InvalidAccountOwner);
        }
        if delegator == [0; 32]
            || delegate == [0; 32]
            || app_account == [0; 32]
            || delegate == WILDCARD_APP_CONTEXT
            || delegator == delegate
            || delegator == app_account
            || delegate == app_account
        {
            return Err(SolanaAclVerificationError::DelegationMismatch);
        }
        if delegation.delegator != delegator
            || delegation.delegate != delegate
            || delegation.app_account != app_account
        {
            return Err(SolanaAclVerificationError::DelegationMismatch);
        }
        if delegation.delegation_counter != expected_delegation_counter
            || expected_delegation_counter == 0
        {
            return Err(SolanaAclVerificationError::DelegationCounterMismatch);
        }
        if delegation.revoked
            || delegation.expiration_slot < observed_slot
            || delegation.last_update_slot > observed_slot
        {
            return Err(SolanaAclVerificationError::DelegationNotActive);
        }

        let (expected_key, expected_bump) = user_decryption_delegation_address(
            self.host_program_id,
            delegator,
            delegate,
            app_account,
        );
        if delegation.account_key != expected_key {
            return Err(SolanaAclVerificationError::NonCanonicalDelegation);
        }
        if delegation.bump != expected_bump {
            return Err(SolanaAclVerificationError::DelegationBumpMismatch);
        }
        Ok(())
    }
}

pub fn decode_user_decryption_delegation_witness(
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: &[u8],
) -> Result<UserDecryptionDelegationWitness, SolanaAclVerificationError> {
    require_anchor_account_data(
        data,
        "UserDecryptionDelegation",
        USER_DECRYPTION_DELEGATION_SPACE,
    )?;
    let mut cursor = AccountDataCursor::new(&data[ANCHOR_DISCRIMINATOR_LEN..]);
    Ok(UserDecryptionDelegationWitness {
        account_key,
        owner,
        delegator: cursor.read_bytes_32()?,
        delegate: cursor.read_bytes_32()?,
        app_account: cursor.read_bytes_32()?,
        expiration_slot: cursor.read_u64()?,
        delegation_counter: cursor.read_u64()?,
        last_update_slot: cursor.read_u64()?,
        revoked: cursor.read_bool()?,
        bump: cursor.read_u8()?,
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
    app_account: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            app_account.as_ref(),
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

fn require_anchor_account_data(
    data: &[u8],
    account_name: &str,
    body_len: usize,
) -> Result<(), SolanaAclVerificationError> {
    if data.len() != ANCHOR_DISCRIMINATOR_LEN + body_len {
        return Err(SolanaAclVerificationError::AccountDataLengthMismatch);
    }
    if data[..ANCHOR_DISCRIMINATOR_LEN] != anchor_account_discriminator(account_name) {
        return Err(SolanaAclVerificationError::AccountDiscriminatorMismatch);
    }
    Ok(())
}

struct AccountDataCursor<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> AccountDataCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn read_bytes_32(&mut self) -> Result<[u8; 32], SolanaAclVerificationError> {
        let bytes = self.read_exact(32)?;
        let mut output = [0; 32];
        output.copy_from_slice(bytes);
        Ok(output)
    }

    fn read_u64(&mut self) -> Result<u64, SolanaAclVerificationError> {
        let bytes = self.read_exact(8)?;
        let mut value = [0; 8];
        value.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(value))
    }

    fn read_u8(&mut self) -> Result<u8, SolanaAclVerificationError> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_bool(&mut self) -> Result<bool, SolanaAclVerificationError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(SolanaAclVerificationError::InvalidAccountData),
        }
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], SolanaAclVerificationError> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(SolanaAclVerificationError::AccountDataLengthMismatch)?;
        if end > self.data.len() {
            return Err(SolanaAclVerificationError::AccountDataLengthMismatch);
        }
        let slice = &self.data[self.offset..end];
        self.offset = end;
        Ok(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const APP_ACCOUNT: SolanaPubkeyBytes = [2; 32];
    const DELEGATE: SolanaPubkeyBytes = [5; 32];
    const OBSERVED_SLOT: u64 = 500;

    fn delegation_for_app(app_account: SolanaPubkeyBytes) -> UserDecryptionDelegationWitness {
        let (account_key, bump) =
            user_decryption_delegation_address(HOST_PROGRAM_ID, OWNER, DELEGATE, app_account);
        UserDecryptionDelegationWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            delegator: OWNER,
            delegate: DELEGATE,
            app_account,
            expiration_slot: OBSERVED_SLOT + 20,
            delegation_counter: 9,
            last_update_slot: OBSERVED_SLOT - 1,
            revoked: false,
            bump,
        }
    }

    fn delegation() -> UserDecryptionDelegationWitness {
        delegation_for_app(APP_ACCOUNT)
    }

    fn encode_delegation(delegation: &UserDecryptionDelegationWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("UserDecryptionDelegation").to_vec();
        data.extend_from_slice(&delegation.delegator);
        data.extend_from_slice(&delegation.delegate);
        data.extend_from_slice(&delegation.app_account);
        data.extend_from_slice(&delegation.expiration_slot.to_le_bytes());
        data.extend_from_slice(&delegation.delegation_counter.to_le_bytes());
        data.extend_from_slice(&delegation.last_update_slot.to_le_bytes());
        data.push(delegation.revoked as u8);
        data.push(delegation.bump);
        data
    }

    #[test]
    fn verifies_delegation() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        assert_eq!(
            verifier.verify_delegation(
                &delegation(),
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT
            ),
            Ok(())
        );
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
    fn rejects_invalid_delegation_witnesses() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);

        let mut wrong_delegate = delegation();
        wrong_delegate.delegate = [6; 32];
        assert_eq!(
            verifier.verify_delegation(
                &wrong_delegate,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT
            ),
            Err(SolanaAclVerificationError::DelegationMismatch)
        );

        let mut wildcard_delegate = delegation();
        wildcard_delegate.delegate = WILDCARD_APP_CONTEXT;
        let (account_key, bump) = user_decryption_delegation_address(
            HOST_PROGRAM_ID,
            wildcard_delegate.delegator,
            wildcard_delegate.delegate,
            wildcard_delegate.app_account,
        );
        wildcard_delegate.account_key = account_key;
        wildcard_delegate.bump = bump;
        assert_eq!(
            verifier.verify_delegation(
                &wildcard_delegate,
                OWNER,
                WILDCARD_APP_CONTEXT,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT
            ),
            Err(SolanaAclVerificationError::DelegationMismatch)
        );

        let mut wrong_pda = delegation();
        wrong_pda.account_key = [14; 32];
        assert_eq!(
            verifier.verify_delegation(&wrong_pda, OWNER, DELEGATE, APP_ACCOUNT, 9, OBSERVED_SLOT),
            Err(SolanaAclVerificationError::NonCanonicalDelegation)
        );

        assert_eq!(
            verifier.verify_delegation(
                &delegation(),
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                8,
                OBSERVED_SLOT
            ),
            Err(SolanaAclVerificationError::DelegationCounterMismatch)
        );

        let mut revoked = delegation();
        revoked.revoked = true;
        assert_eq!(
            verifier.verify_delegation(&revoked, OWNER, DELEGATE, APP_ACCOUNT, 9, OBSERVED_SLOT),
            Err(SolanaAclVerificationError::DelegationNotActive)
        );
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
