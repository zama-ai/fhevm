use crate::{
    config::settings::{HostChainConfig, RetrySettings},
    core::{
        event::{HandleContractPair, HandleEntry},
        job_id::JobId,
    },
    host::{
        error_redact::redact_alloy_error,
        handle_chain_id::{extract_chain_id_from_handle, extract_chain_id_from_u256},
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
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
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

struct HostChainAcl {
    acl: HostAcl,
}

/// Checks handle permissions against host chain ACL contracts via multicall.
pub struct HostAclChecker {
    chains: HashMap<u64, HostChainAcl>,
    /// RFC-021 Solana host chains (chain-type high bit), keyed by chain id. A Solana
    /// host carries a base58 `acl_address` (the zama-host program) and has no EVM ACL
    /// contract to `eth_call`. Its ACL is enforced authoritatively by the KMS
    /// (`solana_acl` witness verification) and on-chain `secp256k1` cert checks; the
    /// relayer cannot derive the ACL-record PDA from a bare handle, so it does not
    /// perform the EVM-style pre-check for these chains.
    solana_chains: std::collections::HashSet<u64>,
    retry_config: RetrySettings,
}

impl HostAclChecker {
    pub fn new(host_chains: &[HostChainConfig], retry: RetrySettings) -> anyhow::Result<Self> {
        let mut chains = HashMap::new();
        let mut solana_chains = std::collections::HashSet::new();

        for hc in host_chains {
            // The chain id is the sole chain-kind discriminator. Settings validation
            // enforces the matching address encoding; keep this constructor fail-closed
            // for direct callers too.
            if crate::core::event::is_solana_host_chain_id(hc.chain_id) {
                if !crate::http::utils::solana_address::is_solana_address(&hc.acl_address) {
                    anyhow::bail!(
                        "Invalid Solana ACL address for chain {}: {}",
                        hc.chain_id,
                        hc.acl_address
                    );
                }
                solana_chains.insert(hc.chain_id);
                continue;
            }

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

            let acl = ACL::new(acl_address, provider);

            chains.insert(hc.chain_id, HostChainAcl { acl });
        }

        Ok(Self {
            chains,
            solana_chains,
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
            // RFC-021 Solana host: ACL enforced authoritatively by the KMS (solana_acl)
            // and on-chain secp256k1 cert checks; no EVM eth_call pre-check applies.
            if self.solana_chains.contains(chain_id) {
                continue;
            }
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;

            // Build multicall data: one isAllowedForDecryption call per handle
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
                .multicall_with_retry(job_id, chain_acl, &calls, *chain_id)
                .await?;

            if results.len() != chain_handles.len() {
                return Err(HostAclError::CallFailed {
                    chain_id: *chain_id,
                    message: format!(
                        "expected {} multicall results, got {}",
                        chain_handles.len(),
                        results.len()
                    ),
                });
            }

            // Decode results: each is abi-encoded bool
            for (i, result_bytes) in results.iter().enumerate() {
                let allowed =
                    decode_bool(result_bytes).map_err(|msg| HostAclError::CallFailed {
                        chain_id: *chain_id,
                        message: msg.to_string(),
                    })?;
                if !allowed {
                    all_failures.push(AclFailure {
                        handle: format!("0x{}", hex::encode(chain_handles[i])),
                        check: "isAllowedForDecryption".to_string(),
                    });
                }
            }
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
        pairs: &[HandleContractPair],
        user: Address,
    ) -> Result<(), HostAclError> {
        if pairs.is_empty() {
            return Ok(());
        }

        // Group pairs by chain_id (extracted from handle)
        let grouped = group_pairs_by_chain(pairs);

        let mut all_failures = Vec::new();

        for (chain_id, chain_pairs) in &grouped {
            // RFC-021 Solana host: ACL enforced authoritatively by the KMS (solana_acl)
            // and on-chain secp256k1 cert checks; no EVM eth_call pre-check applies.
            if self.solana_chains.contains(chain_id) {
                continue;
            }
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;

            // Build multicall data: 2 calls per pair (user + contract)
            let mut calls: Vec<Bytes> = Vec::with_capacity(chain_pairs.len() * 2);
            for pair in chain_pairs {
                let handle_bytes: [u8; 32] = pair.ct_handle.to_be_bytes();
                let handle = FixedBytes::from(handle_bytes);

                // isAllowed(handle, user)
                let user_call = ACL::isAllowedCall {
                    handle,
                    account: user,
                };
                calls.push(Bytes::from(user_call.abi_encode()));

                // isAllowed(handle, contract)
                let contract_call = ACL::isAllowedCall {
                    handle,
                    account: pair.contract_address,
                };
                calls.push(Bytes::from(contract_call.abi_encode()));
            }

            let results = self
                .multicall_with_retry(job_id, chain_acl, &calls, *chain_id)
                .await?;

            let expected = chain_pairs.len() * 2;
            if results.len() != expected {
                return Err(HostAclError::CallFailed {
                    chain_id: *chain_id,
                    message: format!(
                        "expected {} multicall results, got {}",
                        expected,
                        results.len()
                    ),
                });
            }

            // Decode results: pairs of (user_allowed, contract_allowed)
            for (i, pair) in chain_pairs.iter().enumerate() {
                let handle_hex = format!("0x{:064x}", pair.ct_handle);
                let user_allowed =
                    decode_bool(&results[i * 2]).map_err(|msg| HostAclError::CallFailed {
                        chain_id: *chain_id,
                        message: msg.to_string(),
                    })?;
                let contract_allowed =
                    decode_bool(&results[i * 2 + 1]).map_err(|msg| HostAclError::CallFailed {
                        chain_id: *chain_id,
                        message: msg.to_string(),
                    })?;

                if !user_allowed {
                    all_failures.push(AclFailure {
                        handle: handle_hex.clone(),
                        check: format!("isAllowed(user {})", user),
                    });
                }
                if !contract_allowed {
                    all_failures.push(AclFailure {
                        handle: handle_hex,
                        check: format!("isAllowed(contract {})", pair.contract_address),
                    });
                }
            }
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
        pairs: &[HandleContractPair],
        delegator: Address,
        delegate: Address,
    ) -> Result<(), HostAclError> {
        if pairs.is_empty() {
            return Ok(());
        }

        let grouped = group_pairs_by_chain(pairs);

        let mut all_failures = Vec::new();

        for (chain_id, chain_pairs) in &grouped {
            // RFC-021 Solana host: ACL enforced authoritatively by the KMS (solana_acl)
            // and on-chain secp256k1 cert checks; no EVM eth_call pre-check applies.
            if self.solana_chains.contains(chain_id) {
                continue;
            }
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;

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
                .multicall_with_retry(job_id, chain_acl, &calls, *chain_id)
                .await?;

            if results.len() != chain_pairs.len() {
                return Err(HostAclError::CallFailed {
                    chain_id: *chain_id,
                    message: format!(
                        "expected {} multicall results, got {}",
                        chain_pairs.len(),
                        results.len()
                    ),
                });
            }

            for (i, pair) in chain_pairs.iter().enumerate() {
                let allowed = decode_bool(&results[i]).map_err(|msg| HostAclError::CallFailed {
                    chain_id: *chain_id,
                    message: msg.to_string(),
                })?;
                if !allowed {
                    all_failures.push(AclFailure {
                        handle: format!("0x{:064x}", pair.ct_handle),
                        check: "isHandleDelegatedForUserDecryption".to_string(),
                    });
                }
            }
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

    /// Check ACL for a unified EIP-712 user decryption request.
    ///
    /// Each `HandleEntry` is classified per-handle by its `owner_address`:
    /// - When `owner_address == user`, the entry is a direct access and
    ///   only `isAllowed(handle, user)` is asserted.
    /// - Otherwise the entry is delegated, and
    ///   `isHandleDelegatedForUserDecryption(owner, user, contract, handle)`
    ///   is asserted (the contract itself rolls in `isAllowed(handle,
    ///   owner)`, `isAllowed(handle, contract)`, and the delegation lookup).
    ///
    /// Calls are grouped by host chain id (extracted from the handle bytes)
    /// and dispatched through one multicall per chain.
    pub async fn check_unified_user_decrypt(
        &self,
        job_id: &JobId,
        handles: &[HandleEntry],
        user: Address,
    ) -> Result<(), HostAclError> {
        if handles.is_empty() {
            return Ok(());
        }

        let grouped = group_handle_entries_by_chain(handles);
        let mut all_failures = Vec::new();

        for (chain_id, chain_entries) in &grouped {
            // RFC-021 Solana host: ACL enforced authoritatively by the KMS (solana_acl)
            // and on-chain secp256k1 cert checks; no EVM eth_call pre-check applies.
            if self.solana_chains.contains(chain_id) {
                continue;
            }
            let chain_acl = self
                .chains
                .get(chain_id)
                .ok_or(HostAclError::UnsupportedChain {
                    chain_id: *chain_id,
                })?;

            let calls: Vec<Bytes> = chain_entries
                .iter()
                .map(|entry| {
                    let handle_bytes: [u8; 32] = entry.ct_handle.to_be_bytes();
                    let handle = FixedBytes::from(handle_bytes);
                    if entry.owner_address == user {
                        Bytes::from(
                            ACL::isAllowedCall {
                                handle,
                                account: user,
                            }
                            .abi_encode(),
                        )
                    } else {
                        Bytes::from(
                            ACL::isHandleDelegatedForUserDecryptionCall {
                                delegator: entry.owner_address,
                                delegate: user,
                                contractAddress: entry.contract_address,
                                handle,
                            }
                            .abi_encode(),
                        )
                    }
                })
                .collect();

            let results = self
                .multicall_with_retry(job_id, chain_acl, &calls, *chain_id)
                .await?;

            if results.len() != chain_entries.len() {
                return Err(HostAclError::CallFailed {
                    chain_id: *chain_id,
                    message: format!(
                        "expected {} multicall results, got {}",
                        chain_entries.len(),
                        results.len()
                    ),
                });
            }

            for (i, entry) in chain_entries.iter().enumerate() {
                let allowed = decode_bool(&results[i]).map_err(|msg| HostAclError::CallFailed {
                    chain_id: *chain_id,
                    message: msg.to_string(),
                })?;
                if !allowed {
                    let handle_hex = format!("0x{:064x}", entry.ct_handle);
                    let check = if entry.owner_address == user {
                        format!("isAllowed(user {})", user)
                    } else {
                        format!(
                            "isHandleDelegatedForUserDecryption(owner {}, user {})",
                            entry.owner_address, user
                        )
                    };
                    all_failures.push(AclFailure {
                        handle: handle_hex,
                        check,
                    });
                }
            }
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

    /// Execute a multicall against a host chain ACL contract with retry on RPC errors.
    async fn multicall_with_retry(
        &self,
        job_id: &JobId,
        chain_acl: &HostChainAcl,
        calls: &[Bytes],
        chain_id: u64,
    ) -> Result<Vec<Bytes>, HostAclError> {
        let max_attempts = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);
        let mut last_error = String::new();
        let calls_vec = calls.to_vec();

        for attempt in 0..max_attempts {
            match chain_acl.acl.multicall(calls_vec.clone()).call().await {
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

fn group_handle_entries_by_chain(handles: &[HandleEntry]) -> HashMap<u64, Vec<HandleEntry>> {
    let mut grouped: HashMap<u64, Vec<HandleEntry>> = HashMap::new();
    for entry in handles {
        let chain_id = extract_chain_id_from_u256(&entry.ct_handle);
        grouped.entry(chain_id).or_default().push(entry.clone());
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

    #[tokio::test]
    async fn solana_host_starts_and_skips_evm_precheck() {
        use crate::config::settings::{HostChainConfig, RetrySettings};

        // RFC-021 Solana host: chain-type high bit + base58 acl_address (zama-host program).
        let solana_chain_id = (1u64 << 63) | 12345;
        let host_chains = vec![HostChainConfig {
            chain_id: solana_chain_id,
            url: "http://127.0.0.1:8899".to_string(),
            acl_address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
        }];

        // new() must not panic on the base58 acl_address (the prior bug).
        let checker = HostAclChecker::new(
            &host_chains,
            RetrySettings {
                max_attempts: 1,
                retry_interval_ms: 1,
            },
        )
        .expect("base58 Solana acl_address must not fail HostAclChecker::new");

        assert!(checker.solana_chains.contains(&solana_chain_id));
        assert!(
            checker.chains.is_empty(),
            "Solana host must not create an EVM ACL eth_call client"
        );

        // A Solana handle (chain id in bytes 22..30): the EVM pre-check is deferred to the
        // KMS solana_acl + on-chain secp path, so this returns Ok without any chain RPC.
        let mut handle = [0u8; 32];
        handle[22..30].copy_from_slice(&solana_chain_id.to_be_bytes());
        let job_id = JobId::from([0u8; 32]);
        assert!(checker
            .check_public_decrypt(&job_id, &[handle])
            .await
            .is_ok());
    }

    #[test]
    fn chain_id_discriminator_rejects_mismatched_acl_address_formats() {
        use crate::config::settings::{HostChainConfig, RetrySettings};

        let retry = RetrySettings {
            max_attempts: 1,
            retry_interval_ms: 1,
        };
        let evm_address = "0x339EBB773A9bC1deCFfD5ef4BC7c907e26C1f836";
        let solana_address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

        for (chain_id, acl_address) in
            [((1u64 << 63) | 12345, evm_address), (12345, solana_address)]
        {
            let result = HostAclChecker::new(
                &[HostChainConfig {
                    chain_id,
                    url: "http://127.0.0.1:8899".to_string(),
                    acl_address: acl_address.to_string(),
                }],
                retry.clone(),
            );
            assert!(result.is_err(), "chain/address mismatch must be rejected");
        }
    }
}
