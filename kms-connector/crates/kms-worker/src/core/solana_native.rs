use crate::core::{
    solana_acl::{
        SolanaAclVerifier, SolanaNativeAcceptedRequestV0, SolanaNativeDecryptionRequestV0,
        SolanaNativeReplayAction, SolanaNativeRequestError, SolanaNativeRequestLimits,
        SolanaPubkeyBytes, verify_solana_native_request_signature,
    },
    solana_replay::{SolanaNativeReplayStore, SolanaNativeReplayStoreError},
};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct SolanaNativeRequestAdmission<S> {
    verifier: SolanaAclVerifier,
    replay_store: S,
    limits: SolanaNativeRequestLimits,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaNativeAdmittedRequestV0 {
    pub accepted: SolanaNativeAcceptedRequestV0,
    pub replay_action: Option<SolanaNativeReplayAction>,
}

#[derive(Debug, Error)]
pub enum SolanaNativeAdmissionError {
    #[error("native Solana request admission failed: {0}")]
    Request(#[from] SolanaNativeRequestError),
    #[error("native Solana accepted request does not match the request signer")]
    AcceptedRequestSignerMismatch,
    #[error("native Solana replay reservation failed: {0}")]
    Replay(#[from] SolanaNativeReplayStoreError),
}

impl<S> SolanaNativeRequestAdmission<S> {
    pub fn new(
        verifier: SolanaAclVerifier,
        replay_store: S,
        limits: SolanaNativeRequestLimits,
    ) -> Self {
        Self {
            verifier,
            replay_store,
            limits,
        }
    }

    pub fn host_program_id(&self) -> SolanaPubkeyBytes {
        self.verifier.host_program_id
    }

    pub fn request_limits(&self) -> SolanaNativeRequestLimits {
        self.limits
    }

    pub async fn admit_v0_signed_request(
        &self,
        request: &SolanaNativeDecryptionRequestV0,
        observed_slot: u64,
        request_signature: &[u8],
    ) -> Result<SolanaNativeAdmittedRequestV0, SolanaNativeAdmissionError>
    where
        S: SolanaNativeReplayStore + Sync,
    {
        let accepted =
            self.verifier
                .verify_native_v0_request(request, observed_slot, self.limits)?;
        admit_verified_native_v0_request(
            accepted,
            request.payload.request_signer_pubkey,
            request_signature,
            &self.replay_store,
        )
        .await
    }
}

pub async fn admit_verified_native_v0_request<S>(
    accepted: SolanaNativeAcceptedRequestV0,
    request_signer_pubkey: SolanaPubkeyBytes,
    request_signature: &[u8],
    replay_store: &S,
) -> Result<SolanaNativeAdmittedRequestV0, SolanaNativeAdmissionError>
where
    S: SolanaNativeReplayStore + Sync,
{
    if let Some(replay_key) = accepted.replay_key.as_ref() {
        if replay_key.request_signer_pubkey != request_signer_pubkey {
            return Err(SolanaNativeAdmissionError::AcceptedRequestSignerMismatch);
        }
        verify_solana_native_request_signature(
            request_signer_pubkey,
            accepted.request_hash,
            request_signature,
        )?;
    } else if request_signer_pubkey != [0; 32] || !request_signature.is_empty() {
        return Err(SolanaNativeRequestError::InvalidRequestSignature.into());
    }

    let replay_action = replay_store.reserve_accepted_request(&accepted).await?;
    Ok(SolanaNativeAdmittedRequestV0 {
        accepted,
        replay_action,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        solana_acl::{SolanaNativeReplayKeyV0, check_solana_native_replay},
        solana_replay::SolanaNativeReplayStoreError,
    };
    use ring::signature::KeyPair;
    use std::sync::Mutex;

    #[derive(Default)]
    struct InMemoryReplayStore {
        seen: Mutex<Vec<(SolanaNativeReplayKeyV0, [u8; 32])>>,
    }

    impl SolanaNativeReplayStore for InMemoryReplayStore {
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

    fn replay_key(signer: SolanaPubkeyBytes) -> SolanaNativeReplayKeyV0 {
        SolanaNativeReplayKeyV0 {
            host_chain_id: 900,
            solana_cluster_id: [9; 32],
            kms_context_id: [8; 32],
            request_signer_pubkey: signer,
            nonce: [77; 32],
        }
    }

    fn signed_request_hash(
        request_hash: [u8; 32],
    ) -> (SolanaPubkeyBytes, ring::signature::Signature) {
        let key_pair = ring::signature::Ed25519KeyPair::from_seed_unchecked(&[11; 32]).unwrap();
        let signature = key_pair
            .sign(&crate::core::solana_acl::solana_native_request_signature_message(request_hash));
        let public_key = key_pair.public_key().as_ref().try_into().unwrap();
        (public_key, signature)
    }

    #[tokio::test]
    async fn signed_native_request_admission_reserves_then_reuses_replay_key() {
        let request_hash = [42; 32];
        let (signer, signature) = signed_request_hash(request_hash);
        let accepted = SolanaNativeAcceptedRequestV0 {
            request_hash,
            replay_key: Some(replay_key(signer)),
        };
        let replay_store = InMemoryReplayStore::default();

        let first = admit_verified_native_v0_request(
            accepted.clone(),
            signer,
            signature.as_ref(),
            &replay_store,
        )
        .await
        .unwrap();
        assert_eq!(first.replay_action, Some(SolanaNativeReplayAction::Reserve));

        let second =
            admit_verified_native_v0_request(accepted, signer, signature.as_ref(), &replay_store)
                .await
                .unwrap();
        assert_eq!(
            second.replay_action,
            Some(SolanaNativeReplayAction::ReuseExisting)
        );
    }

    #[tokio::test]
    async fn signed_native_request_admission_rejects_bad_signature_before_replay() {
        let request_hash = [42; 32];
        let (signer, signature) = signed_request_hash(request_hash);
        let accepted = SolanaNativeAcceptedRequestV0 {
            request_hash,
            replay_key: Some(replay_key(signer)),
        };
        let replay_store = InMemoryReplayStore::default();
        let mut bad_signature = signature.as_ref().to_vec();
        bad_signature[0] ^= 0xff;

        assert!(matches!(
            admit_verified_native_v0_request(accepted, signer, &bad_signature, &replay_store).await,
            Err(SolanaNativeAdmissionError::Request(
                SolanaNativeRequestError::InvalidRequestSignature
            ))
        ));
        assert!(replay_store.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn signed_native_request_admission_rejects_replay_key_signer_mismatch_before_replay() {
        let request_hash = [42; 32];
        let (signer, signature) = signed_request_hash(request_hash);
        let accepted = SolanaNativeAcceptedRequestV0 {
            request_hash,
            replay_key: Some(replay_key([99; 32])),
        };
        let replay_store = InMemoryReplayStore::default();

        assert!(matches!(
            admit_verified_native_v0_request(accepted, signer, signature.as_ref(), &replay_store)
                .await,
            Err(SolanaNativeAdmissionError::AcceptedRequestSignerMismatch)
        ));
        assert!(replay_store.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn signed_native_request_admission_rejects_replay_with_different_hash() {
        let request_hash = [42; 32];
        let (signer, signature) = signed_request_hash(request_hash);
        let replay_key = replay_key(signer);
        let replay_store = InMemoryReplayStore::default();

        admit_verified_native_v0_request(
            SolanaNativeAcceptedRequestV0 {
                request_hash,
                replay_key: Some(replay_key.clone()),
            },
            signer,
            signature.as_ref(),
            &replay_store,
        )
        .await
        .unwrap();

        let different_hash = [43; 32];
        let (_, different_signature) = signed_request_hash(different_hash);
        assert!(matches!(
            admit_verified_native_v0_request(
                SolanaNativeAcceptedRequestV0 {
                    request_hash: different_hash,
                    replay_key: Some(replay_key),
                },
                signer,
                different_signature.as_ref(),
                &replay_store,
            )
            .await,
            Err(SolanaNativeAdmissionError::Replay(
                SolanaNativeReplayStoreError::ReplayDetected
            ))
        ));
    }

    #[tokio::test]
    async fn public_native_request_admission_rejects_unexpected_signature() {
        let replay_store = InMemoryReplayStore::default();
        let accepted = SolanaNativeAcceptedRequestV0 {
            request_hash: [42; 32],
            replay_key: None,
        };

        assert!(matches!(
            admit_verified_native_v0_request(accepted, [0; 32], &[1; 64], &replay_store).await,
            Err(SolanaNativeAdmissionError::Request(
                SolanaNativeRequestError::InvalidRequestSignature
            ))
        ));
    }

    #[tokio::test]
    async fn public_native_request_admission_rejects_nonzero_signer_without_replay_key() {
        let replay_store = InMemoryReplayStore::default();
        let accepted = SolanaNativeAcceptedRequestV0 {
            request_hash: [42; 32],
            replay_key: None,
        };

        assert!(matches!(
            admit_verified_native_v0_request(accepted, [7; 32], &[], &replay_store).await,
            Err(SolanaNativeAdmissionError::Request(
                SolanaNativeRequestError::InvalidRequestSignature
            ))
        ));
        assert!(replay_store.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn public_native_request_admission_skips_signature_and_replay() {
        let replay_store = InMemoryReplayStore::default();
        let admitted = admit_verified_native_v0_request(
            SolanaNativeAcceptedRequestV0 {
                request_hash: [42; 32],
                replay_key: None,
            },
            [0; 32],
            &[],
            &replay_store,
        )
        .await
        .unwrap();

        assert_eq!(admitted.replay_action, None);
        assert!(replay_store.seen.lock().unwrap().is_empty());
    }
}
