//! Whether a decryption request is authorized on its host chain: through the ACL contract on an
//! EVM host, through the host program's accounts and the coprocessors' leaf proofs on Solana.
//! Every check here reads live host state and runs on every worker attempt.

use crate::core::{
    config::Config,
    event_processor::{HostRpcClient, RequestCheckError, RequestCheckKind},
    solana::{
        SolanaHost,
        pipeline::{AuthorizationContext, authorize_request},
        public_decrypt::check_public_decrypt,
    },
};
use alloy::{
    hex,
    primitives::{Address, B256, FixedBytes, U256, map::DefaultHashBuilder},
    providers::Provider,
    sol_types::{Eip712Domain, SolCall},
};
use anyhow::anyhow;
use connector_utils::types::{
    handle::extract_chain_id_from_handle, solana_request::SolanaUserDecryptionRequestV1,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    HandleEntry, UserDecryptionRequest_3 as UserDecryptionRequestV2,
    delegatedUserDecryptionRequestCall, userDecryptionRequest_2Call as userDecryptionRequestCall,
};
use futures::{
    future::try_join_all,
    stream::{FuturesUnordered, StreamExt},
};
use kms_connector_api::ErrorCode;
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use tracing::info;
use user_decryption_signature::compute_user_decrypt_digest;

/// A host chain the connector serves: its ACL contract on EVM, or the host program's accounts and
/// the coprocessors' leaf proofs on Solana.
#[derive(Clone)]
pub enum HostChain<HP: Provider> {
    Evm(HostRpcClient<HP>),
    // Boxed: two readers are far larger than the EVM variant's contract handle.
    Solana(Box<SolanaHost>),
}

/// Decides whether a decryption request is authorized on the host chain its handles name.
#[derive(Clone)]
pub struct HostDecryptionVerifier<HP: Provider> {
    /// The host chains, by chain id.
    hosts: HashMap<u64, HostChain<HP>>,

    /// The Decryption contract's EIP-712 domain without a chain id: an EVM user-decryption
    /// signature is over this domain at the host chain id.
    user_decryption_domain: Eip712Domain,

    /// Gas cap for the `IERC1271.isValidSignature` static call (RFC-012).
    erc1271_gas_limit: u64,
}

impl<HP: Provider> HostDecryptionVerifier<HP> {
    pub fn new(config: &Config, hosts: HashMap<u64, HostChain<HP>>) -> Self {
        let user_decryption_domain = Eip712Domain {
            name: Some(config.decryption_contract.domain_name.clone().into()),
            version: Some(config.decryption_contract.domain_version.clone().into()),
            chain_id: None,
            verifying_contract: Some(config.decryption_contract.address),
            salt: None,
        };
        Self {
            hosts,
            user_decryption_domain,
            erc1271_gas_limit: config.erc1271_gas_limit,
        }
    }

    fn host(&self, chain_id: u64) -> Result<&HostChain<HP>, RequestCheckError> {
        self.hosts.get(&chain_id).ok_or_else(|| {
            RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::UpstreamTransient,
                anyhow!("No host chain configured for chain id {chain_id}"),
            )
        })
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        handles: &[B256],
        extra_data: &[u8],
    ) -> Result<(), RequestCheckError> {
        info!("Starting ACL check for {} handles...", handles.len());

        try_join_all(handles.iter().map(|handle| async move {
            let ct_chain_id = extract_chain_id_from_handle(handle).map_err(|e| {
                RequestCheckError::irrecoverable(RequestCheckKind::Acl, ErrorCode::Unprocessable, e)
            })?;

            match self.host(ct_chain_id)? {
                HostChain::Solana(host) => {
                    // Public access is proven by a PublicDecryptLeaf MMR proof and verified
                    // against the encrypted store observed at confirmed commitment.
                    Ok(check_public_decrypt(host, handle.0, extra_data).await?)
                }
                HostChain::Evm(host_client) => {
                    if !host_client.is_allowed_for_decryption(*handle).await? {
                        return Err(RequestCheckError::recoverable(
                            RequestCheckKind::Acl,
                            ErrorCode::AclDenied,
                            anyhow!("Decryption is not allowed for {handle}"),
                        ));
                    }
                    Ok(())
                }
            }
        }))
        .await?;

        info!("ACL check passed for {} handles!", handles.len());
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_user_decryption(
        &self,
        calldata: Vec<u8>,
        handles: &[B256],
        user_address: Address,
    ) -> Result<(), RequestCheckError> {
        info!("Starting ACL check for {} handles...", handles.len());

        let (ct_handle_contract_pairs, delegator_address) =
            match delegatedUserDecryptionRequestCall::abi_decode(calldata.as_slice()) {
                Ok(parsed_calldata) => (
                    parsed_calldata.ctHandleContractPairs,
                    Some(parsed_calldata.delegationAccounts.delegatorAddress),
                ),
                Err(e) => {
                    let parsed_calldata = userDecryptionRequestCall::abi_decode(
                        calldata.as_slice(),
                    )
                    .map_err(|e2| {
                        RequestCheckError::irrecoverable(
                            RequestCheckKind::Acl,
                            ErrorCode::Unprocessable,
                            anyhow!(
                                "Was not able to parse calldata for both userDecryptionRequestCall \
                                {e2} and delegatedUserDecryptionRequestCall ({e})!"
                            ),
                        )
                    })?;
                    (parsed_calldata.ctHandleContractPairs, None)
                }
            };

        let contracts_map = HashMap::<FixedBytes<32>, Address, DefaultHashBuilder>::from_iter(
            ct_handle_contract_pairs
                .iter()
                .map(|c| (c.ctHandle, c.contractAddress)),
        );
        let contracts_map_ref = &contracts_map;

        try_join_all(handles.iter().map(|handle| async move {
            let ct_chain_id = extract_chain_id_from_handle(handle).map_err(|e| {
                RequestCheckError::irrecoverable(RequestCheckKind::Acl, ErrorCode::Unprocessable, e)
            })?;
            let HostChain::Evm(host_client) = self.host(ct_chain_id)? else {
                return Err(wrong_host_kind(ct_chain_id, "EVM"));
            };
            let contract_address = contracts_map_ref.get(handle.as_slice()).ok_or_else(|| {
                RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::Unprocessable,
                    anyhow!("Could not find contract address for handle {handle}"),
                )
            })?;

            if let Some(delegator_addr) = delegator_address {
                self.inner_acl_check_for_delegated_user_decryption(
                    host_client,
                    *handle,
                    user_address,
                    *contract_address,
                    delegator_addr,
                )
                .await
            } else {
                self.inner_acl_check_for_user_decryption(
                    host_client,
                    *handle,
                    user_address,
                    *contract_address,
                )
                .await
            }
        }))
        .await?;

        info!("ACL check passed for {} handles!", handles.len());
        Ok(())
    }

    async fn inner_acl_check_for_delegated_user_decryption(
        &self,
        host_client: &HostRpcClient<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        contract_address: Address,
        delegator_address: Address,
    ) -> Result<(), RequestCheckError> {
        let is_delegated = host_client
            .is_handle_delegated_for_user_decryption(
                delegator_address,
                user_address,
                contract_address,
                handle,
            )
            .await?;

        if !is_delegated {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                anyhow!(
                    "{user_address} is not a delegate of {delegator_address} for contract \
                    {contract_address} and handle {handle}!",
                ),
            ));
        }

        Ok(())
    }

    /// Verify that a RFC 016 EVM `UserDecryptionRequestV2` is internally consistent before the
    /// ACL phase: every handle resolves to the same host chain id. Returns that shared chain id.
    fn validate_handles_and_extract_chain_id(
        handles: &[HandleEntry],
    ) -> Result<u64, RequestCheckError> {
        let chain_id = handles
            .first()
            .ok_or_else(|| {
                RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::Unprocessable,
                    anyhow!("request contains no handles"),
                )
            })
            .map(|h| extract_chain_id_from_handle(&h.handle))?
            .map_err(|e| {
                RequestCheckError::irrecoverable(RequestCheckKind::Acl, ErrorCode::Unprocessable, e)
            })?;

        for h in handles.iter() {
            match extract_chain_id_from_handle(&h.handle) {
                Ok(id) if id == chain_id => (),
                Ok(other) => {
                    return Err(RequestCheckError::irrecoverable(
                        RequestCheckKind::Acl,
                        ErrorCode::Unprocessable,
                        anyhow!(
                            "user decryption request handles span multiple chains ({chain_id}, {other})",
                        ),
                    ));
                }
                Err(e) => {
                    return Err(RequestCheckError::irrecoverable(
                        RequestCheckKind::Acl,
                        ErrorCode::Unprocessable,
                        anyhow!(
                            "Failed to extract chain_id from handle {}: {e}",
                            hex::encode(h.handle),
                        ),
                    ));
                }
            }
        }

        Ok(chain_id)
    }

    /// RFC016 unified user decryption check — verifies the full ACL authorization for a
    /// `UserDecryptionRequestV2` payload.
    ///
    /// 1. validity window (`startTimestamp <= now <= startTimestamp + durationSeconds`)
    /// 2. `userAddress ∉ allowedContracts` when `allowedContracts` is non-empty
    /// 3. concurrent host-chain checks (one RPC round-trip wave):
    ///    - EIP-712 signature verification with `ecrecover` → ERC-1271 fallback (RFC-012)
    ///    - signature invalidation: `startTimestamp >= ACL.decryptionSignatureInvalidatedBefore(userAddress)`
    ///    - per-handle ownership (direct `isAllowed` if `ownerAddress == userAddress`, else
    ///      `isHandleDelegatedForUserDecryption`)
    ///    - per-handle contract allowance (any `isAllowed(handle, c)` for
    ///      `c ∈ allowedContracts`, no-op in permissive mode)
    #[tracing::instrument(skip_all)]
    pub async fn check_user_decryption_request_v2(
        &self,
        request: &UserDecryptionRequestV2,
    ) -> Result<(), RequestCheckError> {
        info!(
            "Starting RFC016 check for {} handles...",
            request.handles.len()
        );

        let chain_id = Self::validate_handles_and_extract_chain_id(&request.handles)?;

        let payload = &request.payload;

        // Validity window
        let start = payload.requestValidity.startTimestamp;
        let now = U256::from(Utc::now().timestamp() as u64);
        let end = start.saturating_add(payload.requestValidity.durationSeconds);
        if now < start {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Signature,
                ErrorCode::UserSignatureRejected,
                anyhow!(
                    "RFC016 user decryption request not yet valid: now {now} < startTimestamp {start}",
                ),
            ));
        }
        if now > end {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Signature,
                ErrorCode::UserSignatureRejected,
                anyhow!(
                    "RFC016 user decryption request validity window expired: now {now} > end {end}"
                ),
            ));
        }

        // `userAddress` must not appear in a non-empty `allowedContracts` list.
        if payload.allowedContracts.contains(&payload.userAddress) {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Signature,
                ErrorCode::Unprocessable,
                anyhow!(
                    "userAddress {} is listed in allowedContracts — request rejected",
                    payload.userAddress
                ),
            ));
        }

        let HostChain::Evm(host_client) = self.host(chain_id)? else {
            return Err(wrong_host_kind(chain_id, "EVM"));
        };

        // RFC-012: EIP-712 signature verification with ecrecover → ERC-1271 fallback.
        // The domain is the Decryption contract's, with the host `contractsChainId` in place of
        // the Gateway chain id: the KMS gRPC domain targets the Gateway, the user-decryption
        // signature targets the host chain.
        let domain = Eip712Domain {
            chain_id: Some(U256::from(chain_id)),
            ..self.user_decryption_domain.clone()
        };
        let digest = compute_user_decrypt_digest(payload, &domain);

        // Signature verification, invalidation, and per-handle ACL checks are all independent
        // host-chain reads. Fire them concurrently so the smart-account happy path is faster.
        // `biased;` polls branches in order so tests can deterministically craft the mock-queue
        // order.
        tokio::try_join!(
            biased;
            async {
                host_client
                    .verify_signature(
                        payload.userAddress,
                        digest,
                        payload.signature.as_ref(),
                        self.erc1271_gas_limit,
                    )
                    .await
            },
            self.inner_invalidation_check_for_user_decryption_v2(
                host_client,
                payload.userAddress,
                start,
            ),
            try_join_all(request.handles.iter().map(|handle_entry| async move {
                tokio::try_join!(
                    biased;
                    self.inner_ownership_check_for_user_decryption_v2(
                        host_client,
                        handle_entry,
                        payload.userAddress,
                    ),
                    self.inner_allowed_contracts_check_for_user_decryption_v2(
                        host_client,
                        handle_entry.handle,
                        &payload.allowedContracts,
                    ),
                )
            })),
        )?;

        info!(
            "RFC016 ACL check passed for {} handles!",
            request.handles.len()
        );
        Ok(())
    }

    /// Rechecks the Solana permit and host authorization state on every worker attempt.
    pub async fn check_solana_user_decryption_request(
        &self,
        request: &SolanaUserDecryptionRequestV1,
    ) -> Result<(), RequestCheckError> {
        let chain_id = request.permit().chain_id();
        let HostChain::Solana(host) = self.host(chain_id)? else {
            return Err(wrong_host_kind(chain_id, "Solana"));
        };
        let context = AuthorizationContext {
            program_id: host.program_id,
            now_unix_seconds: Utc::now().timestamp() as u64,
        };
        authorize_request(&host.reader, &host.proofs, context, request).await?;
        info!(
            "Solana user decryption check passed for {} handles!",
            request.handles().len()
        );
        Ok(())
    }

    /// RFC016 per-handle ownership check. Direct path (`ownerAddress == userAddress`) calls
    /// `isAllowed(handle, userAddress)`; delegated path calls
    /// `isHandleDelegatedForUserDecryption(ownerAddress, userAddress, contractAddress, handle)`.
    async fn inner_ownership_check_for_user_decryption_v2(
        &self,
        host_client: &HostRpcClient<HP>,
        entry: &HandleEntry,
        user_address: Address,
    ) -> Result<(), RequestCheckError> {
        let handle_hex = hex::encode(entry.handle);
        if entry.ownerAddress == user_address {
            let user_allowed = host_client.is_allowed(entry.handle, user_address).await?;
            if !user_allowed {
                return Err(RequestCheckError::recoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::AclDenied,
                    anyhow!("{user_address} is not allowed to decrypt {handle_hex}"),
                ));
            }
        } else {
            let is_delegated = host_client
                .is_handle_delegated_for_user_decryption(
                    entry.ownerAddress,
                    user_address,
                    entry.contractAddress,
                    entry.handle,
                )
                .await?;
            if !is_delegated {
                return Err(RequestCheckError::recoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::AclDenied,
                    anyhow!(
                        "{user_address} is not a delegate of {} for contract {} and handle {handle_hex}",
                        entry.ownerAddress,
                        entry.contractAddress,
                    ),
                ));
            }
        }
        Ok(())
    }

    /// RFC016 per-handle `allowedContracts` check — succeeds if at least one contract in the list
    /// has `isAllowed(handle, contract)` returning true. Returns `Ok(())` without any RPC call in
    /// permissive mode (empty list) so callers can invoke it unconditionally.
    async fn inner_allowed_contracts_check_for_user_decryption_v2(
        &self,
        host_client: &HostRpcClient<HP>,
        handle: FixedBytes<32>,
        allowed_contracts: &[Address],
    ) -> Result<(), RequestCheckError> {
        if allowed_contracts.is_empty() {
            return Ok(());
        }

        let mut calls: FuturesUnordered<_> = allowed_contracts
            .iter()
            .map(|c| async move { (c, host_client.is_allowed(handle, *c).await) })
            .collect();

        // All calls run concurrently; short-circuit on the first positive, dropping `calls`
        // to cancel the remaining in-flight requests.
        let mut rejections = Vec::with_capacity(allowed_contracts.len());
        while let Some((contract, result)) = calls.next().await {
            match result {
                Ok(true) => return Ok(()),
                Ok(false) => rejections.push(format!("{contract}: not allowed")),
                Err(e) => rejections.push(format!("{contract}: {e}")),
            }
        }

        // No contract returned true (all denied, all RPC calls failed, or a mix of both).
        Err(RequestCheckError::recoverable(
            RequestCheckKind::Acl,
            ErrorCode::AclDenied,
            anyhow!(
                "No contract in allowedContracts is allowed to decrypt handle {handle} ({rejections:?})",
            ),
        ))
    }

    /// RFC016 signature invalidation check. Rejects if `startTimestamp < invalidationTs`, meaning
    /// the user has invalidated all signatures issued before `invalidationTs`.
    async fn inner_invalidation_check_for_user_decryption_v2(
        &self,
        host_client: &HostRpcClient<HP>,
        user_address: Address,
        start_timestamp: U256,
    ) -> Result<(), RequestCheckError> {
        let invalidation_ts = host_client
            .decryption_signature_invalidated_before(user_address)
            .await?;
        if start_timestamp < invalidation_ts {
            return Err(RequestCheckError::irrecoverable(
                // TODO: reconsider Signature naming
                RequestCheckKind::Signature,
                ErrorCode::UserSignatureRejected,
                anyhow!(
                    "RFC016 signature invalidated: startTimestamp {start_timestamp} < \
                     invalidatedBefore {invalidation_ts} for userAddress {user_address}"
                ),
            ));
        }
        Ok(())
    }

    async fn inner_acl_check_for_user_decryption(
        &self,
        host_client: &HostRpcClient<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        contract_address: Address,
    ) -> Result<(), RequestCheckError> {
        let (user_allowed, contract_allowed) = tokio::try_join!(
            biased;
            host_client.is_allowed(handle, user_address),
            host_client.is_allowed(handle, contract_address),
        )?;

        if !user_allowed {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                anyhow!("{user_address} is not allowed to decrypt {handle}!"),
            ));
        }
        if !contract_allowed {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                anyhow!("{contract_address} is not allowed to decrypt {handle}!"),
            ));
        }

        Ok(())
    }
}

/// A request whose handles name a host chain of the other kind.
fn wrong_host_kind(chain_id: u64, required: &str) -> RequestCheckError {
    RequestCheckError::irrecoverable(
        RequestCheckKind::Acl,
        ErrorCode::Unprocessable,
        anyhow!("this request requires {required}, but host chain {chain_id} is of the other kind"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::solana_host_chain_id;
    use crate::core::event_processor::{ProcessingError, ProcessingErrorKind};
    use crate::core::solana::{proof::CoprocessorProofClient, snapshot::SolanaRpcClient};
    use alloy::{
        primitives::Bytes,
        providers::{ProviderBuilder, RootProvider, mock::Asserter},
        signers::{SignerSync, local::PrivateKeySigner},
        sol_types::SolValue,
    };
    use connector_utils::tests::rand::{rand_address, rand_handle, rand_public_key, rand_u256};
    use fhevm_gateway_bindings::decryption::{
        Decryption::CtHandleContractPair,
        IDecryption::{RequestValiditySeconds, UserDecryptionRequestPayload},
    };
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;
    use user_decryption_signature::{ERC1271_MAGIC_VALUE, default_user_decrypt_domain};

    enum ExpectedOutcome {
        Ok,
        Recoverable,
        Irrecoverable,
    }

    fn assert_kind(result: &Result<(), ProcessingError>, expected: &ExpectedOutcome) {
        match expected {
            ExpectedOutcome::Ok => unreachable!(),
            ExpectedOutcome::Recoverable => {
                assert_eq!(
                    result.as_ref().unwrap_err().kind,
                    ProcessingErrorKind::Recoverable
                )
            }
            ExpectedOutcome::Irrecoverable => {
                assert_eq!(
                    result.as_ref().unwrap_err().kind,
                    ProcessingErrorKind::Irrecoverable
                )
            }
        }
    }

    fn setup_test_verifier(
        asserter: Asserter,
        handle: B256,
    ) -> HostDecryptionVerifier<RootProvider> {
        setup_test_verifier_with_host(asserter, handle, TestHost::Evm)
    }

    /// Which host chain the verifier under test serves for the handle's chain id. `Missing`
    /// leaves the map empty, to exercise the unconfigured-chain path.
    enum TestHost {
        Evm,
        Solana,
        Missing,
    }

    fn setup_test_verifier_with_host(
        asserter: Asserter,
        handle: B256,
        host_kind: TestHost,
    ) -> HostDecryptionVerifier<RootProvider> {
        let config = Config::default();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let chain_id = extract_chain_id_from_handle(&handle).unwrap();
        let hosts = match host_kind {
            TestHost::Evm => HashMap::from([(
                chain_id,
                HostChain::Evm(HostRpcClient::new(
                    chain_id,
                    ACL::new(Address::default(), mock_provider),
                )),
            )]),
            TestHost::Solana => HashMap::from([(
                chain_id,
                HostChain::Solana(Box::new(SolanaHost {
                    program_id: [7; 32],
                    reader: SolanaRpcClient::new(
                        config.host_chains[0].url.clone(),
                        config.host_rpc_call_timeout,
                        std::num::NonZeroUsize::new(1).unwrap(),
                    ),
                    proofs: CoprocessorProofClient::new(
                        &config.host_chains[0].solana_proof_endpoints,
                        config.host_chains[0]
                            .solana_proof_api_key
                            .clone()
                            .unwrap_or_default(),
                        ::reqwest::Client::new(),
                    ),
                })),
            )]),
            TestHost::Missing => HashMap::new(),
        };
        HostDecryptionVerifier::new(&config, hosts)
    }

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
        let verifier = setup_test_verifier(asserter.clone(), handle);
        let handles = vec![handle];

        match mock_response {
            PubDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            PubDecryptACLMock::Success(val) => asserter.push_success(&val.abi_encode()),
        }

        let result = verifier
            .check_ciphertexts_allowed_for_public_decryption(&handles, &[0])
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
        let verifier = setup_test_verifier(asserter.clone(), handle);
        asserter.push_failure_msg("connection refused");

        let err = verifier
            .check_ciphertexts_allowed_for_public_decryption(&[handle], &[0])
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

    enum UserDecryptACLMock {
        Failure(&'static str),
        Success {
            user_allowed: bool,
            contract_allowed: bool,
        },
    }

    #[rstest]
    #[case::transport_error(
        UserDecryptACLMock::Failure("Transport Error"),
        ExpectedOutcome::Recoverable
    )]
    #[case::allowed(
        UserDecryptACLMock::Success { user_allowed: true, contract_allowed: true },
        ExpectedOutcome::Ok
    )]
    #[case::not_allowed(
        UserDecryptACLMock::Success { user_allowed: false, contract_allowed: false },
        ExpectedOutcome::Recoverable
    )]
    #[case::user_allowed_contract_not_allowed(
        UserDecryptACLMock::Success { user_allowed: true, contract_allowed: false },
        ExpectedOutcome::Recoverable
    )]
    #[case::user_not_allowed_contract_allowed(
        UserDecryptACLMock::Success { user_allowed: false, contract_allowed: true },
        ExpectedOutcome::Recoverable
    )]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_user_decryption(
        #[case] mock_response: UserDecryptACLMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        // Use non-delegated userDecryptionRequestCall (requires only 2 ACL checks)
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: handle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();
        let handles = vec![handle];
        let user_address = Address::default();

        match mock_response {
            UserDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            UserDecryptACLMock::Success {
                user_allowed,
                contract_allowed,
            } => {
                asserter.push_success(&user_allowed.abi_encode());
                asserter.push_success(&contract_allowed.abi_encode());
            }
        }

        let result = verifier
            .check_ciphertexts_allowed_for_user_decryption(calldata, &handles, user_address)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    enum DelegatedUserDecryptACLMock {
        Failure(&'static str),
        Success { is_delegated: bool },
    }

    #[rstest]
    #[case::transport_error(
        DelegatedUserDecryptACLMock::Failure("Transport Error"),
        ExpectedOutcome::Recoverable,
        None
    )]
    #[case::allowed(
        DelegatedUserDecryptACLMock::Success { is_delegated: true },
        ExpectedOutcome::Ok,
        None
    )]
    #[case::not_delegated(
        DelegatedUserDecryptACLMock::Success { is_delegated: false },
        ExpectedOutcome::Recoverable,
        Some("is not a delegate of")
    )]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_delegated_user_decryption(
        #[case] mock_response: DelegatedUserDecryptACLMock,
        #[case] expected: ExpectedOutcome,
        #[case] expected_error_msg: Option<&str>,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: handle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();
        let handles = vec![handle];
        let user_address = Address::default();

        match mock_response {
            DelegatedUserDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            DelegatedUserDecryptACLMock::Success { is_delegated } => {
                asserter.push_success(&is_delegated.abi_encode());
            }
        }

        let result = verifier
            .check_ciphertexts_allowed_for_user_decryption(calldata, &handles, user_address)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            ExpectedOutcome::Recoverable => assert_kind(&result, &expected),
            ExpectedOutcome::Irrecoverable => {
                assert_kind(&result, &expected);
                let expected_msg = expected_error_msg.unwrap();
                let msg = format!("{:#}", result.unwrap_err().source);
                assert!(
                    msg.contains(expected_msg),
                    "Expected error message to contain '{expected_msg}', got: {msg}",
                );
            }
        }
    }

    /// Builds a `UserDecryptionRequestV2` whose payload carries a valid 65-byte ECDSA signature
    /// over the EIP-712 digest.
    ///
    /// `user_address` and `signing_key` are intentionally decoupled: the EOA-direct case
    /// passes `signing_key.address()` for both, and the smart-account case passes the
    /// contract address as `user_address` while `signing_key` plays the role of the wallet's
    /// owner EOA — its signature recovers to a different address, forcing the ERC-1271
    /// fallback in `verify_signature`.
    ///
    /// The digest is computed against `Config::default().decryption_contract.address` — the
    /// same gateway address `setup_test_verifier` configures the verifier with.
    fn make_v2_request(
        handle: B256,
        owner_address: Address,
        user_address: Address,
        signing_key: &PrivateKeySigner,
        allowed_contracts: Vec<Address>,
        start_offset_secs: i64,
        duration_secs: u64,
    ) -> UserDecryptionRequestV2 {
        let start = (Utc::now().timestamp() + start_offset_secs) as u64;
        let mut payload = UserDecryptionRequestPayload {
            userAddress: user_address,
            publicKey: Bytes::from(rand_public_key()),
            allowedContracts: allowed_contracts,
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from(start),
                durationSeconds: U256::from(duration_secs),
            },
            extraData: Bytes::default(),
            signature: Bytes::default(),
        };

        let chain_id = extract_chain_id_from_handle(&handle).unwrap();
        let gateway_addr = Config::default().decryption_contract.address;
        let domain = default_user_decrypt_domain(chain_id, gateway_addr);
        let digest = compute_user_decrypt_digest(&payload, &domain);
        let sig = signing_key.sign_hash_sync(&digest).unwrap();
        payload.signature = Bytes::from(sig.as_bytes().to_vec());

        UserDecryptionRequestV2 {
            decryptionId: rand_u256(),
            handles: vec![HandleEntry {
                handle,
                contractAddress: rand_address(),
                ownerAddress: owner_address,
            }],
            payload,
        }
    }

    #[rstest]
    #[case::not_yet_valid(3600_i64, 86400_u64, ExpectedOutcome::Recoverable)]
    #[case::expired(-(2 * 3600_i64), 3600_u64, ExpectedOutcome::Irrecoverable)]
    #[tokio::test]
    async fn check_user_decryption_request_v2_validity_window(
        #[case] start_offset_secs: i64,
        #[case] duration_secs: u64,
        #[case] expected: ExpectedOutcome,
    ) {
        let handle = rand_handle();
        let user_signer = PrivateKeySigner::random();
        let user_address = user_signer.address();
        let verifier = setup_test_verifier(Asserter::new(), handle);
        let request = make_v2_request(
            handle,
            user_address,
            user_address,
            &user_signer,
            vec![],
            start_offset_secs,
            duration_secs,
        );

        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    // Test userAddress ∈ allowedContracts
    #[tokio::test]
    async fn check_user_decryption_request_v2_user_in_allowed_contracts() {
        let handle = rand_handle();
        let user_signer = PrivateKeySigner::random();
        let user_address = user_signer.address();
        let verifier = setup_test_verifier(Asserter::new(), handle);
        let request = make_v2_request(
            handle,
            user_address,
            user_address,
            &user_signer,
            vec![user_address],
            -3600,
            86400,
        );

        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);
        assert_kind(&result, &ExpectedOutcome::Irrecoverable);
    }

    // -------------------------------------------------------------------------
    // Invalidation check (validity window passes, empty allowedContracts, direct ownership)
    // -------------------------------------------------------------------------
    enum InvalidationMock {
        Zero,         // invalidation_ts = 0 → start (≈ now-3600) >= 0 → passes
        AboveStart,   // invalidation_ts = u64::MAX → start < u64::MAX → fails
        EqualToStart, // invalidation_ts = start → start < start is false → passes
        TransportError,
    }

    #[rstest]
    #[case::not_invalidated(InvalidationMock::Zero, ExpectedOutcome::Ok)]
    #[case::invalidated(InvalidationMock::AboveStart, ExpectedOutcome::Irrecoverable)]
    #[case::boundary_passes(InvalidationMock::EqualToStart, ExpectedOutcome::Ok)]
    #[case::transport_error(InvalidationMock::TransportError, ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_user_decryption_request_v2_invalidation(
        #[case] mock: InvalidationMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let user_signer = PrivateKeySigner::random();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        const START_OFFSET_SECS: i64 = -3600;
        let start = U256::from((Utc::now().timestamp() + START_OFFSET_SECS) as u64);

        let passes = match mock {
            InvalidationMock::Zero => {
                asserter.push_success(&U256::ZERO.abi_encode());
                true
            }
            InvalidationMock::AboveStart => {
                asserter.push_success(&U256::from(u64::MAX).abi_encode());
                false
            }
            InvalidationMock::EqualToStart => {
                asserter.push_success(&start.abi_encode());
                true
            }
            InvalidationMock::TransportError => {
                asserter.push_failure_msg("transport error");
                false
            }
        };

        if passes {
            asserter.push_success(&true.abi_encode()); // ownership: direct path passes
        }

        let request = make_v2_request(
            handle,
            user_signer.address(),
            user_signer.address(),
            &user_signer,
            vec![],
            START_OFFSET_SECS,
            86400,
        );
        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    // -------------------------------------------------------------------------
    // Ownership check (empty allowedContracts → 1 RPC per test)
    // -------------------------------------------------------------------------
    enum OwnershipMock {
        DirectPath(Option<bool>),
        DelegatedPath(Option<bool>),
    }

    #[rstest]
    #[case::direct_transport_error(OwnershipMock::DirectPath(None), ExpectedOutcome::Recoverable)]
    #[case::direct_allowed(OwnershipMock::DirectPath(Some(true)), ExpectedOutcome::Ok)]
    #[case::direct_not_allowed(
        OwnershipMock::DirectPath(Some(false)),
        ExpectedOutcome::Recoverable
    )]
    #[case::delegated_transport_error(
        OwnershipMock::DelegatedPath(None),
        ExpectedOutcome::Recoverable
    )]
    #[case::delegated_yes(OwnershipMock::DelegatedPath(Some(true)), ExpectedOutcome::Ok)]
    #[case::delegated_no(
        OwnershipMock::DelegatedPath(Some(false)),
        ExpectedOutcome::Recoverable
    )]
    #[tokio::test]
    async fn check_user_decryption_request_v2_ownership(
        #[case] mock: OwnershipMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let user_signer = PrivateKeySigner::random();
        let user_address = user_signer.address();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        let (owner_address, acl_response) = match mock {
            OwnershipMock::DirectPath(r) => (user_address, r),
            OwnershipMock::DelegatedPath(r) => (rand_address(), r),
        };
        asserter.push_success(&U256::ZERO.abi_encode()); // invalidation check: not invalidated
        match acl_response {
            Some(v) => asserter.push_success(&v.abi_encode()),
            None => asserter.push_failure_msg("transport error"),
        }

        let request = make_v2_request(
            handle,
            owner_address,
            user_address,
            &user_signer,
            vec![],
            -3600,
            86400,
        );
        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    // -------------------------------------------------------------------------
    // Allowed contracts check (direct ownership always passes → 2 RPCs)
    //
    // Two `isAllowed` calls are made concurrently via `tokio::try_join!`. The Asserter
    // serves responses in FIFO order, and poll ordering between the two futures is
    // guaranteed by the `biased` annotation.
    // -------------------------------------------------------------------------
    #[rstest]
    #[case::transport_error(None, ExpectedOutcome::Recoverable)]
    #[case::at_least_one_allowed(Some(true), ExpectedOutcome::Ok)]
    #[case::none_allowed(Some(false), ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_user_decryption_request_v2_allowed_contracts(
        #[case] contract_response: Option<bool>,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let user_signer = PrivateKeySigner::random();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        asserter.push_success(&U256::ZERO.abi_encode()); // invalidation check: not invalidated
        asserter.push_success(&true.abi_encode()); // ownership always passes
        match contract_response {
            Some(v) => asserter.push_success(&v.abi_encode()),
            None => asserter.push_failure_msg("transport error"),
        }

        let request = make_v2_request(
            handle,
            user_signer.address(),
            user_signer.address(),
            &user_signer,
            vec![rand_address()],
            -3600,
            86400,
        );
        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
        }
    }

    // -------------------------------------------------------------------------
    // RFC-012: signature verification wired into check_user_decryption_request_v2
    // -------------------------------------------------------------------------

    /// A flipped byte in `payload.signature` makes ecrecover return some other address; with
    /// no contract code at `userAddress`, the ERC-1271 fallback rejects with Irrecoverable.
    /// No invalidation/ACL RPC is reached.
    #[tokio::test]
    async fn check_user_decryption_request_v2_signature_mismatch() {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let user_signer = PrivateKeySigner::random();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        // STATICCALL to a no-code address returns empty returndata at the EVM level →
        // `EoaMismatchNoCode` rejection.
        asserter.push_success(&Bytes::default());

        let mut request = make_v2_request(
            handle,
            user_signer.address(),
            user_signer.address(),
            &user_signer,
            vec![],
            -3600,
            86400,
        );
        // Flip a byte in the signature
        let mut sig = request.payload.signature.to_vec();
        sig[0] ^= 0xFF;
        request.payload.signature = Bytes::from(sig);

        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);
        assert_kind(&result, &ExpectedOutcome::Irrecoverable);
    }

    /// A smart-account user (Safe-style) whose contract returns the ERC-1271 magic value
    /// passes the signature check, then the rest of the pipeline (invalidation + ownership)
    /// proceeds normally.
    #[tokio::test]
    async fn check_user_decryption_request_v2_smart_account_accepts() {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let verifier = setup_test_verifier(asserter.clone(), handle);

        // Random "smart account" address; no off-chain key controls it, so ecrecover will
        // never match — verification only succeeds via the ERC-1271 fallback.
        let smart_account = rand_address();
        // The wallet's owner EOA: produces real 65-byte signature bytes whose recovered
        // address is *not* `smart_account`, forcing the ERC-1271 path.
        let owner = PrivateKeySigner::random();
        let request = make_v2_request(
            handle,
            smart_account, // owner == userAddress: direct path
            smart_account,
            &owner,
            vec![],
            -3600,
            86400,
        );

        // Mock the host RPC sequence:
        //   1. isValidSignature → magic value (left-aligned in a 32-byte word)
        //   2. invalidation → 0
        //   3. ownership: isAllowed → true
        let mut magic_word = [0u8; 32];
        magic_word[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        asserter.push_success(&magic_word); // isValidSignature
        asserter.push_success(&U256::ZERO.abi_encode()); // invalidation
        asserter.push_success(&true.abi_encode()); // ownership

        verifier
            .check_user_decryption_request_v2(&request)
            .await
            .unwrap();
    }

    fn legacy_request_calldata(handle: FixedBytes<32>) -> Vec<u8> {
        userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: handle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode()
    }

    fn assert_irrecoverable_contains(result: Result<(), ProcessingError>, expected: &str) {
        match result {
            Err(error) if error.kind == ProcessingErrorKind::Irrecoverable => {
                assert!(
                    error.source.to_string().contains(expected),
                    "unexpected error: {}",
                    error.source
                );
            }
            other => panic!("expected irrecoverable error containing '{expected}', got {other:?}"),
        }
    }

    fn make_solana_request(handle: B256) -> SolanaUserDecryptionRequestV1 {
        connector_utils::tests::rand::solana_user_decryption_request(U256::from(1), handle)
    }

    fn rand_solana_handle() -> B256 {
        let mut bytes = *rand_handle();
        bytes[22..30].copy_from_slice(&solana_host_chain_id(12345).to_be_bytes());
        bytes.into()
    }

    #[tokio::test]
    async fn public_decryption_dispatches_to_solana_host() {
        let handle = rand_solana_handle();
        let verifier = setup_test_verifier_with_host(Asserter::new(), handle, TestHost::Solana);

        let result = verifier
            .check_ciphertexts_allowed_for_public_decryption(&[handle], &[0])
            .await
            .map_err(RequestCheckError::record);

        match result {
            Err(error) if error.kind == ProcessingErrorKind::Irrecoverable => {
                assert!(
                    error
                        .source
                        .to_string()
                        .contains("requires the version-4 extraData")
                );
            }
            other => panic!("expected Solana public-decrypt rejection, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn legacy_user_decryption_rejects_solana_host() {
        let handle = rand_solana_handle();
        let verifier = setup_test_verifier_with_host(Asserter::new(), handle, TestHost::Solana);

        let result = verifier
            .check_ciphertexts_allowed_for_user_decryption(
                legacy_request_calldata(handle),
                &[handle],
                Address::ZERO,
            )
            .await
            .map_err(RequestCheckError::record);

        assert_irrecoverable_contains(result, "request requires EVM");
    }

    #[tokio::test]
    async fn rfc016_user_decryption_rejects_solana_host() {
        let handle = rand_solana_handle();
        let verifier = setup_test_verifier_with_host(Asserter::new(), handle, TestHost::Solana);
        let signer = PrivateKeySigner::random();
        let request = make_v2_request(
            handle,
            signer.address(),
            signer.address(),
            &signer,
            vec![],
            -60,
            3_600,
        );

        let result = verifier
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

        assert_irrecoverable_contains(result, "request requires EVM");
    }

    #[tokio::test]
    async fn a_solana_request_rejects_the_evm_host() {
        let handle = rand_handle();
        let verifier = setup_test_verifier(Asserter::new(), handle);
        let request = make_solana_request(handle);

        let result = verifier
            .check_solana_user_decryption_request(&request)
            .await
            .map_err(RequestCheckError::record);

        assert_irrecoverable_contains(result, "request requires Solana");
    }

    #[tokio::test]
    async fn unknown_host_is_recoverable_for_all_decryption_families() {
        let handle = rand_handle();
        let verifier = setup_test_verifier_with_host(Asserter::new(), handle, TestHost::Missing);
        let signer = PrivateKeySigner::random();
        let evm_unified_request = make_v2_request(
            handle,
            signer.address(),
            signer.address(),
            &signer,
            vec![],
            -60,
            3_600,
        );
        let solana_request = make_solana_request(handle);

        let public = verifier
            .check_ciphertexts_allowed_for_public_decryption(&[handle], &[0])
            .await
            .map_err(RequestCheckError::record);
        let legacy = verifier
            .check_ciphertexts_allowed_for_user_decryption(
                legacy_request_calldata(handle),
                &[handle],
                Address::ZERO,
            )
            .await
            .map_err(RequestCheckError::record);
        let evm_unified = verifier
            .check_user_decryption_request_v2(&evm_unified_request)
            .await
            .map_err(RequestCheckError::record);
        let solana = verifier
            .check_solana_user_decryption_request(&solana_request)
            .await
            .map_err(RequestCheckError::record);

        for result in [public, legacy, evm_unified, solana] {
            match result {
                Err(error) if error.kind == ProcessingErrorKind::Recoverable => assert!(
                    error
                        .source
                        .to_string()
                        .contains("No host chain configured")
                ),
                other => panic!("expected recoverable unknown-host error, got {other:?}"),
            }
        }
    }
}
