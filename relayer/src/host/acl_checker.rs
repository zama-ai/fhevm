use crate::{
    config::settings::{HostChainConfig, RetrySettings},
    core::{
        event::{HandleContractPair, HandleEntry},
        job_id::JobId,
    },
    host::{
        error_redact::redact_alloy_error,
        handle_chain_id::{extract_chain_id_from_handle, extract_chain_id_from_u256},
        solana_delegation_precheck::{
            encrypted_value_read_addresses, judge_planned_entries, plan_row_reads, DelegatedEntry,
            RawAccount,
        },
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

/// One configured Solana host chain: where to read, and whose accounts count.
struct SolanaHostChain {
    rpc_url: Url,
    /// The zama-host program id — the `acl_address` of the chain's config entry.
    program_id: [u8; 32],
    http: reqwest::Client,
}

/// Checks handle permissions against host chain ACL contracts via multicall.
pub struct HostAclChecker {
    chains: HashMap<u64, HostChainAcl>,
    /// RFC-021 Solana host chains (chain-type high bit), keyed by chain id. A Solana
    /// host carries a base58 `acl_address` (the zama-host program) and has no EVM ACL
    /// contract to `eth_call`; its ACL is enforced authoritatively by the KMS Connector.
    /// Direct entries and public decrypts are not pre-checked here — their authorization
    /// is membership in the encrypted value account, and this checker has no cheaper
    /// reading of it than the connector's own. Delegated user-decrypt entries ARE: the v3
    /// request carries the encrypted value id and the subject, which is everything the advisory
    /// negative-only pre-check (`check_solana_delegated_user_decrypt`) needs to read the
    /// delegation rows.
    solana_chains: HashMap<u64, SolanaHostChain>,
    retry_config: RetrySettings,
}

impl HostAclChecker {
    pub fn new(host_chains: &[HostChainConfig], retry: RetrySettings) -> anyhow::Result<Self> {
        let mut chains = HashMap::new();
        let mut solana_chains = HashMap::new();

        for hc in host_chains {
            // The chain id is the sole chain-kind discriminator. Settings validation
            // enforces the matching address encoding; keep this constructor fail-closed
            // for direct callers too.
            if crate::core::event::is_solana_host_chain_id(hc.chain_id) {
                let program_id =
                    crate::http::utils::solana_address::decode_solana_address(&hc.acl_address)
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "Invalid Solana ACL address for chain {}: {}: {e}",
                                hc.chain_id,
                                hc.acl_address
                            )
                        })?;
                let rpc_url = Url::parse(&hc.url).map_err(|e| {
                    anyhow::anyhow!("Invalid host chain URL for chain {}: {}", hc.chain_id, e)
                })?;
                solana_chains.insert(
                    hc.chain_id,
                    SolanaHostChain {
                        rpc_url,
                        program_id,
                        http: reqwest::Client::new(),
                    },
                );
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
            if self.solana_chains.contains_key(chain_id) {
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
            if self.solana_chains.contains_key(chain_id) {
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
            if self.solana_chains.contains_key(chain_id) {
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
            if self.solana_chains.contains_key(chain_id) {
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
    /// Advisory, negative-only pre-check of a Solana delegated user-decrypt request.
    ///
    /// The rule lives in [`super::solana_delegation_precheck`]; this method is the transport:
    /// two batched `getMultipleAccounts` reads at `confirmed` — the encrypted value accounts
    /// first (to
    /// learn each entry's encrypted value account authority), then the delegation rows, with the row read's own slot
    /// deciding liveness. Direct entries are never checked. Transport failures follow the EVM
    /// pre-check's policy: retried, then surfaced as [`HostAclError::CallFailed`] rather than
    /// failed-open. Ambiguity of *data* (an account this reader cannot judge) passes — the
    /// authoritative check is the KMS connectors'.
    ///
    /// The chain whose state is read is named by the permit's SIGNED `chain_id` — the same
    /// field the connector authorizes against — never by the unsigned chain-id bytes embedded
    /// in a handle: an advisory check that read a different chain's rows than its authority
    /// could refuse what the authority would allow.
    pub async fn check_solana_delegated_user_decrypt(
        &self,
        job_id: &JobId,
        solana_request: &[u8],
    ) -> Result<(), HostAclError> {
        // Admission already verified the blob's shape and the permit signature over it; a
        // re-decode failure here is this relayer's own defect, and an advisory check does not
        // refuse users on its own defects.
        let Ok(wire) = zama_solana_request::decode_solana_request(solana_request) else {
            warn!(
                int_job_id = %job_id,
                "Solana delegation pre-check could not re-decode the request blob; passing"
            );
            return Ok(());
        };
        let chain_id = wire.permit.chain_id;
        let Some(chain) = self.solana_chains.get(&chain_id) else {
            return Err(HostAclError::UnsupportedChain { chain_id });
        };
        let Ok(user_pubkey) = <[u8; 32]>::try_from(wire.permit.user_pubkey.as_slice()) else {
            warn!(
                int_job_id = %job_id,
                chain_id,
                "Solana delegation pre-check saw a non-32-byte user pubkey; passing"
            );
            return Ok(());
        };

        let delegated: Vec<DelegatedEntry> = wire
            .handles
            .iter()
            .filter_map(|entry| {
                let subject = <[u8; 32]>::try_from(entry.subject.as_slice()).ok()?;
                let encrypted_value_id =
                    <[u8; 32]>::try_from(entry.encrypted_value_id.as_slice()).ok()?;
                (subject != user_pubkey).then(|| DelegatedEntry {
                    handle_hex: format!("0x{}", hex::encode(&entry.handle)),
                    subject,
                    encrypted_value_id,
                })
            })
            .collect();
        if delegated.is_empty() {
            return Ok(());
        }

        // Round 1: the encrypted value accounts, to learn each entry's authority.
        let encrypted_value_addresses =
            encrypted_value_read_addresses(&delegated, chain.program_id);
        let (_, encrypted_value_accounts) = self
            .solana_accounts_with_retry(job_id, chain, chain_id, &encrypted_value_addresses)
            .await?;

        // Entries whose encrypted value account this check cannot judge drop out of the plan
        // (indeterminate).
        let plan = match plan_row_reads(
            chain.program_id,
            user_pubkey,
            delegated,
            encrypted_value_accounts,
        ) {
            Ok(plan) => plan,
            Err(defect) => {
                warn!(
                    int_job_id = %job_id,
                    chain_id,
                    ?defect,
                    "Solana delegation pre-check mispaired its own reads; passing"
                );
                return Ok(());
            }
        };
        if plan.entries.is_empty() {
            return Ok(());
        }

        // Round 2: the rows, whose read slot is the one liveness is decided at.
        let (slot, row_accounts) = self
            .solana_accounts_with_retry(job_id, chain, chain_id, &plan.addresses)
            .await?;

        let refusals = match judge_planned_entries(
            chain.program_id,
            user_pubkey,
            &plan.entries,
            &row_accounts,
            slot,
        ) {
            Ok(refusals) => refusals,
            Err(defect) => {
                warn!(
                    int_job_id = %job_id,
                    chain_id,
                    ?defect,
                    "Solana delegation pre-check mispaired its own reads; passing"
                );
                return Ok(());
            }
        };

        if refusals.is_empty() {
            Ok(())
        } else {
            let failures: Vec<AclFailure> = refusals
                .into_iter()
                .map(|refusal| AclFailure {
                    handle: refusal.handle_hex,
                    check: refusal.reason,
                })
                .collect();
            Err(HostAclError::NotAllowed {
                count: failures.len(),
                failures,
            })
        }
    }

    /// One `getMultipleAccounts` read at `confirmed`, retried on the EVM pre-check's policy.
    /// Returns the response's own slot beside the accounts: one read, one observation point.
    async fn solana_accounts_with_retry(
        &self,
        job_id: &JobId,
        chain: &SolanaHostChain,
        chain_id: u64,
        addresses: &[[u8; 32]],
    ) -> Result<(u64, Vec<Option<RawAccount>>), HostAclError> {
        let max_attempts = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);
        let mut last_error = String::new();

        for attempt in 0..max_attempts {
            match solana_get_multiple_accounts(chain, addresses).await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    last_error = err;
                    if attempt + 1 < max_attempts {
                        warn!(
                            int_job_id = %job_id,
                            chain_id,
                            attempt = attempt + 1,
                            max_attempts,
                            error = %last_error,
                            "Solana delegation pre-check read failed, retrying"
                        );
                        tokio::time::sleep(retry_interval).await;
                    }
                }
            }
        }
        error!(
            int_job_id = %job_id,
            chain_id,
            error = %last_error,
            "Solana delegation pre-check read failed after retries"
        );
        Err(HostAclError::CallFailed {
            chain_id,
            message: last_error,
        })
    }

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

/// One JSON-RPC `getMultipleAccounts` at `confirmed`, base64-encoded. Returns the response's
/// slot and one entry per requested address (`None` where no account exists). Every failure —
/// transport, a non-JSON body, a malformed entry — is a `String` for the retry loop.
async fn solana_get_multiple_accounts(
    chain: &SolanaHostChain,
    addresses: &[[u8; 32]],
) -> Result<(u64, Vec<Option<RawAccount>>), String> {
    use base64::Engine as _;

    let params = serde_json::json!([
        addresses
            .iter()
            .map(|address| solana_pubkey::Pubkey::new_from_array(*address).to_string())
            .collect::<Vec<_>>(),
        { "encoding": "base64", "commitment": "confirmed" }
    ]);
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getMultipleAccounts",
        "params": params,
    });

    let response = chain
        .http
        .post(chain.rpc_url.clone())
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("transport: {e}"))?;
    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("non-JSON response: {e}"))?;
    if let Some(error) = payload.get("error") {
        return Err(format!("rpc error: {error}"));
    }

    let result = payload
        .get("result")
        .ok_or_else(|| "response carries no result".to_string())?;
    let slot = result
        .pointer("/context/slot")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "response carries no context slot".to_string())?;
    let values = result
        .get("value")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "response carries no account list".to_string())?;
    if values.len() != addresses.len() {
        return Err(format!(
            "response carries {} accounts for {} addresses",
            values.len(),
            addresses.len()
        ));
    }

    let mut accounts = Vec::with_capacity(values.len());
    for value in values {
        if value.is_null() {
            accounts.push(None);
            continue;
        }
        let owner_base58 = value
            .get("owner")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "account entry carries no owner".to_string())?;
        let owner = crate::http::utils::solana_address::decode_solana_address(owner_base58)
            .map_err(|_| format!("account owner is not a Solana address: {owner_base58}"))?;
        let data_base64 = value
            .pointer("/data/0")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "account entry carries no data".to_string())?;
        let data = base64::engine::general_purpose::STANDARD
            .decode(data_base64)
            .map_err(|e| format!("account data is not base64: {e}"))?;
        accounts.push(Some(RawAccount { owner, data }));
    }
    Ok((slot, accounts))
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

        assert!(checker.solana_chains.contains_key(&solana_chain_id));
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
