//! Pre-check on v3 user-decryption forwarding: what can be refused before a transaction is paid.
//!
//! One stage, one error contract, one check per arm:
//! - the EIP-712 unified arm recomputes the unified digest and runs the shared verifier
//!   (`user_decryption_signature::verify_signature`) against the host chain — a network check,
//!   with ERC-1271 and retries. The digest's domain is built from the handles' chain, so the
//!   check also ties the signature to that chain;
//! - the host-generic Solana arm verifies the ed25519 permit signature over the reconstructed
//!   envelope, then that the permit names a deployment this relayer serves and handles from its
//!   chain — a pure check, in [`crate::host::solana_permit_prechecker`].
//!
//! Both refuse a detectably bad request before forwarding, returning a specific error to the
//! caller instead of letting the request fail downstream.

use crate::config::settings::{HostChainConfig, RetrySettings};
use crate::core::event::{
    is_solana_host_chain_id, HandleEntry, RequestValiditySeconds, UserDecryptRequest,
};
use crate::host::handle_chain_id::extract_chain_id_from_u256;
use crate::host::solana_permit_prechecker::{verify_solana_permit, SolanaDeployments};
use crate::http::utils::solana_address::decode_solana_address;
use alloy::primitives::{Address, Bytes, U256};
use alloy::providers::{ProviderBuilder, RootProvider};
use fhevm_gateway_bindings::decryption::IDecryption::{
    RequestValiditySeconds as SolRequestValiditySeconds, UserDecryptionRequestPayload,
};
use reqwest::Url;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use tracing::warn;
use user_decryption_signature::{
    compute_user_decrypt_digest, default_user_decrypt_domain, verify_signature, Erc1271Error,
};

/// Who a refused signature claims to be from — one form per arm, for logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreCheckSigner {
    /// EIP-712 arm: the `userAddress` the signature must recover to.
    Evm(Address),
    /// Solana arm: the permit's `userPubkey`.
    Solana(solana_pubkey::Pubkey),
}

impl fmt::Display for PreCheckSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreCheckSigner::Evm(address) => write!(f, "{address}"),
            PreCheckSigner::Solana(pubkey) => write!(f, "{pubkey}"),
        }
    }
}

/// Outcome of a failed pre-check.
#[derive(Debug, thiserror::Error)]
pub enum SigPreCheckError {
    /// The signature is invalid, or reverted without a reason on every attempt — either
    /// way the request must not be forwarded.
    #[error("invalid user-decryption signature for {signer}: {reason}")]
    Invalid {
        signer: PreCheckSigner,
        reason: String,
    },
    /// The permit names a Solana host chain or a zama-host program this relayer does not serve,
    /// or a handle from another host chain than the permit names. The connector refuses the same
    /// request, but only after the gateway fee is paid; refused here instead, keyed by the
    /// payload field at fault, the way a form refusal is.
    #[error("{field}: {issue}")]
    Deployment { field: String, issue: String },
    /// The host-chain call could not complete (transport error after retries). Surfaced as a
    /// server error, mirroring how host ACL call failures are handled.
    #[error("host-chain call failed during signature pre-check: {0}")]
    HostCallFailed(String),
}

/// The EIP-712 unified fields the network check reads, borrowed from the request.
struct Eip712UnifiedFields<'a> {
    handles: &'a [HandleEntry],
    user_address: Address,
    allowed_contracts: &'a [Address],
    request_validity: &'a RequestValiditySeconds,
    signature: &'a Bytes,
    public_key: &'a Bytes,
    extra_data: &'a Bytes,
}

/// The pre-check stage of the v3 endpoint: dispatches each request to its arm's check.
///
/// The EIP-712 arm's state is one read-only provider per EVM host chain, the verifying
/// contract, the ERC-1271 gas cap and the retry policy. The Solana arm's state is the zama-host
/// program per Solana host chain; its check is otherwise a pure function over the request bytes.
pub struct UserDecryptSignaturePreChecker {
    /// One read-only provider per EVM host chain, keyed by chain id.
    providers: HashMap<u64, RootProvider>,
    /// The zama-host program per Solana host chain, keyed by chain id.
    solana_deployments: SolanaDeployments,
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
        let mut solana_deployments = SolanaDeployments::new();
        for hc in host_chains {
            // A Solana host chain has no EIP-712 verifier to dial; what the Solana arm needs
            // from its entry is the zama-host program (`acl_address`) the permits must name.
            // Settings validation already ties the type byte to the address encoding; decoding
            // here keeps the constructor fail-closed for direct callers too.
            if is_solana_host_chain_id(hc.chain_id) {
                let program_id = decode_solana_address(&hc.acl_address).map_err(|e| {
                    anyhow::anyhow!(
                        "Invalid Solana ACL address for chain {}: {}: {e}",
                        hc.chain_id,
                        hc.acl_address
                    )
                })?;
                solana_deployments.insert(hc.chain_id, program_id);
                continue;
            }
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
            solana_deployments,
            decryption_contract,
            erc1271_gas_limit,
            retry,
        })
    }

    /// Runs the arm's check on a v3 request.
    ///
    /// The EIP-712 unified arm goes to the host chain, retrying transport failures and reasonless
    /// reverts like the host ACL checks do and rejecting on definitive verification failures. The
    /// Solana arm is verified locally: the signature, then the deployment the permit names. The
    /// legacy variants are served by the v2 endpoint, which has no pre-check, and cannot arrive
    /// here from the v3 envelope; they pass unchecked.
    pub async fn verify(&self, request: &UserDecryptRequest) -> Result<(), SigPreCheckError> {
        match request {
            UserDecryptRequest::Eip712UnifiedV1 {
                handles,
                user_address,
                allowed_contracts,
                request_validity,
                signature,
                public_key,
                extra_data,
            } => {
                self.verify_eip712(Eip712UnifiedFields {
                    handles,
                    user_address: *user_address,
                    allowed_contracts,
                    request_validity,
                    signature,
                    public_key,
                    extra_data,
                })
                .await
            }
            UserDecryptRequest::SolanaSrfc38V1 {
                ct_handles,
                solana_request,
                ..
            } => verify_solana_permit(solana_request, ct_handles, &self.solana_deployments),
            UserDecryptRequest::LegacyDirect { .. }
            | UserDecryptRequest::LegacyDelegated { .. } => Ok(()),
        }
    }

    /// Recomputes the unified EIP-712 digest and verifies it via the shared RFC-012 helper.
    async fn verify_eip712(&self, fields: Eip712UnifiedFields<'_>) -> Result<(), SigPreCheckError> {
        let Eip712UnifiedFields {
            handles,
            user_address,
            allowed_contracts,
            request_validity,
            signature,
            public_key,
            extra_data,
        } = fields;

        let chain_id = single_chain_id(handles.iter().map(|h| &h.ct_handle), user_address)?;
        let provider = self.providers.get(&chain_id).ok_or_else(|| {
            SigPreCheckError::HostCallFailed(format!("no provider configured for chain {chain_id}"))
        })?;

        let domain = default_user_decrypt_domain(chain_id, self.decryption_contract);
        let payload = UserDecryptionRequestPayload {
            userAddress: user_address,
            publicKey: public_key.clone(),
            allowedContracts: allowed_contracts.to_vec(),
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
        let mut last_retryable: Option<Erc1271Error> = None;

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
                // Retry what is not proof of a bad signature: transport failures, and
                // reasonless reverts, which an under-gassed call produces too.
                Err(e @ (Erc1271Error::Transport(_) | Erc1271Error::EmptyRevert(_))) => {
                    if attempt + 1 < max_attempts {
                        warn!(
                            signer = %user_address,
                            chain_id,
                            attempt = attempt + 1,
                            max_attempts,
                            error = %e,
                            "Signature pre-check RPC failed, retrying"
                        );
                        tokio::time::sleep(interval).await;
                    }
                    last_retryable = Some(e);
                }
                // Every other variant is a definitive rejection (ecrecover mismatch, wrong/empty
                // ERC-1271 magic, reasoned revert, short returndata). The Display encodes the path.
                Err(e) => {
                    return Err(SigPreCheckError::Invalid {
                        signer: PreCheckSigner::Evm(user_address),
                        reason: e.to_string(),
                    });
                }
            }
        }

        match last_retryable {
            // Survived every attempt: no longer plausibly transient, so answer as a
            // rejection rather than a server error.
            Some(e @ Erc1271Error::EmptyRevert(_)) => Err(SigPreCheckError::Invalid {
                signer: PreCheckSigner::Evm(user_address),
                reason: e.to_string(),
            }),
            Some(e) => Err(SigPreCheckError::HostCallFailed(e.to_string())),
            None => Err(SigPreCheckError::HostCallFailed(
                "signature pre-check exhausted its attempts without a result".to_string(),
            )),
        }
    }
}

/// All handles in a unified request share one host chain (the EIP-712 domain carries a single
/// `contractsChainId`). Returns that chain id, erroring if handles are empty or span chains.
fn single_chain_id<'a, I>(handles: I, signer: Address) -> Result<u64, SigPreCheckError>
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
                    signer: PreCheckSigner::Evm(signer),
                    reason: format!("handles span multiple host chains ({seen} and {id})"),
                });
            }
            _ => {}
        }
    }
    chain_id.ok_or_else(|| SigPreCheckError::Invalid {
        signer: PreCheckSigner::Evm(signer),
        reason: "request has no handles".to_string(),
    })
}

#[cfg(test)]
mod tests {
    // The `verify_signature` outcomes (ecrecover / ERC-1271 magic / wrong magic / short
    // returndata / empty signature) are covered by the `user-decryption-signature` crate, the
    // ed25519 rules (canonical pubkey, small-order keys, envelope text) by `zama-solana-permit`,
    // and the accept/reject paths end-to-end by `tests/user_decrypt_v3_test.rs`. These cover the
    // glue this module adds: the dispatch by arm, chain-id handling, the Solana deployment tie,
    // the legacy no-op, and the transport-retry-then-`HostCallFailed` mapping.
    use super::*;
    use crate::core::event::{solana_host_chain_id, HandleEntry, RequestValiditySeconds};
    use alloy::primitives::Bytes;
    use alloy::providers::mock::Asserter;
    use ed25519_dalek::{Signer, SigningKey};
    use zama_solana_permit::{build_envelope, PermitFields, PermitWireFields};
    use zama_solana_request::{
        encode_solana_request, SolanaHandleEntryWire, SolanaUserDecryptRequestWire,
    };

    const TEST_CHAIN_ID: u64 = 8009;
    /// The Solana host chain the checker under test serves, and the zama-host program on it.
    const SOLANA_TEST_CHAIN_ID: u64 = solana_host_chain_id(1);
    const SOLANA_TEST_PROGRAM_ID: [u8; 32] = [0x02; 32];

    fn handle_bytes_for_chain(chain_id: u64) -> [u8; 32] {
        let mut bytes = [0x11u8; 32];
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        bytes
    }

    fn handle_for_chain(chain_id: u64) -> U256 {
        U256::from_be_bytes(handle_bytes_for_chain(chain_id))
    }

    /// A node's reasonless-revert response: "execution reverted" with empty `data`.
    fn empty_revert() -> alloy::rpc::json_rpc::ErrorPayload {
        alloy::rpc::json_rpc::ErrorPayload::internal_error_with_message_and_obj(
            std::borrow::Cow::Borrowed("execution reverted"),
            serde_json::value::RawValue::from_string("\"0x\"".to_string()).unwrap(),
        )
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
            solana_deployments: HashMap::from([(SOLANA_TEST_CHAIN_ID, SOLANA_TEST_PROGRAM_ID)]),
            decryption_contract: Address::from([0xCA; 20]),
            erc1271_gas_limit: 250_000,
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

    /// The wallet whose pubkey the Solana fixture permit names.
    fn solana_wallet() -> SigningKey {
        SigningKey::from_bytes(&[0x42; 32])
    }

    /// A permit in transport form for the given deployment: every field shaped to pass the
    /// typed decode.
    fn solana_permit_wire_for(chain_id: u64, program_id: [u8; 32]) -> PermitWireFields {
        let mut extra_data = vec![0x02u8];
        extra_data.extend_from_slice(&[0u8; 64]);
        PermitWireFields {
            user_pubkey: solana_wallet().verifying_key().to_bytes().to_vec(),
            transport_key: vec![0u8; 869],
            allowed_scopes: vec![[[0x05u8; 32], [0x06u8; 32]].concat()],
            start_timestamp: 1_700_000_000,
            duration_seconds: 604_800,
            verifying_program_id: program_id.to_vec(),
            chain_id,
            extra_data,
        }
    }

    /// The fixture permit: for the deployment the checker under test serves.
    fn solana_permit_wire() -> PermitWireFields {
        solana_permit_wire_for(SOLANA_TEST_CHAIN_ID, SOLANA_TEST_PROGRAM_ID)
    }

    /// A permit's envelope, signed by `signer` — who need not be the permit's user.
    fn solana_signature_over(permit: &PermitWireFields, signer: &SigningKey) -> Vec<u8> {
        let fields = PermitFields::decode(permit).expect("fixture permit decodes");
        signer.sign(&build_envelope(&fields)).to_bytes().to_vec()
    }

    /// The fixture permit's envelope, signed by `signer`.
    fn solana_signature_by(signer: &SigningKey) -> Vec<u8> {
        solana_signature_over(&solana_permit_wire(), signer)
    }

    /// A host-generic request carrying `permit`, one direct entry on `handle`, and the given
    /// signature bytes.
    fn solana_request_with(
        permit: PermitWireFields,
        handle: [u8; 32],
        signature: Vec<u8>,
    ) -> UserDecryptRequest {
        let wire = SolanaUserDecryptRequestWire {
            handles: vec![SolanaHandleEntryWire {
                handle: handle.to_vec(),
                allowed_key: permit.user_pubkey.clone(),
                encrypted_store: vec![0x22; 32],
            }],
            permit,
            signature,
        };
        let blob = encode_solana_request(&wire).expect("fixture request encodes");
        UserDecryptRequest::SolanaSrfc38V1 {
            ct_handles: vec![U256::from_be_bytes(handle)],
            request_validity: RequestValiditySeconds {
                start_timestamp: U256::from(1_700_000_000u64),
                duration_seconds: U256::from(604_800u64),
            },
            public_key: Bytes::new(),
            extra_data: Bytes::new(),
            solana_request: Bytes::from(blob),
        }
    }

    /// A host-generic request carrying the fixture permit, a handle from its chain, and the
    /// given signature bytes.
    fn solana_request(signature: Vec<u8>) -> UserDecryptRequest {
        solana_request_with(
            solana_permit_wire(),
            handle_bytes_for_chain(SOLANA_TEST_CHAIN_ID),
            signature,
        )
    }

    /// The field a deployment refusal names, or a panic naming what came instead.
    fn deployment_field(err: SigPreCheckError) -> String {
        match err {
            SigPreCheckError::Deployment { field, .. } => field,
            other => panic!("expected Deployment, got {other:?}"),
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
    async fn intermittent_empty_revert_is_retried() {
        let asserter = Asserter::new();
        // Attempt 1 reverts with no reason; attempt 2 succeeds.
        asserter.push_failure(empty_revert());
        let mut returndata = [0u8; 32];
        returndata[..4].copy_from_slice(&[0x16, 0x26, 0xba, 0x7e]);
        asserter.push_success(&returndata);

        checker(asserter)
            .verify(&unified_request(Bytes::from(vec![0x11; 65])))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn persistent_empty_revert_is_invalid_not_server_error() {
        let asserter = Asserter::new();
        // Three attempts, all reasonless reverts: no longer plausibly transient.
        for _ in 0..3 {
            asserter.push_failure(empty_revert());
        }

        let err = checker(asserter)
            .verify(&unified_request(Bytes::from(vec![0x11; 65])))
            .await
            .unwrap_err();
        assert!(matches!(err, SigPreCheckError::Invalid { .. }), "{err:?}");
    }

    #[tokio::test]
    async fn legacy_request_is_skipped() {
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

    /// The Solana arm never touches a provider: the mock asserter is left empty on purpose, so
    /// any RPC call would fail the test.
    #[tokio::test]
    async fn solana_permit_signed_by_its_user_is_accepted() {
        let request = solana_request(solana_signature_by(&solana_wallet()));
        checker(Asserter::new()).verify(&request).await.unwrap();
    }

    #[tokio::test]
    async fn solana_permit_signed_by_another_wallet_is_invalid() {
        // A genuine ed25519 signature over the very same envelope, from a key that is not the
        // permit's user: the bytes are well formed, the consent is not the named user's.
        let stranger = SigningKey::from_bytes(&[0x99; 32]);
        let request = solana_request(solana_signature_by(&stranger));

        let err = checker(Asserter::new()).verify(&request).await.unwrap_err();
        let SigPreCheckError::Invalid { signer, reason } = err else {
            panic!("expected Invalid, got {err:?}");
        };
        assert_eq!(
            signer,
            PreCheckSigner::Solana(solana_pubkey::Pubkey::new_from_array(
                solana_wallet().verifying_key().to_bytes()
            )),
            "the refusal names the permit's user, not the stranger"
        );
        assert!(reason.contains("does not verify"), "got: {reason}");
    }

    #[tokio::test]
    async fn solana_signature_of_the_wrong_length_is_invalid() {
        // One byte short of an ed25519 signature: refused as a bad signature, like a
        // wrong-length EIP-712 signature is on the other arm.
        let request = solana_request(vec![0x11; 63]);
        let err = checker(Asserter::new()).verify(&request).await.unwrap_err();
        assert!(matches!(err, SigPreCheckError::Invalid { .. }), "{err:?}");
    }

    #[tokio::test]
    async fn unreadable_solana_blob_passes_as_the_relayers_own_defect() {
        // A blob admission could not have produced: unknown version byte. Not a client fault the
        // stage can attribute, so it warns and passes; the connector's check is authoritative.
        let request = UserDecryptRequest::SolanaSrfc38V1 {
            ct_handles: vec![],
            request_validity: RequestValiditySeconds {
                start_timestamp: U256::ZERO,
                duration_seconds: U256::ZERO,
            },
            public_key: Bytes::new(),
            extra_data: Bytes::new(),
            solana_request: Bytes::from(vec![0xEE, 0x01, 0x02, 0x03]),
        };
        checker(Asserter::new()).verify(&request).await.unwrap();
    }

    /// A permit for a Solana host chain this relayer has no entry for. The handle is from the
    /// served chain, so the early handle gate lets the request through; only the permit says
    /// otherwise.
    #[tokio::test]
    async fn solana_permit_for_an_unserved_chain_is_refused_on_chain_id() {
        let permit = solana_permit_wire_for(solana_host_chain_id(2), SOLANA_TEST_PROGRAM_ID);
        let signature = solana_signature_over(&permit, &solana_wallet());
        let request = solana_request_with(
            permit,
            handle_bytes_for_chain(SOLANA_TEST_CHAIN_ID),
            signature,
        );

        let err = checker(Asserter::new()).verify(&request).await.unwrap_err();
        assert_eq!(deployment_field(err), "chainId");
    }

    /// A permit for the served chain but another zama-host program on it: a signature the
    /// connector would refuse under its own deployment identity, so refused here.
    #[tokio::test]
    async fn solana_permit_for_another_program_is_refused_on_program_id() {
        let permit = solana_permit_wire_for(SOLANA_TEST_CHAIN_ID, [0x03; 32]);
        let signature = solana_signature_over(&permit, &solana_wallet());
        let request = solana_request_with(
            permit,
            handle_bytes_for_chain(SOLANA_TEST_CHAIN_ID),
            signature,
        );

        let err = checker(Asserter::new()).verify(&request).await.unwrap_err();
        assert_eq!(deployment_field(err), "verifyingProgramId");
    }

    /// The permit names the served deployment; the handle was written on another chain. The
    /// EVM arm cannot get here (its signature is verified under the handles' chain); the Solana
    /// arm needs the explicit tie.
    #[tokio::test]
    async fn solana_handle_from_another_chain_than_the_permit_is_refused_on_the_handle() {
        let request = solana_request_with(
            solana_permit_wire(),
            handle_bytes_for_chain(TEST_CHAIN_ID),
            solana_signature_by(&solana_wallet()),
        );

        let err = checker(Asserter::new()).verify(&request).await.unwrap_err();
        let SigPreCheckError::Deployment { field, issue } = err else {
            panic!("expected Deployment, got {err:?}");
        };
        assert_eq!(field, "handles[0].handle");
        assert!(
            issue.contains(&TEST_CHAIN_ID.to_string())
                && issue.contains(&SOLANA_TEST_CHAIN_ID.to_string()),
            "the refusal names both chains: {issue}"
        );
    }

    /// The signature is judged before the deployment: a forged permit for the wrong chain is
    /// reported as forged, not as misaddressed.
    #[tokio::test]
    async fn solana_signature_is_checked_before_the_deployment() {
        let permit = solana_permit_wire_for(solana_host_chain_id(2), SOLANA_TEST_PROGRAM_ID);
        let stranger = SigningKey::from_bytes(&[0x99; 32]);
        let signature = solana_signature_over(&permit, &stranger);
        let request = solana_request_with(
            permit,
            handle_bytes_for_chain(SOLANA_TEST_CHAIN_ID),
            signature,
        );

        let err = checker(Asserter::new()).verify(&request).await.unwrap_err();
        assert!(matches!(err, SigPreCheckError::Invalid { .. }), "{err:?}");
    }

    #[test]
    fn single_chain_id_detects_mismatch() {
        let signer = Address::from([0xDE; 20]);
        let h1 = handle_for_chain(8009);
        let h2 = handle_for_chain(9000);
        assert!(single_chain_id([&h1, &h2], signer).is_err());
        assert_eq!(single_chain_id([&h1, &h1], signer).unwrap(), 8009);
        assert!(single_chain_id(std::iter::empty(), signer).is_err());
    }
}
