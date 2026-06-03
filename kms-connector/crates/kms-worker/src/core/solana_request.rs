use crate::core::solana_acl::{
    AclPermissionWitness, AclRecordWitness, HandleMaterialCommitmentWitness,
    SOLANA_NATIVE_ED25519_SIGNATURE_LEN, SolanaNativeDecryptionRequestV0,
    SolanaNativeHandleEntryV0, SolanaNativeRequestLimits, SolanaPubkeyBytes,
    SolanaUserDecryptionPayloadV0, UserDecryptionDelegationWitness, decode_acl_permission_witness,
    decode_acl_record_witness, decode_handle_material_commitment_witness,
    decode_solana_kms_extra_data_v0, decode_user_decryption_delegation_witness,
    solana_native_extra_data_hash, solana_native_handle_encrypted_bits, solana_native_update_ascii,
};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use thiserror::Error;

pub const SOLANA_NATIVE_REQUEST_WIRE_LAYOUT_V0: u8 = 0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeParsedRequestV0 {
    pub payload: SolanaUserDecryptionPayloadV0,
    pub entries: Vec<SolanaNativeParsedHandleEntryV0>,
    pub raw_extra_data: Vec<u8>,
    pub user_reencryption_public_key: Vec<u8>,
    pub request_signature: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeParsedHandleEntryV0 {
    pub handle: [u8; 32],
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeAccountFetchPlanV0 {
    pub account_keys: Vec<SolanaPubkeyBytes>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeAccountWitnessV0 {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub executable: bool,
    pub data: Vec<u8>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaNativeRequestParseError {
    #[error("native Solana request exceeds configured byte or entry limits")]
    RequestTooLarge,
    #[error("native Solana request has an unsupported wire layout")]
    UnsupportedLayout,
    #[error("native Solana request has an invalid length field")]
    InvalidLength,
    #[error("native Solana request signature length is invalid")]
    InvalidSignatureLength,
    #[error("native Solana request bytes ended before the expected field")]
    UnexpectedEof,
    #[error("native Solana request has trailing bytes")]
    TrailingBytes,
    #[error("native Solana request handle metadata is invalid")]
    InvalidHandleMetadata,
    #[error("native Solana request extra-data layout is invalid")]
    InvalidExtraData,
    #[error("native Solana request extra-data context does not match the payload")]
    ExtraDataContextMismatch,
    #[error("native Solana request extra-data hash does not match the raw extra-data bytes")]
    ExtraDataHashMismatch,
    #[error("native Solana request entries hash does not match the signed handle entries")]
    EntriesHashMismatch,
    #[error("native Solana account witness list contains a duplicate account")]
    DuplicateAccountWitness,
    #[error("native Solana account witness was not requested by the signed payload")]
    ExtraAccountWitness,
    #[error("native Solana account witness is executable")]
    ExecutableAccountWitness,
    #[error("native Solana request contains a zero mandatory account key")]
    ZeroMandatoryAccount,
    #[error("native Solana request is missing a fetched account witness")]
    MissingAccountWitness,
    #[error("native Solana account witness could not be decoded: {0}")]
    InvalidAccountWitness(String),
}

impl SolanaNativeParsedRequestV0 {
    pub fn account_fetch_plan(&self) -> SolanaNativeAccountFetchPlanV0 {
        let mut account_keys = Vec::new();
        for entry in &self.entries {
            push_unique_nonzero(&mut account_keys, entry.acl_record_account);
            push_unique_nonzero(&mut account_keys, entry.material_commitment_account);
            push_unique_nonzero(&mut account_keys, entry.owner_permission_account);
            push_unique_nonzero(&mut account_keys, entry.delegator_permission_account);
            push_unique_nonzero(&mut account_keys, entry.app_context_permission_account);
            push_unique_nonzero(&mut account_keys, entry.delegation_record_account);
        }
        SolanaNativeAccountFetchPlanV0 { account_keys }
    }

    pub fn attach_account_witnesses(
        &self,
        account_witnesses: &[SolanaNativeAccountWitnessV0],
    ) -> Result<SolanaNativeDecryptionRequestV0, SolanaNativeRequestParseError> {
        let expected_account_keys = self.account_fetch_plan().account_keys;
        let mut witnesses_by_key = HashMap::with_capacity(account_witnesses.len());
        for witness in account_witnesses {
            if witness.executable {
                return Err(SolanaNativeRequestParseError::ExecutableAccountWitness);
            }
            if !expected_account_keys.contains(&witness.account_key) {
                return Err(SolanaNativeRequestParseError::ExtraAccountWitness);
            }
            if witnesses_by_key
                .insert(witness.account_key, witness)
                .is_some()
            {
                return Err(SolanaNativeRequestParseError::DuplicateAccountWitness);
            }
        }

        let entries = self
            .entries
            .iter()
            .map(|entry| attach_entry_witnesses(entry, &witnesses_by_key))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SolanaNativeDecryptionRequestV0 {
            payload: self.payload.clone(),
            entries,
            raw_extra_data: self.raw_extra_data.clone(),
            user_reencryption_public_key: self.user_reencryption_public_key.clone(),
        })
    }
}

pub fn parse_solana_native_request_v0(
    request_bytes: &[u8],
    limits: SolanaNativeRequestLimits,
) -> Result<SolanaNativeParsedRequestV0, SolanaNativeRequestParseError> {
    if request_bytes.len() > limits.max_signed_request_bytes {
        return Err(SolanaNativeRequestParseError::RequestTooLarge);
    }
    let mut cursor = RequestCursor::new(request_bytes);
    let layout = cursor.read_u8()?;
    if layout != SOLANA_NATIVE_REQUEST_WIRE_LAYOUT_V0 {
        return Err(SolanaNativeRequestParseError::UnsupportedLayout);
    }

    let payload = SolanaUserDecryptionPayloadV0 {
        domain_separator: cursor.read_bytes_32()?,
        host_chain_id: cursor.read_u64()?,
        config_version: cursor.read_u64()?,
        solana_cluster_id: cursor.read_bytes_32()?,
        kms_context_id: cursor.read_bytes_32()?,
        user_reencryption_pubkey_len: cursor.read_u32()?,
        user_reencryption_pubkey_hash: cursor.read_bytes_32()?,
        request_signer_pubkey: cursor.read_bytes_32()?,
        acl_program_id: cursor.read_bytes_32()?,
        request_mode: cursor.read_u8()?,
        material_source_mode: cursor.read_u8()?,
        commitment_level: cursor.read_u8()?,
        min_context_slot: cursor.read_u64()?,
        expiration_slot: cursor.read_u64()?,
        nonce: cursor.read_bytes_32()?,
        extra_data_hash: cursor.read_bytes_32()?,
        entries_hash: cursor.read_bytes_32()?,
    };

    let entry_count = cursor.read_u16()? as usize;
    if entry_count == 0 || entry_count > limits.max_handles_per_request {
        return Err(SolanaNativeRequestParseError::RequestTooLarge);
    }
    let mut entries = Vec::with_capacity(entry_count);
    let mut encrypted_bits = 0usize;
    for _ in 0..entry_count {
        let handle = cursor.read_bytes_32()?;
        let handle_bits = solana_native_handle_encrypted_bits(handle, payload.host_chain_id)
            .ok_or(SolanaNativeRequestParseError::InvalidHandleMetadata)?;
        encrypted_bits = encrypted_bits
            .checked_add(handle_bits)
            .ok_or(SolanaNativeRequestParseError::RequestTooLarge)?;
        entries.push(SolanaNativeParsedHandleEntryV0 {
            handle,
            owner_pubkey: cursor.read_bytes_32()?,
            owner_permission_account: cursor.read_bytes_32()?,
            delegator_pubkey: cursor.read_bytes_32()?,
            delegator_permission_account: cursor.read_bytes_32()?,
            delegate_pubkey: cursor.read_bytes_32()?,
            app_context_pubkey: cursor.read_bytes_32()?,
            app_context_permission_account: cursor.read_bytes_32()?,
            acl_record_account: cursor.read_bytes_32()?,
            delegation_record_account: cursor.read_bytes_32()?,
            expected_delegation_counter: cursor.read_u64()?,
            material_commitment_account: cursor.read_bytes_32()?,
            material_commitment_hash: cursor.read_bytes_32()?,
            min_context_slot: cursor.read_u64()?,
            config_version: cursor.read_u64()?,
            material_source_mode: cursor.read_u8()?,
            acl_layout_version: cursor.read_u8()?,
            handle_derivation_version: cursor.read_u8()?,
            material_commitment_version: cursor.read_u8()?,
            expected_key_id: cursor.read_bytes_32()?,
        });
    }
    if encrypted_bits > limits.max_encrypted_bits_per_request {
        return Err(SolanaNativeRequestParseError::RequestTooLarge);
    }
    if payload.entries_hash != solana_native_parsed_entries_hash(&entries) {
        return Err(SolanaNativeRequestParseError::EntriesHashMismatch);
    }

    let raw_extra_data = cursor.read_vec_u32(limits.max_extra_data_bytes)?;
    let extra_data = decode_solana_kms_extra_data_v0(&raw_extra_data, limits)
        .map_err(|_| SolanaNativeRequestParseError::InvalidExtraData)?;
    if extra_data.kms_context_id != payload.kms_context_id {
        return Err(SolanaNativeRequestParseError::ExtraDataContextMismatch);
    }
    if payload.extra_data_hash != solana_native_extra_data_hash(&raw_extra_data) {
        return Err(SolanaNativeRequestParseError::ExtraDataHashMismatch);
    }
    let user_reencryption_public_key =
        cursor.read_vec_u32(limits.max_user_reencryption_pubkey_bytes)?;
    if payload.user_reencryption_pubkey_len as usize != user_reencryption_public_key.len() {
        return Err(SolanaNativeRequestParseError::InvalidLength);
    }

    let request_signature = cursor.read_vec_u16(SOLANA_NATIVE_ED25519_SIGNATURE_LEN)?;
    if !request_signature.is_empty()
        && request_signature.len() != SOLANA_NATIVE_ED25519_SIGNATURE_LEN
    {
        return Err(SolanaNativeRequestParseError::InvalidSignatureLength);
    }
    cursor.finish()?;

    Ok(SolanaNativeParsedRequestV0 {
        payload,
        entries,
        raw_extra_data,
        user_reencryption_public_key,
        request_signature,
    })
}

fn attach_entry_witnesses(
    entry: &SolanaNativeParsedHandleEntryV0,
    witnesses_by_key: &HashMap<SolanaPubkeyBytes, &SolanaNativeAccountWitnessV0>,
) -> Result<SolanaNativeHandleEntryV0, SolanaNativeRequestParseError> {
    let acl_record = decode_acl_record(entry.acl_record_account, witnesses_by_key)?;
    let material = decode_material_commitment(entry.material_commitment_account, witnesses_by_key)?;
    let owner_permission =
        decode_optional_permission(entry.owner_permission_account, witnesses_by_key)?;
    let delegator_permission =
        decode_optional_permission(entry.delegator_permission_account, witnesses_by_key)?;
    let app_context_permission =
        decode_optional_permission(entry.app_context_permission_account, witnesses_by_key)?;
    let delegation = decode_optional_delegation(entry.delegation_record_account, witnesses_by_key)?;

    Ok(SolanaNativeHandleEntryV0 {
        handle: entry.handle,
        owner_pubkey: entry.owner_pubkey,
        owner_permission_account: entry.owner_permission_account,
        delegator_pubkey: entry.delegator_pubkey,
        delegator_permission_account: entry.delegator_permission_account,
        delegate_pubkey: entry.delegate_pubkey,
        app_context_pubkey: entry.app_context_pubkey,
        app_context_permission_account: entry.app_context_permission_account,
        acl_record_account: entry.acl_record_account,
        delegation_record_account: entry.delegation_record_account,
        expected_delegation_counter: entry.expected_delegation_counter,
        material_commitment_account: entry.material_commitment_account,
        material_commitment_hash: entry.material_commitment_hash,
        min_context_slot: entry.min_context_slot,
        config_version: entry.config_version,
        material_source_mode: entry.material_source_mode,
        acl_layout_version: entry.acl_layout_version,
        handle_derivation_version: entry.handle_derivation_version,
        material_commitment_version: entry.material_commitment_version,
        expected_key_id: entry.expected_key_id,
        acl_record,
        material,
        owner_permission,
        delegator_permission,
        app_context_permission,
        delegation,
    })
}

fn decode_acl_record(
    account_key: SolanaPubkeyBytes,
    witnesses_by_key: &HashMap<SolanaPubkeyBytes, &SolanaNativeAccountWitnessV0>,
) -> Result<AclRecordWitness, SolanaNativeRequestParseError> {
    let witness = required_witness(account_key, witnesses_by_key)?;
    decode_acl_record_witness(witness.account_key, witness.owner, &witness.data)
        .map_err(|e| SolanaNativeRequestParseError::InvalidAccountWitness(e.to_string()))
}

fn decode_material_commitment(
    account_key: SolanaPubkeyBytes,
    witnesses_by_key: &HashMap<SolanaPubkeyBytes, &SolanaNativeAccountWitnessV0>,
) -> Result<HandleMaterialCommitmentWitness, SolanaNativeRequestParseError> {
    let witness = required_witness(account_key, witnesses_by_key)?;
    decode_handle_material_commitment_witness(witness.account_key, witness.owner, &witness.data)
        .map_err(|e| SolanaNativeRequestParseError::InvalidAccountWitness(e.to_string()))
}

fn decode_optional_permission(
    account_key: SolanaPubkeyBytes,
    witnesses_by_key: &HashMap<SolanaPubkeyBytes, &SolanaNativeAccountWitnessV0>,
) -> Result<Option<AclPermissionWitness>, SolanaNativeRequestParseError> {
    if account_key == [0; 32] {
        return Ok(None);
    }
    let witness = required_witness(account_key, witnesses_by_key)?;
    decode_acl_permission_witness(witness.account_key, witness.owner, &witness.data)
        .map(Some)
        .map_err(|e| SolanaNativeRequestParseError::InvalidAccountWitness(e.to_string()))
}

fn decode_optional_delegation(
    account_key: SolanaPubkeyBytes,
    witnesses_by_key: &HashMap<SolanaPubkeyBytes, &SolanaNativeAccountWitnessV0>,
) -> Result<Option<UserDecryptionDelegationWitness>, SolanaNativeRequestParseError> {
    if account_key == [0; 32] {
        return Ok(None);
    }
    let witness = required_witness(account_key, witnesses_by_key)?;
    decode_user_decryption_delegation_witness(witness.account_key, witness.owner, &witness.data)
        .map(Some)
        .map_err(|e| SolanaNativeRequestParseError::InvalidAccountWitness(e.to_string()))
}

fn required_witness<'a>(
    account_key: SolanaPubkeyBytes,
    witnesses_by_key: &'a HashMap<SolanaPubkeyBytes, &SolanaNativeAccountWitnessV0>,
) -> Result<&'a SolanaNativeAccountWitnessV0, SolanaNativeRequestParseError> {
    if account_key == [0; 32] {
        return Err(SolanaNativeRequestParseError::ZeroMandatoryAccount);
    }
    witnesses_by_key
        .get(&account_key)
        .copied()
        .ok_or(SolanaNativeRequestParseError::MissingAccountWitness)
}

fn push_unique_nonzero(account_keys: &mut Vec<SolanaPubkeyBytes>, account_key: SolanaPubkeyBytes) {
    if account_key != [0; 32] && !account_keys.contains(&account_key) {
        account_keys.push(account_key);
    }
}

pub(crate) fn solana_native_parsed_entries_hash(
    entries: &[SolanaNativeParsedHandleEntryV0],
) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    solana_native_update_ascii(&mut hasher, "zama-solana-handle-entries-v0");
    hasher.update((entries.len() as u32).to_le_bytes());
    for entry in entries {
        hash_solana_native_parsed_entry(&mut hasher, entry);
    }
    hasher.finalize().into()
}

fn hash_solana_native_parsed_entry(
    hasher: &mut Keccak256,
    entry: &SolanaNativeParsedHandleEntryV0,
) {
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

struct RequestCursor<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> RequestCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn finish(&self) -> Result<(), SolanaNativeRequestParseError> {
        if self.offset == self.data.len() {
            Ok(())
        } else {
            Err(SolanaNativeRequestParseError::TrailingBytes)
        }
    }

    fn read_vec_u16(&mut self, max_len: usize) -> Result<Vec<u8>, SolanaNativeRequestParseError> {
        let len = self.read_u16()? as usize;
        if len > max_len {
            return Err(SolanaNativeRequestParseError::InvalidLength);
        }
        Ok(self.read_exact(len)?.to_vec())
    }

    fn read_vec_u32(&mut self, max_len: usize) -> Result<Vec<u8>, SolanaNativeRequestParseError> {
        let len = self.read_u32()? as usize;
        if len > max_len {
            return Err(SolanaNativeRequestParseError::InvalidLength);
        }
        Ok(self.read_exact(len)?.to_vec())
    }

    fn read_bytes_32(&mut self) -> Result<[u8; 32], SolanaNativeRequestParseError> {
        let bytes = self.read_exact(32)?;
        let mut output = [0; 32];
        output.copy_from_slice(bytes);
        Ok(output)
    }

    fn read_u64(&mut self) -> Result<u64, SolanaNativeRequestParseError> {
        let bytes = self.read_exact(8)?;
        let mut value = [0; 8];
        value.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(value))
    }

    fn read_u32(&mut self) -> Result<u32, SolanaNativeRequestParseError> {
        let bytes = self.read_exact(4)?;
        let mut value = [0; 4];
        value.copy_from_slice(bytes);
        Ok(u32::from_le_bytes(value))
    }

    fn read_u16(&mut self) -> Result<u16, SolanaNativeRequestParseError> {
        let bytes = self.read_exact(2)?;
        let mut value = [0; 2];
        value.copy_from_slice(bytes);
        Ok(u16::from_le_bytes(value))
    }

    fn read_u8(&mut self) -> Result<u8, SolanaNativeRequestParseError> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], SolanaNativeRequestParseError> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(SolanaNativeRequestParseError::InvalidLength)?;
        if end > self.data.len() {
            return Err(SolanaNativeRequestParseError::UnexpectedEof);
        }
        let slice = &self.data[self.offset..end];
        self.offset = end;
        Ok(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::solana_acl::{
        ACL_ROLE_PUBLIC_DECRYPT, ACL_ROLE_USE, HANDLE_MATERIAL_STATE_COMMITTED,
        SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED, SOLANA_NATIVE_SUPPORTED_ACL_LAYOUT_VERSION,
        SOLANA_NATIVE_SUPPORTED_HANDLE_DERIVATION_VERSION, SOLANA_NATIVE_SUPPORTED_HANDLE_VERSION,
        SOLANA_NATIVE_SUPPORTED_MATERIAL_COMMITMENT_VERSION,
        SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE, SubjectRole, acl_nonce_key,
        acl_record_address, anchor_account_discriminator, encode_solana_kms_extra_data_v0,
        handle_material_address, handle_material_commitment_hash, solana_native_domain_separator,
        solana_native_entries_hash, solana_native_extra_data_hash,
        solana_native_reencryption_pubkey_hash,
    };
    use crate::core::solana_acl::{SolanaAclVerifier, SolanaKmsExtraDataV0};

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const HOST_CHAIN_ID: u64 = 900;
    const HANDLE: [u8; 32] = [
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 3, 132,
        5, 0,
    ];
    const DOMAIN: SolanaPubkeyBytes = [1; 32];
    const APP_ACCOUNT: SolanaPubkeyBytes = [2; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const LABEL: [u8; 32] = *b"balance_________________________";
    const OBSERVED_SLOT: u64 = 500;

    fn test_handle(tag: u8, fhe_type: u8) -> [u8; 32] {
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

    fn native_entry(
        record: AclRecordWitness,
        material: HandleMaterialCommitmentWitness,
    ) -> SolanaNativeHandleEntryV0 {
        SolanaNativeHandleEntryV0 {
            handle: record.handle,
            owner_pubkey: OWNER,
            owner_permission_account: [0; 32],
            delegator_pubkey: [0; 32],
            delegator_permission_account: [0; 32],
            delegate_pubkey: [0; 32],
            app_context_pubkey: APP_ACCOUNT,
            app_context_permission_account: [0; 32],
            acl_record_account: record.account_key,
            delegation_record_account: [0; 32],
            expected_delegation_counter: 0,
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
            delegation: None,
        }
    }

    fn request_fixture() -> (
        SolanaNativeDecryptionRequestV0,
        Vec<SolanaNativeAccountWitnessV0>,
        Vec<u8>,
    ) {
        let record = base_record();
        let material = material_commitment(&record);
        let entry = native_entry(record.clone(), material.clone());
        let entries = vec![entry];
        let kms_context_id = [8; 32];
        let raw_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id,
            response_context: b"context".to_vec(),
        });
        let user_reencryption_public_key = b"reencryption-key".to_vec();
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
            user_reencryption_pubkey_hash: solana_native_reencryption_pubkey_hash(
                &user_reencryption_public_key,
            ),
            request_signer_pubkey: OWNER,
            acl_program_id: HOST_PROGRAM_ID,
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: 1,
            min_context_slot: OBSERVED_SLOT - 2,
            expiration_slot: OBSERVED_SLOT + 20,
            nonce: [77; 32],
            extra_data_hash: solana_native_extra_data_hash(&raw_extra_data),
            entries_hash: solana_native_entries_hash(&entries),
        };
        let request = SolanaNativeDecryptionRequestV0 {
            payload,
            entries,
            raw_extra_data,
            user_reencryption_public_key,
        };
        let account_witnesses = vec![
            SolanaNativeAccountWitnessV0 {
                account_key: record.account_key,
                owner: record.owner,
                executable: false,
                data: encode_acl_record(&record),
            },
            SolanaNativeAccountWitnessV0 {
                account_key: material.account_key,
                owner: material.owner,
                executable: false,
                data: encode_material_commitment(&material),
            },
        ];
        let request_bytes =
            encode_wire_request(&request, &[9; SOLANA_NATIVE_ED25519_SIGNATURE_LEN]);
        (request, account_witnesses, request_bytes)
    }

    fn encode_wire_request(request: &SolanaNativeDecryptionRequestV0, signature: &[u8]) -> Vec<u8> {
        let mut output = vec![SOLANA_NATIVE_REQUEST_WIRE_LAYOUT_V0];
        encode_payload(&mut output, &request.payload);
        output.extend_from_slice(&(request.entries.len() as u16).to_le_bytes());
        for entry in &request.entries {
            encode_entry(&mut output, entry);
        }
        output.extend_from_slice(&(request.raw_extra_data.len() as u32).to_le_bytes());
        output.extend_from_slice(&request.raw_extra_data);
        output
            .extend_from_slice(&(request.user_reencryption_public_key.len() as u32).to_le_bytes());
        output.extend_from_slice(&request.user_reencryption_public_key);
        output.extend_from_slice(&(signature.len() as u16).to_le_bytes());
        output.extend_from_slice(signature);
        output
    }

    fn encode_payload(output: &mut Vec<u8>, payload: &SolanaUserDecryptionPayloadV0) {
        output.extend_from_slice(&payload.domain_separator);
        output.extend_from_slice(&payload.host_chain_id.to_le_bytes());
        output.extend_from_slice(&payload.config_version.to_le_bytes());
        output.extend_from_slice(&payload.solana_cluster_id);
        output.extend_from_slice(&payload.kms_context_id);
        output.extend_from_slice(&payload.user_reencryption_pubkey_len.to_le_bytes());
        output.extend_from_slice(&payload.user_reencryption_pubkey_hash);
        output.extend_from_slice(&payload.request_signer_pubkey);
        output.extend_from_slice(&payload.acl_program_id);
        output.push(payload.request_mode);
        output.push(payload.material_source_mode);
        output.push(payload.commitment_level);
        output.extend_from_slice(&payload.min_context_slot.to_le_bytes());
        output.extend_from_slice(&payload.expiration_slot.to_le_bytes());
        output.extend_from_slice(&payload.nonce);
        output.extend_from_slice(&payload.extra_data_hash);
        output.extend_from_slice(&payload.entries_hash);
    }

    fn encode_entry(output: &mut Vec<u8>, entry: &SolanaNativeHandleEntryV0) {
        output.extend_from_slice(&entry.handle);
        output.extend_from_slice(&entry.owner_pubkey);
        output.extend_from_slice(&entry.owner_permission_account);
        output.extend_from_slice(&entry.delegator_pubkey);
        output.extend_from_slice(&entry.delegator_permission_account);
        output.extend_from_slice(&entry.delegate_pubkey);
        output.extend_from_slice(&entry.app_context_pubkey);
        output.extend_from_slice(&entry.app_context_permission_account);
        output.extend_from_slice(&entry.acl_record_account);
        output.extend_from_slice(&entry.delegation_record_account);
        output.extend_from_slice(&entry.expected_delegation_counter.to_le_bytes());
        output.extend_from_slice(&entry.material_commitment_account);
        output.extend_from_slice(&entry.material_commitment_hash);
        output.extend_from_slice(&entry.min_context_slot.to_le_bytes());
        output.extend_from_slice(&entry.config_version.to_le_bytes());
        output.push(entry.material_source_mode);
        output.push(entry.acl_layout_version);
        output.push(entry.handle_derivation_version);
        output.push(entry.material_commitment_version);
        output.extend_from_slice(&entry.expected_key_id);
    }

    fn encode_acl_record(record: &AclRecordWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("AclRecord").to_vec();
        data.extend_from_slice(&record.handle);
        data.extend_from_slice(&record.nonce_key);
        data.extend_from_slice(&record.nonce_sequence.to_le_bytes());
        data.extend_from_slice(&record.acl_domain_key);
        data.extend_from_slice(&record.app_account);
        data.extend_from_slice(&record.encrypted_value_label);
        for index in 0..8 {
            data.extend_from_slice(
                &record
                    .subjects
                    .get(index)
                    .map(|entry| entry.subject)
                    .unwrap_or([0; 32]),
            );
        }
        for index in 0..8 {
            data.push(
                record
                    .subjects
                    .get(index)
                    .map(|entry| entry.role_flags)
                    .unwrap_or(0),
            );
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
    fn parses_native_request_and_attaches_fetched_accounts() {
        let (expected_request, account_witnesses, request_bytes) = request_fixture();
        let parsed =
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default())
                .unwrap();

        assert_eq!(
            parsed.account_fetch_plan().account_keys,
            vec![
                expected_request.entries[0].acl_record_account,
                expected_request.entries[0].material_commitment_account,
            ]
        );
        assert_eq!(
            parsed.request_signature,
            vec![9; SOLANA_NATIVE_ED25519_SIGNATURE_LEN]
        );

        let attached = parsed.attach_account_witnesses(&account_witnesses).unwrap();
        assert_eq!(attached, expected_request);
        SolanaAclVerifier::new(HOST_PROGRAM_ID)
            .verify_native_v0_request(
                &attached,
                OBSERVED_SLOT,
                SolanaNativeRequestLimits::default(),
            )
            .unwrap();
    }

    #[test]
    fn parse_rejects_too_many_entries_before_fetching_accounts() {
        let (request, _, _) = request_fixture();
        let request_bytes = encode_wire_request(&request, &[]);
        let limits = SolanaNativeRequestLimits {
            max_handles_per_request: 0,
            ..SolanaNativeRequestLimits::default()
        };

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, limits),
            Err(SolanaNativeRequestParseError::RequestTooLarge)
        );
    }

    #[test]
    fn parse_rejects_invalid_handle_metadata_before_fetching_accounts() {
        let (mut request, _, _) = request_fixture();
        request.entries[0].handle[31] = 1;
        let request_bytes = encode_wire_request(&request, &[]);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::InvalidHandleMetadata)
        );
    }

    #[test]
    fn parse_rejects_encrypted_bit_limit_before_fetching_accounts() {
        let (mut request, _, _) = request_fixture();
        request.entries.clear();
        for index in 0..9 {
            let mut entry = native_entry(base_record(), material_commitment(&base_record()));
            entry.handle = test_handle(80 + index, 8);
            request.entries.push(entry);
        }
        let request_bytes = encode_wire_request(&request, &[]);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::RequestTooLarge)
        );
    }

    #[test]
    fn parse_rejects_invalid_extra_data_before_fetching_accounts() {
        let (mut request, _, _) = request_fixture();
        request.raw_extra_data[0] = 1;
        let request_bytes = encode_wire_request(&request, &[]);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::InvalidExtraData)
        );
    }

    #[test]
    fn parse_rejects_extra_data_context_mismatch_before_fetching_accounts() {
        let (mut request, _, _) = request_fixture();
        request.raw_extra_data[1] ^= 0xff;
        let request_bytes = encode_wire_request(&request, &[]);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::ExtraDataContextMismatch)
        );
    }

    #[test]
    fn parse_rejects_extra_data_hash_mismatch_before_fetching_accounts() {
        let (mut request, _, _) = request_fixture();
        let last = request.raw_extra_data.last_mut().unwrap();
        *last ^= 0xff;
        let request_bytes = encode_wire_request(&request, &[]);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::ExtraDataHashMismatch)
        );
    }

    #[test]
    fn parse_rejects_entries_hash_mismatch_before_fetching_accounts() {
        let (mut request, _, _) = request_fixture();
        request.payload.entries_hash[0] ^= 0xff;
        let request_bytes = encode_wire_request(&request, &[]);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::EntriesHashMismatch)
        );
    }

    #[test]
    fn parse_rejects_trailing_bytes() {
        let (_, _, mut request_bytes) = request_fixture();
        request_bytes.push(1);

        assert_eq!(
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default()),
            Err(SolanaNativeRequestParseError::TrailingBytes)
        );
    }

    #[test]
    fn attach_rejects_missing_mandatory_account_witness() {
        let (_, account_witnesses, request_bytes) = request_fixture();
        let parsed =
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default())
                .unwrap();

        assert_eq!(
            parsed.attach_account_witnesses(&account_witnesses[..1]),
            Err(SolanaNativeRequestParseError::MissingAccountWitness)
        );
    }

    #[test]
    fn attach_rejects_executable_account_witness() {
        let (_, mut account_witnesses, request_bytes) = request_fixture();
        let parsed =
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default())
                .unwrap();
        account_witnesses[0].executable = true;

        assert_eq!(
            parsed.attach_account_witnesses(&account_witnesses),
            Err(SolanaNativeRequestParseError::ExecutableAccountWitness)
        );
    }

    #[test]
    fn attach_rejects_extra_account_witness() {
        let (_, mut account_witnesses, request_bytes) = request_fixture();
        let parsed =
            parse_solana_native_request_v0(&request_bytes, SolanaNativeRequestLimits::default())
                .unwrap();
        account_witnesses.push(SolanaNativeAccountWitnessV0 {
            account_key: [99; 32],
            owner: HOST_PROGRAM_ID,
            executable: false,
            data: Vec::new(),
        });

        assert_eq!(
            parsed.attach_account_witnesses(&account_witnesses),
            Err(SolanaNativeRequestParseError::ExtraAccountWitness)
        );
    }
}
