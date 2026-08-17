//! Signature pre-check on v3 user-decryption forwarding.
//!
//! Recomputes the unified EIP-712 digest and runs the shared verifier
//! (`user_decryption_signature::verify_signature`) to detect bad signatures before forwarding,
//! returning a specific error to the caller instead of letting the request fail downstream.

use crate::config::settings::{HostChainConfig, RetrySettings};
use crate::core::event::UserDecryptRequest;
use crate::host::handle_chain_id::extract_chain_id_from_u256;
use alloy::primitives::{Address, U256};
use alloy::providers::{ProviderBuilder, RootProvider};
use fhevm_gateway_bindings::decryption::IDecryption::{
    RequestValiditySeconds as SolRequestValiditySeconds, UserDecryptionRequestPayload,
};
use reqwest::Url;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tracing::warn;
use user_decryption_signature::{
    compute_user_decrypt_digest, default_user_decrypt_domain, verify_signature, Erc1271Error,
};

/// Outcome of a failed pre-check.
#[derive(Debug, thiserror::Error)]
pub enum SigPreCheckError {
    /// The signature is definitively invalid — the request must not be forwarded.
    #[error("invalid user-decryption signature for {signer}: {reason}")]
    Invalid { signer: Address, reason: String },
    /// The host-chain call could not complete (transport error after retries). Surfaced as a
    /// server error, mirroring how host ACL call failures are handled.
    #[error("host-chain call failed during signature pre-check: {0}")]
    HostCallFailed(String),
}

/// Recomputes the unified EIP-712 digest and verifies it via the shared RFC-012 helper.
pub struct UserDecryptSignaturePreChecker {
    /// One read-only provider per supported host chain, keyed by chain id.
    providers: HashMap<u64, RootProvider>,
    /// Gateway `Decryption` contract — the EIP-712 verifying contract.
    decryption_contract: Address,
    /// Gas cap for the ERC-1271 `isValidSignature` static call.
    erc1271_gas_limit: u64,
    /// Retry policy for transport errors, shared with the host ACL checks.
    retry: RetrySettings,
}

impl UserDecryptSignaturePreChecker {
    pub fn new(
        host_chains: &[HostChainConfig],
        decryption_address: &str,
        erc1271_gas_limit: u64,
        retry: RetrySettings,
    ) -> anyhow::Result<Self> {
        let decryption_contract = Address::from_str(decryption_address)
            .map_err(|e| anyhow::anyhow!("Invalid decryption address: {e}"))?;

        let mut providers = HashMap::new();
        for hc in host_chains {
            let url = Url::parse(&hc.url).map_err(|e| {
                anyhow::anyhow!("Invalid host chain URL for chain {}: {}", hc.chain_id, e)
            })?;
            // Read-only provider: no fillers needed for a plain `eth_call` / `eth_getCode`.
            let provider = ProviderBuilder::new()
                .disable_recommended_fillers()
                .connect_http(url);
            providers.insert(hc.chain_id, provider);
        }

        Ok(Self {
            providers,
            decryption_contract,
            erc1271_gas_limit,
            retry,
        })
    }

    /// Verify the signature on a unified v3 request. Non-unified variants are a no-op (the
    /// pre-check is wired only into the v3 endpoint). Recomputes the EIP-712 digest, then runs
    /// the shared verifier, retrying transport errors like the host ACL checks do and rejecting
    /// only on definitive verification failures.
    pub async fn verify(&self, request: &UserDecryptRequest) -> Result<(), SigPreCheckError> {
        let UserDecryptRequest::Eip712UnifiedV1 {
            handles,
            user_address,
            allowed_contracts,
            request_validity,
            signature,
            public_key,
            extra_data,
        } = request
        else {
            return Ok(());
        };
        let user_address = *user_address;

        let chain_id = single_chain_id(handles.iter().map(|h| &h.ct_handle))?;
        let provider = self.providers.get(&chain_id).ok_or_else(|| {
            SigPreCheckError::HostCallFailed(format!("no provider configured for chain {chain_id}"))
        })?;

        let domain = default_user_decrypt_domain(chain_id, self.decryption_contract);
        let payload = UserDecryptionRequestPayload {
            userAddress: user_address,
            publicKey: public_key.clone(),
            allowedContracts: allowed_contracts.clone(),
            requestValidity: SolRequestValiditySeconds {
                startTimestamp: request_validity.start_timestamp,
                durationSeconds: request_validity.duration_seconds,
            },
            extraData: extra_data.clone(),
            signature: signature.clone(),
        };
        let digest = compute_user_decrypt_digest(&payload, &domain);

        let max_attempts = self.retry.max_attempts.max(1);
        let interval = Duration::from_millis(self.retry.retry_interval_ms);
        let mut last_transport_err = String::new();

        for attempt in 0..max_attempts {
            match verify_signature(
                provider,
                user_address,
                digest,
                signature.as_ref(),
                self.erc1271_gas_limit,
            )
            .await
            {
                Ok(()) => return Ok(()),
                // Transport errors are non-deterministic — retry like the host ACL checks do.
                Err(Erc1271Error::Transport(msg)) => {
                    last_transport_err = msg;
                    if attempt + 1 < max_attempts {
                        warn!(
                            signer = %user_address,
                            chain_id,
                            attempt = attempt + 1,
                            max_attempts,
                            error = %last_transport_err,
                            "Signature pre-check RPC failed, retrying"
                        );
                        tokio::time::sleep(interval).await;
                    }
                }
                // Every other variant is a definitive rejection (ecrecover mismatch, wrong/empty
                // ERC-1271 magic, revert, short returndata). The error Display encodes the path.
                Err(e) => {
                    return Err(SigPreCheckError::Invalid {
                        signer: user_address,
                        reason: e.to_string(),
                    });
                }
            }
        }

        Err(SigPreCheckError::HostCallFailed(last_transport_err))
    }
}

/// All handles in a unified request share one host chain (the EIP-712 domain carries a single
/// `contractsChainId`). Returns that chain id, erroring if handles are empty or span chains.
fn single_chain_id<'a, I>(handles: I) -> Result<u64, SigPreCheckError>
where
    I: IntoIterator<Item = &'a U256>,
{
    let mut chain_id = None;
    for handle in handles {
        let id = extract_chain_id_from_u256(handle);
        match chain_id {
            None => chain_id = Some(id),
            Some(seen) if seen != id => {
                return Err(SigPreCheckError::Invalid {
                    signer: Address::ZERO,
                    reason: format!("handles span multiple host chains ({seen} and {id})"),
                });
            }
            _ => {}
        }
    }
    chain_id.ok_or_else(|| SigPreCheckError::Invalid {
        signer: Address::ZERO,
        reason: "request has no handles".to_string(),
    })
}

#[cfg(test)]
mod tests {
    // The `verify_signature` outcomes (ecrecover / ERC-1271 magic / wrong magic / short
    // returndata / empty signature) are covered by the `user-decryption-signature` crate, and
    // the accept/reject paths end-to-end by `tests/user_decrypt_v3_test.rs`. These cover only
    // the glue this module adds: chain-id handling, the non-unified no-op, and the
    // transport-retry-then-`HostCallFailed` mapping.
    use super::*;
    use crate::core::event::{HandleEntry, RequestValiditySeconds};
    use alloy::primitives::Bytes;
    use alloy::providers::mock::Asserter;

    const TEST_CHAIN_ID: u64 = 8009;

    fn handle_for_chain(chain_id: u64) -> U256 {
        let mut bytes = [0u8; 32];
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        U256::from_be_bytes(bytes)
    }

    fn checker(asserter: Asserter) -> UserDecryptSignaturePreChecker {
        let mut providers = HashMap::new();
        providers.insert(
            TEST_CHAIN_ID,
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .connect_mocked_client(asserter),
        );
        UserDecryptSignaturePreChecker {
            providers,
            decryption_contract: Address::from([0xCA; 20]),
            erc1271_gas_limit: 100_000,
            retry: RetrySettings {
                max_attempts: 3,
                retry_interval_ms: 0,
            },
        }
    }

    fn unified_request(signature: Bytes) -> UserDecryptRequest {
        UserDecryptRequest::Eip712UnifiedV1 {
            handles: vec![HandleEntry {
                ct_handle: handle_for_chain(TEST_CHAIN_ID),
                contract_address: Address::from([0xAB; 20]),
                owner_address: Address::from([0xDE; 20]),
            }],
            user_address: Address::from([0xDE; 20]),
            allowed_contracts: vec![],
            request_validity: RequestValiditySeconds {
                start_timestamp: U256::ZERO,
                duration_seconds: U256::ZERO,
            },
            signature,
            public_key: Bytes::new(),
            extra_data: Bytes::new(),
        }
    }

    #[tokio::test]
    async fn transport_error_retries_then_host_call_failed() {
        let asserter = Asserter::new();
        // Every attempt is a non-revert RPC error → exhaust retries → HostCallFailed.
        asserter.push_failure_msg("rate limit exceeded");
        asserter.push_failure_msg("rate limit exceeded");
        asserter.push_failure_msg("rate limit exceeded");

        let err = checker(asserter)
            .verify(&unified_request(Bytes::from(vec![0x11; 65])))
            .await
            .unwrap_err();
        assert!(matches!(err, SigPreCheckError::HostCallFailed(_)));
    }

    #[tokio::test]
    async fn non_unified_request_is_skipped() {
        // Legacy variants flow through the v2 handler; verify() must be a no-op for them.
        let request = UserDecryptRequest::LegacyDirect {
            ct_handle_contract_pairs: vec![],
            request_validity: crate::core::event::RequestValidity {
                start_timestamp: U256::ZERO,
                duration_days: U256::ZERO,
            },
            contracts_chain_id: TEST_CHAIN_ID,
            contract_addresses: vec![],
            user_address: Address::ZERO,
            signature: Bytes::new(),
            public_key: Bytes::new(),
            extra_data: Bytes::new(),
        };
        checker(Asserter::new()).verify(&request).await.unwrap();
    }

    #[test]
    fn single_chain_id_detects_mismatch() {
        let h1 = handle_for_chain(8009);
        let h2 = handle_for_chain(9000);
        assert!(single_chain_id([&h1, &h2]).is_err());
        assert_eq!(single_chain_id([&h1, &h1]).unwrap(), 8009);
        assert!(single_chain_id(std::iter::empty()).is_err());
    }
}
