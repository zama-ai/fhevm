use crate::core::event_processor::{
    AclCheck, DecryptionProcessor, ProcessingError, RequestCheckError, RequestCheckKind,
    SIGNING_SCHEMES, ciphertext::VerifiedCiphertexts,
};
use alloy::{
    primitives::{B256, Bytes, U256},
    providers::Provider,
};
use connector_utils::types::{
    KmsGrpcRequest, extra_data::parse_extra_data, handle::extract_chain_id_from_handle,
    u256_to_request_id,
};
use fhevm_host_bindings::acl::ACL::isAllowedForDecryptionCall;
use futures::future::try_join_all;
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::PublicDecryptionRequest;
use std::collections::HashMap;
use tracing::info;

impl<GP, HP> DecryptionProcessor<GP, HP>
where
    GP: Provider + Clone + 'static,
    HP: Provider,
{
    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        handles: &[B256],
    ) -> Result<(), RequestCheckError> {
        info!("Starting ACL check for {} handles...", handles.len());

        // Public decryption handles can span multiple host chains: one multicall per chain
        let mut checks_by_chain: HashMap<u64, Vec<AclCheck>> = HashMap::new();
        for &handle in handles {
            let chain_id = extract_chain_id_from_handle(&handle).map_err(|e| {
                RequestCheckError::irrecoverable(RequestCheckKind::Acl, ErrorCode::Unprocessable, e)
            })?;
            checks_by_chain
                .entry(chain_id)
                .or_default()
                .push(AclCheck::AllowedForDecryption(isAllowedForDecryptionCall {
                    handle,
                }));
        }
        try_join_all(
            checks_by_chain
                .into_iter()
                .map(|(chain_id, checks)| async move {
                    let host_client = self.host_client(chain_id)?;
                    let calls = checks.iter().flat_map(AclCheck::abi_encode).collect();
                    let results = host_client.acl_multicall(calls).await?;
                    AclCheck::verify_all(&checks, &results)
                }),
        )
        .await?;

        info!("ACL check passed for {} handles!", handles.len());
        Ok(())
    }

    pub async fn prepare_public_decryption_request(
        &self,
        decryption_id: U256,
        handles: &[B256],
        extra_data: &Bytes,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        let parsed_extra_data = parse_extra_data(extra_data)
            .map_err(|e| ProcessingError::irrecoverable(ErrorCode::Unprocessable, e))?;
        let VerifiedCiphertexts {
            ciphertexts,
            key_id,
        } = self.ciphertext_manager.verify_and_retrieve(handles).await?;
        let public_decryption_request = PublicDecryptionRequest {
            request_id: Some(u256_to_request_id(decryption_id)),
            ciphertexts,
            key_id: Some(u256_to_request_id(key_id)),
            domain: Some(self.domain.clone()),
            extra_data: extra_data.to_vec(),
            epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
            context_id: Some(u256_to_request_id(parsed_extra_data.context_id)),
            signing_schemes: SIGNING_SCHEMES.to_vec(),
        };
        Ok(public_decryption_request.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::Config,
        event_processor::{
            CiphertextManager, HostRpcClient,
            decryption::tests::{ExpectedOutcome, assert_kind, setup_test_processor},
        },
    };
    use alloy::{
        primitives::Address,
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };
    use connector_utils::tests::{rand::rand_handle, setup::acl_multicall_response};
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;

    enum PubDecryptACLMock {
        Failure(&'static str),
        Success(bool),
    }

    #[rstest]
    #[case::transport_error(
        PubDecryptACLMock::Failure("Transport Error"),
        ExpectedOutcome::Recoverable
    )]
    #[case::allowed(PubDecryptACLMock::Success(true), ExpectedOutcome::Ok)]
    #[case::not_allowed(PubDecryptACLMock::Success(false), ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_public_decryption(
        #[case] mock_response: PubDecryptACLMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let decryption_processor = setup_test_processor(asserter.clone(), handle);
        let handles = vec![handle];

        match mock_response {
            PubDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            PubDecryptACLMock::Success(val) => {
                asserter.push_success(&acl_multicall_response(vec![val.abi_encode()]))
            }
        }

        let result = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&handles)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    #[rstest]
    #[case::all_allowed(true, ExpectedOutcome::Ok)]
    #[case::other_chain_not_allowed(false, ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_public_decryption_multi_chain(
        #[case] other_chain_allowed: bool,
        #[case] expected: ExpectedOutcome,
    ) {
        let handle = rand_handle();
        // Same handle with a different chain id (bytes 22..30).
        let mut other_chain_handle = handle;
        other_chain_handle[29] ^= 0xFF;

        // Each host chain has its own mocked RPC node, and receives its own multicall.
        let asserter = Asserter::new();
        let other_asserter = Asserter::new();
        let host_clients = [(handle, &asserter), (other_chain_handle, &other_asserter)]
            .into_iter()
            .map(|(handle, asserter)| {
                let chain_id = extract_chain_id_from_handle(&handle).unwrap();
                let provider = ProviderBuilder::new()
                    .disable_recommended_fillers()
                    .connect_mocked_client(asserter.clone());
                let acl_contract = ACL::new(Address::default(), provider);
                (chain_id, HostRpcClient::new(chain_id, acl_contract))
            })
            .collect();
        let gateway_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new());
        let decryption_processor = DecryptionProcessor::new(
            &Config::default(),
            gateway_provider.clone(),
            host_clients,
            CiphertextManager::for_test(gateway_provider),
        );
        asserter.push_success(&acl_multicall_response(vec![
            true.abi_encode(),
            true.abi_encode(),
        ]));
        other_asserter.push_success(&acl_multicall_response(vec![
            other_chain_allowed.abi_encode(),
        ]));

        let result = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&[handle, other_chain_handle, handle])
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    #[tokio::test]
    async fn acl_errors_carry_host_chain_context() {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let decryption_processor = setup_test_processor(asserter.clone(), handle);
        asserter.push_failure_msg("connection refused");

        let err = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&[handle])
            .await
            .map_err(RequestCheckError::record)
            .unwrap_err();

        let chain_id = extract_chain_id_from_handle(&handle).unwrap();
        let msg = err.to_string();
        assert!(msg.contains(&chain_id.to_string()), "{msg}");
        assert!(
            msg.contains(&format!("ACL contract {}", Address::ZERO)),
            "{msg}"
        );
        assert!(msg.contains("connection refused"), "{msg}");
    }
}
