use crate::core::{
    solana_acl::{
        SolanaKmsExtraDataV0, SolanaNativeRequestError, SolanaNativeRequestLimits,
        decode_solana_kms_extra_data_v0, encode_solana_kms_extra_data_v0,
        solana_native_extra_data_hash,
    },
    solana_live::{
        SolanaNativeAccountFetcher, SolanaNativeLiveAdmissionError,
        SolanaNativeLiveAdmittedRequestV0, SolanaNativeLiveRequestPolicy,
        admit_live_solana_native_request_v0, recheck_live_solana_native_request_before_release_v0,
    },
    solana_native::SolanaNativeRequestAdmission,
    solana_replay::SolanaNativeReplayStore,
    solana_request::{
        SolanaNativeAccountWitnessV0, SolanaNativeParsedRequestV0, SolanaNativeRequestParseError,
        parse_solana_native_request_v0,
    },
    solana_response::{
        SolanaKmsResponseCertificateV0, SolanaKmsResponsePayloadV0,
        SolanaKmsResponseVerificationConfigV0, SolanaKmsResponseVerificationError,
        SolanaKmsVerifiedResponseV0, verify_solana_kms_response_v0,
    },
};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeLiveRequestReleaseV0 {
    pub parsed_request: SolanaNativeParsedRequestV0,
    pub final_admission: SolanaNativeLiveAdmittedRequestV0,
    pub response_context: Vec<u8>,
}

impl SolanaNativeLiveRequestReleaseV0 {
    pub fn account_witnesses(&self) -> &[SolanaNativeAccountWitnessV0] {
        &self.final_admission.account_witnesses
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeResponseRouteV0 {
    pub host_chain_id: u64,
    pub solana_cluster_id: [u8; 32],
    pub kms_context_id: [u8; 32],
    pub request_hash: [u8; 32],
    pub request_mode: u8,
    pub response_kind: u8,
    pub response_context: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeVerifiedResponsePublicationV0 {
    pub route: SolanaNativeResponseRouteV0,
    pub verified: SolanaKmsVerifiedResponseV0,
    pub response_payload: SolanaKmsResponsePayloadV0,
    pub raw_response_body: Vec<u8>,
    pub certificate: SolanaKmsResponseCertificateV0,
}

#[derive(Debug, Error)]
pub enum SolanaNativeLiveFlowError {
    #[error("native Solana request parsing failed: {0}")]
    Parse(#[from] SolanaNativeRequestParseError),
    #[error("native Solana request live admission failed: {0}")]
    Admission(#[from] SolanaNativeLiveAdmissionError),
    #[error("native Solana request validation failed: {0}")]
    Request(#[from] SolanaNativeRequestError),
    #[error("native Solana response verification failed: {0}")]
    Response(#[from] SolanaKmsResponseVerificationError),
    #[error("native Solana response route context does not match the signed extra-data hash")]
    RouteContextMismatch,
}

#[derive(Clone, Debug)]
pub struct SolanaNativeLiveRequestProcessor<S, F> {
    request_limits: SolanaNativeRequestLimits,
    live_policy: SolanaNativeLiveRequestPolicy,
    admission: SolanaNativeRequestAdmission<S>,
    account_fetcher: F,
}

impl<S, F> SolanaNativeLiveRequestProcessor<S, F> {
    pub fn new(
        request_limits: SolanaNativeRequestLimits,
        live_policy: SolanaNativeLiveRequestPolicy,
        admission: SolanaNativeRequestAdmission<S>,
        account_fetcher: F,
    ) -> Self {
        Self {
            request_limits,
            live_policy,
            admission,
            account_fetcher,
        }
    }

    pub async fn admit_request_bytes_for_release_v0(
        &self,
        request_bytes: &[u8],
    ) -> Result<SolanaNativeLiveRequestReleaseV0, SolanaNativeLiveFlowError>
    where
        S: SolanaNativeReplayStore + Sync,
        F: SolanaNativeAccountFetcher + Sync,
    {
        let parsed_request = parse_solana_native_request_v0(request_bytes, self.request_limits)?;
        let first_admission = admit_live_solana_native_request_v0(
            &parsed_request,
            &self.admission,
            &self.account_fetcher,
            self.live_policy,
        )
        .await?;
        let final_admission = recheck_live_solana_native_request_before_release_v0(
            &parsed_request,
            &first_admission,
            &self.admission,
            &self.account_fetcher,
            self.live_policy,
        )
        .await?;
        let extra_data =
            decode_solana_kms_extra_data_v0(&parsed_request.raw_extra_data, self.request_limits)?;

        Ok(SolanaNativeLiveRequestReleaseV0 {
            parsed_request,
            final_admission,
            response_context: extra_data.response_context,
        })
    }
}

pub fn verify_solana_native_response_for_release_v0(
    release: &SolanaNativeLiveRequestReleaseV0,
    response_config: &SolanaKmsResponseVerificationConfigV0,
    response_payload: SolanaKmsResponsePayloadV0,
    raw_response_body: Vec<u8>,
    certificate: SolanaKmsResponseCertificateV0,
) -> Result<SolanaNativeVerifiedResponsePublicationV0, SolanaNativeLiveFlowError> {
    let accepted = &release.final_admission.admitted.accepted;
    verify_release_response_context_v0(release)?;
    let verified = verify_solana_kms_response_v0(
        response_config,
        accepted,
        &release.parsed_request.payload,
        &response_payload,
        &raw_response_body,
        &certificate,
    )?;
    let route = SolanaNativeResponseRouteV0 {
        host_chain_id: release.parsed_request.payload.host_chain_id,
        solana_cluster_id: release.parsed_request.payload.solana_cluster_id,
        kms_context_id: release.parsed_request.payload.kms_context_id,
        request_hash: accepted.request_hash,
        request_mode: release.parsed_request.payload.request_mode,
        response_kind: response_payload.response_kind,
        response_context: release.response_context.clone(),
    };

    Ok(SolanaNativeVerifiedResponsePublicationV0 {
        route,
        verified,
        response_payload,
        raw_response_body,
        certificate,
    })
}

fn verify_release_response_context_v0(
    release: &SolanaNativeLiveRequestReleaseV0,
) -> Result<(), SolanaNativeLiveFlowError> {
    let expected_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
        kms_context_id: release.parsed_request.payload.kms_context_id,
        response_context: release.response_context.clone(),
    });
    if solana_native_extra_data_hash(&expected_extra_data)
        != release.parsed_request.payload.extra_data_hash
    {
        return Err(SolanaNativeLiveFlowError::RouteContextMismatch);
    }
    Ok(())
}

pub fn solana_native_response_route_from_extra_data_v0(
    request: &SolanaNativeParsedRequestV0,
    accepted_request_hash: [u8; 32],
    response_kind: u8,
    extra_data: &SolanaKmsExtraDataV0,
) -> SolanaNativeResponseRouteV0 {
    SolanaNativeResponseRouteV0 {
        host_chain_id: request.payload.host_chain_id,
        solana_cluster_id: request.payload.solana_cluster_id,
        kms_context_id: request.payload.kms_context_id,
        request_hash: accepted_request_hash,
        request_mode: request.payload.request_mode,
        response_kind,
        response_context: extra_data.response_context.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        solana_acl::{
            SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED, SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            SolanaAclVerifier, SolanaNativeAcceptedRequestV0, SolanaNativeReplayKeyV0,
            SolanaPubkeyBytes, SolanaUserDecryptionPayloadV0, encode_solana_kms_extra_data_v0,
            solana_native_domain_separator, solana_native_extra_data_hash,
            solana_native_reencryption_pubkey_hash, solana_native_request_hash,
        },
        solana_live::{
            SOLANA_NATIVE_COMMITMENT_CONFIRMED, SolanaNativeAccountFetchError,
            SolanaNativeAccountSnapshotV0,
        },
        solana_native::SolanaNativeAdmittedRequestV0,
        solana_replay::{SolanaNativeReplayStore, SolanaNativeReplayStoreError},
        solana_response::{
            KmsResponseSignatureV0, SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            solana_native_kms_response_hash, solana_native_kms_response_signature_message,
            solana_native_kms_response_signer_set_hash, solana_native_response_body_hash,
        },
    };
    use ring::signature::KeyPair;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[derive(Clone, Default)]
    struct CountingFetcher {
        fetch_count: Arc<AtomicUsize>,
    }

    impl SolanaNativeAccountFetcher for CountingFetcher {
        #[allow(clippy::manual_async_fn)]
        fn fetch_accounts(
            &self,
            _account_keys: &[SolanaPubkeyBytes],
            _commitment_level: u8,
        ) -> impl std::future::Future<
            Output = Result<SolanaNativeAccountSnapshotV0, SolanaNativeAccountFetchError>,
        > + Send {
            let fetch_count = self.fetch_count.clone();
            async move {
                fetch_count.fetch_add(1, Ordering::SeqCst);
                Err(SolanaNativeAccountFetchError::Unavailable(
                    "test fetcher has no accounts".to_string(),
                ))
            }
        }
    }

    #[derive(Clone, Default, Debug)]
    struct NoopReplayStore;

    impl SolanaNativeReplayStore for NoopReplayStore {
        #[allow(clippy::manual_async_fn)]
        fn reserve_accepted_request(
            &self,
            _accepted: &SolanaNativeAcceptedRequestV0,
        ) -> impl std::future::Future<
            Output = Result<
                Option<crate::core::solana_acl::SolanaNativeReplayAction>,
                SolanaNativeReplayStoreError,
            >,
        > + Send {
            async { Ok(None) }
        }
    }

    fn request_payload_fixture() -> SolanaUserDecryptionPayloadV0 {
        let raw_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id: [8; 32],
            response_context: b"solana-route".to_vec(),
        });
        let reencryption_key = b"reencryption-key".to_vec();
        SolanaUserDecryptionPayloadV0 {
            domain_separator: solana_native_domain_separator(900, [9; 32], [42; 32], [8; 32]),
            host_chain_id: 900,
            config_version: 3,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            user_reencryption_pubkey_len: reencryption_key.len() as u32,
            user_reencryption_pubkey_hash: solana_native_reencryption_pubkey_hash(
                &reencryption_key,
            ),
            request_signer_pubkey: [7; 32],
            acl_program_id: [42; 32],
            request_mode: SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED,
            material_source_mode: SOLANA_NATIVE_SUPPORTED_MATERIAL_SOURCE_MODE,
            commitment_level: SOLANA_NATIVE_COMMITMENT_CONFIRMED,
            min_context_slot: 500,
            expiration_slot: 520,
            nonce: [77; 32],
            extra_data_hash: solana_native_extra_data_hash(&raw_extra_data),
            allowed_acl_domain_keys: vec![[1; 32]],
            entries_hash: [99; 32],
        }
    }

    fn release_fixture(response_context: Vec<u8>) -> SolanaNativeLiveRequestReleaseV0 {
        let payload = request_payload_fixture();
        let request_hash = solana_native_request_hash(&payload);
        let accepted = SolanaNativeAcceptedRequestV0 {
            request_hash,
            replay_key: Some(SolanaNativeReplayKeyV0 {
                host_chain_id: payload.host_chain_id,
                solana_cluster_id: payload.solana_cluster_id,
                kms_context_id: payload.kms_context_id,
                request_signer_pubkey: payload.request_signer_pubkey,
                nonce: payload.nonce,
            }),
        };
        SolanaNativeLiveRequestReleaseV0 {
            parsed_request: SolanaNativeParsedRequestV0 {
                payload,
                entries: Vec::new(),
                raw_extra_data: Vec::new(),
                user_reencryption_public_key: Vec::new(),
                request_signature: Vec::new(),
            },
            final_admission: SolanaNativeLiveAdmittedRequestV0 {
                admitted: SolanaNativeAdmittedRequestV0 {
                    accepted,
                    replay_action: None,
                },
                observed_slot: 500,
                observed_commitment_level: SOLANA_NATIVE_COMMITMENT_CONFIRMED,
                finality_action: crate::core::solana_live::SolanaNativeFinalityAction::ReleaseNow,
                account_witnesses: Vec::new(),
            },
            response_context,
        }
    }

    #[tokio::test]
    async fn native_live_flow_rejects_bad_wire_request_before_fetch() {
        let fetcher = CountingFetcher::default();
        let fetch_count = fetcher.fetch_count.clone();
        let processor = SolanaNativeLiveRequestProcessor::new(
            SolanaNativeRequestLimits::default(),
            SolanaNativeLiveRequestPolicy::default(),
            SolanaNativeRequestAdmission::new(
                SolanaAclVerifier::new([42; 32]),
                NoopReplayStore,
                SolanaNativeRequestLimits::default(),
            ),
            fetcher,
        );

        let err = processor
            .admit_request_bytes_for_release_v0(&[255])
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            SolanaNativeLiveFlowError::Parse(SolanaNativeRequestParseError::UnsupportedLayout)
        ));
        assert_eq!(fetch_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn native_response_publication_binds_verified_route() {
        let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(&[11; 32]).unwrap();
        let signer: SolanaPubkeyBytes = key_pair.public_key().as_ref().try_into().unwrap();
        let signer_pubkeys = vec![signer];
        let signer_set_hash =
            solana_native_kms_response_signer_set_hash([8; 32], 1, &signer_pubkeys);
        let response_config = SolanaKmsResponseVerificationConfigV0 {
            kms_context_id: [8; 32],
            signer_set_hash,
            threshold: 1,
            signer_pubkeys,
            max_signers: 4,
            max_signatures: 4,
        };
        let raw_response_body = b"verified-response-body".to_vec();
        let release = release_fixture(b"solana-route".to_vec());
        let request_hash = release.final_admission.admitted.accepted.request_hash;
        let response_payload = SolanaKmsResponsePayloadV0 {
            domain_separator: release.parsed_request.payload.domain_separator,
            host_chain_id: release.parsed_request.payload.host_chain_id,
            config_version: release.parsed_request.payload.config_version,
            solana_cluster_id: release.parsed_request.payload.solana_cluster_id,
            kms_context_id: release.parsed_request.payload.kms_context_id,
            request_hash,
            request_mode: release.parsed_request.payload.request_mode,
            response_kind: SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            nonce: release.parsed_request.payload.nonce,
            entries_hash: release.parsed_request.payload.entries_hash,
            extra_data_hash: release.parsed_request.payload.extra_data_hash,
            user_reencryption_pubkey_hash: release
                .parsed_request
                .payload
                .user_reencryption_pubkey_hash,
            response_body_len: raw_response_body.len() as u32,
            response_body_hash: solana_native_response_body_hash(&raw_response_body),
        };
        let response_hash = solana_native_kms_response_hash(&response_payload);
        let signature = key_pair.sign(&solana_native_kms_response_signature_message(response_hash));
        let certificate = SolanaKmsResponseCertificateV0 {
            kms_context_id: [8; 32],
            signer_set_hash,
            threshold: 1,
            signatures: vec![KmsResponseSignatureV0 {
                signer_pubkey: signer,
                signature: signature.as_ref().try_into().unwrap(),
            }],
        };

        let publication = verify_solana_native_response_for_release_v0(
            &release,
            &response_config,
            response_payload,
            raw_response_body.clone(),
            certificate,
        )
        .unwrap();

        assert_eq!(publication.verified.response_hash, response_hash);
        assert_eq!(publication.raw_response_body, raw_response_body);
        assert_eq!(publication.route.host_chain_id, 900);
        assert_eq!(publication.route.solana_cluster_id, [9; 32]);
        assert_eq!(publication.route.kms_context_id, [8; 32]);
        assert_eq!(publication.route.request_hash, request_hash);
        assert_eq!(publication.route.response_context, b"solana-route".to_vec());
    }

    #[test]
    fn native_response_publication_rejects_unsigned_route_context() {
        let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(&[11; 32]).unwrap();
        let signer: SolanaPubkeyBytes = key_pair.public_key().as_ref().try_into().unwrap();
        let signer_pubkeys = vec![signer];
        let signer_set_hash =
            solana_native_kms_response_signer_set_hash([8; 32], 1, &signer_pubkeys);
        let response_config = SolanaKmsResponseVerificationConfigV0 {
            kms_context_id: [8; 32],
            signer_set_hash,
            threshold: 1,
            signer_pubkeys,
            max_signers: 4,
            max_signatures: 4,
        };
        let raw_response_body = b"verified-response-body".to_vec();
        let release = release_fixture(b"wrong-route".to_vec());
        let request_hash = release.final_admission.admitted.accepted.request_hash;
        let response_payload = SolanaKmsResponsePayloadV0 {
            domain_separator: release.parsed_request.payload.domain_separator,
            host_chain_id: release.parsed_request.payload.host_chain_id,
            config_version: release.parsed_request.payload.config_version,
            solana_cluster_id: release.parsed_request.payload.solana_cluster_id,
            kms_context_id: release.parsed_request.payload.kms_context_id,
            request_hash,
            request_mode: release.parsed_request.payload.request_mode,
            response_kind: SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            nonce: release.parsed_request.payload.nonce,
            entries_hash: release.parsed_request.payload.entries_hash,
            extra_data_hash: release.parsed_request.payload.extra_data_hash,
            user_reencryption_pubkey_hash: release
                .parsed_request
                .payload
                .user_reencryption_pubkey_hash,
            response_body_len: raw_response_body.len() as u32,
            response_body_hash: solana_native_response_body_hash(&raw_response_body),
        };
        let response_hash = solana_native_kms_response_hash(&response_payload);
        let signature = key_pair.sign(&solana_native_kms_response_signature_message(response_hash));
        let certificate = SolanaKmsResponseCertificateV0 {
            kms_context_id: [8; 32],
            signer_set_hash,
            threshold: 1,
            signatures: vec![KmsResponseSignatureV0 {
                signer_pubkey: signer,
                signature: signature.as_ref().try_into().unwrap(),
            }],
        };

        assert!(matches!(
            verify_solana_native_response_for_release_v0(
                &release,
                &response_config,
                response_payload,
                raw_response_body,
                certificate,
            ),
            Err(SolanaNativeLiveFlowError::RouteContextMismatch)
        ));
    }

    #[test]
    fn route_helper_uses_decoded_extra_data_context() {
        let raw_extra_data = encode_solana_kms_extra_data_v0(&SolanaKmsExtraDataV0 {
            kms_context_id: [8; 32],
            response_context: b"client-context".to_vec(),
        });
        let request = SolanaNativeParsedRequestV0 {
            payload: request_payload_fixture(),
            entries: Vec::new(),
            raw_extra_data: raw_extra_data.clone(),
            user_reencryption_public_key: Vec::new(),
            request_signature: Vec::new(),
        };
        let extra_data =
            decode_solana_kms_extra_data_v0(&raw_extra_data, SolanaNativeRequestLimits::default())
                .unwrap();

        let route = solana_native_response_route_from_extra_data_v0(
            &request,
            [55; 32],
            SOLANA_NATIVE_RESPONSE_KIND_DIRECT_SCOPED,
            &extra_data,
        );

        assert_eq!(route.request_hash, [55; 32]);
        assert_eq!(route.request_mode, SOLANA_NATIVE_REQUEST_MODE_DIRECT_SCOPED);
        assert_eq!(route.response_context, b"client-context".to_vec());
    }
}
