//! Pad decryption processing. Authorization only: the KMS Core request is not built yet.

use crate::core::event_processor::{
    AclCheck, DecryptionProcessor, ProcessingError, RequestCheckError, RequestCheckKind,
    decryption::check_validity_window,
};
use alloy::{primitives::U256, providers::Provider, sol_types::SolStruct};
use anyhow::anyhow;
use connector_utils::types::event::PadDecryptionEvent;
use kms_connector_api::ErrorCode;
use sqlx::types::chrono::Utc;
use std::iter;
use user_decryption_signature::UserDecryptRequestVerification;

/// The rejection of authorized pad decryptions, until they can be processed.
pub fn unsupported() -> ProcessingError {
    ProcessingError::irrecoverable(
        ErrorCode::Unprocessable,
        anyhow!("pad decryption is not supported yet"),
    )
}

impl<GP, HP> DecryptionProcessor<GP, HP>
where
    GP: Provider + Clone + 'static,
    HP: Provider,
{
    /// Checks `userAddress` is authorized to decrypt every pad of the request:
    /// 1. the request validity window
    /// 2. per pad: unexpired witness, label derived from the witness, `consumer` in
    ///    `allowedContracts` when non-empty, and `userAddress` being the `submitter` or one of the
    ///    `decryptors` of the witness
    /// 3. concurrent host chain reads:
    ///    - EIP-712 signature verification with `ecrecover` → ERC-1271 fallback
    ///    - signature invalidation
    ///    - for each pad `userAddress` is not a member of: `userAddress` being a user decryption
    ///      delegate of the `submitter` or of one of the `decryptors` for the `consumer`
    #[tracing::instrument(skip_all)]
    pub async fn check_pad_decryption_request(
        &self,
        request: &PadDecryptionEvent,
    ) -> Result<(), RequestCheckError> {
        let payload = &request.payload;
        let user_address = payload.userAddress;
        let start = U256::from(payload.requestValidity.startTimestamp);
        let duration = U256::from(payload.requestValidity.durationSeconds);
        check_validity_window(start, duration)?;

        let host_client = self.host_client(payload.chainId)?;
        let mut checks = vec![AclCheck::signature_not_invalidated(user_address, start)];
        let now = Utc::now().timestamp() as u64;
        for pad in &payload.pads {
            let witness = &pad.witness;
            let unprocessable = |e| {
                Err(RequestCheckError::irrecoverable(
                    RequestCheckKind::Signature,
                    ErrorCode::Unprocessable,
                    e,
                ))
            };
            if witness.expirationTimestamp <= now {
                return unprocessable(anyhow!(
                    "Pad {} expired at {}, now is {now}",
                    pad.label,
                    witness.expirationTimestamp
                ));
            }
            if payload.label_of(witness) != pad.label {
                return unprocessable(anyhow!("Pad {} does not match its witness", pad.label));
            }
            if !payload.allowedContracts.is_empty()
                && !payload.allowedContracts.contains(&witness.consumer)
            {
                return unprocessable(anyhow!(
                    "Consumer {} of pad {} is not in allowedContracts",
                    witness.consumer,
                    pad.label
                ));
            }

            if witness.submitter == user_address || witness.decryptors.contains(&user_address) {
                continue;
            }
            checks.push(AclCheck::AnyDelegated {
                delegate: user_address,
                contract: witness.consumer,
                delegators: iter::once(witness.submitter)
                    .chain(witness.decryptors.iter().copied())
                    .collect(),
            });
        }

        let digest = UserDecryptRequestVerification {
            userAddress: user_address,
            publicKey: payload.publicKey.clone(),
            allowedContracts: payload.allowedContracts.clone(),
            startTimestamp: start,
            durationSeconds: duration,
            extraData: payload.extraData.clone(),
        }
        .eip712_signing_hash(&self.user_decrypt_domain(payload.chainId));

        // TODO: reject the request if `payload.prfId` is not a live PRF instance of the host
        // chain's `PaddedInputter` contract, once the contract is available in `host-contracts`.

        tokio::try_join!(
            biased;
            host_client.verify_signature(
                user_address,
                digest,
                request.signature.as_ref(),
                self.erc1271_gas_limit,
            ),
            async {
                let calls = checks.iter().flat_map(AclCheck::abi_encode).collect();
                let results = host_client.acl_multicall(calls).await?;
                AclCheck::verify_all(&checks, &results)
            },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::Config,
        event_processor::{CiphertextManager, HostRpcClient, ProcessingErrorKind},
    };
    use alloy::{
        primitives::{Address, Bytes, FixedBytes},
        providers::{ProviderBuilder, mock::Asserter},
        signers::{SignerSync, local::PrivateKeySigner},
        sol_types::SolValue,
    };
    use connector_utils::tests::{
        rand::{rand_address, rand_digest, rand_public_key, rand_u256},
        setup::acl_multicall_response,
    };
    use fhevm_host_bindings::acl::ACL;
    use kms_connector_api::{PadDecryptionPayload, PadEntry, PadWitness, RequestValidity};
    use rstest::rstest;
    use std::collections::HashMap;
    use user_decryption_signature::default_user_decrypt_domain;

    const CHAIN_ID: u64 = 12345;

    fn setup_test_processor(
        asserter: Asserter,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>> {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let host_clients = HashMap::from([(
            CHAIN_ID,
            HostRpcClient::new(
                CHAIN_ID,
                ACL::new(Address::default(), mock_provider.clone()),
            ),
        )]);
        let ciphertext_manager = CiphertextManager::for_test(mock_provider.clone());
        DecryptionProcessor::new(
            &Config::default(),
            mock_provider,
            host_clients,
            ciphertext_manager,
        )
    }

    /// A valid witness, expiring in an hour.
    fn witness(submitter: Address, decryptors: Vec<Address>) -> PadWitness {
        PadWitness {
            submitter,
            consumer: rand_address(),
            expirationTimestamp: Utc::now().timestamp() as u64 + 3600,
            decryptors,
            nonce: FixedBytes::ZERO,
        }
    }

    /// A pad decryption of `witness` on `CHAIN_ID`, signed by `signer` for `sign_chain_id`.
    fn pad_request(
        signer: &PrivateKeySigner,
        witness: PadWitness,
        allowed_contracts: Vec<Address>,
        sign_chain_id: u64,
    ) -> PadDecryptionEvent {
        let mut payload = PadDecryptionPayload {
            witnessVersion: 0,
            prfId: 1,
            chainId: CHAIN_ID,
            pads: vec![],
            userAddress: signer.address(),
            publicKey: Bytes::from(rand_public_key()),
            allowedContracts: allowed_contracts,
            requestValidity: RequestValidity {
                startTimestamp: Utc::now().timestamp() as u64 - 60,
                durationSeconds: 3600,
            },
            extraData: Bytes::default(),
        };
        payload.pads.push(PadEntry {
            label: payload.label_of(&witness),
            witness,
            padLength: 64,
        });

        let domain = default_user_decrypt_domain(
            sign_chain_id,
            Config::default().decryption_contract.address,
        );
        let digest = UserDecryptRequestVerification {
            userAddress: payload.userAddress,
            publicKey: payload.publicKey.clone(),
            allowedContracts: payload.allowedContracts.clone(),
            startTimestamp: U256::from(payload.requestValidity.startTimestamp),
            durationSeconds: U256::from(payload.requestValidity.durationSeconds),
            extraData: payload.extraData.clone(),
        }
        .eip712_signing_hash(&domain);
        let signature = signer.sign_hash_sync(&digest).unwrap();

        PadDecryptionEvent {
            decryption_id: rand_u256(),
            payload,
            signature: Bytes::from(signature.as_bytes().to_vec()),
        }
    }

    enum PadScenario {
        UserIsSubmitter,
        UserIsDecryptor,
        UserIsDelegateOfDecryptor,
        UserIsNotDelegate,
        ConsumerNotInAllowedContracts,
        WitnessExpired,
        LabelNotFromWitness,
        AclMulticallFails,
    }

    #[rstest]
    #[case::user_is_submitter(PadScenario::UserIsSubmitter, None)]
    #[case::user_is_decryptor(PadScenario::UserIsDecryptor, None)]
    #[case::user_is_delegate_of_decryptor(PadScenario::UserIsDelegateOfDecryptor, None)]
    #[case::user_is_not_delegate(
        PadScenario::UserIsNotDelegate,
        Some((ProcessingErrorKind::Recoverable, ErrorCode::AclDenied))
    )]
    #[case::consumer_not_in_allowed_contracts(
        PadScenario::ConsumerNotInAllowedContracts,
        Some((ProcessingErrorKind::Irrecoverable, ErrorCode::Unprocessable))
    )]
    #[case::witness_expired(
        PadScenario::WitnessExpired,
        Some((ProcessingErrorKind::Irrecoverable, ErrorCode::Unprocessable))
    )]
    #[case::label_not_from_witness(
        PadScenario::LabelNotFromWitness,
        Some((ProcessingErrorKind::Irrecoverable, ErrorCode::Unprocessable))
    )]
    #[case::acl_multicall_fails(
        PadScenario::AclMulticallFails,
        Some((ProcessingErrorKind::Recoverable, ErrorCode::UpstreamTransient))
    )]
    #[tokio::test]
    async fn check_pad_decryption_request(
        #[case] scenario: PadScenario,
        #[case] expected: Option<(ProcessingErrorKind, ErrorCode)>,
    ) {
        let asserter = Asserter::new();
        let processor = setup_test_processor(asserter.clone());
        let user = PrivateKeySigner::random();
        let not_invalidated = U256::ZERO.abi_encode();

        let mut allowed_contracts = vec![];
        let mut witness = witness(rand_address(), vec![rand_address()]);
        let mut acl_results = None;
        match scenario {
            PadScenario::UserIsSubmitter => {
                witness.submitter = user.address();
                acl_results = Some(vec![not_invalidated]);
            }
            PadScenario::UserIsDecryptor => {
                witness.decryptors.push(user.address());
                acl_results = Some(vec![not_invalidated]);
            }
            PadScenario::UserIsDelegateOfDecryptor => {
                // Delegation checks: submitter, then the decryptor.
                acl_results = Some(vec![not_invalidated, false.abi_encode(), true.abi_encode()]);
            }
            PadScenario::UserIsNotDelegate => {
                acl_results = Some(vec![
                    not_invalidated,
                    false.abi_encode(),
                    false.abi_encode(),
                ]);
            }
            PadScenario::ConsumerNotInAllowedContracts => {
                witness.submitter = user.address();
                allowed_contracts.push(rand_address());
            }
            PadScenario::WitnessExpired => {
                witness.submitter = user.address();
                witness.expirationTimestamp = Utc::now().timestamp() as u64 - 1;
            }
            PadScenario::LabelNotFromWitness | PadScenario::AclMulticallFails => {}
        }
        match acl_results {
            Some(results) => asserter.push_success(&acl_multicall_response(results)),
            None => asserter.push_failure_msg("transport error"),
        }

        let mut request = pad_request(&user, witness, allowed_contracts, CHAIN_ID);
        if let PadScenario::LabelNotFromWitness = scenario {
            request.payload.pads[0].label = rand_digest();
        }

        let result = processor
            .check_pad_decryption_request(&request)
            .await
            .map_err(RequestCheckError::record);
        match expected {
            None => result.unwrap(),
            Some((kind, code)) => {
                let err = result.unwrap_err();
                assert_eq!((err.kind, err.code), (kind, code));
            }
        }
    }

    #[tokio::test]
    async fn check_pad_decryption_request_rejects_signature_for_another_chain() {
        let asserter = Asserter::new();
        let processor = setup_test_processor(asserter.clone());
        let user = PrivateKeySigner::random();

        // `ecrecover` mismatch, and no contract code at `userAddress` for the ERC-1271 fallback.
        asserter.push_success(&Bytes::default());

        let request = pad_request(&user, witness(user.address(), vec![]), vec![], CHAIN_ID + 1);
        let err = processor
            .check_pad_decryption_request(&request)
            .await
            .map_err(RequestCheckError::record)
            .unwrap_err();
        assert_eq!(
            (err.kind, err.code),
            (
                ProcessingErrorKind::Irrecoverable,
                ErrorCode::UserSignatureRejected
            )
        );
    }
}
