//! Solana ACL witness verification helpers for the KMS connector.
//!
//! The event processor can consume the branch-local Gateway-PoC `extraData`
//! witness envelope for end-to-end validation. Native-v0 request admission
//! helpers live here too; `solana_request` parses signed request bytes and
//! attaches account witnesses, `solana_live` enforces account-fetch and
//! finality policy before replay, `solana_native` composes verified requests
//! with signature and replay checks, and `solana_response` verifies accepted
//! response certificates. `solana_rpc` supplies the JSON-RPC account fetcher.
//! Native response publication is intentionally a separate integration step.

use sha2::Sha256;
use sha3::{Digest, Keccak256};
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
pub const SOLANA_NATIVE_EXTRA_DATA_LAYOUT_V0: u8 = 0;
pub const SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED: u8 = 0;
pub const SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED: u8 = 1;
pub const SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED: u8 = 2;
pub const SOLANA_NATIVE_REQUEST_MODE_PUBLIC: u8 = 3;
pub const SOLANA_NATIVE_SUPPORTED_ACL_LAYOUT_VERSION: u8 = 0;
pub const SOLANA_NATIVE_SUPPORTED_HANDLE_DERIVATION_VERSION: u8 = 0;
pub const SOLANA_NATIVE_SUPPORTED_HANDLE_VERSION: u8 = 0;
pub const SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE: u8 = 1;
pub const SOLANA_NATIVE_SUPPORTED_MATERIAL_COMMITMENT_VERSION: u8 = 0;
pub const SOLANA_NATIVE_COMMITMENT_CONFIRMED: u8 = 1;
pub const SOLANA_NATIVE_COMMITMENT_FINALIZED: u8 = 2;
pub const SOLANA_NATIVE_MAX_ENCRYPTED_BITS_PER_REQUEST: usize = 2048;
pub const SOLANA_NATIVE_ED25519_SIGNATURE_LEN: usize = 64;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaKmsExtraDataV0 {
    pub kms_context_id: [u8; 32],
    pub response_context: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeHandleEntryV0 {
    pub handle: HandleBytes,
    pub owner_pubkey: SolanaPubkeyBytes,
    pub owner_permission_account: SolanaPubkeyBytes,
    pub delegator_pubkey: SolanaPubkeyBytes,
    pub delegator_permission_account: SolanaPubkeyBytes,
    pub delegate_pubkey: SolanaPubkeyBytes,
    pub app_context_pubkey: SolanaPubkeyBytes,
    pub app_context_permission_account: SolanaPubkeyBytes,
    pub acl_record_account: SolanaPubkeyBytes,
    pub delegation_record_account: SolanaPubkeyBytes,
    pub expected_delegation_counter: u64,
    pub material_commitment_account: SolanaPubkeyBytes,
    pub material_commitment_hash: [u8; 32],
    pub min_context_slot: u64,
    pub config_version: u64,
    pub material_source_mode: u8,
    pub acl_layout_version: u8,
    pub handle_derivation_version: u8,
    pub material_commitment_version: u8,
    pub expected_key_id: [u8; 32],
    pub acl_record: AclRecordWitness,
    pub material: HandleMaterialCommitmentWitness,
    pub owner_permission: Option<AclPermissionWitness>,
    pub delegator_permission: Option<AclPermissionWitness>,
    pub app_context_permission: Option<AclPermissionWitness>,
    pub delegation: Option<UserDecryptionDelegationWitness>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaUserDecryptionPayloadV0 {
    pub domain_separator: [u8; 32],
    pub host_chain_id: u64,
    pub config_version: u64,
    pub solana_cluster_id: [u8; 32],
    pub kms_context_id: [u8; 32],
    pub user_reencryption_pubkey_len: u32,
    pub user_reencryption_pubkey_hash: [u8; 32],
    pub request_signer_pubkey: SolanaPubkeyBytes,
    pub acl_program_id: SolanaPubkeyBytes,
    pub request_mode: u8,
    pub material_source_mode: u8,
    pub commitment_level: u8,
    pub min_context_slot: u64,
    pub expiration_slot: u64,
    pub nonce: [u8; 32],
    pub extra_data_hash: [u8; 32],
    pub entries_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeDecryptionRequestV0 {
    pub payload: SolanaUserDecryptionPayloadV0,
    pub entries: Vec<SolanaNativeHandleEntryV0>,
    pub raw_extra_data: Vec<u8>,
    pub user_reencryption_public_key: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaNativeRequestLimits {
    pub max_user_reencryption_pubkey_bytes: usize,
    pub max_extra_data_bytes: usize,
    pub max_handles_per_request: usize,
    pub max_signed_request_bytes: usize,
    pub max_encrypted_bits_per_request: usize,
}

impl Default for SolanaNativeRequestLimits {
    fn default() -> Self {
        Self {
            max_user_reencryption_pubkey_bytes: 4096,
            max_extra_data_bytes: 4096,
            max_handles_per_request: 128,
            max_signed_request_bytes: 64 * 1024,
            max_encrypted_bits_per_request: SOLANA_NATIVE_MAX_ENCRYPTED_BITS_PER_REQUEST,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeReplayKeyV0 {
    pub host_chain_id: u64,
    pub solana_cluster_id: [u8; 32],
    pub kms_context_id: [u8; 32],
    pub request_signer_pubkey: SolanaPubkeyBytes,
    pub nonce: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeAcceptedRequestV0 {
    pub request_hash: [u8; 32],
    pub replay_key: Option<SolanaNativeReplayKeyV0>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolanaNativeReplayAction {
    Reserve,
    ReuseExisting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaAclVerifier {
    pub host_program_id: SolanaPubkeyBytes,
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaNativeRequestError {
    #[error("native Solana request has no handle entries")]
    EmptyEntries,
    #[error("native Solana request contains duplicate handle entries")]
    DuplicateHandle,
    #[error("native Solana request exceeds configured limits")]
    RequestTooLarge,
    #[error("native Solana request has an unsupported request mode")]
    UnsupportedRequestMode,
    #[error("native Solana request references a different ACL program id")]
    AclProgramMismatch,
    #[error("native Solana request domain separator is invalid")]
    DomainSeparatorMismatch,
    #[error("native Solana request entries hash is invalid")]
    EntriesHashMismatch,
    #[error("native Solana request extra data is invalid")]
    InvalidExtraData,
    #[error("native Solana request extra data hash is invalid")]
    ExtraDataHashMismatch,
    #[error("native Solana request KMS context id is invalid")]
    KmsContextMismatch,
    #[error("native Solana request reencryption public key is invalid")]
    ReencryptionPublicKeyMismatch,
    #[error("native Solana request nonce is invalid for its mode")]
    InvalidNonce,
    #[error("native Solana request expired or references a future context slot")]
    SlotWindowInvalid,
    #[error("native Solana request signature is missing or invalid")]
    InvalidRequestSignature,
    #[error("native Solana request key id is invalid")]
    InvalidKeyId,
    #[error("native Solana request profile fields are invalid or unsupported")]
    InvalidProfile,
    #[error("native Solana request handle metadata is invalid")]
    InvalidHandleMetadata,
    #[error("native Solana request batch fields are not uniform")]
    BatchNotUniform,
    #[error("native Solana request role fields are invalid for its mode")]
    RoleFieldsInvalid,
    #[error("native Solana request account fields do not match supplied witnesses")]
    WitnessAccountMismatch,
    #[error("native Solana request uses an unsupported account layout or material version")]
    UnsupportedVersion,
    #[error("native Solana ACL witness verification failed: {0}")]
    Acl(#[from] SolanaAclVerificationError),
    #[error("native Solana replay key was already used for a different request")]
    ReplayDetected,
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
        handle: HandleBytes,
        delegator: SolanaPubkeyBytes,
        delegate: SolanaPubkeyBytes,
        app_account: SolanaPubkeyBytes,
        expected_delegation_counter: u64,
        observed_slot: u64,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_acl_record(record, handle)?;
        if !allowed_acl_domain_keys.contains(&record.acl_domain_key) {
            return Err(SolanaAclVerificationError::DomainNotAllowed);
        }
        self.verify_subject_role(record, overflow_permissions, delegator, ACL_ROLE_USE)?;
        self.verify_delegation(
            delegation,
            delegator,
            delegate,
            app_account,
            expected_delegation_counter,
            observed_slot,
        )
    }

    pub fn verify_delegated_user_decrypt_with_material(
        &self,
        record: &AclRecordWitness,
        overflow_permissions: &[AclPermissionWitness],
        delegation: &UserDecryptionDelegationWitness,
        material: &HandleMaterialCommitmentWitness,
        handle: HandleBytes,
        delegator: SolanaPubkeyBytes,
        delegate: SolanaPubkeyBytes,
        app_account: SolanaPubkeyBytes,
        expected_delegation_counter: u64,
        observed_slot: u64,
        allowed_acl_domain_keys: &[SolanaPubkeyBytes],
    ) -> Result<(), SolanaAclVerificationError> {
        self.verify_delegated_user_decrypt(
            record,
            overflow_permissions,
            delegation,
            handle,
            delegator,
            delegate,
            app_account,
            expected_delegation_counter,
            observed_slot,
            allowed_acl_domain_keys,
        )?;
        self.verify_material_commitment(record, material, handle)
    }

    pub fn verify_native_v0_request(
        &self,
        request: &SolanaNativeDecryptionRequestV0,
        observed_slot: u64,
        limits: SolanaNativeRequestLimits,
    ) -> Result<SolanaNativeAcceptedRequestV0, SolanaNativeRequestError> {
        let payload = &request.payload;
        if request.entries.is_empty() {
            return Err(SolanaNativeRequestError::EmptyEntries);
        }
        if request.entries.len() > limits.max_handles_per_request {
            return Err(SolanaNativeRequestError::RequestTooLarge);
        }
        for (index, entry) in request.entries.iter().enumerate() {
            if request.entries[index + 1..]
                .iter()
                .any(|other| other.handle == entry.handle)
            {
                return Err(SolanaNativeRequestError::DuplicateHandle);
            }
        }
        if payload.acl_program_id != self.host_program_id {
            return Err(SolanaNativeRequestError::AclProgramMismatch);
        }
        self.verify_native_payload_profile(payload)?;
        if payload.domain_separator
            != solana_native_domain_separator(
                payload.host_chain_id,
                payload.solana_cluster_id,
                payload.acl_program_id,
                payload.kms_context_id,
            )
        {
            return Err(SolanaNativeRequestError::DomainSeparatorMismatch);
        }
        if payload.entries_hash != solana_native_entries_hash(&request.entries) {
            return Err(SolanaNativeRequestError::EntriesHashMismatch);
        }
        let expected_key_id = request.entries[0].expected_key_id;
        if expected_key_id == [0; 32] {
            return Err(SolanaNativeRequestError::InvalidKeyId);
        }
        if request
            .entries
            .iter()
            .any(|entry| entry.expected_key_id != expected_key_id)
        {
            return Err(SolanaNativeRequestError::BatchNotUniform);
        }
        let encrypted_bits = request.entries.iter().try_fold(0usize, |total, entry| {
            let entry_bits =
                solana_native_handle_encrypted_bits(entry.handle, payload.host_chain_id)
                    .ok_or(SolanaNativeRequestError::InvalidHandleMetadata)?;
            total
                .checked_add(entry_bits)
                .ok_or(SolanaNativeRequestError::RequestTooLarge)
        })?;
        if encrypted_bits > limits.max_encrypted_bits_per_request {
            return Err(SolanaNativeRequestError::RequestTooLarge);
        }

        let decoded_extra_data = decode_solana_kms_extra_data_v0(&request.raw_extra_data, limits)?;
        if decoded_extra_data.kms_context_id != payload.kms_context_id {
            return Err(SolanaNativeRequestError::KmsContextMismatch);
        }
        if payload.extra_data_hash != solana_native_extra_data_hash(&request.raw_extra_data) {
            return Err(SolanaNativeRequestError::ExtraDataHashMismatch);
        }
        self.verify_native_reencryption_key(
            payload,
            &request.user_reencryption_public_key,
            limits,
        )?;

        if payload.min_context_slot > observed_slot || payload.expiration_slot < observed_slot {
            return Err(SolanaNativeRequestError::SlotWindowInvalid);
        }

        match payload.request_mode {
            SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED => {
                self.verify_native_direct_request(payload, &request.entries, observed_slot)?;
            }
            SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED
            | SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED => {
                self.verify_native_delegated_request(payload, &request.entries, observed_slot)?;
            }
            SOLANA_NATIVE_REQUEST_MODE_PUBLIC => {
                self.verify_native_public_request(payload, &request.entries, observed_slot)?;
            }
            _ => return Err(SolanaNativeRequestError::UnsupportedRequestMode),
        }

        let request_hash = solana_native_request_hash(payload);
        let replay_key = if payload.request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            None
        } else {
            Some(SolanaNativeReplayKeyV0 {
                host_chain_id: payload.host_chain_id,
                solana_cluster_id: payload.solana_cluster_id,
                kms_context_id: payload.kms_context_id,
                request_signer_pubkey: payload.request_signer_pubkey,
                nonce: payload.nonce,
            })
        };
        Ok(SolanaNativeAcceptedRequestV0 {
            request_hash,
            replay_key,
        })
    }

    fn verify_native_payload_profile(
        &self,
        payload: &SolanaUserDecryptionPayloadV0,
    ) -> Result<(), SolanaNativeRequestError> {
        if payload.host_chain_id == 0
            || payload.config_version == 0
            || payload.solana_cluster_id == [0; 32]
            || payload.kms_context_id == [0; 32]
            || payload.material_source_mode != SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE
            || !matches!(
                payload.commitment_level,
                SOLANA_NATIVE_COMMITMENT_CONFIRMED | SOLANA_NATIVE_COMMITMENT_FINALIZED
            )
        {
            return Err(SolanaNativeRequestError::InvalidProfile);
        }
        Ok(())
    }

    fn verify_native_reencryption_key(
        &self,
        payload: &SolanaUserDecryptionPayloadV0,
        raw_key: &[u8],
        limits: SolanaNativeRequestLimits,
    ) -> Result<(), SolanaNativeRequestError> {
        if payload.request_mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            if payload.user_reencryption_pubkey_len != 0
                || payload.user_reencryption_pubkey_hash != [0; 32]
                || !raw_key.is_empty()
            {
                return Err(SolanaNativeRequestError::ReencryptionPublicKeyMismatch);
            }
            return Ok(());
        }

        if raw_key.is_empty()
            || raw_key.len() > limits.max_user_reencryption_pubkey_bytes
            || payload.user_reencryption_pubkey_len as usize != raw_key.len()
            || payload.user_reencryption_pubkey_hash
                != solana_native_reencryption_pubkey_hash(raw_key)
            || payload.user_reencryption_pubkey_hash == [0; 32]
        {
            return Err(SolanaNativeRequestError::ReencryptionPublicKeyMismatch);
        }
        Ok(())
    }

    fn verify_native_direct_request(
        &self,
        payload: &SolanaUserDecryptionPayloadV0,
        entries: &[SolanaNativeHandleEntryV0],
        observed_slot: u64,
    ) -> Result<(), SolanaNativeRequestError> {
        if payload.request_signer_pubkey == [0; 32] || payload.nonce == [0; 32] {
            return Err(SolanaNativeRequestError::InvalidNonce);
        }
        let first = &entries[0];
        if first.owner_pubkey == [0; 32]
            || first.app_context_pubkey == [0; 32]
            || first.owner_pubkey == first.app_context_pubkey
            || payload.request_signer_pubkey != first.owner_pubkey
        {
            return Err(SolanaNativeRequestError::RoleFieldsInvalid);
        }

        for entry in entries {
            self.verify_native_common_entry(payload, entry, observed_slot)?;
            if entry.owner_pubkey != first.owner_pubkey
                || entry.app_context_pubkey != first.app_context_pubkey
                || entry.delegator_pubkey != [0; 32]
                || entry.delegate_pubkey != [0; 32]
                || entry.delegation.is_some()
                || entry.delegation_record_account != [0; 32]
                || entry.expected_delegation_counter != 0
            {
                return Err(SolanaNativeRequestError::BatchNotUniform);
            }
            self.verify_native_subject(
                &entry.acl_record,
                entry.owner_permission.as_ref(),
                entry.owner_pubkey,
                entry.owner_permission_account,
            )?;
            self.verify_native_subject(
                &entry.acl_record,
                entry.app_context_permission.as_ref(),
                entry.app_context_pubkey,
                entry.app_context_permission_account,
            )?;
            if entry.delegator_permission.is_some() || entry.delegator_permission_account != [0; 32]
            {
                return Err(SolanaNativeRequestError::RoleFieldsInvalid);
            }
        }
        Ok(())
    }

    fn verify_native_delegated_request(
        &self,
        payload: &SolanaUserDecryptionPayloadV0,
        entries: &[SolanaNativeHandleEntryV0],
        observed_slot: u64,
    ) -> Result<(), SolanaNativeRequestError> {
        if payload.request_signer_pubkey == [0; 32] || payload.nonce == [0; 32] {
            return Err(SolanaNativeRequestError::InvalidNonce);
        }
        let first = &entries[0];
        if first.delegator_pubkey == [0; 32]
            || first.delegate_pubkey == [0; 32]
            || first.delegate_pubkey == WILDCARD_APP_CONTEXT
            || first.app_context_pubkey == [0; 32]
            || first.delegator_pubkey == first.app_context_pubkey
            || first.delegator_pubkey == first.delegate_pubkey
            || first.expected_delegation_counter == 0
            || payload.request_signer_pubkey != first.delegate_pubkey
        {
            return Err(SolanaNativeRequestError::RoleFieldsInvalid);
        }
        if payload.request_mode == SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED
            && first.delegate_pubkey == first.app_context_pubkey
        {
            return Err(SolanaNativeRequestError::RoleFieldsInvalid);
        }

        let expected_delegation_app =
            if payload.request_mode == SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED {
                WILDCARD_APP_CONTEXT
            } else {
                first.app_context_pubkey
            };

        for entry in entries {
            self.verify_native_common_entry(payload, entry, observed_slot)?;
            if entry.owner_pubkey != [0; 32]
                || entry.owner_permission.is_some()
                || entry.owner_permission_account != [0; 32]
                || entry.delegator_pubkey != first.delegator_pubkey
                || entry.delegate_pubkey != first.delegate_pubkey
                || entry.app_context_pubkey != first.app_context_pubkey
                || entry.expected_delegation_counter != first.expected_delegation_counter
            {
                return Err(SolanaNativeRequestError::BatchNotUniform);
            }
            self.verify_native_subject(
                &entry.acl_record,
                entry.delegator_permission.as_ref(),
                entry.delegator_pubkey,
                entry.delegator_permission_account,
            )?;
            self.verify_native_subject(
                &entry.acl_record,
                entry.app_context_permission.as_ref(),
                entry.app_context_pubkey,
                entry.app_context_permission_account,
            )?;
            let delegation = entry
                .delegation
                .as_ref()
                .ok_or(SolanaNativeRequestError::WitnessAccountMismatch)?;
            if entry.delegation_record_account != delegation.account_key {
                return Err(SolanaNativeRequestError::WitnessAccountMismatch);
            }
            self.verify_delegation(
                delegation,
                entry.delegator_pubkey,
                entry.delegate_pubkey,
                expected_delegation_app,
                entry.expected_delegation_counter,
                observed_slot,
            )?;
        }
        Ok(())
    }

    fn verify_native_public_request(
        &self,
        payload: &SolanaUserDecryptionPayloadV0,
        entries: &[SolanaNativeHandleEntryV0],
        observed_slot: u64,
    ) -> Result<(), SolanaNativeRequestError> {
        if payload.request_signer_pubkey != [0; 32] || payload.nonce != [0; 32] {
            return Err(SolanaNativeRequestError::RoleFieldsInvalid);
        }
        for entry in entries {
            self.verify_native_common_entry(payload, entry, observed_slot)?;
            if entry.owner_pubkey != [0; 32]
                || entry.owner_permission_account != [0; 32]
                || entry.owner_permission.is_some()
                || entry.delegator_pubkey != [0; 32]
                || entry.delegator_permission_account != [0; 32]
                || entry.delegator_permission.is_some()
                || entry.delegate_pubkey != [0; 32]
                || entry.app_context_pubkey != [0; 32]
                || entry.app_context_permission_account != [0; 32]
                || entry.app_context_permission.is_some()
                || entry.delegation_record_account != [0; 32]
                || entry.delegation.is_some()
                || entry.expected_delegation_counter != 0
            {
                return Err(SolanaNativeRequestError::RoleFieldsInvalid);
            }
            self.verify_public_decrypt_with_material(
                &entry.acl_record,
                &entry.material,
                entry.handle,
            )?;
        }
        Ok(())
    }

    fn verify_native_common_entry(
        &self,
        payload: &SolanaUserDecryptionPayloadV0,
        entry: &SolanaNativeHandleEntryV0,
        observed_slot: u64,
    ) -> Result<(), SolanaNativeRequestError> {
        if entry.config_version != payload.config_version
            || entry.material_source_mode != payload.material_source_mode
        {
            return Err(SolanaNativeRequestError::BatchNotUniform);
        }
        if entry.acl_layout_version != SOLANA_NATIVE_SUPPORTED_ACL_LAYOUT_VERSION
            || entry.handle_derivation_version != SOLANA_NATIVE_SUPPORTED_HANDLE_DERIVATION_VERSION
            || entry.material_commitment_version
                != SOLANA_NATIVE_SUPPORTED_MATERIAL_COMMITMENT_VERSION
        {
            return Err(SolanaNativeRequestError::UnsupportedVersion);
        }
        if entry.min_context_slot > observed_slot {
            return Err(SolanaNativeRequestError::SlotWindowInvalid);
        }
        if entry.acl_record_account != entry.acl_record.account_key
            || entry.material_commitment_account != entry.material.account_key
            || entry.material_commitment_hash != entry.material.material_commitment_hash
            || entry.expected_key_id != entry.material.key_id
        {
            return Err(SolanaNativeRequestError::WitnessAccountMismatch);
        }
        self.verify_acl_record(&entry.acl_record, entry.handle)?;
        self.verify_material_commitment(&entry.acl_record, &entry.material, entry.handle)?;
        if entry.acl_record.created_slot > observed_slot
            || entry.material.created_slot > observed_slot
        {
            return Err(SolanaNativeRequestError::SlotWindowInvalid);
        }
        Ok(())
    }

    fn verify_native_subject(
        &self,
        record: &AclRecordWitness,
        permission: Option<&AclPermissionWitness>,
        subject: SolanaPubkeyBytes,
        permission_account: SolanaPubkeyBytes,
    ) -> Result<(), SolanaNativeRequestError> {
        if let Some(inline) = record
            .subjects
            .iter()
            .find(|entry| entry.subject == subject)
        {
            if permission_account != [0; 32] || permission.is_some() {
                return Err(SolanaNativeRequestError::WitnessAccountMismatch);
            }
            require_role(inline.role_flags, ACL_ROLE_USE)?;
            return Ok(());
        }
        let permission = permission.ok_or(SolanaNativeRequestError::WitnessAccountMismatch)?;
        if permission_account == [0; 32] || permission.account_key != permission_account {
            return Err(SolanaNativeRequestError::WitnessAccountMismatch);
        }
        self.verify_subject_role(
            record,
            std::slice::from_ref(permission),
            subject,
            ACL_ROLE_USE,
        )?;
        Ok(())
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

        for permission in overflow_permissions
            .iter()
            .filter(|permission| permission.subject == subject)
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

pub fn encode_solana_kms_extra_data_v0(extra_data: &SolanaKmsExtraDataV0) -> Vec<u8> {
    let mut output = Vec::with_capacity(1 + 32 + 4 + extra_data.response_context.len());
    output.push(SOLANA_NATIVE_EXTRA_DATA_LAYOUT_V0);
    output.extend_from_slice(&extra_data.kms_context_id);
    output.extend_from_slice(&(extra_data.response_context.len() as u32).to_le_bytes());
    output.extend_from_slice(&extra_data.response_context);
    output
}

pub fn decode_solana_kms_extra_data_v0(
    raw_extra_data: &[u8],
    limits: SolanaNativeRequestLimits,
) -> Result<SolanaKmsExtraDataV0, SolanaNativeRequestError> {
    if raw_extra_data.len() > limits.max_extra_data_bytes || raw_extra_data.len() < 37 {
        return Err(SolanaNativeRequestError::InvalidExtraData);
    }
    if raw_extra_data[0] != SOLANA_NATIVE_EXTRA_DATA_LAYOUT_V0 {
        return Err(SolanaNativeRequestError::InvalidExtraData);
    }
    let mut kms_context_id = [0; 32];
    kms_context_id.copy_from_slice(&raw_extra_data[1..33]);
    if kms_context_id == [0; 32] {
        return Err(SolanaNativeRequestError::KmsContextMismatch);
    }
    let response_context_len = u32::from_le_bytes(
        raw_extra_data[33..37]
            .try_into()
            .expect("slice has 4 bytes"),
    ) as usize;
    if raw_extra_data.len() != 37 + response_context_len {
        return Err(SolanaNativeRequestError::InvalidExtraData);
    }
    Ok(SolanaKmsExtraDataV0 {
        kms_context_id,
        response_context: raw_extra_data[37..].to_vec(),
    })
}

pub fn solana_native_domain_separator(
    host_chain_id: u64,
    solana_cluster_id: [u8; 32],
    acl_program_id: SolanaPubkeyBytes,
    kms_context_id: [u8; 32],
) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-user-decryption-domain-v0");
    hasher.update(host_chain_id.to_le_bytes());
    hasher.update(solana_cluster_id);
    hasher.update(acl_program_id);
    hasher.update(kms_context_id);
    hasher.finalize().into()
}

pub fn solana_native_reencryption_pubkey_hash(raw_key: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-user-public-key-v0");
    hasher.update((raw_key.len() as u32).to_le_bytes());
    hasher.update(raw_key);
    hasher.finalize().into()
}

pub fn solana_native_extra_data_hash(raw_extra_data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-extra-data-v0");
    hasher.update((raw_extra_data.len() as u32).to_le_bytes());
    hasher.update(raw_extra_data);
    hasher.finalize().into()
}

pub fn solana_native_handle_encrypted_bits(
    handle: HandleBytes,
    expected_host_chain_id: u64,
) -> Option<usize> {
    if solana_native_handle_chain_id(handle) != expected_host_chain_id
        || handle[31] != SOLANA_NATIVE_SUPPORTED_HANDLE_VERSION
    {
        return None;
    }
    solana_native_fhe_type_encrypted_bits(handle[30])
}

fn solana_native_handle_chain_id(handle: HandleBytes) -> u64 {
    let mut chain_id = [0u8; 8];
    chain_id.copy_from_slice(&handle[22..30]);
    u64::from_be_bytes(chain_id)
}

fn solana_native_fhe_type_encrypted_bits(fhe_type: u8) -> Option<usize> {
    match fhe_type {
        0 => Some(2),
        2 => Some(8),
        3 => Some(16),
        4 => Some(32),
        5 => Some(64),
        6 => Some(128),
        7 => Some(160),
        8 => Some(256),
        _ => None,
    }
}

pub fn solana_native_entries_hash(entries: &[SolanaNativeHandleEntryV0]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-handle-entries-v0");
    hasher.update((entries.len() as u32).to_le_bytes());
    for entry in entries {
        hash_solana_native_entry(&mut hasher, entry);
    }
    hasher.finalize().into()
}

pub fn solana_native_request_hash(payload: &SolanaUserDecryptionPayloadV0) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-kms-request-v0");
    hasher.update(payload.domain_separator);
    hasher.update(payload.host_chain_id.to_le_bytes());
    hasher.update(payload.config_version.to_le_bytes());
    hasher.update(payload.solana_cluster_id);
    hasher.update(payload.kms_context_id);
    hasher.update(payload.user_reencryption_pubkey_len.to_le_bytes());
    hasher.update(payload.user_reencryption_pubkey_hash);
    hasher.update(payload.request_signer_pubkey);
    hasher.update(payload.acl_program_id);
    hasher.update([payload.request_mode]);
    hasher.update([payload.material_source_mode]);
    hasher.update([payload.commitment_level]);
    hasher.update(payload.min_context_slot.to_le_bytes());
    hasher.update(payload.expiration_slot.to_le_bytes());
    hasher.update(payload.nonce);
    hasher.update(payload.extra_data_hash);
    hasher.update(payload.entries_hash);
    hasher.finalize().into()
}

pub fn solana_native_request_signature_message(request_hash: [u8; 32]) -> Vec<u8> {
    solana_native_signature_message("zama-solana-user-decryption-signature-v0", request_hash)
}

pub(crate) fn solana_native_update_ascii(hasher: &mut Keccak256, value: &str) {
    let value = value.as_bytes();
    let len = u16::try_from(value.len()).expect("native-v0 ASCII domain length fits u16");
    hasher.update(len.to_le_bytes());
    hasher.update(value);
}

pub(crate) fn solana_native_signature_message(domain: &str, hash: [u8; 32]) -> Vec<u8> {
    let domain = domain.as_bytes();
    let domain_len =
        u16::try_from(domain.len()).expect("native-v0 signature domain length fits u16");
    let mut output = Vec::with_capacity(2 + domain.len() + 32);
    output.extend_from_slice(&domain_len.to_le_bytes());
    output.extend_from_slice(domain);
    output.extend_from_slice(&hash);
    output
}

pub fn verify_solana_native_request_signature(
    request_signer_pubkey: SolanaPubkeyBytes,
    request_hash: [u8; 32],
    signature: &[u8],
) -> Result<(), SolanaNativeRequestError> {
    if signature.len() != SOLANA_NATIVE_ED25519_SIGNATURE_LEN {
        return Err(SolanaNativeRequestError::InvalidRequestSignature);
    }
    let message = solana_native_request_signature_message(request_hash);
    ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, request_signer_pubkey)
        .verify(&message, signature)
        .map_err(|_| SolanaNativeRequestError::InvalidRequestSignature)
}

pub fn check_solana_native_replay(
    existing_request_hash: Option<[u8; 32]>,
    request_hash: [u8; 32],
) -> Result<SolanaNativeReplayAction, SolanaNativeRequestError> {
    match existing_request_hash {
        None => Ok(SolanaNativeReplayAction::Reserve),
        Some(existing) if existing == request_hash => Ok(SolanaNativeReplayAction::ReuseExisting),
        Some(_) => Err(SolanaNativeRequestError::ReplayDetected),
    }
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

fn hash_solana_native_entry(hasher: &mut Keccak256, entry: &SolanaNativeHandleEntryV0) {
    hasher.update(entry.handle);
    hasher.update(entry.owner_pubkey);
    hasher.update(entry.owner_permission_account);
    hasher.update(entry.delegator_pubkey);
    hasher.update(entry.delegator_permission_account);
    hasher.update(entry.delegate_pubkey);
    hasher.update(entry.app_context_pubkey);
    hasher.update(entry.app_context_permission_account);
    hasher.update(entry.acl_record_account);
    hasher.update(entry.delegation_record_account);
    hasher.update(entry.expected_delegation_counter.to_le_bytes());
    hasher.update(entry.material_commitment_account);
    hasher.update(entry.material_commitment_hash);
    hasher.update(entry.min_context_slot.to_le_bytes());
    hasher.update(entry.config_version.to_le_bytes());
    hasher.update([entry.material_source_mode]);
    hasher.update([entry.acl_layout_version]);
    hasher.update([entry.handle_derivation_version]);
    hasher.update([entry.material_commitment_version]);
    hasher.update(entry.expected_key_id);
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
        Ok(u64::from_le_bytes(
            bytes.try_into().expect("slice has 8 bytes"),
        ))
    }

    fn read_u32(&mut self) -> Result<u32, SolanaAclVerificationError> {
        let bytes = self.read_exact(4)?;
        Ok(u32::from_le_bytes(
            bytes.try_into().expect("slice has 4 bytes"),
        ))
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
    use ring::signature::KeyPair;

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const HOST_CHAIN_ID: u64 = 900;
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

    fn hex32(value: &str) -> [u8; 32] {
        assert_eq!(value.len(), 64);
        let mut output = [0u8; 32];
        for (index, byte) in output.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).unwrap();
        }
        output
    }

    fn hex_bytes(value: &str) -> Vec<u8> {
        assert_eq!(value.len() % 2, 0);
        value
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let hex = std::str::from_utf8(pair).unwrap();
                u8::from_str_radix(hex, 16).unwrap()
            })
            .collect()
    }

    fn test_handle(tag: u8, fhe_type: u8) -> HandleBytes {
        let mut handle = [tag; 32];
        handle[22..30].copy_from_slice(&HOST_CHAIN_ID.to_be_bytes());
        handle[30] = fhe_type;
        handle[31] = SOLANA_NATIVE_SUPPORTED_HANDLE_VERSION;
        handle
    }

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

    fn record_variant(
        handle: HandleBytes,
        nonce_sequence: u64,
        material_key_id: [u8; 32],
    ) -> AclRecordWitness {
        let mut record = base_record();
        let (account_key, bump) =
            acl_record_address(HOST_PROGRAM_ID, record.nonce_key, nonce_sequence);
        let (material_commitment, _) = handle_material_address(HOST_PROGRAM_ID, account_key);
        record.handle = handle;
        record.account_key = account_key;
        record.nonce_sequence = nonce_sequence;
        record.material_commitment = material_commitment;
        record.material_commitment_hash = handle_material_commitment_hash(
            HOST_PROGRAM_ID,
            material_commitment,
            account_key,
            material_key_id,
            [22; 32],
            [23; 32],
            [24; 32],
        );
        record.material_key_id = material_key_id;
        record.bump = bump;
        record
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

    fn wildcard_delegation() -> UserDecryptionDelegationWitness {
        delegation_for_app(WILDCARD_APP_CONTEXT)
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

    fn native_entry(
        record: AclRecordWitness,
        material: HandleMaterialCommitmentWitness,
        mode: u8,
    ) -> SolanaNativeHandleEntryV0 {
        let (owner, delegator, delegate, app_context, delegation_record, counter, delegation) =
            match mode {
                SOLANA_NATIVE_REQUEST_MODE_PUBLIC => {
                    ([0; 32], [0; 32], [0; 32], [0; 32], [0; 32], 0, None)
                }
                SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED => {
                    (OWNER, [0; 32], [0; 32], APP_ACCOUNT, [0; 32], 0, None)
                }
                SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED => {
                    let delegation = delegation();
                    (
                        [0; 32],
                        OWNER,
                        DELEGATE,
                        APP_ACCOUNT,
                        delegation.account_key,
                        delegation.delegation_counter,
                        Some(delegation),
                    )
                }
                SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED => {
                    let delegation = wildcard_delegation();
                    (
                        [0; 32],
                        OWNER,
                        DELEGATE,
                        APP_ACCOUNT,
                        delegation.account_key,
                        delegation.delegation_counter,
                        Some(delegation),
                    )
                }
                _ => unreachable!("unsupported test mode"),
            };

        SolanaNativeHandleEntryV0 {
            handle: record.handle,
            owner_pubkey: owner,
            owner_permission_account: [0; 32],
            delegator_pubkey: delegator,
            delegator_permission_account: [0; 32],
            delegate_pubkey: delegate,
            app_context_pubkey: app_context,
            app_context_permission_account: [0; 32],
            acl_record_account: record.account_key,
            delegation_record_account: delegation_record,
            expected_delegation_counter: counter,
            material_commitment_account: material.account_key,
            material_commitment_hash: material.material_commitment_hash,
            min_context_slot: OBSERVED_SLOT - 2,
            config_version: 3,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            acl_layout_version: SOLANA_NATIVE_SUPPORTED_ACL_LAYOUT_VERSION,
            handle_derivation_version: SOLANA_NATIVE_SUPPORTED_HANDLE_DERIVATION_VERSION,
            material_commitment_version: SOLANA_NATIVE_SUPPORTED_MATERIAL_COMMITMENT_VERSION,
            expected_key_id: material.key_id,
            acl_record: record,
            material,
            owner_permission: None,
            delegator_permission: None,
            app_context_permission: None,
            delegation,
        }
    }

    fn native_request(mode: u8) -> SolanaNativeDecryptionRequestV0 {
        let record = base_record();
        let material = material_commitment(&record);
        let entries = vec![native_entry(record, material, mode)];
        let kms_context_id = [8; 32];
        let raw_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id,
            response_context: b"context".to_vec(),
        });
        let user_reencryption_public_key = if mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            Vec::new()
        } else {
            b"reencryption-key".to_vec()
        };
        let user_reencryption_pubkey_hash = if user_reencryption_public_key.is_empty() {
            [0; 32]
        } else {
            solana_native_reencryption_pubkey_hash(&user_reencryption_public_key)
        };
        let request_signer_pubkey = match mode {
            SOLANA_NATIVE_REQUEST_MODE_PUBLIC => [0; 32],
            SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED => OWNER,
            SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED
            | SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED => DELEGATE,
            _ => unreachable!("unsupported test mode"),
        };
        let nonce = if mode == SOLANA_NATIVE_REQUEST_MODE_PUBLIC {
            [0; 32]
        } else {
            [77; 32]
        };
        let payload = SolanaUserDecryptionPayloadV0 {
            domain_separator: solana_native_domain_separator(
                HOST_CHAIN_ID,
                [9; 32],
                HOST_PROGRAM_ID,
                kms_context_id,
            ),
            host_chain_id: HOST_CHAIN_ID,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id,
            user_reencryption_pubkey_len: user_reencryption_public_key.len() as u32,
            user_reencryption_pubkey_hash,
            request_signer_pubkey,
            acl_program_id: HOST_PROGRAM_ID,
            request_mode: mode,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: 1,
            min_context_slot: OBSERVED_SLOT - 2,
            expiration_slot: OBSERVED_SLOT + 20,
            nonce,
            extra_data_hash: solana_native_extra_data_hash(&raw_extra_data),
            entries_hash: solana_native_entries_hash(&entries),
        };
        SolanaNativeDecryptionRequestV0 {
            payload,
            entries,
            raw_extra_data,
            user_reencryption_public_key,
        }
    }

    fn replace_native_delegation(
        request: &mut SolanaNativeDecryptionRequestV0,
        delegation: UserDecryptionDelegationWitness,
    ) {
        request.entries[0].delegation_record_account = delegation.account_key;
        request.entries[0].expected_delegation_counter = delegation.delegation_counter;
        request.entries[0].delegation = Some(delegation);
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);
    }

    #[test]
    fn native_v0_request_hash_helpers_match_spec_vectors() {
        let domain_separator = solana_native_domain_separator(900, [9; 32], [42; 32], [8; 32]);
        assert_eq!(
            domain_separator,
            hex32("cef4f5bcd72d6ce7b41ff0644fac3c19ab8b9d4ea376cd4f47e9f1f636318d3e")
        );
        assert_ne!(
            domain_separator,
            solana_native_domain_separator(900, [9; 32], [42; 32], [7; 32])
        );

        let reencryption_key = b"reencryption-key".to_vec();
        let user_reencryption_pubkey_hash =
            solana_native_reencryption_pubkey_hash(&reencryption_key);
        assert_eq!(
            user_reencryption_pubkey_hash,
            hex32("b912b7cb96960cad5718c858e4bf6cb7176cbcf96807607561037061a38567fc")
        );

        let extra_data_hash = solana_native_extra_data_hash(b"extra");
        assert_eq!(
            extra_data_hash,
            hex32("ae6b7cb429d69f3b06bd367b8362c44e9b5be702a2788a5136925333dddeb1c9")
        );

        let entries_hash = solana_native_entries_hash(&[]);
        assert_eq!(
            entries_hash,
            hex32("17af075816b9871ecaa47d89d5bdc192e6bedfa1842aa49cef988f59e2eb60c4")
        );

        let payload = SolanaUserDecryptionPayloadV0 {
            domain_separator,
            host_chain_id: 900,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            user_reencryption_pubkey_len: reencryption_key.len() as u32,
            user_reencryption_pubkey_hash,
            request_signer_pubkey: [7; 32],
            acl_program_id: [42; 32],
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: 2,
            min_context_slot: 10,
            expiration_slot: 20,
            nonce: [6; 32],
            extra_data_hash,
            entries_hash,
        };
        assert_eq!(
            solana_native_request_hash(&payload),
            hex32("b435c1d350e62d5b945dc8acba09cdf4d300aaf9e2073b4fe53cee3f56d704b2")
        );
        assert_eq!(
            solana_native_request_signature_message(solana_native_request_hash(&payload)),
            hex_bytes(
                "28007a616d612d736f6c616e612d757365722d64656372797074696f6e2d7369676e61747572652d7630b435c1d350e62d5b945dc8acba09cdf4d300aaf9e2073b4fe53cee3f56d704b2"
            )
        );
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
                HANDLE,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
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
                HANDLE,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
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
                HANDLE,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
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
                HANDLE,
                OWNER,
                WILDCARD_APP_CONTEXT,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
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
                HANDLE,
                OWNER,
                APP_ACCOUNT,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
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
                HANDLE,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
            ),
            Err(SolanaAclVerificationError::NonCanonicalDelegation)
        );

        assert_eq!(
            verifier.verify_delegated_user_decrypt(
                &base_record(),
                &[],
                &delegation(),
                HANDLE,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                8,
                OBSERVED_SLOT,
                &[DOMAIN],
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
                HANDLE,
                OWNER,
                DELEGATE,
                APP_ACCOUNT,
                9,
                OBSERVED_SLOT,
                &[DOMAIN],
            ),
            Err(SolanaAclVerificationError::DelegationNotActive)
        );
    }

    #[test]
    fn verifies_native_v0_direct_request_and_replay_decision() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);

        let accepted = verifier
            .verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default(),
            )
            .unwrap();

        assert_eq!(
            accepted.replay_key,
            Some(SolanaNativeReplayKeyV0 {
                host_chain_id: request.payload.host_chain_id,
                solana_cluster_id: request.payload.solana_cluster_id,
                kms_context_id: request.payload.kms_context_id,
                request_signer_pubkey: OWNER,
                nonce: [77; 32],
            })
        );
        assert_eq!(
            check_solana_native_replay(None, accepted.request_hash),
            Ok(SolanaNativeReplayAction::Reserve)
        );
        assert_eq!(
            check_solana_native_replay(Some(accepted.request_hash), accepted.request_hash),
            Ok(SolanaNativeReplayAction::ReuseExisting)
        );
        assert_eq!(
            check_solana_native_replay(Some([99; 32]), accepted.request_hash),
            Err(SolanaNativeRequestError::ReplayDetected)
        );
    }

    #[test]
    fn verifies_native_v0_request_signature() {
        let seed = [11; 32];
        let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(&seed).unwrap();
        let request_hash = [29; 32];
        let signature = key_pair.sign(&solana_native_request_signature_message(request_hash));
        let public_key: SolanaPubkeyBytes = key_pair.public_key().as_ref().try_into().unwrap();

        assert_eq!(
            verify_solana_native_request_signature(public_key, request_hash, signature.as_ref()),
            Ok(())
        );

        let mut tampered_signature = signature.as_ref().to_vec();
        tampered_signature[0] ^= 0xff;
        assert_eq!(
            verify_solana_native_request_signature(public_key, request_hash, &tampered_signature),
            Err(SolanaNativeRequestError::InvalidRequestSignature)
        );

        assert_eq!(
            verify_solana_native_request_signature(
                public_key,
                request_hash,
                &signature.as_ref()[..63]
            ),
            Err(SolanaNativeRequestError::InvalidRequestSignature)
        );
    }

    #[test]
    fn verifies_native_v0_delegated_request() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        verifier
            .verify_native_v0_request(
                &native_request(SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED),
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default(),
            )
            .unwrap();
    }

    #[test]
    fn verifies_native_v0_delegated_wildcard_request() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let request = native_request(SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED);

        assert_eq!(request.entries[0].app_context_pubkey, APP_ACCOUNT);
        assert_eq!(
            request.entries[0].delegation.as_ref().unwrap().app_account,
            WILDCARD_APP_CONTEXT
        );

        let accepted = verifier
            .verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default(),
            )
            .unwrap();

        assert_eq!(
            accepted.replay_key,
            Some(SolanaNativeReplayKeyV0 {
                host_chain_id: request.payload.host_chain_id,
                solana_cluster_id: request.payload.solana_cluster_id,
                kms_context_id: request.payload.kms_context_id,
                request_signer_pubkey: DELEGATE,
                nonce: [77; 32],
            })
        );
    }

    #[test]
    fn native_v0_wildcard_request_rejects_scoped_delegation_record() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED);
        replace_native_delegation(&mut request, delegation());

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::Acl(
                SolanaAclVerificationError::DelegationMismatch
            ))
        );
    }

    #[test]
    fn native_v0_scoped_request_rejects_wildcard_delegation_record() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED);
        replace_native_delegation(&mut request, wildcard_delegation());

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::Acl(
                SolanaAclVerificationError::DelegationMismatch
            ))
        );
    }

    #[test]
    fn native_v0_rejects_wildcard_delegate_pubkey() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED);
        request.payload.request_signer_pubkey = WILDCARD_APP_CONTEXT;
        request.entries[0].delegate_pubkey = WILDCARD_APP_CONTEXT;
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::RoleFieldsInvalid)
        );
    }

    #[test]
    fn verifies_native_v0_public_request_without_replay_key() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let accepted = verifier
            .verify_native_v0_request(
                &native_request(SOLANA_NATIVE_REQUEST_MODE_PUBLIC),
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default(),
            )
            .unwrap();
        assert_eq!(accepted.replay_key, None);
    }

    #[test]
    fn native_v0_rejects_expired_public_request() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_PUBLIC);
        request.payload.expiration_slot = OBSERVED_SLOT - 1;

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::SlotWindowInvalid)
        );
    }

    #[test]
    fn native_v0_rejects_future_acl_record_created_slot() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.entries[0].acl_record.created_slot = OBSERVED_SLOT + 1;

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::SlotWindowInvalid)
        );
    }

    #[test]
    fn native_v0_rejects_future_material_created_slot() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.entries[0].material.created_slot = OBSERVED_SLOT + 1;

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::SlotWindowInvalid)
        );
    }

    #[test]
    fn native_v0_rejects_tampered_entries_hash() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.payload.entries_hash[0] ^= 0xff;

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::EntriesHashMismatch)
        );
    }

    #[test]
    fn native_v0_rejects_duplicate_handle_entries() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.entries.push(request.entries[0].clone());
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::DuplicateHandle)
        );
    }

    #[test]
    fn native_v0_rejects_mixed_key_id_batch() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let record = record_variant([88; 32], 9, [31; 32]);
        let material = material_commitment(&record);
        request.entries.push(native_entry(
            record,
            material,
            SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
        ));
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::BatchNotUniform)
        );
    }

    #[test]
    fn native_v0_rejects_zero_expected_key_id() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.entries[0].expected_key_id = [0; 32];
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::InvalidKeyId)
        );
    }

    #[test]
    fn native_v0_rejects_material_entry_binding_mismatches() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let cases: [(&str, fn(&mut SolanaNativeDecryptionRequestV0)); 3] = [
            ("material account", |request| {
                request.entries[0].material_commitment_account = [31; 32];
            }),
            ("material hash", |request| {
                request.entries[0].material_commitment_hash = [32; 32];
            }),
            ("key id", |request| {
                request.entries[0].expected_key_id = [33; 32];
            }),
        ];

        for (case, mutate) in cases {
            let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
            mutate(&mut request);
            request.payload.entries_hash = solana_native_entries_hash(&request.entries);

            assert_eq!(
                verifier.verify_native_v0_request(
                    &request,
                    OBSERVED_SLOT,
                    SolanaNativeRequestLimits::default()
                ),
                Err(SolanaNativeRequestError::WitnessAccountMismatch),
                "{case}"
            );
        }
    }

    #[test]
    fn native_v0_rejects_invalid_payload_profile_fields() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let cases: [fn(&mut SolanaNativeDecryptionRequestV0); 7] = [
            |request: &mut SolanaNativeDecryptionRequestV0| request.payload.host_chain_id = 0,
            |request: &mut SolanaNativeDecryptionRequestV0| request.payload.config_version = 0,
            |request: &mut SolanaNativeDecryptionRequestV0| {
                request.payload.solana_cluster_id = [0; 32]
            },
            |request: &mut SolanaNativeDecryptionRequestV0| {
                request.payload.kms_context_id = [0; 32]
            },
            |request: &mut SolanaNativeDecryptionRequestV0| {
                request.payload.material_source_mode = 99;
                request.entries[0].material_source_mode = 99;
                request.payload.entries_hash = solana_native_entries_hash(&request.entries);
            },
            |request: &mut SolanaNativeDecryptionRequestV0| request.payload.commitment_level = 0,
            |request: &mut SolanaNativeDecryptionRequestV0| request.payload.commitment_level = 99,
        ];

        for mutate in cases {
            let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
            mutate(&mut request);
            request.payload.domain_separator = solana_native_domain_separator(
                request.payload.host_chain_id,
                request.payload.solana_cluster_id,
                request.payload.acl_program_id,
                request.payload.kms_context_id,
            );

            assert_eq!(
                verifier.verify_native_v0_request(
                    &request,
                    OBSERVED_SLOT,
                    SolanaNativeRequestLimits::default()
                ),
                Err(SolanaNativeRequestError::InvalidProfile)
            );
        }
    }

    #[test]
    fn native_v0_rejects_invalid_handle_metadata() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        for invalid_handle in [
            {
                let mut handle = HANDLE;
                handle[29] ^= 0xff;
                handle
            },
            {
                let mut handle = HANDLE;
                handle[30] = 1;
                handle
            },
            {
                let mut handle = HANDLE;
                handle[31] = 1;
                handle
            },
        ] {
            let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
            request.entries[0].handle = invalid_handle;
            request.entries[0].acl_record.handle = invalid_handle;
            request.entries[0].material.handle = invalid_handle;
            request.payload.entries_hash = solana_native_entries_hash(&request.entries);

            assert_eq!(
                verifier.verify_native_v0_request(
                    &request,
                    OBSERVED_SLOT,
                    SolanaNativeRequestLimits::default()
                ),
                Err(SolanaNativeRequestError::InvalidHandleMetadata)
            );
        }
    }

    #[test]
    fn native_v0_rejects_encrypted_bit_limit_excess() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let first_handle = test_handle(7, 8);
        request.entries[0].handle = first_handle;
        request.entries[0].acl_record.handle = first_handle;
        request.entries[0].material.handle = first_handle;

        for index in 0..8 {
            let record = record_variant(test_handle(80 + index, 8), 9 + index as u64, [21; 32]);
            let material = material_commitment(&record);
            request.entries.push(native_entry(
                record,
                material,
                SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            ));
        }
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::RequestTooLarge)
        );
    }

    #[test]
    fn native_v0_rejects_legacy_gateway_extra_data() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.raw_extra_data = vec![1; 37];
        request.payload.extra_data_hash = solana_native_extra_data_hash(&request.raw_extra_data);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::InvalidExtraData)
        );
    }

    #[test]
    fn native_v0_rejects_direct_request_without_app_context_membership() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        request.entries[0]
            .acl_record
            .subjects
            .retain(|entry| entry.subject != APP_ACCOUNT);
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::WitnessAccountMismatch)
        );
    }

    #[test]
    fn native_v0_rejects_overflow_witness_for_inline_subject_without_use_role() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        let owner_entry = request.entries[0]
            .acl_record
            .subjects
            .iter_mut()
            .find(|entry| entry.subject == OWNER)
            .expect("owner is inline");
        owner_entry.role_flags = ACL_ROLE_PUBLIC_DECRYPT;

        let (account_key, bump) = acl_permission_address(
            HOST_PROGRAM_ID,
            request.entries[0].acl_record.account_key,
            OWNER,
        );
        request.entries[0].owner_permission_account = account_key;
        request.entries[0].owner_permission = Some(AclPermissionWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            acl_record: request.entries[0].acl_record.account_key,
            subject: OWNER,
            role_flags: ACL_ROLE_USE,
            bump,
        });
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::WitnessAccountMismatch)
        );
    }

    #[test]
    fn native_v0_rejects_public_request_with_subject_fields() {
        let verifier = SolanaAclVerifier::new(HOST_PROGRAM_ID);
        let mut request = native_request(SOLANA_NATIVE_REQUEST_MODE_PUBLIC);
        request.entries[0].owner_pubkey = OWNER;
        request.payload.entries_hash = solana_native_entries_hash(&request.entries);

        assert_eq!(
            verifier.verify_native_v0_request(
                &request,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default()
            ),
            Err(SolanaNativeRequestError::RoleFieldsInvalid)
        );
    }
}
