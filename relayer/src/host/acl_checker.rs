use crate::{
    config::settings::{HostChainConfig, HostChainKind, RetrySettings},
    core::{
        event::{DelegatedUserDecryptRequest, HandleContractPair, UserDecryptRequest},
        job_id::JobId,
    },
    host::{
        error_redact::redact_alloy_error,
        handle_chain_id::{extract_chain_id_from_handle, extract_chain_id_from_u256},
        solana_state::SolanaStateClient,
    },
};
use alloy::{
    primitives::{Address, Bytes, FixedBytes},
    providers::{fillers::FillProvider, ProviderBuilder, RootProvider},
    sol_types::SolCall,
};
use fhevm_host_bindings::acl::ACL;
use fhevm_host_bindings::acl::ACL::ACLInstance;
use reqwest::Url;
use solana_host_contracts_core::{
    host_identity_from_evm_address, EvmAddress as SolanaEvmAddress, Handle as SolanaHandle,
    HostProgramState, Pubkey as SolanaHostPubkey,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{error, warn};

type Provider = FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::Identity,
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::GasFiller,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::BlobGasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::NonceFiller,
                    alloy::providers::fillers::ChainIdFiller,
                >,
            >,
        >,
    >,
    RootProvider<alloy::network::AnyNetwork>,
    alloy::network::AnyNetwork,
>;

type HostAcl = ACLInstance<Arc<Provider>, alloy::network::AnyNetwork>;

/// A single failing ACL check.
#[derive(Debug, Clone)]
pub struct AclFailure {
    pub handle: String,
    pub check: String,
}

impl std::fmt::Display for AclFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handle={} check={}", self.handle, self.check)
    }
}

/// Errors from host chain ACL permission checks (analogous to `alloy::contract::Error` for gateway).
#[derive(Debug, thiserror::Error)]
pub enum HostAclError {
    #[error("ACL check failed for {count} handle(s): {}", failures.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))]
    NotAllowed {
        count: usize,
        failures: Vec<AclFailure>,
    },

    /// Transport failure, unexpected multicall result count, or ABI decode error.
    #[error("Host chain {chain_id} call failed: {message}")]
    CallFailed { chain_id: u64, message: String },

    #[error("No ACL binding configured for host chain {chain_id}")]
    UnsupportedChain { chain_id: u64 },
}

enum HostChainAcl {
    Evm { acl: HostAcl },
    Solana { client: SolanaStateClient },
}

/// Checks handle permissions against host chain ACL contracts via multicall.
pub struct HostAclChecker {
    chains: HashMap<u64, HostChainAcl>,
    retry_config: RetrySettings,
}

impl HostAclChecker {
    pub fn new(host_chains: &[HostChainConfig], retry: RetrySettings) -> anyhow::Result<Self> {
        let mut chains = HashMap::new();

        for hc in host_chains {
            let backend = match hc.chain_kind {
                HostChainKind::Evm => {
                    let url = Url::parse(&hc.url).map_err(|e| {
                        anyhow::anyhow!("Invalid host chain URL for chain {}: {}", hc.chain_id, e)
                    })?;

                    let acl_address = Address::from_str(&hc.acl_address).map_err(|e| {
                        anyhow::anyhow!("Invalid ACL address for chain {}: {}", hc.chain_id, e)
                    })?;

                    let provider = Arc::new(
                        ProviderBuilder::new()
                            .network::<alloy::network::AnyNetwork>()
                            .connect_http(url),
                    );

                    HostChainAcl::Evm {
                        acl: ACL::new(acl_address, provider),
                    }
                }
                HostChainKind::Solana => HostChainAcl::Solana {
                    client: SolanaStateClient::new(
                        hc.url.clone(),
                        hc.state_pda.clone().ok_or_else(|| {
                            anyhow::anyhow!(
                                "Missing state_pda for Solana host chain {}",
                                hc.chain_id
                            )
                        })?,
                    ),
                },
            };

            chains.insert(hc.chain_id, backend);
        }

        Ok(Self {
            chains,
            retry_config: retry,
        })
    }

    /// Check ACL for public decryption: `isAllowedForDecryption` per handle.
    pub async fn check_public_decrypt(
        &self,
        job_id: &JobId,
        handles: &[[u8; 32]],
    ) -> Result<(), HostAclError> {
        if handles.is_empty() {
            return Ok(());
        }

        // Group handles by chain_id
        let grouped = group_handles_by_chain(handles);

        let mut all_failures = Vec::new();

        for (chain_id, chain_handles) in &grouped {
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;
            let allowed = self
                .check_public_decrypt_on_chain(job_id, chain_acl, *chain_id, chain_handles)
                .await?;
            all_failures.extend(allowed);
        }

        if all_failures.is_empty() {
            Ok(())
        } else {
            Err(HostAclError::NotAllowed {
                count: all_failures.len(),
                failures: all_failures,
            })
        }
    }

    /// Check ACL for user decryption: `isAllowed(handle, user)` + `isAllowed(handle, contract)` per pair.
    pub async fn check_user_decrypt(
        &self,
        job_id: &JobId,
        request: &UserDecryptRequest,
    ) -> Result<(), HostAclError> {
        let pairs = &request.ct_handle_contract_pairs;
        if pairs.is_empty() {
            return Ok(());
        }

        // Group pairs by chain_id (extracted from handle)
        let grouped = group_pairs_by_chain(pairs);
        let identity_overrides = if let Some(contract_ids) = request.contract_ids.as_ref() {
            SolanaIdentityOverrides {
                contract_ids: Some(zip_solana_contract_identities_from_fixed_bytes(
                    &request.contract_addresses,
                    contract_ids,
                )?),
            }
        } else {
            parse_solana_user_identity_overrides(
                &request.contract_addresses,
                request.extra_data.as_ref(),
            )?
        };

        let mut all_failures = Vec::new();

        for (chain_id, chain_pairs) in &grouped {
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;
            let failures = self
                .check_user_decrypt_on_chain(
                    job_id,
                    chain_acl,
                    *chain_id,
                    chain_pairs,
                    request.user_address,
                    request.user_id.map(solana_host_identity_from_fixed_bytes),
                    identity_overrides.contract_ids.as_ref(),
                )
                .await?;
            all_failures.extend(failures);
        }

        if all_failures.is_empty() {
            Ok(())
        } else {
            Err(HostAclError::NotAllowed {
                count: all_failures.len(),
                failures: all_failures,
            })
        }
    }

    /// Check ACL for delegated user decryption: `isHandleDelegatedForUserDecryption` per pair.
    pub async fn check_delegated_user_decrypt(
        &self,
        job_id: &JobId,
        request: &DelegatedUserDecryptRequest,
    ) -> Result<(), HostAclError> {
        let pairs = &request.ct_handle_contract_pairs;
        if pairs.is_empty() {
            return Ok(());
        }

        let grouped = group_pairs_by_chain(pairs);
        let identity_overrides = if let Some(contract_ids) = request.contract_ids.as_ref() {
            SolanaIdentityOverrides {
                contract_ids: Some(zip_solana_contract_identities_from_fixed_bytes(
                    &request.contract_addresses,
                    contract_ids,
                )?),
            }
        } else {
            parse_solana_delegated_identity_overrides(
                &request.contract_addresses,
                request.extra_data.as_ref(),
            )?
        };

        let mut all_failures = Vec::new();

        for (chain_id, chain_pairs) in &grouped {
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;
            let failures = self
                .check_delegated_user_decrypt_on_chain(
                    job_id,
                    chain_acl,
                    *chain_id,
                    chain_pairs,
                    request.delegator_address,
                    request
                        .delegator_id
                        .map(solana_host_identity_from_fixed_bytes),
                    request.delegate_address,
                    request
                        .delegate_id
                        .map(solana_host_identity_from_fixed_bytes),
                    identity_overrides.contract_ids.as_ref(),
                )
                .await?;
            all_failures.extend(failures);
        }

        if all_failures.is_empty() {
            Ok(())
        } else {
            Err(HostAclError::NotAllowed {
                count: all_failures.len(),
                failures: all_failures,
            })
        }
    }

    async fn check_public_decrypt_on_chain(
        &self,
        job_id: &JobId,
        chain_acl: &HostChainAcl,
        chain_id: u64,
        chain_handles: &[[u8; 32]],
    ) -> Result<Vec<AclFailure>, HostAclError> {
        match chain_acl {
            HostChainAcl::Evm { acl } => {
                let calls: Vec<Bytes> = chain_handles
                    .iter()
                    .map(|h| {
                        let call = ACL::isAllowedForDecryptionCall {
                            handle: FixedBytes::from(*h),
                        };
                        Bytes::from(call.abi_encode())
                    })
                    .collect();

                let results = self
                    .multicall_with_retry(job_id, acl, &calls, chain_id)
                    .await?;
                if results.len() != chain_handles.len() {
                    return Err(HostAclError::CallFailed {
                        chain_id,
                        message: format!(
                            "expected {} multicall results, got {}",
                            chain_handles.len(),
                            results.len()
                        ),
                    });
                }

                let mut failures = Vec::new();
                for (i, result_bytes) in results.iter().enumerate() {
                    let allowed =
                        decode_bool(result_bytes).map_err(|msg| HostAclError::CallFailed {
                            chain_id,
                            message: msg.to_string(),
                        })?;
                    if !allowed {
                        failures.push(AclFailure {
                            handle: format!("0x{}", hex::encode(chain_handles[i])),
                            check: "isAllowedForDecryption".to_string(),
                        });
                    }
                }

                Ok(failures)
            }
            HostChainAcl::Solana { client } => {
                let state = fetch_solana_state(client, chain_id).await?;
                let mut failures = Vec::new();
                for handle in chain_handles {
                    if !state
                        .acl()
                        .is_allowed_for_decryption(SolanaHandle::from(*handle))
                    {
                        failures.push(AclFailure {
                            handle: format!("0x{}", hex::encode(handle)),
                            check: "isAllowedForDecryption".to_string(),
                        });
                    }
                }
                Ok(failures)
            }
        }
    }

    async fn check_user_decrypt_on_chain(
        &self,
        job_id: &JobId,
        chain_acl: &HostChainAcl,
        chain_id: u64,
        chain_pairs: &[HandleContractPair],
        user: Address,
        native_user_id: Option<SolanaHostPubkey>,
        native_contract_id_overrides: Option<&HashMap<Address, SolanaHostPubkey>>,
    ) -> Result<Vec<AclFailure>, HostAclError> {
        match chain_acl {
            HostChainAcl::Evm { acl } => {
                let mut calls: Vec<Bytes> = Vec::with_capacity(chain_pairs.len() * 2);
                for pair in chain_pairs {
                    let handle_bytes: [u8; 32] = pair.ct_handle.to_be_bytes();
                    let handle = FixedBytes::from(handle_bytes);

                    calls.push(Bytes::from(
                        ACL::isAllowedCall {
                            handle,
                            account: user,
                        }
                        .abi_encode(),
                    ));
                    calls.push(Bytes::from(
                        ACL::isAllowedCall {
                            handle,
                            account: pair.contract_address,
                        }
                        .abi_encode(),
                    ));
                }

                let results = self
                    .multicall_with_retry(job_id, acl, &calls, chain_id)
                    .await?;
                let expected = chain_pairs.len() * 2;
                if results.len() != expected {
                    return Err(HostAclError::CallFailed {
                        chain_id,
                        message: format!(
                            "expected {} multicall results, got {}",
                            expected,
                            results.len()
                        ),
                    });
                }

                let mut failures = Vec::new();
                for (i, pair) in chain_pairs.iter().enumerate() {
                    let handle_hex = format!("0x{:064x}", pair.ct_handle);
                    let user_allowed =
                        decode_bool(&results[i * 2]).map_err(|msg| HostAclError::CallFailed {
                            chain_id,
                            message: msg.to_string(),
                        })?;
                    let contract_allowed = decode_bool(&results[i * 2 + 1]).map_err(|msg| {
                        HostAclError::CallFailed {
                            chain_id,
                            message: msg.to_string(),
                        }
                    })?;

                    if !user_allowed {
                        failures.push(AclFailure {
                            handle: handle_hex.clone(),
                            check: format!("isAllowed(user {})", user),
                        });
                    }
                    if !contract_allowed {
                        failures.push(AclFailure {
                            handle: handle_hex,
                            check: format!("isAllowed(contract {})", pair.contract_address),
                        });
                    }
                }

                Ok(failures)
            }
            HostChainAcl::Solana { client } => {
                let state = fetch_solana_state(client, chain_id).await?;
                let user_identity =
                    native_user_id.unwrap_or_else(|| solana_host_identity_from_evm_address(user));
                let mut failures = Vec::new();
                for pair in chain_pairs {
                    let handle = SolanaHandle::from(pair.ct_handle.to_be_bytes());
                    let contract_identity = native_contract_id_overrides
                        .and_then(|overrides| overrides.get(&pair.contract_address).copied())
                        .unwrap_or_else(|| {
                            solana_host_identity_from_evm_address(pair.contract_address)
                        });
                    let handle_hex = format!("0x{:064x}", pair.ct_handle);

                    if !state.acl().persist_allowed(handle, user_identity) {
                        failures.push(AclFailure {
                            handle: handle_hex.clone(),
                            check: format!("isAllowed(user {})", user),
                        });
                    }
                    if !state.acl().persist_allowed(handle, contract_identity) {
                        failures.push(AclFailure {
                            handle: handle_hex,
                            check: format!("isAllowed(contract {})", pair.contract_address),
                        });
                    }
                }
                Ok(failures)
            }
        }
    }

    async fn check_delegated_user_decrypt_on_chain(
        &self,
        job_id: &JobId,
        chain_acl: &HostChainAcl,
        chain_id: u64,
        chain_pairs: &[HandleContractPair],
        delegator: Address,
        native_delegator_id: Option<SolanaHostPubkey>,
        delegate: Address,
        native_delegate_id: Option<SolanaHostPubkey>,
        native_contract_id_overrides: Option<&HashMap<Address, SolanaHostPubkey>>,
    ) -> Result<Vec<AclFailure>, HostAclError> {
        match chain_acl {
            HostChainAcl::Evm { acl } => {
                let calls: Vec<Bytes> = chain_pairs
                    .iter()
                    .map(|pair| {
                        let handle_bytes: [u8; 32] = pair.ct_handle.to_be_bytes();
                        let call = ACL::isHandleDelegatedForUserDecryptionCall {
                            delegator,
                            delegate,
                            contractAddress: pair.contract_address,
                            handle: FixedBytes::from(handle_bytes),
                        };
                        Bytes::from(call.abi_encode())
                    })
                    .collect();

                let results = self
                    .multicall_with_retry(job_id, acl, &calls, chain_id)
                    .await?;
                if results.len() != chain_pairs.len() {
                    return Err(HostAclError::CallFailed {
                        chain_id,
                        message: format!(
                            "expected {} multicall results, got {}",
                            chain_pairs.len(),
                            results.len()
                        ),
                    });
                }

                let mut failures = Vec::new();
                for (i, pair) in chain_pairs.iter().enumerate() {
                    let allowed =
                        decode_bool(&results[i]).map_err(|msg| HostAclError::CallFailed {
                            chain_id,
                            message: msg.to_string(),
                        })?;
                    if !allowed {
                        failures.push(AclFailure {
                            handle: format!("0x{:064x}", pair.ct_handle),
                            check: "isHandleDelegatedForUserDecryption".to_string(),
                        });
                    }
                }

                Ok(failures)
            }
            HostChainAcl::Solana { client } => {
                let state = fetch_solana_state(client, chain_id).await?;
                let delegator_identity = native_delegator_id
                    .unwrap_or_else(|| solana_host_identity_from_evm_address(delegator));
                let delegate_identity = native_delegate_id
                    .unwrap_or_else(|| solana_host_identity_from_evm_address(delegate));
                let now = current_unix_timestamp();
                let mut failures = Vec::new();

                for pair in chain_pairs {
                    let contract_identity = native_contract_id_overrides
                        .and_then(|overrides| overrides.get(&pair.contract_address).copied())
                        .unwrap_or_else(|| {
                            solana_host_identity_from_evm_address(pair.contract_address)
                        });
                    let handle = SolanaHandle::from(pair.ct_handle.to_be_bytes());
                    if !state.acl().is_handle_delegated_for_user_decryption(
                        delegator_identity,
                        delegate_identity,
                        contract_identity,
                        handle,
                        now,
                    ) {
                        failures.push(AclFailure {
                            handle: format!("0x{:064x}", pair.ct_handle),
                            check: "isHandleDelegatedForUserDecryption".to_string(),
                        });
                    }
                }

                Ok(failures)
            }
        }
    }

    /// Execute a multicall against a host chain ACL contract with retry on RPC errors.
    async fn multicall_with_retry(
        &self,
        job_id: &JobId,
        acl: &HostAcl,
        calls: &[Bytes],
        chain_id: u64,
    ) -> Result<Vec<Bytes>, HostAclError> {
        let max_attempts = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);
        let mut last_error = String::new();
        let calls_vec = calls.to_vec();

        for attempt in 0..max_attempts {
            match acl.multicall(calls_vec.clone()).call().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    last_error = redact_alloy_error(&err);
                    if attempt + 1 < max_attempts {
                        warn!(
                            int_job_id = %job_id,
                            chain_id = chain_id,
                            attempt = attempt + 1,
                            max_attempts = max_attempts,
                            error = %last_error,
                            "Host ACL multicall failed, retrying"
                        );
                        tokio::time::sleep(retry_interval).await;
                    }
                }
            }
        }

        error!(
            int_job_id = %job_id,
            chain_id = chain_id,
            error = %last_error,
            "Host ACL multicall failed after all retries"
        );

        Err(HostAclError::CallFailed {
            chain_id,
            message: last_error,
        })
    }
}

/// Group `[u8; 32]` handles by chain_id extracted from bytes 22..30.
fn group_handles_by_chain(handles: &[[u8; 32]]) -> HashMap<u64, Vec<[u8; 32]>> {
    let mut grouped: HashMap<u64, Vec<[u8; 32]>> = HashMap::new();
    for handle in handles {
        let chain_id = extract_chain_id_from_handle(handle);
        grouped.entry(chain_id).or_default().push(*handle);
    }
    grouped
}

/// Group HandleContractPairs by chain_id extracted from the U256 handle.
fn group_pairs_by_chain(pairs: &[HandleContractPair]) -> HashMap<u64, Vec<HandleContractPair>> {
    let mut grouped: HashMap<u64, Vec<HandleContractPair>> = HashMap::new();
    for pair in pairs {
        let chain_id = extract_chain_id_from_u256(&pair.ct_handle);
        grouped.entry(chain_id).or_default().push(pair.clone());
    }
    grouped
}

/// Decode ABI-encoded bool from multicall result bytes.
fn decode_bool(data: &[u8]) -> Result<bool, &'static str> {
    if data.len() < 32 {
        return Err("malformed multicall result: data shorter than 32 bytes");
    }
    // ABI-encoded bool: 32 bytes, last byte is 0 or 1
    Ok(data[31] != 0)
}

async fn fetch_solana_state(
    client: &SolanaStateClient,
    chain_id: u64,
) -> Result<HostProgramState, HostAclError> {
    client
        .fetch_state()
        .await
        .map_err(|err| HostAclError::CallFailed {
            chain_id,
            message: err.to_string(),
        })
}

fn solana_host_identity_from_evm_address(address: Address) -> SolanaHostPubkey {
    host_identity_from_evm_address(SolanaEvmAddress::from(address.into_array()))
}

fn solana_host_identity_from_fixed_bytes(identity: FixedBytes<32>) -> SolanaHostPubkey {
    SolanaHostPubkey::from(*identity)
}

#[derive(Default)]
struct SolanaIdentityOverrides {
    contract_ids: Option<HashMap<Address, SolanaHostPubkey>>,
}

fn parse_solana_decryption_identity_sequence(
    extra_data: &[u8],
) -> Result<Option<Vec<[u8; 32]>>, HostAclError> {
    if extra_data.is_empty() || extra_data == [0x00] || extra_data[0] != 0x02 {
        return Ok(None);
    }

    if extra_data.len() < 34 {
        return Err(HostAclError::CallFailed {
            chain_id: 0,
            message: format!(
                "invalid v2 decryption extra_data: expected at least 34 bytes, got {}",
                extra_data.len()
            ),
        });
    }

    let identity_count = extra_data[33] as usize;
    let expected_len = 34 + identity_count * 32;
    if extra_data.len() != expected_len {
        return Err(HostAclError::CallFailed {
            chain_id: 0,
            message: format!(
                "invalid v2 decryption extra_data length: expected {} bytes, got {}",
                expected_len,
                extra_data.len()
            ),
        });
    }

    let mut identities = Vec::with_capacity(identity_count);
    for index in 0..identity_count {
        let start = 34 + index * 32;
        let end = start + 32;
        let identity: [u8; 32] =
            extra_data[start..end]
                .try_into()
                .map_err(|_| HostAclError::CallFailed {
                    chain_id: 0,
                    message: format!("failed to decode decryption identity at index {index}"),
                })?;
        identities.push(identity);
    }

    Ok(Some(identities))
}

fn parse_solana_user_identity_overrides(
    contract_addresses: &[Address],
    extra_data: &[u8],
) -> Result<SolanaIdentityOverrides, HostAclError> {
    let Some(identities) = parse_solana_decryption_identity_sequence(extra_data)? else {
        return Ok(SolanaIdentityOverrides::default());
    };

    if identities.len() == contract_addresses.len() {
        return Ok(SolanaIdentityOverrides {
            contract_ids: Some(zip_solana_contract_identities(
                contract_addresses,
                identities,
            )?),
        });
    }

    if identities.len() == contract_addresses.len() + 1 {
        return Ok(SolanaIdentityOverrides {
            contract_ids: Some(zip_solana_contract_identities(
                contract_addresses,
                identities.into_iter().skip(1).collect(),
            )?),
        });
    }

    Err(HostAclError::CallFailed {
        chain_id: 0,
        message: format!(
            "invalid v2 decryption identity count: expected {} or {}, got {}",
            contract_addresses.len(),
            contract_addresses.len() + 1,
            identities.len()
        ),
    })
}

fn parse_solana_delegated_identity_overrides(
    contract_addresses: &[Address],
    extra_data: &[u8],
) -> Result<SolanaIdentityOverrides, HostAclError> {
    let Some(identities) = parse_solana_decryption_identity_sequence(extra_data)? else {
        return Ok(SolanaIdentityOverrides::default());
    };

    if identities.len() == contract_addresses.len() {
        return Ok(SolanaIdentityOverrides {
            contract_ids: Some(zip_solana_contract_identities(
                contract_addresses,
                identities,
            )?),
        });
    }

    if identities.len() == contract_addresses.len() + 2 {
        return Ok(SolanaIdentityOverrides {
            contract_ids: Some(zip_solana_contract_identities(
                contract_addresses,
                identities.into_iter().skip(2).collect(),
            )?),
        });
    }

    Err(HostAclError::CallFailed {
        chain_id: 0,
        message: format!(
            "invalid delegated v2 decryption identity count: expected {} or {}, got {}",
            contract_addresses.len(),
            contract_addresses.len() + 2,
            identities.len()
        ),
    })
}

fn zip_solana_contract_identities(
    contract_addresses: &[Address],
    contract_ids: Vec<[u8; 32]>,
) -> Result<HashMap<Address, SolanaHostPubkey>, HostAclError> {
    if contract_ids.len() != contract_addresses.len() {
        return Err(HostAclError::CallFailed {
            chain_id: 0,
            message: format!(
                "invalid v2 decryption contract identity count: expected {}, got {}",
                contract_addresses.len(),
                contract_ids.len()
            ),
        });
    }

    Ok(HashMap::from_iter(
        contract_addresses
            .iter()
            .copied()
            .zip(contract_ids.into_iter().map(SolanaHostPubkey::from)),
    ))
}

fn zip_solana_contract_identities_from_fixed_bytes(
    contract_addresses: &[Address],
    contract_ids: &[FixedBytes<32>],
) -> Result<HashMap<Address, SolanaHostPubkey>, HostAclError> {
    zip_solana_contract_identities(
        contract_addresses,
        contract_ids
            .iter()
            .map(|identity| {
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(identity.as_slice());
                bytes
            })
            .collect(),
    )
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bool_true() {
        let mut data = [0u8; 32];
        data[31] = 1;
        assert_eq!(decode_bool(&data), Ok(true));
    }

    #[test]
    fn test_decode_bool_false() {
        let data = [0u8; 32];
        assert_eq!(decode_bool(&data), Ok(false));
    }

    #[test]
    fn test_decode_bool_short_data() {
        assert!(decode_bool(&[1u8; 4]).is_err());
    }

    #[test]
    fn test_decode_bool_empty() {
        assert!(decode_bool(&[]).is_err());
    }

    #[test]
    fn test_group_handles_by_chain() {
        fn make_handle(chain_id: u64) -> [u8; 32] {
            let mut h = [0u8; 32];
            h[22..30].copy_from_slice(&chain_id.to_be_bytes());
            h
        }

        let handles = vec![make_handle(8009), make_handle(9000), make_handle(8009)];
        let grouped = group_handles_by_chain(&handles);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[&8009].len(), 2);
        assert_eq!(grouped[&9000].len(), 1);
    }

    #[test]
    fn test_group_handles_empty() {
        let grouped = group_handles_by_chain(&[]);
        assert!(grouped.is_empty());
    }
}
