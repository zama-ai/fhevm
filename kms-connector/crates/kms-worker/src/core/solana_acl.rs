//! Solana ACL witness verification helpers for the KMS connector.
//!
//! These verifiers decode on-chain ZamaHost ACL accounts (record, permission,
//! delegation, and handle-material commitment) and check that a requested
//! decryption is authorized for a given handle and subject.

use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;
use thiserror::Error;

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

pub const ACL_RECORD_SEED: &[u8] = b"acl-record";
pub const ACL_PERMISSION_SEED: &[u8] = b"acl-permission";
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
pub const HANDLE_MATERIAL_SEED: &[u8] = b"handle-material";
pub const ACL_ROLE_USE: u8 = 0x01;
pub const ACL_ROLE_GRANT: u8 = 0x02;
pub const ACL_ROLE_PUBLIC_DECRYPT: u8 = 0x04;
pub const ACL_ROLE_COMPUTE: u8 = 0x08;
pub const ACL_ROLE_KNOWN: u8 =
    ACL_ROLE_USE | ACL_ROLE_GRANT | ACL_ROLE_PUBLIC_DECRYPT | ACL_ROLE_COMPUTE;
pub const HANDLE_MATERIAL_STATE_COMMITTED: u8 = 1;
pub const WILDCARD_APP_CONTEXT: SolanaPubkeyBytes = [0xff; 32];
const ANCHOR_DISCRIMINATOR_LEN: usize = 8;
const MAX_ACL_SUBJECTS: usize = 8;
const ACL_RECORD_SPACE: usize = 32
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
const ACL_PERMISSION_SPACE: usize = 32 + 32 + 1 + 1;
const USER_DECRYPTION_DELEGATION_SPACE: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1;
const HANDLE_MATERIAL_COMMITMENT_SPACE: usize = 32 + 32 + (32 * 5) + 8 + 1 + 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectRole {
    pub subject: SolanaPubkeyBytes,
    pub role_flags: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AclRecordWitness {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub handle: HandleBytes,
    pub nonce_key: [u8; 32],
    pub nonce_sequence: u64,
    pub acl_domain_key: SolanaPubkeyBytes,
    pub app_account: SolanaPubkeyBytes,
    pub encrypted_value_label: [u8; 32],
    pub subjects: Vec<SubjectRole>,
    pub overflow_subject_count: u32,
    pub public_decrypt: bool,
    pub material_commitment: SolanaPubkeyBytes,
    pub material_commitment_hash: [u8; 32],
    pub material_key_id: [u8; 32],
    pub created_slot: u64,
    pub bump: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AclPermissionWitness {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub acl_record: SolanaPubkeyBytes,
    pub subject: SolanaPubkeyBytes,
    pub role_flags: u8,
    pub bump: u8,
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandleMaterialCommitmentWitness {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub acl_record: SolanaPubkeyBytes,
    pub handle: HandleBytes,
    pub key_id: [u8; 32],
    pub ciphertext_digest: [u8; 32],
    pub sns_ciphertext_digest: [u8; 32],
    pub coprocessor_set_digest: [u8; 32],
    pub material_commitment_hash: [u8; 32],
    pub created_slot: u64,
    pub state: u8,
    pub bump: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaAclVerifier {
    pub host_program_id: SolanaPubkeyBytes,
}

/// Request parameters for verifying a delegated Solana user decryption against on-chain witnesses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DelegatedUserDecryptRequest<'a> {
    pub handle: HandleBytes,
    pub delegator: SolanaPubkeyBytes,
    pub delegate: SolanaPubkeyBytes,
    pub app_account: SolanaPubkeyBytes,
    pub expected_delegation_counter: u64,
    pub observed_slot: u64,
    pub allowed_acl_domain_keys: &'a [SolanaPubkeyBytes],
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaAclVerificationError {
    #[error("ACL account is not owned by the configured ZamaHost program")]
    InvalidAccountOwner,
    #[error("ACL record account is not the canonical PDA for its nonce metadata")]
    NonCanonicalAclRecord,
    #[error("ACL record bump does not match the canonical PDA bump")]
    AclRecordBumpMismatch,
    #[error("ACL record nonce key does not match its decoded domain/app/label metadata")]
    NonceKeyMismatch,
    #[error("ACL record handle does not match the requested handle")]
    HandleMismatch,
    #[error("ACL record domain is outside the signed authorization scope")]
    DomainNotAllowed,
    #[error("ACL record is not marked public-decryptable")]
    PublicDecryptNotAllowed,
    #[error("material commitment account is not the canonical PDA for the ACL record")]
    NonCanonicalMaterialCommitment,
    #[error("material commitment bump does not match the canonical PDA bump")]
    MaterialCommitmentBumpMismatch,
    #[error("material commitment account data does not match the ACL record or handle")]
    MaterialCommitmentMismatch,
    #[error("material commitment is not committed")]
    MaterialNotCommitted,
    #[error("material commitment hash does not match its decoded fields")]
    MaterialCommitmentHashMismatch,
    #[error("subject is not authorized by the ACL record or supplied overflow witnesses")]
    SubjectMissing,
    #[error("subject is present but lacks the required role flags")]
    SubjectRoleMismatch,
    #[error("overflow permission account is not the canonical PDA for the ACL record and subject")]
    NonCanonicalOverflowPermission,
    #[error("overflow permission bump does not match the canonical PDA bump")]
    OverflowPermissionBumpMismatch,
    #[error("overflow permission account data does not match the requested ACL record or subject")]
    OverflowPermissionMismatch,
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
}

impl SolanaAclVerifier {
    pub fn new(host_program_id: SolanaPubkeyBytes) -> Self {
        Self { host_program_id }
    }

    pub fn verify_public_decrypt(
        &self,
        record: &AclRecordWitness,
        handle: HandleBytes,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_acl_record(record, handle)?;
        if !record.public_decrypt {
            return Err(SolanaAclVerificationError::PublicDecryptNotAllowed);
        }
        Ok(())
    }

    pub fn verify_public_decrypt_with_material(
        &self,
        record: &AclRecordWitness,
        material: &HandleMaterialCommitmentWitness,
        handle: HandleBytes,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_public_decrypt(record, handle)?;
        self.verify_material_commitment(record, material, handle)
    }

    pub fn verify_user_decrypt(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        handle: HandleBytes,
        subject: SolanaPubkeyBytes,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_acl_record(record, handle)?;
        if !allowed_acl_domain_keys.contains(&record.acl_domain_key) {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        self.verify_subject_role(record, overflow_permissions, subject, ACL_ROLE_USE)
    }

    /// Verifies `subject` holds the USE role for `handle` on its on-chain ACL record, without the
    /// `allowed_acl_domain_keys` scoping that [`Self::verify_user_decrypt`] applies. The Solana
    /// user-decrypt request does not yet carry the user's authorized domains (the EVM
    /// `allowedContracts` analog), so this enforces the essential authorization — that the
    /// requester was granted access to the handle — which closes the "decrypt any handle" gap.
    /// Restricting to the user's authorized domains is a follow-up, tied to carrying and
    /// ed25519-signing those domains in the Solana user-decrypt request.
    pub fn verify_user_decrypt_subject(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        handle: HandleBytes,
        subject: SolanaPubkeyBytes,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_acl_record(record, handle)?;
        self.verify_subject_role(record, overflow_permissions, subject, ACL_ROLE_USE)
    }

    pub fn verify_user_decrypt_with_material(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        material: &HandleMaterialCommitmentWitness,
        handle: HandleBytes,
        subject: SolanaPubkeyBytes,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_user_decrypt(
            record,
            overflow_permissions,
            handle,
            subject,
            allowed_acl_domain_keys,
        )?;
        self.verify_material_commitment(record, material, handle)
    }

    pub fn verify_delegated_user_decrypt(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        delegation: &UserDecryptionDelegationWitness,
        request: DelegatedUserDecryptRequest<'_>,
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_acl_record(record, request.handle)?;
        if !request
            .allowed_acl_domain_keys
            .contains(&record.acl_domain_key)
        {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        self.verify_subject_role(
            record,
            overflow_permissions,
            request.delegator,
            ACL_ROLE_USE,
        )?;
        self.verify_delegation(
            delegation,
            request.delegator,
            request.delegate,
            request.app_account,
            request.expected_delegation_counter,
            request.observed_slot,
        )
    }

    pub fn verify_delegated_user_decrypt_with_material(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        delegation: &UserDecryptionDelegationWitness,
        material: &HandleMaterialCommitmentWitness,
        request: DelegatedUserDecryptRequest<'_>,
    ) -> Result<(), SolanaAclVerificationError> {
        let handle = request.handle;
        self.verify_delegated_user_decrypt(record, overflow_permissions, delegation, request)?;
        self.verify_material_commitment(record, material, handle)
    }
    fn verify_acl_record(
        &self,
        record: &AclRecordWitness,
        handle: HandleBytes,
    ) -> Result<(), SolanaAclVerificationError> {
        if record.owner != self.host_program_id {
            return Err(SolanaAclVerificationError::InvalidAccountOwner);
        }
        if record.handle != handle {
            return Err(SolanaAclVerificationError::HandleMismatch);
        }

        let expected_nonce_key = acl_nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        if record.nonce_key != expected_nonce_key {
            return Err(SolanaAclVerificationError::NonceKeyMismatch);
        }

        let (expected_key, expected_bump) = acl_record_address(
            self.host_program_id,
            record.nonce_key,
            record.nonce_sequence,
        );
        if record.account_key != expected_key {
            return Err(SolanaAclVerificationError::NonCanonicalAclRecord);
        }
        if record.bump != expected_bump {
            return Err(SolanaAclVerificationError::AclRecordBumpMismatch);
        }
        Ok(())
    }

    fn verify_material_commitment(
        &self,
        record: &AclRecordWitness,
        material: &HandleMaterialCommitmentWitness,
        handle: HandleBytes,
    ) -> Result<(), SolanaAclVerificationError> {
        if material.owner != self.host_program_id {
            return Err(SolanaAclVerificationError::InvalidAccountOwner);
        }
        if material.acl_record != record.account_key || material.handle != handle {
            return Err(SolanaAclVerificationError::MaterialCommitmentMismatch);
        }
        if material.state != HANDLE_MATERIAL_STATE_COMMITTED {
            return Err(SolanaAclVerificationError::MaterialNotCommitted);
        }

        let (expected_key, expected_bump) =
            handle_material_address(self.host_program_id, record.account_key);
        if material.account_key != expected_key {
            return Err(SolanaAclVerificationError::NonCanonicalMaterialCommitment);
        }
        if material.bump != expected_bump {
            return Err(SolanaAclVerificationError::MaterialCommitmentBumpMismatch);
        }

        let expected_hash = handle_material_commitment_hash(
            self.host_program_id,
            material.account_key,
            material.acl_record,
            material.key_id,
            material.ciphertext_digest,
            material.sns_ciphertext_digest,
            material.coprocessor_set_digest,
        );
        if material.material_commitment_hash != expected_hash {
            return Err(SolanaAclVerificationError::MaterialCommitmentHashMismatch);
        }
        if record.material_commitment != material.account_key
            || record.material_commitment_hash != material.material_commitment_hash
            || record.material_key_id != material.key_id
        {
            return Err(SolanaAclVerificationError::MaterialCommitmentMismatch);
        }
        Ok(())
    }

    fn verify_subject_role(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        subject: SolanaPubkeyBytes,
        required_role: u8,
    ) -> Result<(), SolanaAclVerificationError> {
        if let Some(inline) = record
            .subjects
            .iter()
            .find(|entry| entry.subject == subject)
        {
            return require_role(inline.role_flags, required_role);
        }

        if let Some(permission) = overflow_permissions
            .iter()
            .find(|permission| permission.subject == subject)
        {
            self.verify_overflow_permission(record, permission)?;
            return require_role(permission.role_flags, required_role);
        }

        Err(SolanaAclVerificationError::SubjectMissing)
    }

    fn verify_overflow_permission(
        &self,
        record: &AclRecordWitness,
        permission: &AclPermissionWitness,
    ) -> Result<(), SolanaAclVerificationError> {
        if permission.owner != self.host_program_id {
            return Err(SolanaAclVerificationError::InvalidAccountOwner);
        }
        if permission.acl_record != record.account_key {
            return Err(SolanaAclVerificationError::OverflowPermissionMismatch);
        }

        let (expected_key, expected_bump) =
            acl_permission_address(self.host_program_id, record.account_key, permission.subject);
        if permission.account_key != expected_key {
            return Err(SolanaAclVerificationError::NonCanonicalOverflowPermission);
        }
        if permission.bump != expected_bump {
            return Err(SolanaAclVerificationError::OverflowPermissionBumpMismatch);
        }
        Ok(())
    }

    fn verify_delegation(
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

pub fn decode_acl_record_witness(
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: &[u8],
) -> Result<AclRecordWitness, SolanaAclVerificationError> {
    require_anchor_account_data(data, "AclRecord", ACL_RECORD_SPACE)?;
    let mut cursor = AccountDataCursor::new(&data[ANCHOR_DISCRIMINATOR_LEN..]);
    let handle = cursor.read_bytes_32()?;
    let nonce_key = cursor.read_bytes_32()?;
    let nonce_sequence = cursor.read_u64()?;
    let acl_domain_key = cursor.read_bytes_32()?;
    let app_account = cursor.read_bytes_32()?;
    let encrypted_value_label = cursor.read_bytes_32()?;

    let mut subject_pubkeys = Vec::with_capacity(MAX_ACL_SUBJECTS);
    for _ in 0..MAX_ACL_SUBJECTS {
        subject_pubkeys.push(cursor.read_bytes_32()?);
    }

    let mut subject_roles = Vec::with_capacity(MAX_ACL_SUBJECTS);
    for _ in 0..MAX_ACL_SUBJECTS {
        subject_roles.push(cursor.read_u8()?);
    }
    let subject_count = cursor.read_u8()? as usize;
    if subject_count > MAX_ACL_SUBJECTS {
        return Err(SolanaAclVerificationError::InvalidAccountData);
    }
    for index in 0..MAX_ACL_SUBJECTS {
        let subject = subject_pubkeys[index];
        let role_flags = subject_roles[index];
        if index < subject_count {
            if subject == [0; 32]
                || !role_flags_are_valid(role_flags)
                || subject_pubkeys[..index].contains(&subject)
            {
                return Err(SolanaAclVerificationError::InvalidAccountData);
            }
        } else if subject != [0; 32] || role_flags != 0 {
            return Err(SolanaAclVerificationError::InvalidAccountData);
        }
    }
    let subjects = subject_pubkeys
        .into_iter()
        .zip(subject_roles)
        .take(subject_count)
        .map(|(subject, role_flags)| SubjectRole {
            subject,
            role_flags,
        })
        .collect();

    Ok(AclRecordWitness {
        account_key,
        owner,
        handle,
        nonce_key,
        nonce_sequence,
        acl_domain_key,
        app_account,
        encrypted_value_label,
        subjects,
        overflow_subject_count: cursor.read_u32()?,
        public_decrypt: cursor.read_bool()?,
        material_commitment: cursor.read_bytes_32()?,
        material_commitment_hash: cursor.read_bytes_32()?,
        material_key_id: cursor.read_bytes_32()?,
        created_slot: cursor.read_u64()?,
        bump: cursor.read_u8()?,
    })
}

pub fn decode_acl_permission_witness(
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: &[u8],
) -> Result<AclPermissionWitness, SolanaAclVerificationError> {
    require_anchor_account_data(data, "AclPermission", ACL_PERMISSION_SPACE)?;
    let mut cursor = AccountDataCursor::new(&data[ANCHOR_DISCRIMINATOR_LEN..]);
    let acl_record = cursor.read_bytes_32()?;
    let subject = cursor.read_bytes_32()?;
    let role_flags = cursor.read_u8()?;
    if acl_record == [0; 32] || subject == [0; 32] || !role_flags_are_valid(role_flags) {
        return Err(SolanaAclVerificationError::InvalidAccountData);
    }
    Ok(AclPermissionWitness {
        account_key,
        owner,
        acl_record,
        subject,
        role_flags,
        bump: cursor.read_u8()?,
    })
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

pub fn decode_handle_material_commitment_witness(
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: &[u8],
) -> Result<HandleMaterialCommitmentWitness, SolanaAclVerificationError> {
    require_anchor_account_data(
        data,
        "HandleMaterialCommitment",
        HANDLE_MATERIAL_COMMITMENT_SPACE,
    )?;
    let mut cursor = AccountDataCursor::new(&data[ANCHOR_DISCRIMINATOR_LEN..]);
    Ok(HandleMaterialCommitmentWitness {
        account_key,
        owner,
        acl_record: cursor.read_bytes_32()?,
        handle: cursor.read_bytes_32()?,
        key_id: cursor.read_bytes_32()?,
        ciphertext_digest: cursor.read_bytes_32()?,
        sns_ciphertext_digest: cursor.read_bytes_32()?,
        coprocessor_set_digest: cursor.read_bytes_32()?,
        material_commitment_hash: cursor.read_bytes_32()?,
        created_slot: cursor.read_u64()?,
        state: cursor.read_u8()?,
        bump: cursor.read_u8()?,
    })
}

pub fn acl_nonce_key(
    acl_domain_key: SolanaPubkeyBytes,
    app_account: SolanaPubkeyBytes,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"zama-acl-nonce-key-v1");
    hasher.update(acl_domain_key);
    hasher.update(app_account);
    hasher.update(encrypted_value_label);
    hasher.finalize().into()
}

pub fn acl_record_address(
    host_program_id: SolanaPubkeyBytes,
    nonce_key: [u8; 32],
    nonce_sequence: u64,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let sequence = nonce_sequence.to_le_bytes();
    let (address, bump) = Pubkey::find_program_address(
        &[ACL_RECORD_SEED, nonce_key.as_ref(), &sequence],
        &host_program_id,
    );
    (address.to_bytes(), bump)
}

pub fn acl_permission_address(
    host_program_id: SolanaPubkeyBytes,
    acl_record: SolanaPubkeyBytes,
    subject: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(
        &[ACL_PERMISSION_SEED, acl_record.as_ref(), subject.as_ref()],
        &host_program_id,
    );
    (address.to_bytes(), bump)
}

pub fn host_config_address(host_program_id: SolanaPubkeyBytes) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(&[HOST_CONFIG_SEED], &host_program_id);
    (address.to_bytes(), bump)
}

pub fn handle_material_address(
    host_program_id: SolanaPubkeyBytes,
    acl_record: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (host_config, _) = host_config_address(host_program_id.to_bytes());
    let (address, bump) = Pubkey::find_program_address(
        &[
            HANDLE_MATERIAL_SEED,
            host_config.as_ref(),
            acl_record.as_ref(),
        ],
        &host_program_id,
    );
    (address.to_bytes(), bump)
}

pub fn handle_material_commitment_hash(
    host_program_id: SolanaPubkeyBytes,
    material_commitment: SolanaPubkeyBytes,
    acl_record: SolanaPubkeyBytes,
    key_id: [u8; 32],
    ciphertext_digest: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_set_digest: [u8; 32],
) -> [u8; 32] {
    let (host_config, _) = host_config_address(host_program_id);
    let mut hasher = Sha256::new();
    hasher.update(b"zama-solana-material-commitment-v1");
    hasher.update(host_config);
    hasher.update(host_program_id);
    hasher.update(material_commitment);
    hasher.update(acl_record);
    hasher.update(key_id);
    hasher.update(ciphertext_digest);
    hasher.update(sns_ciphertext_digest);
    hasher.update(coprocessor_set_digest);
    hasher.finalize().into()
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

    fn read_u32(&mut self) -> Result<u32, SolanaAclVerificationError> {
        let bytes = self.read_exact(4)?;
        let mut value = [0; 4];
        value.copy_from_slice(bytes);
        Ok(u32::from_le_bytes(value))
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

fn require_role(role_flags: u8, required_role: u8) -> Result<(), SolanaAclVerificationError> {
    if role_flags & required_role == required_role {
        Ok(())
    } else {
        Err(SolanaAclVerificationError::SubjectRoleMismatch)
    }
}

fn role_flags_are_valid(role_flags: u8) -> bool {
    role_flags != 0 && role_flags & !ACL_ROLE_KNOWN == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const HANDLE: HandleBytes = [
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 3, 132,
        5, 0,
    ];
    const DOMAIN: SolanaPubkeyBytes = [1; 32];
    const APP_ACCOUNT: SolanaPubkeyBytes = [2; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const OVERFLOW_SUBJECT: SolanaPubkeyBytes = [4; 32];
    const DELEGATE: SolanaPubkeyBytes = [5; 32];
    const OBSERVED_SLOT: u64 = 500;
    const LABEL: [u8; 32] = *b"balance_________________________";

    fn base_record() -> AclRecordWitness {
        let nonce_key = acl_nonce_key(DOMAIN, APP_ACCOUNT, LABEL);
        let (account_key, bump) = acl_record_address(HOST_PROGRAM_ID, nonce_key, 8);
        let (material_commitment, _) = handle_material_address(HOST_PROGRAM_ID, account_key);
        let material_key_id = [21; 32];
        let material_commitment_hash = handle_material_commitment_hash(
            HOST_PROGRAM_ID,
            material_commitment,
            account_key,
            material_key_id,
            [22; 32],
            [23; 32],
            [24; 32],
        );
        AclRecordWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            handle: HANDLE,
            nonce_key,
            nonce_sequence: 8,
            acl_domain_key: DOMAIN,
            app_account: APP_ACCOUNT,
            encrypted_value_label: LABEL,
            subjects: vec![
                SubjectRole {
                    subject: OWNER,
                    role_flags: ACL_ROLE_USE | ACL_ROLE_PUBLIC_DECRYPT,
                },
                SubjectRole {
                    subject: APP_ACCOUNT,
                    role_flags: ACL_ROLE_USE,
                },
            ],
            overflow_subject_count: 0,
            public_decrypt: true,
            material_commitment,
            material_commitment_hash,
            material_key_id,
            created_slot: OBSERVED_SLOT,
            bump,
        }
    }

    fn overflow_permission(record: &AclRecordWitness) -> AclPermissionWitness {
        let (account_key, bump) =
            acl_permission_address(HOST_PROGRAM_ID, record.account_key, OVERFLOW_SUBJECT);
        AclPermissionWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            acl_record: record.account_key,
            subject: OVERFLOW_SUBJECT,
            role_flags: ACL_ROLE_USE,
            bump,
        }
    }

    fn material_commitment(record: &AclRecordWitness) -> HandleMaterialCommitmentWitness {
        let (account_key, bump) = handle_material_address(HOST_PROGRAM_ID, record.account_key);
        let key_id = if record.material_key_id == [0; 32] {
            [21; 32]
        } else {
            record.material_key_id
        };
        let ciphertext_digest = [22; 32];
        let sns_ciphertext_digest = [23; 32];
        let coprocessor_set_digest = [24; 32];
        let material_commitment_hash = handle_material_commitment_hash(
            HOST_PROGRAM_ID,
            account_key,
            record.account_key,
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
        );
        HandleMaterialCommitmentWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            acl_record: record.account_key,
            handle: record.handle,
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
            material_commitment_hash,
            created_slot: OBSERVED_SLOT,
            state: HANDLE_MATERIAL_STATE_COMMITTED,
            bump,
        }
    }

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
    fn encode_acl_record(record: &AclRecordWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("AclRecord").to_vec();
        data.extend_from_slice(&record.handle);
        data.extend_from_slice(&record.nonce_key);
        data.extend_from_slice(&record.nonce_sequence.to_le_bytes());
        data.extend_from_slice(&record.acl_domain_key);
        data.extend_from_slice(&record.app_account);
        data.extend_from_slice(&record.encrypted_value_label);
        for index in 0..MAX_ACL_SUBJECTS {
            let subject = record
                .subjects
                .get(index)
                .map(|entry| entry.subject)
                .unwrap_or([0; 32]);
            data.extend_from_slice(&subject);
        }
        for index in 0..MAX_ACL_SUBJECTS {
            let role_flags = record
                .subjects
                .get(index)
                .map(|entry| entry.role_flags)
                .unwrap_or(0);
            data.push(role_flags);
        }
        data.push(record.subjects.len() as u8);
        data.extend_from_slice(&record.overflow_subject_count.to_le_bytes());
        data.push(record.public_decrypt as u8);
        data.extend_from_slice(&record.material_commitment);
        data.extend_from_slice(&record.material_commitment_hash);
        data.extend_from_slice(&record.material_key_id);
        data.extend_from_slice(&record.created_slot.to_le_bytes());
        data.push(record.bump);
        data
    }

    fn encode_acl_permission(permission: &AclPermissionWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("AclPermission").to_vec();
        data.extend_from_slice(&permission.acl_record);
        data.extend_from_slice(&permission.subject);
        data.push(permission.role_flags);
        data.push(permission.bump);
        data
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

    fn encode_material_commitment(material: &HandleMaterialCommitmentWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("HandleMaterialCommitment").to_vec();
        data.extend_from_slice(&material.acl_record);
        data.extend_from_slice(&material.handle);
        data.extend_from_slice(&material.key_id);
        data.extend_from_slice(&material.ciphertext_digest);
        data.extend_from_slice(&material.sns_ciphertext_digest);
        data.extend_from_slice(&material.coprocessor_set_digest);
        data.extend_from_slice(&material.material_commitment_hash);
        data.extend_from_slice(&material.created_slot.to_le_bytes());
        data.push(material.state);
        data.push(material.bump);
        data
    }
    #[test]
    fn verifies_public_decrypt_acl_record_witness() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        assert_eq!(
            verifier.verify_public_decrypt(&base_record(), HANDLE),
            Ok(())
        );
    }

    #[test]
    fn verifies_public_decrypt_with_material_witness() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let record = base_record();
        let material = material_commitment(&record);

        assert_eq!(
            verifier.verify_public_decrypt_with_material(&record, &material, HANDLE),
            Ok(())
        );
    }

    #[test]
    fn rejects_public_decrypt_without_public_flag() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut record = base_record();
        record.public_decrypt = false;

        assert_eq!(
            verifier.verify_public_decrypt(&record, HANDLE),
            Err(SolanaAclVerificationError::PublicDecryptNotAllowed)
        );
    }

    #[test]
    fn verifies_user_decrypt_inline_subject_and_domain_scope() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        assert_eq!(
            verifier.verify_user_decrypt(&base_record(), &[], HANDLE, OWNER, &[DOMAIN]),
            Ok(())
        );
    }

    #[test]
    fn decodes_anchor_account_data_before_verification() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let record = base_record();
        let permission = overflow_permission(&record);
        let delegation = delegation();
        let material = material_commitment(&record);

        assert_eq!(
            encode_acl_record(&record).len(),
            ANCHOR_DISCRIMINATOR_LEN + ACL_RECORD_SPACE
        );
        let decoded_record = decode_acl_record_witness(
            record.account_key,
            HOST_PROGRAM_ID,
            &encode_acl_record(&record),
        )
        .expect("record decodes");
        assert_eq!(decoded_record, record);
        assert_eq!(
            verifier.verify_public_decrypt(&decoded_record, HANDLE),
            Ok(())
        );

        let decoded_permission = decode_acl_permission_witness(
            permission.account_key,
            HOST_PROGRAM_ID,
            &encode_acl_permission(&permission),
        )
        .expect("permission decodes");
        assert_eq!(decoded_permission, permission);

        let decoded_delegation = decode_user_decryption_delegation_witness(
            delegation.account_key,
            HOST_PROGRAM_ID,
            &encode_delegation(&delegation),
        )
        .expect("delegation decodes");
        assert_eq!(decoded_delegation, delegation);
        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &decoded_record,
                &[],
                &decoded_delegation,
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: DELEGATE,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Ok(())
        );

        let decoded_material = decode_handle_material_commitment_witness(
            material.account_key,
            HOST_PROGRAM_ID,
            &encode_material_commitment(&material),
        )
        .expect("material commitment decodes");
        assert_eq!(decoded_material, material);
        assert_eq!(
            verifier.verify_public_decrypt_with_material(
                &decoded_record,
                &decoded_material,
                HANDLE
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_invalid_anchor_account_data() {
        let record = base_record();
        let subjects_offset = ANCHOR_DISCRIMINATOR_LEN + 32 + 32 + 8 + 32 + 32 + 32;
        let roles_offset = subjects_offset + MAX_ACL_SUBJECTS * 32;
        let subject_count_offset = roles_offset + MAX_ACL_SUBJECTS;
        let mut wrong_discriminator = encode_acl_record(&record);
        wrong_discriminator[0] ^= 0xff;
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &wrong_discriminator),
            Err(SolanaAclVerificationError::AccountDiscriminatorMismatch)
        );

        let mut truncated = encode_acl_record(&record);
        truncated.pop();
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &truncated),
            Err(SolanaAclVerificationError::AccountDataLengthMismatch)
        );

        let mut invalid_subject_count = encode_acl_record(&record);
        invalid_subject_count[subject_count_offset] = (MAX_ACL_SUBJECTS + 1) as u8;
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &invalid_subject_count),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let mut duplicate_subject = encode_acl_record(&record);
        duplicate_subject[subjects_offset + 32..subjects_offset + 64].copy_from_slice(&OWNER);
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &duplicate_subject),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let mut unused_subject = encode_acl_record(&record);
        unused_subject[subjects_offset + 64..subjects_offset + 96].copy_from_slice(&[9; 32]);
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &unused_subject),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let mut unknown_role = encode_acl_record(&record);
        unknown_role[roles_offset] = ACL_ROLE_USE | 0x80;
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &unknown_role),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let mut unused_role = encode_acl_record(&record);
        unused_role[roles_offset + 2] = ACL_ROLE_USE;
        assert_eq!(
            decode_acl_record_witness(record.account_key, HOST_PROGRAM_ID, &unused_role),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let permission = overflow_permission(&record);
        let role_offset = ANCHOR_DISCRIMINATOR_LEN + 32 + 32;
        let mut zero_permission_role = encode_acl_permission(&permission);
        zero_permission_role[role_offset] = 0;
        assert_eq!(
            decode_acl_permission_witness(
                permission.account_key,
                HOST_PROGRAM_ID,
                &zero_permission_role,
            ),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let mut unknown_permission_role = encode_acl_permission(&permission);
        unknown_permission_role[role_offset] = ACL_ROLE_USE | 0x80;
        assert_eq!(
            decode_acl_permission_witness(
                permission.account_key,
                HOST_PROGRAM_ID,
                &unknown_permission_role,
            ),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let mut invalid_bool = encode_delegation(&delegation());
        let revoked_offset = invalid_bool.len() - 2;
        invalid_bool[revoked_offset] = 2;
        assert_eq!(
            decode_user_decryption_delegation_witness([0; 32], HOST_PROGRAM_ID, &invalid_bool),
            Err(SolanaAclVerificationError::InvalidAccountData)
        );

        let material = material_commitment(&record);
        let mut truncated_material = encode_material_commitment(&material);
        truncated_material.pop();
        assert_eq!(
            decode_handle_material_commitment_witness(
                material.account_key,
                HOST_PROGRAM_ID,
                &truncated_material,
            ),
            Err(SolanaAclVerificationError::AccountDataLengthMismatch)
        );
    }

    #[test]
    fn verifies_delegated_user_decrypt_acl_and_delegation_witnesses() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &delegation(),
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: DELEGATE,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_wrong_owner_handle_nonce_domain_and_pda() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);

        let mut wrong_owner = base_record();
        wrong_owner.owner = [9; 32];
        assert_eq!(
            verifier.verify_public_decrypt(&wrong_owner, HANDLE),
            Err(SolanaAclVerificationError::InvalidAccountOwner)
        );

        assert_eq!(
            verifier.verify_public_decrypt(&base_record(), [8; 32]),
            Err(SolanaAclVerificationError::HandleMismatch)
        );

        let mut wrong_nonce = base_record();
        wrong_nonce.nonce_key = [11; 32];
        assert_eq!(
            verifier.verify_public_decrypt(&wrong_nonce, HANDLE),
            Err(SolanaAclVerificationError::NonceKeyMismatch)
        );

        let mut wrong_pda = base_record();
        wrong_pda.account_key = [12; 32];
        assert_eq!(
            verifier.verify_public_decrypt(&wrong_pda, HANDLE),
            Err(SolanaAclVerificationError::NonCanonicalAclRecord)
        );

        assert_eq!(
            verifier.verify_user_decrypt(&base_record(), &[], HANDLE, OWNER, &[[99; 32]]),
            Err(SolanaAclVerificationError::DomainNotAllowed)
        );
    }

    #[test]
    fn rejects_invalid_material_witnesses() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let record = base_record();

        let mut wrong_owner = material_commitment(&record);
        wrong_owner.owner = [9; 32];
        assert_eq!(
            verifier.verify_public_decrypt_with_material(&record, &wrong_owner, HANDLE),
            Err(SolanaAclVerificationError::InvalidAccountOwner)
        );

        let mut wrong_pda = material_commitment(&record);
        wrong_pda.account_key = [10; 32];
        assert_eq!(
            verifier.verify_public_decrypt_with_material(&record, &wrong_pda, HANDLE),
            Err(SolanaAclVerificationError::NonCanonicalMaterialCommitment)
        );

        let mut wrong_handle = material_commitment(&record);
        wrong_handle.handle = [11; 32];
        assert_eq!(
            verifier.verify_public_decrypt_with_material(&record, &wrong_handle, HANDLE),
            Err(SolanaAclVerificationError::MaterialCommitmentMismatch)
        );

        let mut uncommitted = material_commitment(&record);
        uncommitted.state = 0;
        assert_eq!(
            verifier.verify_public_decrypt_with_material(&record, &uncommitted, HANDLE),
            Err(SolanaAclVerificationError::MaterialNotCommitted)
        );

        let mut wrong_hash = material_commitment(&record);
        wrong_hash.material_commitment_hash = [12; 32];
        assert_eq!(
            verifier.verify_public_decrypt_with_material(&record, &wrong_hash, HANDLE),
            Err(SolanaAclVerificationError::MaterialCommitmentHashMismatch)
        );

        let mut unsealed_record = record.clone();
        unsealed_record.material_commitment = [0; 32];
        unsealed_record.material_commitment_hash = [0; 32];
        unsealed_record.material_key_id = [0; 32];
        let material = material_commitment(&record);
        assert_eq!(
            verifier.verify_public_decrypt_with_material(&unsealed_record, &material, HANDLE),
            Err(SolanaAclVerificationError::MaterialCommitmentMismatch)
        );
    }

    #[test]
    fn verifies_overflow_subject_witness() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut record = base_record();
        record.subjects.clear();
        record.overflow_subject_count = 1;
        let permission = overflow_permission(&record);

        assert_eq!(
            verifier.verify_user_decrypt(
                &record,
                &[permission],
                HANDLE,
                OVERFLOW_SUBJECT,
                &[DOMAIN]
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_wrong_or_roleless_overflow_witness() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut record = base_record();
        record.subjects.clear();
        record.overflow_subject_count = 1;

        let mut wrong_permission = overflow_permission(&record);
        wrong_permission.account_key = [13; 32];
        assert_eq!(
            verifier.verify_user_decrypt(
                &record,
                &[wrong_permission],
                HANDLE,
                OVERFLOW_SUBJECT,
                &[DOMAIN],
            ),
            Err(SolanaAclVerificationError::NonCanonicalOverflowPermission)
        );

        let mut roleless_permission = overflow_permission(&record);
        roleless_permission.role_flags = 0;
        assert_eq!(
            verifier.verify_user_decrypt(
                &record,
                &[roleless_permission],
                HANDLE,
                OVERFLOW_SUBJECT,
                &[DOMAIN],
            ),
            Err(SolanaAclVerificationError::SubjectRoleMismatch)
        );
    }

    #[test]
    fn rejects_invalid_delegation_witnesses() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);

        let mut wrong_delegate = delegation();
        wrong_delegate.delegate = [6; 32];
        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &wrong_delegate,
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: DELEGATE,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
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
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &wildcard_delegate,
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: WILDCARD_APP_CONTEXT,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Err(SolanaAclVerificationError::DelegationMismatch)
        );

        let mut delegate_is_app = delegation();
        delegate_is_app.delegate = APP_ACCOUNT;
        let (account_key, bump) = user_decryption_delegation_address(
            HOST_PROGRAM_ID,
            delegate_is_app.delegator,
            delegate_is_app.delegate,
            delegate_is_app.app_account,
        );
        delegate_is_app.account_key = account_key;
        delegate_is_app.bump = bump;
        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &delegate_is_app,
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: APP_ACCOUNT,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Err(SolanaAclVerificationError::DelegationMismatch)
        );

        let mut wrong_pda = delegation();
        wrong_pda.account_key = [14; 32];
        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &wrong_pda,
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: DELEGATE,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Err(SolanaAclVerificationError::NonCanonicalDelegation)
        );

        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &delegation(),
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: DELEGATE,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 8,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Err(SolanaAclVerificationError::DelegationCounterMismatch)
        );

        let mut revoked = delegation();
        revoked.revoked = true;
        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &revoked,
                DelegatedUserDecryptRequest {
                    handle: HANDLE,
                    delegator: OWNER,
                    delegate: DELEGATE,
                    app_account: APP_ACCOUNT,
                    expected_delegation_counter: 9,
                    observed_slot: OBSERVED_SLOT,
                    allowed_acl_domain_keys: &[DOMAIN],
                },
            ),
            Err(SolanaAclVerificationError::DelegationNotActive)
        );
    }
}
