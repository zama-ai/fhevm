//! Optional RFC-012 signature pre-check on v3 user-decryption forwarding.
//!
//! Recomputes the unified EIP-712 digest and runs the shared `ecrecover` → ERC-1271 fallback
//! (`user_decryption_signature::verify_signature`) — the same library the KMS Connector uses —
//! so detectably-bad signatures are rejected before the request reaches the gateway. The
//! Connector stays the authoritative verifier; this is a best-effort early reject.

use crate::config::settings::{HostChainConfig, RetrySettings};
use crate::core::event::UserDecryptRequest;
use crate::host::handle_chain_id::extract_chain_id_from_u256;
use alloy::primitives::{Address, Bytes, U256};
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use reqwest::Url;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tracing::warn;
use user_decryption_signature::{
    compute_user_decrypt_digest_from_parts, default_user_decrypt_domain, verify_signature,
    Erc1271Error,
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
    /// pre-check is wired only into the v3 endpoint). Returns `Ok(())` on accept.
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

        let chain_id = single_chain_id(handles.iter().map(|h| &h.ct_handle))?;
        let provider = self.providers.get(&chain_id).ok_or_else(|| {
            SigPreCheckError::HostCallFailed(format!("no provider configured for chain {chain_id}"))
        })?;

        self.verify_fields(
            provider,
            chain_id,
            *user_address,
            public_key,
            allowed_contracts,
            request_validity.start_timestamp,
            request_validity.duration_seconds,
            signature,
            extra_data,
        )
        .await
    }

    /// Compute the digest and run the shared verifier with transport-error retries. Split from
    /// [`verify`] so tests can drive it with a mock provider.
    #[allow(clippy::too_many_arguments)]
    async fn verify_fields<P: Provider>(
        &self,
        provider: &P,
        chain_id: u64,
        user_address: Address,
        public_key: &Bytes,
        allowed_contracts: &[Address],
        start_timestamp: U256,
        duration_seconds: U256,
        signature: &Bytes,
        extra_data: &Bytes,
    ) -> Result<(), SigPreCheckError> {
        let domain = default_user_decrypt_domain(chain_id, self.decryption_contract);
        let digest = compute_user_decrypt_digest_from_parts(
            user_address,
            public_key,
            allowed_contracts,
            start_timestamp,
            duration_seconds,
            extra_data,
            &domain,
        );

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
    use super::*;
    use crate::core::event::{HandleEntry, RequestValiditySeconds};
    use alloy::providers::mock::Asserter;
    use alloy::signers::{local::PrivateKeySigner, SignerSync};
    use user_decryption_signature::ERC1271_MAGIC_VALUE;

    const TEST_CHAIN_ID: u64 = 8009;
    const GAS_LIMIT: u64 = 100_000;

    fn handle_for_chain(chain_id: u64) -> U256 {
        let mut bytes = [0u8; 32];
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        U256::from_be_bytes(bytes)
    }

    fn decryption_contract() -> Address {
        Address::from([0xCA; 20])
    }

    fn prechecker() -> UserDecryptSignaturePreChecker {
        UserDecryptSignaturePreChecker {
            providers: HashMap::new(),
            decryption_contract: decryption_contract(),
            erc1271_gas_limit: GAS_LIMIT,
            retry: RetrySettings {
                max_attempts: 3,
                retry_interval_ms: 0,
            },
        }
    }

    fn mock_provider(asserter: Asserter) -> RootProvider {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter)
    }

    /// Build a request whose signature is a valid EOA signature over the recomputed digest.
    fn signed_request(
        signer: &PrivateKeySigner,
        owner: Address,
    ) -> (Address, Bytes, Vec<Address>, U256, U256, Bytes, Bytes) {
        let user_address = signer.address();
        let public_key = Bytes::from(vec![1, 2, 3, 4]);
        let allowed_contracts = vec![Address::from([0xAB; 20])];
        let start = U256::from(1_700_000_000u64);
        let duration = U256::from(86_400u64);
        let extra_data = Bytes::from(vec![0x00]);

        let domain = default_user_decrypt_domain(TEST_CHAIN_ID, decryption_contract());
        let digest = compute_user_decrypt_digest_from_parts(
            user_address,
            &public_key,
            &allowed_contracts,
            start,
            duration,
            &extra_data,
            &domain,
        );
        let sig = signer.sign_hash_sync(&digest).unwrap();
        let signature = Bytes::from(sig.as_bytes().to_vec());
        let _ = owner;
        (
            user_address,
            public_key,
            allowed_contracts,
            start,
            duration,
            signature,
            extra_data,
        )
    }

    #[tokio::test]
    async fn eoa_valid_signature_accepts_without_rpc() {
        let signer = PrivateKeySigner::random();
        let (user, pk, allowed, start, dur, sig, extra) = signed_request(&signer, signer.address());
        let provider = mock_provider(Asserter::new());

        prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                user,
                &pk,
                &allowed,
                start,
                dur,
                &sig,
                &extra,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn erc1271_magic_accepts() {
        let user = Address::from([0xDE; 20]);
        let mut returndata = [0u8; 32];
        returndata[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        let asserter = Asserter::new();
        asserter.push_success(&returndata);
        let provider = mock_provider(asserter);

        prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                user,
                &Bytes::new(),
                &[],
                U256::ZERO,
                U256::ZERO,
                &Bytes::from(vec![0x11; 65]),
                &Bytes::new(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn erc1271_wrong_magic_rejects() {
        let asserter = Asserter::new();
        asserter.push_success(&[0u8; 32]);
        let provider = mock_provider(asserter);

        let err = prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                Address::from([0xDE; 20]),
                &Bytes::new(),
                &[],
                U256::ZERO,
                U256::ZERO,
                &Bytes::from(vec![0x11; 65]),
                &Bytes::new(),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, SigPreCheckError::Invalid { .. }));
    }

    #[tokio::test]
    async fn empty_sig_smart_account_accepts() {
        let mut returndata = [0u8; 32];
        returndata[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        let asserter = Asserter::new();
        asserter.push_success(&returndata);
        let provider = mock_provider(asserter);

        prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                Address::from([0xDE; 20]),
                &Bytes::new(),
                &[],
                U256::ZERO,
                U256::ZERO,
                &Bytes::new(),
                &Bytes::new(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn empty_sig_on_eoa_rejects() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::default());
        let provider = mock_provider(asserter);

        let err = prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                Address::from([0xDE; 20]),
                &Bytes::new(),
                &[],
                U256::ZERO,
                U256::ZERO,
                &Bytes::new(),
                &Bytes::new(),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, SigPreCheckError::Invalid { .. }));
    }

    #[tokio::test]
    async fn short_returndata_rejects() {
        let asserter = Asserter::new();
        asserter.push_success(&ERC1271_MAGIC_VALUE); // only 4 bytes
        let provider = mock_provider(asserter);

        let err = prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                Address::from([0xDE; 20]),
                &Bytes::new(),
                &[],
                U256::ZERO,
                U256::ZERO,
                &Bytes::from(vec![0x11; 65]),
                &Bytes::new(),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, SigPreCheckError::Invalid { .. }));
    }

    #[tokio::test]
    async fn transport_error_retries_then_host_call_failed() {
        let asserter = Asserter::new();
        // 3 attempts, each a non-revert RPC error → exhaust retries → HostCallFailed.
        asserter.push_failure_msg("rate limit exceeded");
        asserter.push_failure_msg("rate limit exceeded");
        asserter.push_failure_msg("rate limit exceeded");
        let provider = mock_provider(asserter);

        let err = prechecker()
            .verify_fields(
                &provider,
                TEST_CHAIN_ID,
                Address::from([0xDE; 20]),
                &Bytes::new(),
                &[],
                U256::ZERO,
                U256::ZERO,
                &Bytes::from(vec![0x11; 65]),
                &Bytes::new(),
            )
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
        prechecker().verify(&request).await.unwrap();
    }

    #[test]
    fn single_chain_id_detects_mismatch() {
        let h1 = handle_for_chain(8009);
        let h2 = handle_for_chain(9000);
        assert!(single_chain_id([&h1, &h2]).is_err());
        assert_eq!(single_chain_id([&h1, &h1]).unwrap(), 8009);
        assert!(single_chain_id(std::iter::empty()).is_err());
    }

    #[tokio::test]
    async fn verify_extracts_chain_id_and_uses_provider() {
        // End-to-end through `verify`: build a unified request and route it to the mocked
        // provider registered for the handle's chain id.
        let signer = PrivateKeySigner::random();
        let (user, pk, allowed, start, dur, sig, extra) = signed_request(&signer, signer.address());

        let mut checker = prechecker();
        checker
            .providers
            .insert(TEST_CHAIN_ID, mock_provider(Asserter::new()));

        let request = UserDecryptRequest::Eip712UnifiedV1 {
            handles: vec![HandleEntry {
                ct_handle: handle_for_chain(TEST_CHAIN_ID),
                contract_address: Address::from([0xAB; 20]),
                owner_address: user,
            }],
            user_address: user,
            allowed_contracts: allowed,
            request_validity: RequestValiditySeconds {
                start_timestamp: start,
                duration_seconds: dur,
            },
            signature: sig,
            public_key: pk,
            extra_data: extra,
        };

        checker.verify(&request).await.unwrap();
    }
}
