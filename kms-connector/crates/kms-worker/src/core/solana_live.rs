use crate::core::{
    solana_acl::{
        SOLANA_NATIVE_ED25519_SIGNATURE_LEN, SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED,
        SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED,
        SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED, SOLANA_NATIVE_REQUEST_MODE_PUBLIC,
        SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE, SolanaNativeRequestError,
        SolanaNativeRequestLimits, SolanaPubkeyBytes, decode_solana_kms_extra_data_v0,
        solana_native_domain_separator, solana_native_extra_data_hash,
        solana_native_reencryption_pubkey_hash, solana_native_request_hash,
        verify_solana_native_request_signature,
    },
    solana_native::{
        SolanaNativeAdmissionError, SolanaNativeAdmittedRequestV0, SolanaNativeRequestAdmission,
    },
    solana_replay::SolanaNativeReplayStore,
    solana_request::{
        SolanaNativeAccountWitnessV0, SolanaNativeParsedRequestV0, SolanaNativeRequestParseError,
        solana_native_parsed_entries_hash,
    },
};
use thiserror::Error;

pub const SOLANA_NATIVE_COMMITMENT_PROCESSED: u8 = 0;
pub const SOLANA_NATIVE_COMMITMENT_CONFIRMED: u8 = 1;
pub const SOLANA_NATIVE_COMMITMENT_FINALIZED: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaNativeLiveRequestPolicy {
    pub min_commitment_level: u8,
    pub confirmed_mode_authorization_enabled: bool,
    pub max_account_fetches_per_request: usize,
    pub max_rpc_response_bytes: usize,
    pub max_validity_slots: u64,
}

impl Default for SolanaNativeLiveRequestPolicy {
    fn default() -> Self {
        Self {
            min_commitment_level: SOLANA_NATIVE_COMMITMENT_FINALIZED,
            confirmed_mode_authorization_enabled: false,
            max_account_fetches_per_request: 512,
            max_rpc_response_bytes: 8 * 1024 * 1024,
            max_validity_slots: 10_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeAccountSnapshotV0 {
    pub observed_slot: u64,
    pub commitment_level: u8,
    pub rpc_response_bytes: usize,
    pub accounts: Vec<SolanaNativeAccountWitnessV0>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeLiveAdmittedRequestV0 {
    pub admitted: SolanaNativeAdmittedRequestV0,
    pub observed_slot: u64,
    pub observed_commitment_level: u8,
    pub finality_action: SolanaNativeFinalityAction,
    pub account_witnesses: Vec<SolanaNativeAccountWitnessV0>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolanaNativeFinalityAction {
    ReleaseNow,
    RecheckBeforeRelease,
}

pub trait SolanaNativeAccountFetcher {
    fn fetch_accounts(
        &self,
        account_keys: &[SolanaPubkeyBytes],
        commitment_level: u8,
    ) -> impl std::future::Future<
        Output = Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError>,
    > + Send;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolanaNativeAccountFetchError {
    #[error("native Solana account fetch failed: {0}")]
    Unavailable(String),
}

#[derive(Debug, Error)]
pub enum SolanaNativeLiveAdmissionError {
    #[error("native Solana request account fetch plan exceeds configured account limit")]
    TooManyAccountFetches,
    #[error("native Solana RPC account response exceeds configured byte limit")]
    RpcResponseTooLarge,
    #[error("native Solana request uses an unsupported commitment level")]
    UnsupportedCommitmentLevel,
    #[error("native Solana request commitment is below connector policy")]
    CommitmentBelowMinimum,
    #[error("native Solana account snapshot commitment is below the signed request")]
    ObservedCommitmentBelowRequest,
    #[error("native Solana confirmed-mode authorization is disabled by policy")]
    ConfirmedAuthorizationDisabled,
    #[error("native Solana request validity window exceeds connector policy")]
    ValidityWindowExceeded,
    #[error("native Solana account snapshot slot is outside the signed request window")]
    ObservedSlotOutsideRequestWindow,
    #[error("native Solana finality recheck did not return a finalized account snapshot")]
    FinalityRecheckNotFinalized,
    #[error("native Solana finality recheck account witnesses changed before release")]
    FinalityRecheckAccountMismatch,
    #[error("native Solana finality recheck accepted a different request")]
    FinalityRecheckAcceptedRequestMismatch,
    #[error("native Solana account fetch failed: {0}")]
    AccountFetch(#[from] SolanaNativeAccountFetchError),
    #[error("native Solana request parsing or account attach failed: {0}")]
    Parse(#[from] SolanaNativeRequestParseError),
    #[error("native Solana request admission failed: {0}")]
    Admission(#[from] SolanaNativeAdmissionError),
}

pub async fn admit_live_solana_native_request_v0<S, F>(
    parsed_request: &SolanaNativeParsedRequestV0,
    admission: &SolanaNativeRequestAdmission<S>,
    account_fetcher: &F,
    policy: SolanaNativeLiveRequestPolicy,
) -> Result<SolanaNativeLiveAdmittedRequestV0, SolanaNativeLiveAdmissionError>
where
    S: SolanaNativeReplayStore + Sync,
    F: SolanaNativeAccountFetcher + Sync,
{
    validate_prefetch_request(
        parsed_request,
        admission.host_program_id(),
        admission.request_limits(),
        policy,
    )?;

    let fetch_plan = parsed_request.account_fetch_plan();
    if fetch_plan.account_keys.len() > policy.max_account_fetches_per_request {
        return Err(SolanaNativeLiveAdmissionError::TooManyAccountFetches);
    }

    let snapshot = account_fetcher
        .fetch_accounts(
            &fetch_plan.account_keys,
            parsed_request.payload.commitment_level,
        )
        .await?;
    validate_snapshot(parsed_request, &snapshot, policy)?;

    let request = parsed_request.attach_account_witnesses(&snapshot.accounts)?;
    let admitted = admission
        .admit_v0_signed_request(
            &request,
            snapshot.observed_slot,
            &parsed_request.request_signature,
        )
        .await?;
    let finality_action = if snapshot.commitment_level == SOLANA_NATIVE_COMMITMENT_FINALIZED {
        SolanaNativeFinalityAction::ReleaseNow
    } else {
        SolanaNativeFinalityAction::RecheckBeforeRelease
    };

    Ok(SolanaNativeLiveAdmittedRequestV0 {
        admitted,
        observed_slot: snapshot.observed_slot,
        observed_commitment_level: snapshot.commitment_level,
        finality_action,
        account_witnesses: snapshot.accounts,
    })
}

pub async fn recheck_live_solana_native_request_before_release_v0<S, F>(
    parsed_request: &SolanaNativeParsedRequestV0,
    initial_admission: &SolanaNativeLiveAdmittedRequestV0,
    admission: &SolanaNativeRequestAdmission<S>,
    account_fetcher: &F,
    policy: SolanaNativeLiveRequestPolicy,
) -> Result<SolanaNativeLiveAdmittedRequestV0, SolanaNativeLiveAdmissionError>
where
    S: SolanaNativeReplayStore + Sync,
    F: SolanaNativeAccountFetcher + Sync,
{
    if initial_admission.finality_action == SolanaNativeFinalityAction::ReleaseNow {
        return Ok(initial_admission.clone());
    }

    let fetch_plan = parsed_request.account_fetch_plan();
    if fetch_plan.account_keys.len() > policy.max_account_fetches_per_request {
        return Err(SolanaNativeLiveAdmissionError::TooManyAccountFetches);
    }

    let snapshot = account_fetcher
        .fetch_accounts(&fetch_plan.account_keys, SOLANA_NATIVE_COMMITMENT_FINALIZED)
        .await?;
    validate_snapshot(parsed_request, &snapshot, policy)?;
    if snapshot.commitment_level != SOLANA_NATIVE_COMMITMENT_FINALIZED {
        return Err(SolanaNativeLiveAdmissionError::FinalityRecheckNotFinalized);
    }
    if snapshot.accounts != initial_admission.account_witnesses {
        return Err(SolanaNativeLiveAdmissionError::FinalityRecheckAccountMismatch);
    }

    let request = parsed_request.attach_account_witnesses(&snapshot.accounts)?;
    let admitted = admission
        .admit_v0_signed_request(
            &request,
            snapshot.observed_slot,
            &parsed_request.request_signature,
        )
        .await?;
    if admitted.accepted != initial_admission.admitted.accepted {
        return Err(SolanaNativeLiveAdmissionError::FinalityRecheckAcceptedRequestMismatch);
    }

    Ok(SolanaNativeLiveAdmittedRequestV0 {
        admitted,
        observed_slot: snapshot.observed_slot,
        observed_commitment_level: snapshot.commitment_level,
        finality_action: SolanaNativeFinalityAction::ReleaseNow,
        account_witnesses: snapshot.accounts,
    })
}

fn validate_requested_commitment(
    requested_commitment: u8,
    policy: SolanaNativeLiveRequestPolicy,
) -> Result<(), SolanaNativeLiveAdmissionError> {
    validate_commitment_level(requested_commitment)?;
    if requested_commitment < policy.min_commitment_level {
        return Err(SolanaNativeLiveAdmissionError::CommitmentBelowMinimum);
    }
    if requested_commitment < SOLANA_NATIVE_COMMITMENT_FINALIZED
        && !policy.confirmed_mode_authorization_enabled
    {
        return Err(SolanaNativeLiveAdmissionError::ConfirmedAuthorizationDisabled);
    }
    Ok(())
}

fn validate_prefetch_request(
    parsed_request: &SolanaNativeParsedRequestV0,
    host_program_id: SolanaPubkeyBytes,
    limits: SolanaNativeRequestLimits,
    policy: SolanaNativeLiveRequestPolicy,
) -> Result<(), SolanaNativeLiveAdmissionError> {
    let payload = &parsed_request.payload;
    validate_requested_commitment(payload.commitment_level, policy)?;
    if payload.acl_program_id != host_program_id {
        return Err(SolanaNativeRequestError::AclProgramMismatch.into());
    }
    if payload.host_chain_id == 0
        || payload.config_version == 0
        || payload.solana_cluster_id == [0; 32]
        || payload.kms_context_id == [0; 32]
        || payload.material_source_mode != SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE
    {
        return Err(SolanaNativeRequestError::InvalidProfile.into());
    }
    if payload.domain_separator
        != solana_native_domain_separator(
            payload.host_chain_id,
            payload.solana_cluster_id,
            payload.acl_program_id,
            payload.kms_context_id,
        )
    {
        return Err(SolanaNativeRequestError::DomainSeparatorMismatch.into());
    }
    if payload.entries_hash != solana_native_parsed_entries_hash(&parsed_request.entries) {
        return Err(SolanaNativeRequestError::EntriesHashMismatch.into());
    }
    let extra_data = decode_solana_kms_extra_data_v0(&parsed_request.raw_extra_data, limits)?;
    if extra_data.kms_context_id != payload.kms_context_id {
        return Err(SolanaNativeRequestError::KmsContextMismatch.into());
    }
    if payload.extra_data_hash != solana_native_extra_data_hash(&parsed_request.raw_extra_data) {
        return Err(SolanaNativeRequestError::ExtraDataHashMismatch.into());
    }

    match payload.request_mode {
        SOLANA_NATIVE_REQUEST_MODE_PUBLIC => {
            if payload.request_signer_pubkey != [0; 32]
                || payload.nonce != [0; 32]
                || !parsed_request.request_signature.is_empty()
            {
                return Err(SolanaNativeRequestError::InvalidRequestSignature.into());
            }
            if payload.user_reencryption_pubkey_len != 0
                || payload.user_reencryption_pubkey_hash != [0; 32]
                || !parsed_request.user_reencryption_public_key.is_empty()
            {
                return Err(SolanaNativeRequestError::ReencryptionPublicKeyMismatch.into());
            }
        }
        SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED
        | SOLANA_NATIVE_REQUEST_MODE_DELEGATED_SCOPED
        | SOLANA_NATIVE_REQUEST_MODE_DELEGATED_WILDCARD_SCOPED => {
            if payload.request_signer_pubkey == [0; 32] || payload.nonce == [0; 32] {
                return Err(SolanaNativeRequestError::InvalidNonce.into());
            }
            if parsed_request.request_signature.len() != SOLANA_NATIVE_ED25519_SIGNATURE_LEN {
                return Err(SolanaNativeRequestError::InvalidRequestSignature.into());
            }
            if parsed_request.user_reencryption_public_key.is_empty()
                || payload.user_reencryption_pubkey_len as usize
                    != parsed_request.user_reencryption_public_key.len()
                || payload.user_reencryption_pubkey_hash
                    != solana_native_reencryption_pubkey_hash(
                        &parsed_request.user_reencryption_public_key,
                    )
                || payload.user_reencryption_pubkey_hash == [0; 32]
            {
                return Err(SolanaNativeRequestError::ReencryptionPublicKeyMismatch.into());
            }
            verify_solana_native_request_signature(
                payload.request_signer_pubkey,
                solana_native_request_hash(payload),
                &parsed_request.request_signature,
            )?;
        }
        _ => return Err(SolanaNativeRequestError::UnsupportedRequestMode.into()),
    }

    Ok(())
}

fn validate_snapshot(
    parsed_request: &SolanaNativeParsedRequestV0,
    snapshot: &SolanaNativeAccountSnapshotV0,
    policy: SolanaNativeLiveRequestPolicy,
) -> Result<(), SolanaNativeLiveAdmissionError> {
    validate_commitment_level(snapshot.commitment_level)?;
    if snapshot.commitment_level < parsed_request.payload.commitment_level {
        return Err(SolanaNativeLiveAdmissionError::ObservedCommitmentBelowRequest);
    }
    if snapshot.rpc_response_bytes > policy.max_rpc_response_bytes {
        return Err(SolanaNativeLiveAdmissionError::RpcResponseTooLarge);
    }
    if snapshot.observed_slot < parsed_request.payload.min_context_slot
        || parsed_request
            .entries
            .iter()
            .any(|entry| snapshot.observed_slot < entry.min_context_slot)
    {
        return Err(SolanaNativeLiveAdmissionError::ObservedSlotOutsideRequestWindow);
    }
    if snapshot.observed_slot > parsed_request.payload.expiration_slot {
        return Err(SolanaNativeLiveAdmissionError::ObservedSlotOutsideRequestWindow);
    }
    let remaining_validity = parsed_request
        .payload
        .expiration_slot
        .checked_sub(snapshot.observed_slot)
        .ok_or(SolanaNativeLiveAdmissionError::ObservedSlotOutsideRequestWindow)?;
    if remaining_validity > policy.max_validity_slots {
        return Err(SolanaNativeLiveAdmissionError::ValidityWindowExceeded);
    }
    Ok(())
}

fn validate_commitment_level(commitment_level: u8) -> Result<(), SolanaNativeLiveAdmissionError> {
    if !(SOLANA_NATIVE_COMMITMENT_CONFIRMED..=SOLANA_NATIVE_COMMITMENT_FINALIZED)
        .contains(&commitment_level)
    {
        return Err(SolanaNativeLiveAdmissionError::UnsupportedCommitmentLevel);
    }
    Ok(())
}

impl From<SolanaNativeRequestError> for SolanaNativeLiveAdmissionError {
    fn from(value: SolanaNativeRequestError) -> Self {
        SolanaNativeAdmissionError::Request(value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        solana_acl::{
            ACL_ROLE_PUBLIC_DECRYPT, ACL_ROLE_USE, AclRecordWitness,
            HANDLE_MATERIAL_STATE_COMMITTED, HandleMaterialCommitmentWitness,
            SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED, SOLANA_NATIVE_REQUEST_MODE_PUBLIC,
            SOLANA_NATIVE_SUPPORTED_ACL_LAYOUT_VERSION,
            SOLANA_NATIVE_SUPPORTED_HANDLE_DERIVATION_VERSION,
            SOLANA_NATIVE_SUPPORTED_MATERIAL_COMMITMENT_VERSION,
            SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE, SolanaAclVerifier, SolanaKmsExtraDataV0,
            SolanaNativeAcceptedRequestV0, SolanaNativeReplayAction, SolanaNativeReplayKeyV0,
            SolanaNativeRequestLimits, SolanaUserDecryptionPayloadV0, SubjectRole, acl_nonce_key,
            acl_record_address, anchor_account_discriminator, check_solana_native_replay,
            encode_solana_kms_extra_data_v0, handle_material_address,
            handle_material_commitment_hash, solana_native_domain_separator,
            solana_native_entries_hash, solana_native_extra_data_hash,
            solana_native_reencryption_pubkey_hash, solana_native_request_hash,
            solana_native_request_signature_message,
        },
        solana_replay::{SolanaNativeReplayStore, SolanaNativeReplayStoreError},
        solana_request::{SolanaNativeParsedHandleEntryV0, SolanaNativeParsedRequestV0},
    };
    use ring::signature::KeyPair;
    use std::sync::Mutex;

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const HOST_CHAIN_ID: u64 = 900;
    const HANDLE: [u8; 32] = [
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 3, 132,
        5, 0,
    ];
    const DOMAIN: SolanaPubkeyBytes = [1; 32];
    const APP_ACCOUNT: SolanaPubkeyBytes = [2; 32];
    const LABEL: [u8; 32] = *b"balance_________________________";
    const OBSERVED_SLOT: u64 = 500;

    #[derive(Default)]
    struct InMemoryReplayStore {
        seen: Mutex<Vec<(SolanaNativeReplayKeyV0, [u8; 32])>>,
    }

    impl SolanaNativeReplayStore for InMemoryReplayStore {
        #[allow(clippy::manual_async_fn)]
        fn reserve_accepted_request(
            &self,
            accepted: &SolanaNativeAcceptedRequestV0,
        ) -> impl std::future::Future<
            Output = Result<Option<SolanaNativeReplayAction>, SolanaNativeReplayStoreError>,
        > + Send {
            async move {
                let Some(replay_key) = accepted.replay_key.as_ref() else {
                    return Ok(None);
                };
                let mut seen = self.seen.lock().unwrap();
                let existing = seen
                    .iter()
                    .find(|(key, _)| key == replay_key)
                    .map(|(_, request_hash)| *request_hash);
                let action = check_solana_native_replay(existing, accepted.request_hash)?;
                if action == SolanaNativeReplayAction::Reserve {
                    seen.push((replay_key.clone(), accepted.request_hash));
                }
                Ok(Some(action))
            }
        }
    }

    #[derive(Clone)]
    struct StaticAccountFetcher {
        snapshot: SolanaNativeAccountSnapshotV0,
    }

    impl SolanaNativeAccountFetcher for StaticAccountFetcher {
        #[allow(clippy::manual_async_fn)]
        fn fetch_accounts(
            &self,
            account_keys: &[SolanaPubkeyBytes],
            commitment_level: u8,
        ) -> impl std::future::Future<
            Output = Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError>,
        > + Send {
            let snapshot = self.snapshot.clone();
            let expected_keys = snapshot
                .accounts
                .iter()
                .map(|account| account.account_key)
                .collect::<Vec<_>>();
            let account_keys = account_keys.to_vec();
            async move {
                if account_keys != expected_keys {
                    return Err(SolanaNativeAccountFetchError::Unavailable(
                        "unexpected account fetch plan".to_string(),
                    ));
                }
                if commitment_level > snapshot.commitment_level {
                    return Err(SolanaNativeAccountFetchError::Unavailable(
                        "requested commitment above fixture snapshot".to_string(),
                    ));
                }
                Ok(snapshot)
            }
        }
    }

    struct UnexpectedAccountFetcher;

    impl SolanaNativeAccountFetcher for UnexpectedAccountFetcher {
        #[allow(clippy::manual_async_fn)]
        fn fetch_accounts(
            &self,
            _account_keys: &[SolanaPubkeyBytes],
            _commitment_level: u8,
        ) -> impl std::future::Future<
            Output = Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError>,
        > + Send {
            async {
                Err(SolanaNativeAccountFetchError::Unavailable(
                    "unexpected account fetch".to_string(),
                ))
            }
        }
    }

    #[derive(Clone)]
    struct NonFinalizedRecheckFetcher {
        snapshot: SolanaNativeAccountSnapshotV0,
    }

    impl SolanaNativeAccountFetcher for NonFinalizedRecheckFetcher {
        #[allow(clippy::manual_async_fn)]
        fn fetch_accounts(
            &self,
            _account_keys: &[SolanaPubkeyBytes],
            _commitment_level: u8,
        ) -> impl std::future::Future<
            Output = Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError>,
        > + Send {
            let snapshot = self.snapshot.clone();
            async move { Ok(snapshot) }
        }
    }

    fn confirmed_policy() -> SolanaNativeLiveRequestPolicy {
        SolanaNativeLiveRequestPolicy {
            min_commitment_level: SOLANA_NATIVE_COMMITMENT_CONFIRMED,
            confirmed_mode_authorization_enabled: true,
            ..SolanaNativeLiveRequestPolicy::default()
        }
    }

    fn base_record(owner: SolanaPubkeyBytes) -> AclRecordWitness {
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
                    subject: owner,
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

    fn parsed_request_fixture() -> (
        SolanaNativeParsedRequestV0,
        SolanaNativeAccountSnapshotV0,
        SolanaPubkeyBytes,
    ) {
        let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(&[11; 32]).unwrap();
        let owner: SolanaPubkeyBytes = key_pair.public_key().as_ref().try_into().unwrap();
        let record = base_record(owner);
        let material = material_commitment(&record);
        let parsed_entry = SolanaNativeParsedHandleEntryV0 {
            handle: record.handle,
            owner_pubkey: owner,
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
        };
        let raw_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id: [8; 32],
            response_context: b"context".to_vec(),
        });
        let user_reencryption_public_key = b"reencryption-key".to_vec();
        let mut payload = SolanaUserDecryptionPayloadV0 {
            domain_separator: solana_native_domain_separator(
                HOST_CHAIN_ID,
                [9; 32],
                HOST_PROGRAM_ID,
                [8; 32],
            ),
            host_chain_id: HOST_CHAIN_ID,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            user_reencryption_pubkey_len: user_reencryption_public_key.len() as u32,
            user_reencryption_pubkey_hash: solana_native_reencryption_pubkey_hash(
                &user_reencryption_public_key,
            ),
            request_signer_pubkey: owner,
            acl_program_id: HOST_PROGRAM_ID,
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: SOLANA_NATIVE_COMMITMENT_CONFIRMED,
            min_context_slot: OBSERVED_SLOT - 2,
            expiration_slot: OBSERVED_SLOT + 20,
            nonce: [77; 32],
            extra_data_hash: solana_native_extra_data_hash(&raw_extra_data),
            allowed_acl_domain_keys: vec![DOMAIN],
            entries_hash: [0; 32],
        };
        let hash_entry = crate::core::solana_acl::SolanaNativeHandleEntryV0 {
            handle: parsed_entry.handle,
            owner_pubkey: parsed_entry.owner_pubkey,
            owner_permission_account: parsed_entry.owner_permission_account,
            delegator_pubkey: parsed_entry.delegator_pubkey,
            delegator_permission_account: parsed_entry.delegator_permission_account,
            delegate_pubkey: parsed_entry.delegate_pubkey,
            app_context_pubkey: parsed_entry.app_context_pubkey,
            app_context_permission_account: parsed_entry.app_context_permission_account,
            acl_record_account: parsed_entry.acl_record_account,
            delegation_record_account: parsed_entry.delegation_record_account,
            expected_delegation_counter: parsed_entry.expected_delegation_counter,
            material_commitment_account: parsed_entry.material_commitment_account,
            material_commitment_hash: parsed_entry.material_commitment_hash,
            min_context_slot: parsed_entry.min_context_slot,
            config_version: parsed_entry.config_version,
            material_source_mode: parsed_entry.material_source_mode,
            acl_layout_version: parsed_entry.acl_layout_version,
            handle_derivation_version: parsed_entry.handle_derivation_version,
            material_commitment_version: parsed_entry.material_commitment_version,
            expected_key_id: parsed_entry.expected_key_id,
            acl_record: record.clone(),
            material: material.clone(),
            owner_permission: None,
            delegator_permission: None,
            app_context_permission: None,
            delegation: None,
        };
        payload.entries_hash = solana_native_entries_hash(&[hash_entry]);
        let request_hash = solana_native_request_hash(&payload);
        let signature = key_pair.sign(&solana_native_request_signature_message(request_hash));
        let parsed_request = SolanaNativeParsedRequestV0 {
            payload,
            entries: vec![parsed_entry],
            raw_extra_data,
            user_reencryption_public_key,
            request_signature: signature.as_ref().to_vec(),
        };
        let snapshot = SolanaNativeAccountSnapshotV0 {
            observed_slot: OBSERVED_SLOT,
            commitment_level: SOLANA_NATIVE_COMMITMENT_CONFIRMED,
            rpc_response_bytes: 1024,
            accounts: vec![
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
            ],
        };
        (parsed_request, snapshot, owner)
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

    #[tokio::test]
    async fn live_native_request_fetches_accounts_and_admits_with_recheck_action() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        let admitted = admit_live_solana_native_request_v0(
            &parsed_request,
            &admission,
            &fetcher,
            confirmed_policy(),
        )
        .await
        .unwrap();

        assert_eq!(admitted.observed_slot, OBSERVED_SLOT);
        assert_eq!(
            admitted.finality_action,
            SolanaNativeFinalityAction::RecheckBeforeRelease
        );
        assert_eq!(
            admitted.admitted.replay_action,
            Some(SolanaNativeReplayAction::Reserve)
        );
    }

    #[tokio::test]
    async fn live_native_request_default_policy_rejects_confirmed_commitment_before_fetch() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &fetcher,
                SolanaNativeLiveRequestPolicy::default(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::CommitmentBelowMinimum)
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_processed_commitment_before_fetch() {
        let (mut parsed_request, snapshot, _) = parsed_request_fixture();
        parsed_request.payload.commitment_level = SOLANA_NATIVE_COMMITMENT_PROCESSED;
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &fetcher,
                SolanaNativeLiveRequestPolicy::default(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::UnsupportedCommitmentLevel)
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_oversized_rpc_snapshot_before_replay() {
        let (parsed_request, mut snapshot, _) = parsed_request_fixture();
        snapshot.rpc_response_bytes = 2048;
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let policy = SolanaNativeLiveRequestPolicy {
            max_rpc_response_bytes: 1024,
            ..confirmed_policy()
        };

        assert!(matches!(
            admit_live_solana_native_request_v0(&parsed_request, &admission, &fetcher, policy)
                .await,
            Err(SolanaNativeLiveAdmissionError::RpcResponseTooLarge)
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_excessive_validity_window() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let policy = SolanaNativeLiveRequestPolicy {
            max_validity_slots: 1,
            ..confirmed_policy()
        };

        assert!(matches!(
            admit_live_solana_native_request_v0(&parsed_request, &admission, &fetcher, policy)
                .await,
            Err(SolanaNativeLiveAdmissionError::ValidityWindowExceeded)
        ));
    }

    #[tokio::test]
    async fn live_native_public_request_rejects_excessive_validity_window() {
        let (mut parsed_request, snapshot, _) = parsed_request_fixture();
        parsed_request.payload.request_mode = SOLANA_NATIVE_REQUEST_MODE_PUBLIC;
        parsed_request.payload.request_signer_pubkey = [0; 32];
        parsed_request.payload.nonce = [0; 32];
        parsed_request.payload.user_reencryption_pubkey_len = 0;
        parsed_request.payload.user_reencryption_pubkey_hash = [0; 32];
        parsed_request.user_reencryption_public_key.clear();
        parsed_request.request_signature.clear();
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let policy = SolanaNativeLiveRequestPolicy {
            max_validity_slots: 1,
            ..confirmed_policy()
        };

        assert!(matches!(
            admit_live_solana_native_request_v0(&parsed_request, &admission, &fetcher, policy)
                .await,
            Err(SolanaNativeLiveAdmissionError::ValidityWindowExceeded)
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_too_many_account_fetches_before_fetch() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let policy = SolanaNativeLiveRequestPolicy {
            max_account_fetches_per_request: 1,
            ..confirmed_policy()
        };

        assert!(matches!(
            admit_live_solana_native_request_v0(&parsed_request, &admission, &fetcher, policy)
                .await,
            Err(SolanaNativeLiveAdmissionError::TooManyAccountFetches)
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_bad_signature_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.request_signature[0] ^= 0xff;
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(
                    SolanaNativeRequestError::InvalidRequestSignature
                )
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_reencryption_hash_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.payload.user_reencryption_pubkey_hash = [99; 32];
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(
                    SolanaNativeRequestError::ReencryptionPublicKeyMismatch
                )
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_acl_program_mismatch_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.payload.acl_program_id = [77; 32];
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(SolanaNativeRequestError::AclProgramMismatch)
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_entries_hash_mismatch_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.payload.entries_hash[0] ^= 0xff;
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(SolanaNativeRequestError::EntriesHashMismatch)
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_malformed_extra_data_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.raw_extra_data[0] = 1;
        parsed_request.payload.extra_data_hash =
            solana_native_extra_data_hash(&parsed_request.raw_extra_data);
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(SolanaNativeRequestError::InvalidExtraData)
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_extra_data_context_mismatch_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.raw_extra_data[1] ^= 0xff;
        parsed_request.payload.extra_data_hash =
            solana_native_extra_data_hash(&parsed_request.raw_extra_data);
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(SolanaNativeRequestError::KmsContextMismatch)
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_rejects_extra_data_hash_mismatch_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        let last = parsed_request.raw_extra_data.last_mut().unwrap();
        *last ^= 0xff;
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(
                    SolanaNativeRequestError::ExtraDataHashMismatch
                )
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_public_request_rejects_signature_before_fetch() {
        let (mut parsed_request, _, _) = parsed_request_fixture();
        parsed_request.payload.request_mode = SOLANA_NATIVE_REQUEST_MODE_PUBLIC;
        parsed_request.payload.request_signer_pubkey = [0; 32];
        parsed_request.payload.nonce = [0; 32];
        parsed_request.payload.user_reencryption_pubkey_len = 0;
        parsed_request.payload.user_reencryption_pubkey_hash = [0; 32];
        parsed_request.user_reencryption_public_key.clear();
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        assert!(matches!(
            admit_live_solana_native_request_v0(
                &parsed_request,
                &admission,
                &UnexpectedAccountFetcher,
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::Admission(
                SolanaNativeAdmissionError::Request(
                    SolanaNativeRequestError::InvalidRequestSignature
                )
            ))
        ));
    }

    #[tokio::test]
    async fn live_native_request_with_finalized_snapshot_can_release_now() {
        let (parsed_request, mut snapshot, _) = parsed_request_fixture();
        snapshot.commitment_level = SOLANA_NATIVE_COMMITMENT_FINALIZED;
        let fetcher = StaticAccountFetcher { snapshot };
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );

        let admitted = admit_live_solana_native_request_v0(
            &parsed_request,
            &admission,
            &fetcher,
            confirmed_policy(),
        )
        .await
        .unwrap();

        assert_eq!(
            admitted.finality_action,
            SolanaNativeFinalityAction::ReleaseNow
        );
    }

    #[tokio::test]
    async fn finality_recheck_refetches_finalized_accounts_and_reuses_replay_key() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let mut finalized_snapshot = snapshot.clone();
        finalized_snapshot.observed_slot += 1;
        finalized_snapshot.commitment_level = SOLANA_NATIVE_COMMITMENT_FINALIZED;
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let initial = admit_live_solana_native_request_v0(
            &parsed_request,
            &admission,
            &StaticAccountFetcher { snapshot },
            confirmed_policy(),
        )
        .await
        .unwrap();

        let rechecked = recheck_live_solana_native_request_before_release_v0(
            &parsed_request,
            &initial,
            &admission,
            &StaticAccountFetcher {
                snapshot: finalized_snapshot,
            },
            confirmed_policy(),
        )
        .await
        .unwrap();

        assert_eq!(
            rechecked.finality_action,
            SolanaNativeFinalityAction::ReleaseNow
        );
        assert_eq!(
            rechecked.admitted.replay_action,
            Some(SolanaNativeReplayAction::ReuseExisting)
        );
    }

    #[tokio::test]
    async fn finality_recheck_rejects_changed_account_bytes_before_release() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let mut finalized_snapshot = snapshot.clone();
        finalized_snapshot.observed_slot += 1;
        finalized_snapshot.commitment_level = SOLANA_NATIVE_COMMITMENT_FINALIZED;
        finalized_snapshot.accounts[0].data[0] ^= 0xff;
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let initial = admit_live_solana_native_request_v0(
            &parsed_request,
            &admission,
            &StaticAccountFetcher { snapshot },
            confirmed_policy(),
        )
        .await
        .unwrap();

        assert!(matches!(
            recheck_live_solana_native_request_before_release_v0(
                &parsed_request,
                &initial,
                &admission,
                &StaticAccountFetcher {
                    snapshot: finalized_snapshot,
                },
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::FinalityRecheckAccountMismatch)
        ));
    }

    #[tokio::test]
    async fn finality_recheck_rejects_non_finalized_refetch() {
        let (parsed_request, snapshot, _) = parsed_request_fixture();
        let admission = SolanaNativeRequestAdmission::new(
            SolanaAclVerifier::new(HOST_PROGRAM_ID),
            InMemoryReplayStore::default(),
            SolanaNativeRequestLimits::default(),
        );
        let initial = admit_live_solana_native_request_v0(
            &parsed_request,
            &admission,
            &StaticAccountFetcher {
                snapshot: snapshot.clone(),
            },
            confirmed_policy(),
        )
        .await
        .unwrap();

        assert!(matches!(
            recheck_live_solana_native_request_before_release_v0(
                &parsed_request,
                &initial,
                &admission,
                &NonFinalizedRecheckFetcher { snapshot },
                confirmed_policy(),
            )
            .await,
            Err(SolanaNativeLiveAdmissionError::FinalityRecheckNotFinalized)
        ));
    }
}
