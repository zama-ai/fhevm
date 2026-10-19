use crate::core::{
    config::Config,
    event_processor::{
        AclMulticallResults, CiphertextManager, HostRpcClient, ProcessingError, RequestCheckError,
        RequestCheckKind, ciphertext::VerifiedCiphertexts,
    },
};
use alloy::{
    consensus::Transaction,
    hex,
    primitives::{Address, B256, Bytes, FixedBytes, U256, map::DefaultHashBuilder},
    providers::Provider,
    sol_types::{Eip712Domain, SolCall},
};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest, extra_data::parse_extra_data, handle::extract_chain_id_from_handle,
    u256_to_request_id,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, UserDecryptionRequest_1 as UserDecryptionRequestV2,
    delegatedUserDecryptionRequestCall, userDecryptionRequest_1Call as userDecryptionRequestCall,
};
use fhevm_host_bindings::acl::ACL::{
    decryptionSignatureInvalidatedBeforeCall, isAllowedCall, isAllowedForDecryptionCall,
    isHandleDelegatedForUserDecryptionCall,
};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, SigningSchemeType, UserDecryptionRequest,
};
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use tracing::info;
use user_decryption_signature::compute_user_decrypt_digest;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<GP: Provider, HP: Provider> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<GP>,

    /// The instances of the host chains `ACL` contracts used to check the decryption ACL.
    host_clients: HashMap<u64, HostRpcClient<HP>>,

    /// The entity used to verify and collect the ciphertexts of decryption requests.
    ciphertext_manager: CiphertextManager<GP>,

    /// Gas cap for the `IERC1271.isValidSignature` static call (RFC-012).
    erc1271_gas_limit: u64,
}

impl<GP, HP> DecryptionProcessor<GP, HP>
where
    GP: Provider + Clone + 'static,
    HP: Provider,
{
    pub fn new(
        config: &Config,
        gateway_provider: GP,
        host_clients: HashMap<u64, HostRpcClient<HP>>,
        ciphertext_manager: CiphertextManager<GP>,
    ) -> Self {
        let domain = Eip712DomainMsg {
            name: config.decryption_contract.domain_name.clone(),
            version: config.decryption_contract.domain_version.clone(),
            chain_id: U256::from(config.gateway_chain_id).to_be_bytes_vec(),
            verifying_contract: config.decryption_contract.address.to_string(),
            salt: None,
        };
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, gateway_provider);

        Self {
            domain,
            decryption_contract,
            host_clients,
            ciphertext_manager,
            erc1271_gas_limit: config.erc1271_gas_limit,
        }
    }

    /// Returns the client of the host chain `chain_id`.
    fn host_client(&self, chain_id: u64) -> Result<&HostRpcClient<HP>, RequestCheckError> {
        self.host_clients.get(&chain_id).ok_or_else(|| {
            RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::UpstreamTransient,
                anyhow!("No ACL contract config found for chain id {chain_id}"),
            )
        })
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        handles: &[B256],
    ) -> Result<(), RequestCheckError> {
        info!("Starting ACL check for {} handles...", handles.len());

        let host_client = self.host_client(extract_single_chain_id(handles)?)?;
        let calls = handles
            .iter()
            .copied()
            .map(|handle| isAllowedForDecryptionCall { handle }.abi_encode().into())
            .collect();
        let mut results = host_client.acl_multicall(calls).await?;

        for handle in handles {
            if !results.decode_next::<isAllowedForDecryptionCall>()? {
                return Err(RequestCheckError::recoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::AclDenied,
                    anyhow!("Decryption is not allowed for {handle}"),
                ));
            }
        }

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

        let (ct_handle_contract_pairs, delegator_address, start_timestamp, contracts_chain_id) =
            match delegatedUserDecryptionRequestCall::abi_decode(calldata.as_slice()) {
                Ok(parsed_calldata) => (
                    parsed_calldata.ctHandleContractPairs,
                    Some(parsed_calldata.delegationAccounts.delegatorAddress),
                    parsed_calldata.requestValidity.startTimestamp,
                    parsed_calldata.contractsInfo.chainId,
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
                    (
                        parsed_calldata.ctHandleContractPairs,
                        None,
                        parsed_calldata.requestValidity.startTimestamp,
                        parsed_calldata.contractsInfo.chainId,
                    )
                }
            };

        let chain_id = extract_single_chain_id(handles)?;
        if contracts_chain_id != U256::from(chain_id) {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Acl,
                ErrorCode::Unprocessable,
                anyhow!(
                    "Contracts chain id {contracts_chain_id} differs from handles chain id {chain_id}"
                ),
            ));
        }
        let host_client = self.host_client(chain_id)?;
        let contracts_map = HashMap::<FixedBytes<32>, Address, DefaultHashBuilder>::from_iter(
            ct_handle_contract_pairs
                .iter()
                .map(|c| (c.ctHandle, c.contractAddress)),
        );
        let handle_contract_pairs = handles
            .iter()
            .map(|handle| {
                let contract_address = contracts_map.get(handle.as_slice()).ok_or_else(|| {
                    RequestCheckError::irrecoverable(
                        RequestCheckKind::Acl,
                        ErrorCode::Unprocessable,
                        anyhow!("Could not find contract address for handle {handle}"),
                    )
                })?;
                Ok((*handle, *contract_address))
            })
            .collect::<Result<Vec<_>, RequestCheckError>>()?;

        // Layout: the invalidation call, then the per-handle ACL calls.
        let mut calls = vec![signature_invalidation_call(user_address)];
        if let Some(delegator_addr) = delegator_address {
            calls.extend(delegated_user_decryption_acl_calls(
                &handle_contract_pairs,
                user_address,
                delegator_addr,
            ));
        } else {
            calls.extend(user_decryption_acl_calls(
                &handle_contract_pairs,
                user_address,
            ));
        }
        let mut multicall_res = host_client.acl_multicall(calls).await?;

        check_signature_invalidation(&mut multicall_res, user_address, start_timestamp)?;
        if let Some(delegator_addr) = delegator_address {
            check_delegated_user_decryption_acl(
                &mut multicall_res,
                &handle_contract_pairs,
                user_address,
                delegator_addr,
            )?;
        } else {
            check_user_decryption_acl(&mut multicall_res, &handle_contract_pairs, user_address)?;
        }

        info!("ACL check passed for {} handles!", handles.len());
        Ok(())
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

        let chain_id = extract_single_chain_id(request.handles.iter().map(|h| &h.handle))?;

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

        let host_client = self.host_client(chain_id)?;

        // RFC-012: EIP-712 signature verification with ecrecover → ERC-1271 fallback.
        // The domain takes name/version/verifyingContract from `self.domain` (already validated
        // at startup) but substitutes the host `contractsChainId` for the Gateway chain id —
        // `self.domain` targets KMS gRPC requests, the user-decryption signature targets the
        // host chain.
        let domain = Eip712Domain {
            name: Some(self.domain.name.clone().into()),
            version: Some(self.domain.version.clone().into()),
            chain_id: Some(U256::from(chain_id)),
            verifying_contract: Some(*self.decryption_contract.address()),
            salt: None,
        };
        let digest = compute_user_decrypt_digest(payload, &domain);

        // The signature verification and the ACL reads are independent host-chain reads. Fire
        // them concurrently so the smart-account happy path is faster. `biased;` polls branches
        // in order so tests can deterministically craft the mock-queue order.
        tokio::try_join!(
            biased;
            host_client.verify_signature(
                payload.userAddress,
                digest,
                payload.signature.as_ref(),
                self.erc1271_gas_limit,
            ),
            self.inner_acl_check_for_user_decryption_v2(host_client, request, start),
        )?;

        info!(
            "RFC016 ACL check passed for {} handles!",
            request.handles.len()
        );
        Ok(())
    }

    /// RFC016 ACL checks, batched in a single `ACL.multicall` so they are served from the same
    /// block:
    /// - signature invalidation: `startTimestamp >= ACL.decryptionSignatureInvalidatedBefore(userAddress)`
    /// - per-handle ownership: direct `isAllowed(handle, userAddress)` if
    ///   `ownerAddress == userAddress`, else
    ///   `isHandleDelegatedForUserDecryption(ownerAddress, userAddress, contractAddress, handle)`
    /// - per-handle contract allowance: any `isAllowed(handle, c)` for `c ∈ allowedContracts`,
    ///   no-op in permissive mode (empty list)
    async fn inner_acl_check_for_user_decryption_v2(
        &self,
        host_client: &HostRpcClient<HP>,
        request: &UserDecryptionRequestV2,
        start_timestamp: U256,
    ) -> Result<(), RequestCheckError> {
        let user_address = request.payload.userAddress;
        let allowed_contracts = &request.payload.allowedContracts;

        // Layout: the invalidation call, then for each handle its ownership call followed by one
        // `isAllowed` call per allowed contract.
        let mut calls: Vec<Bytes> = vec![signature_invalidation_call(user_address)];
        for entry in request.handles.iter() {
            let ownership_call = if entry.ownerAddress == user_address {
                isAllowedCall {
                    handle: entry.handle,
                    account: user_address,
                }
                .abi_encode()
            } else {
                isHandleDelegatedForUserDecryptionCall {
                    delegator: entry.ownerAddress,
                    delegate: user_address,
                    contractAddress: entry.contractAddress,
                    handle: entry.handle,
                }
                .abi_encode()
            };
            calls.push(ownership_call.into());
            calls.extend(allowed_contracts.iter().map(|contract| {
                isAllowedCall {
                    handle: entry.handle,
                    account: *contract,
                }
                .abi_encode()
                .into()
            }));
        }
        let mut multicall_res = host_client.acl_multicall(calls).await?;

        check_signature_invalidation(&mut multicall_res, user_address, start_timestamp)?;

        for entry in request.handles.iter() {
            let handle_hex = hex::encode(entry.handle);

            if entry.ownerAddress == user_address {
                if !multicall_res.decode_next::<isAllowedCall>()? {
                    return Err(RequestCheckError::recoverable(
                        RequestCheckKind::Acl,
                        ErrorCode::AclDenied,
                        anyhow!("{user_address} is not allowed to decrypt {handle_hex}"),
                    ));
                }
            } else if !multicall_res.decode_next::<isHandleDelegatedForUserDecryptionCall>()? {
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

            if allowed_contracts.is_empty() {
                continue;
            }

            // Every result must be consumed (no short-circuit) to keep the next handle's
            // results aligned.
            let mut any_contract_allowed = false;
            for _ in allowed_contracts {
                any_contract_allowed |= multicall_res.decode_next::<isAllowedCall>()?;
            }
            if !any_contract_allowed {
                return Err(RequestCheckError::recoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::AclDenied,
                    anyhow!(
                        "No contract in allowedContracts ({allowed_contracts:?}) is allowed to \
                        decrypt handle {handle_hex}",
                    ),
                ));
            }
        }

        Ok(())
    }
    pub async fn prepare_decryption_request(
        &self,
        decryption_id: U256,
        handles: &[B256],
        extra_data: &Bytes,
        user_decrypt_data: Option<UserDecryptionExtraData>,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        if handles.is_empty() {
            return Err(ProcessingError::irrecoverable(
                ErrorCode::Unprocessable,
                anyhow!("No handles found in the request, cannot proceed"),
            ));
        }

        let parsed_extra_data = parse_extra_data(extra_data)
            .map_err(|e| ProcessingError::irrecoverable(ErrorCode::Unprocessable, e))?;

        let VerifiedCiphertexts {
            ciphertexts,
            key_id,
        } = self.ciphertext_manager.verify_and_retrieve(handles).await?;

        let request_id = Some(u256_to_request_id(decryption_id));
        let kms_extra_data = kms_decryption_extra_data(extra_data);

        if let Some(user_decrypt_data) = user_decrypt_data {
            let client_address = user_decrypt_data.user_address.to_checksum(None);
            let enc_key = user_decrypt_data.public_key.to_vec();
            let user_decryption_request = UserDecryptionRequest {
                request_id,
                client_address,
                key_id: Some(u256_to_request_id(key_id)),
                domain: Some(self.domain.clone()),
                enc_key,
                typed_ciphertexts: ciphertexts,
                extra_data: kms_extra_data,
                epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
                context_id: parsed_extra_data.context_id.map(u256_to_request_id),
                // Currently hardcoded to Ecdsa256k1 as it is the only scheme used for Ethereum.
                signing_schemes: vec![SigningSchemeType::Ecdsa256k1 as i32],
            };

            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(u256_to_request_id(key_id)),
                domain: Some(self.domain.clone()),
                extra_data: kms_extra_data,
                epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
                context_id: parsed_extra_data.context_id.map(u256_to_request_id),
                // Currently hardcoded to Ecdsa256k1 as it is the only scheme used for Ethereum.
                signing_schemes: vec![SigningSchemeType::Ecdsa256k1 as i32],
            };
            Ok(public_decryption_request.into())
        }
    }

    /// Fetches the calldata of a given transaction.
    ///
    /// Only allows transactions sent directly to the `Decryption` contract.
    pub async fn fetch_calldata(
        &self,
        tx_hash: FixedBytes<32>,
    ) -> Result<Vec<u8>, ProcessingError> {
        let decryption_address = *self.decryption_contract.address();

        let tx = self
            .decryption_contract
            .provider()
            .get_transaction_by_hash(tx_hash)
            .await
            .map_err(ProcessingError::transient)?
            .ok_or_else(|| {
                ProcessingError::transient(anyhow!("No transaction found with hash {tx_hash}!"))
            })?;

        if tx.to() != Some(decryption_address) {
            return Err(ProcessingError::irrecoverable(
                ErrorCode::Unprocessable,
                anyhow!(
                    "Transaction {tx_hash} was sent to {:?} rather than directly to the Decryption \
                    contract {decryption_address}: its calldata cannot be associated with the user \
                    decryption event.",
                    tx.to(),
                ),
            ));
        }

        Ok(tx.input().to_vec())
    }
}

fn signature_invalidation_call(user_address: Address) -> Bytes {
    decryptionSignatureInvalidatedBeforeCall {
        account: user_address,
    }
    .abi_encode()
    .into()
}

fn delegated_user_decryption_acl_calls(
    handle_contract_pairs: &[(B256, Address)],
    user_address: Address,
    delegator_address: Address,
) -> impl Iterator<Item = Bytes> + '_ {
    handle_contract_pairs
        .iter()
        .map(move |(handle, contract_address)| {
            isHandleDelegatedForUserDecryptionCall {
                delegator: delegator_address,
                delegate: user_address,
                contractAddress: *contract_address,
                handle: *handle,
            }
            .abi_encode()
            .into()
        })
}

// Two calls per handle: `isAllowed(handle, user)` then `isAllowed(handle, contract)`.
fn user_decryption_acl_calls(
    handle_contract_pairs: &[(B256, Address)],
    user_address: Address,
) -> impl Iterator<Item = Bytes> + '_ {
    handle_contract_pairs
        .iter()
        .flat_map(move |(handle, contract_address)| {
            [user_address, *contract_address].map(|account| {
                isAllowedCall {
                    handle: *handle,
                    account,
                }
                .abi_encode()
                .into()
            })
        })
}

/// Signature invalidation check. Rejects if `startTimestamp < invalidationTs`, meaning the user
/// has invalidated all signatures issued before `invalidationTs`.
fn check_signature_invalidation<P: Provider>(
    results: &mut AclMulticallResults<'_, P>,
    user_address: Address,
    start_timestamp: U256,
) -> Result<(), RequestCheckError> {
    let invalidation_ts = results.decode_next::<decryptionSignatureInvalidatedBeforeCall>()?;
    if start_timestamp < invalidation_ts {
        return Err(RequestCheckError::irrecoverable(
            RequestCheckKind::Signature,
            ErrorCode::UserSignatureRejected,
            anyhow!(
                "Signature invalidated: startTimestamp {start_timestamp} < \
                 invalidatedBefore {invalidation_ts} for userAddress {user_address}"
            ),
        ));
    }
    Ok(())
}

fn check_delegated_user_decryption_acl<P: Provider>(
    results: &mut AclMulticallResults<'_, P>,
    handle_contract_pairs: &[(B256, Address)],
    user_address: Address,
    delegator_address: Address,
) -> Result<(), RequestCheckError> {
    for (handle, contract_address) in handle_contract_pairs {
        if !results.decode_next::<isHandleDelegatedForUserDecryptionCall>()? {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                anyhow!(
                    "{user_address} is not a delegate of {delegator_address} for contract \
                    {contract_address} and handle {handle}!",
                ),
            ));
        }
    }
    Ok(())
}

fn check_user_decryption_acl<P: Provider>(
    results: &mut AclMulticallResults<'_, P>,
    handle_contract_pairs: &[(B256, Address)],
    user_address: Address,
) -> Result<(), RequestCheckError> {
    for (handle, contract_address) in handle_contract_pairs {
        if !results.decode_next::<isAllowedCall>()? {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                anyhow!("{user_address} is not allowed to decrypt {handle}!"),
            ));
        }
        if !results.decode_next::<isAllowedCall>()? {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::AclDenied,
                anyhow!("{contract_address} is not allowed to decrypt {handle}!"),
            ));
        }
    }
    Ok(())
}

/// Verifies that every handle resolves to the same host chain id, and returns that chain id.
///
/// Mixed-chain requests are rejected.
fn extract_single_chain_id<'a>(
    handles: impl IntoIterator<Item = &'a B256>,
) -> Result<u64, RequestCheckError> {
    let mut chain_id = None;
    for handle in handles {
        let handle_chain_id = extract_chain_id_from_handle(handle).map_err(|e| {
            RequestCheckError::irrecoverable(
                RequestCheckKind::Acl,
                ErrorCode::Unprocessable,
                anyhow!("Failed to extract chain_id from handle {handle}: {e}"),
            )
        })?;
        match chain_id {
            None => chain_id = Some(handle_chain_id),
            Some(expected) if expected != handle_chain_id => {
                return Err(RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    ErrorCode::Unprocessable,
                    anyhow!(
                        "decryption request handles span multiple chains ({expected}, {handle_chain_id})",
                    ),
                ));
            }
            Some(_) => (),
        }
    }
    chain_id.ok_or_else(|| {
        RequestCheckError::irrecoverable(
            RequestCheckKind::Acl,
            ErrorCode::Unprocessable,
            anyhow!("request contains no handles"),
        )
    })
}

fn kms_decryption_extra_data(extra_data: &Bytes) -> Vec<u8> {
    // relayer-sdk <=0.4.2 sends 0x00 but verifies the KMS signature against empty extraData.
    if extra_data.as_ref() == [0x00] {
        Vec::new()
    } else {
        extra_data.to_vec()
    }
}

pub struct UserDecryptionExtraData {
    pub user_address: Address,
    pub public_key: Bytes,
}

impl UserDecryptionExtraData {
    pub fn new(user_address: Address, public_key: Bytes) -> Self {
        Self {
            user_address,
            public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;
    use alloy::{
        providers::{ProviderBuilder, mock::Asserter},
        rpc::types::Transaction as RpcTransaction,
        signers::{SignerSync, local::PrivateKeySigner},
        sol_types::SolValue,
    };
    use connector_utils::tests::rand::{
        rand_address, rand_digest, rand_handle, rand_public_key, rand_u256,
    };
    use connector_utils::tests::setup::acl_multicall_response;
    use fhevm_gateway_bindings::decryption::{
        Decryption::{CtHandleContractPair, HandleEntry},
        IDecryption::{
            ContractsInfo, DelegationAccounts, RequestValidity, RequestValiditySeconds,
            UserDecryptionRequestPayload,
        },
    };
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;
    use user_decryption_signature::{
        ERC1271_MAGIC_VALUE, compute_user_decrypt_digest, default_user_decrypt_domain,
    };

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

    fn setup_test_processor(
        asserter: Asserter,
        handle: B256,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>> {
        setup_test_processor_with_config(asserter, handle, Config::default())
    }

    fn setup_test_processor_with_config(
        asserter: Asserter,
        handle: B256,
        config: Config,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>> {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let chain_id = extract_chain_id_from_handle(&handle).unwrap();
        let host_clients = HashMap::from([(
            chain_id,
            HostRpcClient::new(
                chain_id,
                ACL::new(Address::default(), mock_provider.clone()),
            ),
        )]);
        let ciphertext_manager = CiphertextManager::for_test(mock_provider.clone());
        DecryptionProcessor::new(&config, mock_provider, host_clients, ciphertext_manager)
    }

    #[test]
    fn kms_decryption_extra_data_normalizes_legacy_zero_marker() {
        assert_eq!(
            kms_decryption_extra_data(&Bytes::from_static(&[0x00])),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn kms_decryption_extra_data_keeps_empty_and_versioned_values() {
        assert_eq!(kms_decryption_extra_data(&Bytes::new()), Vec::<u8>::new());
        assert_eq!(
            kms_decryption_extra_data(&Bytes::from_static(&[0x01, 0x02])),
            vec![0x01, 0x02]
        );
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

    #[tokio::test]
    async fn public_decryption_rejects_mixed_chain_handles() {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let decryption_processor = setup_test_processor(asserter.clone(), handle);
        // Same handle with a different chain id (bytes 22..30).
        let mut other_chain_handle = handle;
        other_chain_handle[29] ^= 0xFF;

        let result = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&[handle, other_chain_handle])
            .await
            .map_err(RequestCheckError::record);
        assert_kind(&result, &ExpectedOutcome::Irrecoverable);
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
        let decryption_processor = setup_test_processor(asserter.clone(), handle);

        // Use non-delegated userDecryptionRequestCall (requires only 2 ACL checks)
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: handle,
                contractAddress: rand_address(),
            }],
            contractsInfo: contracts_info_for(handle),
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
                asserter.push_success(&acl_multicall_response(vec![
                    U256::ZERO.abi_encode(), // invalidation check: not invalidated
                    user_allowed.abi_encode(),
                    contract_allowed.abi_encode(),
                ]));
            }
        }

        let result = decryption_processor
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
        let decryption_processor = setup_test_processor(asserter.clone(), handle);

        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: handle,
                contractAddress: rand_address(),
            }],
            contractsInfo: contracts_info_for(handle),
            ..Default::default()
        }
        .abi_encode();
        let handles = vec![handle];
        let user_address = Address::default();

        match mock_response {
            DelegatedUserDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            DelegatedUserDecryptACLMock::Success { is_delegated } => {
                asserter.push_success(&acl_multicall_response(vec![
                    U256::ZERO.abi_encode(), // invalidation check: not invalidated
                    is_delegated.abi_encode(),
                ]));
            }
        }

        let result = decryption_processor
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

    fn contracts_info_for(handle: B256) -> ContractsInfo {
        ContractsInfo {
            chainId: U256::from(extract_chain_id_from_handle(&handle).unwrap()),
            addresses: vec![rand_address()],
        }
    }

    // -------------------------------------------------------------------------
    // Legacy signature invalidation check. Both paths share the same check: the delegated
    // case only covers its distinct calldata decoding (`startTimestamp`, `contractsInfo`).
    // -------------------------------------------------------------------------
    #[rstest]
    #[case::direct_not_invalidated(false, InvalidationMock::Zero, ExpectedOutcome::Ok)]
    #[case::direct_invalidated(false, InvalidationMock::AboveStart, ExpectedOutcome::Irrecoverable)]
    #[case::direct_boundary_passes(false, InvalidationMock::EqualToStart, ExpectedOutcome::Ok)]
    #[case::direct_transport_error(
        false,
        InvalidationMock::TransportError,
        ExpectedOutcome::Recoverable
    )]
    #[case::delegated_invalidated(
        true,
        InvalidationMock::AboveStart,
        ExpectedOutcome::Irrecoverable
    )]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_user_decryption_invalidation(
        #[case] delegated: bool,
        #[case] mock: InvalidationMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let handle = rand_handle();
        let processor = setup_test_processor(asserter.clone(), handle);
        let user_address = rand_address();

        let start = U256::from((Utc::now().timestamp() - 3600) as u64);
        let ct_handle_contract_pairs = vec![CtHandleContractPair {
            ctHandle: handle,
            contractAddress: rand_address(),
        }];
        let request_validity = RequestValidity {
            startTimestamp: start,
            durationDays: U256::from(1),
        };
        let calldata = if delegated {
            delegatedUserDecryptionRequestCall {
                ctHandleContractPairs: ct_handle_contract_pairs,
                requestValidity: request_validity,
                delegationAccounts: DelegationAccounts {
                    delegatorAddress: rand_address(),
                    delegateAddress: user_address,
                },
                contractsInfo: contracts_info_for(handle),
                ..Default::default()
            }
            .abi_encode()
        } else {
            userDecryptionRequestCall {
                ctHandleContractPairs: ct_handle_contract_pairs,
                requestValidity: request_validity,
                contractsInfo: contracts_info_for(handle),
                userAddress: user_address,
                ..Default::default()
            }
            .abi_encode()
        };

        let invalidation_ts = match mock {
            InvalidationMock::Zero => Some(U256::ZERO),
            InvalidationMock::AboveStart => Some(U256::from(u64::MAX)),
            InvalidationMock::EqualToStart => Some(start),
            InvalidationMock::TransportError => None,
        };
        match invalidation_ts {
            // The ACL checks always pass.
            Some(ts) if delegated => asserter.push_success(&acl_multicall_response(vec![
                ts.abi_encode(),
                true.abi_encode(), // isHandleDelegatedForUserDecryption
            ])),
            Some(ts) => asserter.push_success(&acl_multicall_response(vec![
                ts.abi_encode(),
                true.abi_encode(), // isAllowed(handle, user)
                true.abi_encode(), // isAllowed(handle, contract)
            ])),
            None => asserter.push_failure_msg("transport error"),
        }

        let result = processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[handle], user_address)
            .await
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => {
                result.unwrap();
            }
            _ => assert_kind(&result, &expected),
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
    /// same gateway address `setup_test_processor` configures the processor with.
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
        let processor = setup_test_processor(Asserter::new(), handle);
        let request = make_v2_request(
            handle,
            user_address,
            user_address,
            &user_signer,
            vec![],
            start_offset_secs,
            duration_secs,
        );

        let result = processor
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
        let processor = setup_test_processor(Asserter::new(), handle);
        let request = make_v2_request(
            handle,
            user_address,
            user_address,
            &user_signer,
            vec![user_address],
            -3600,
            86400,
        );

        let result = processor
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
        let processor = setup_test_processor(asserter.clone(), handle);

        const START_OFFSET_SECS: i64 = -3600;
        let start = U256::from((Utc::now().timestamp() + START_OFFSET_SECS) as u64);

        let invalidation_ts = match mock {
            InvalidationMock::Zero => Some(U256::ZERO),
            InvalidationMock::AboveStart => Some(U256::from(u64::MAX)),
            InvalidationMock::EqualToStart => Some(start),
            InvalidationMock::TransportError => None,
        };
        match invalidation_ts {
            // Ownership: direct path passes.
            Some(ts) => asserter.push_success(&acl_multicall_response(vec![
                ts.abi_encode(),
                true.abi_encode(),
            ])),
            None => asserter.push_failure_msg("transport error"),
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
        let result = processor
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
    // Ownership check (empty allowedContracts → 1 ACL multicall per test)
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
        let processor = setup_test_processor(asserter.clone(), handle);

        let (owner_address, acl_response) = match mock {
            OwnershipMock::DirectPath(r) => (user_address, r),
            OwnershipMock::DelegatedPath(r) => (rand_address(), r),
        };
        match acl_response {
            // Invalidation check: not invalidated.
            Some(v) => asserter.push_success(&acl_multicall_response(vec![
                U256::ZERO.abi_encode(),
                v.abi_encode(),
            ])),
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
        let result = processor
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
    // Allowed contracts check (direct ownership always passes → 2 `isAllowed` calls, batched
    // with the invalidation check in a single ACL multicall)
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
        let processor = setup_test_processor(asserter.clone(), handle);

        match contract_response {
            // Invalidation check: not invalidated, and ownership always passes.
            Some(v) => asserter.push_success(&acl_multicall_response(vec![
                U256::ZERO.abi_encode(),
                true.abi_encode(),
                v.abi_encode(),
            ])),
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
        let result = processor
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
        let processor = setup_test_processor(asserter.clone(), handle);

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

        let result = processor
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
        let processor = setup_test_processor(asserter.clone(), handle);

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
        //   2. ACL multicall: invalidation → 0, ownership: isAllowed → true
        let mut magic_word = [0u8; 32];
        magic_word[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        asserter.push_success(&magic_word); // isValidSignature
        asserter.push_success(&acl_multicall_response(vec![
            U256::ZERO.abi_encode(),
            true.abi_encode(),
        ]));

        processor
            .check_user_decryption_request_v2(&request)
            .await
            .unwrap();
    }

    /// Where the legacy request transaction was sent, relative to the Decryption contract.
    enum TxTarget {
        /// Sent directly to the Decryption contract (the only accepted case).
        Decryption,
        /// Sent to some intermediary contract.
        Intermediary,
        /// Contract-creation transaction (`to` is `None`). Should be unreachable in theory.
        Creation,
    }

    /// Only calldata coming from a transaction sent directly to the Decryption contract can be
    /// associated with the user decryption event, so any other target is rejected.
    #[rstest]
    #[case::accepts_direct_decryption_tx(TxTarget::Decryption, true)]
    #[case::rejects_tx_not_sent_to_decryption_contract(TxTarget::Intermediary, false)]
    #[case::rejects_contract_creation_tx(TxTarget::Creation, false)]
    #[tokio::test]
    async fn fetch_calldata_only_accepts_direct_decryption_tx(
        #[case] target: TxTarget,
        #[case] should_succeed: bool,
    ) {
        let asserter = Asserter::new();
        let decryption_address = rand_address();
        let mut config = Config::default();
        config.decryption_contract.address = decryption_address;
        let processor = setup_test_processor_with_config(asserter.clone(), rand_handle(), config);

        let tx_hash = rand_digest();
        let calldata = legacy_request_calldata(rand_handle());

        let to = match target {
            TxTarget::Decryption => Some(decryption_address),
            TxTarget::Intermediary => Some(rand_address()),
            TxTarget::Creation => None,
        };
        asserter.push_success(&mock_legacy_request_tx(tx_hash, to, &calldata));

        let result = processor.fetch_calldata(tx_hash).await;
        if should_succeed {
            assert_eq!(result.unwrap(), calldata);
        } else {
            let err = result.unwrap_err();
            assert_eq!(err.kind, ProcessingErrorKind::Irrecoverable);
        }
    }

    /// Builds the mocked `eth_getTransactionByHash` response for a legacy user decryption
    /// request carrying `calldata`, sent to `to` (`None` models a contract-creation tx).
    fn mock_legacy_request_tx(
        tx_hash: FixedBytes<32>,
        to: Option<Address>,
        calldata: &[u8],
    ) -> RpcTransaction {
        serde_json::from_value(serde_json::json!({
            "hash": tx_hash,
            "nonce": "0x0",
            "blockHash": null,
            "blockNumber": null,
            "transactionIndex": null,
            "from": Address::ZERO,
            "to": to,
            "value": "0x0",
            "gasPrice": "0x0",
            "gas": "0x0",
            "input": format!("0x{}", hex::encode(calldata)),
            "v": "0x1b",
            "r": "0x0000000000000000000000000000000000000000000000000000000000000001",
            "s": "0x0000000000000000000000000000000000000000000000000000000000000001",
            "type": "0x0"
        }))
        .unwrap()
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
}
