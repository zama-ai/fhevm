//! Ciphertext readiness: whether a decryption request's ciphertext material may be decrypted.
//!
//! Two sources are selectable via `gw_ciphertext_check.source`: `gateway_chain` asks the Gateway
//! chain directly; `coprocessor_attestations` evaluates Coprocessor attestation consensus
//! off-chain, per handle (RFC 023). Both fail closed — a handle that is not confirmed ready
//! rejects the whole request before any Gateway transaction is sent.

use crate::{
    config::settings::{AppConfigError, GatewayConfig, GwCiphertextCheckConfig, RetrySettings},
    core::{errors::EventProcessingError, event::HandleContractPair, job_id::JobId},
    gateway::arbitrum::bindings::Decryption,
    host::{
        provider::{build_gateway_provider, Provider},
        redact_alloy_error,
    },
    readiness::ReadinessCheckError,
};
use alloy::{
    network::AnyNetwork,
    primitives::{Address, Bytes, FixedBytes},
    transports::http::Client,
};
use ciphertext_attestation::{
    fetch_attestations_and_check_consensus, BoundedClient, ConsensusCheckError,
    CoprocessorRegistry, COPROCESSOR_CONTEXT_ID_V1,
};
use fhevm_gateway_bindings::decryption::Decryption::DecryptionInstance;
use futures::stream::{self, StreamExt};
use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::readiness::ReadinessStep;

type GatewayDecryption = DecryptionInstance<Arc<Provider>, AnyNetwork>;

/// Checks whether ciphertext material is ready for decryption. One variant per
/// `gw_ciphertext_check.source`; each keeps its own state, and only the public interface is
/// shared.
pub enum CiphertextChecker {
    GatewayChain(GatewayChainCheck),
    CoprocessorAttestations(CoprocessorAttestationCheck),
}

impl CiphertextChecker {
    pub async fn new(
        gateway_config: &GatewayConfig,
        cancel_token: CancellationToken,
    ) -> Result<Self, EventProcessingError> {
        match &gateway_config.readiness_checker.gw_ciphertext_check {
            // The on-chain Gateway check starts no background task, so it has nothing to cancel;
            // only the Coprocessor registry's refresh task needs the shutdown token.
            GwCiphertextCheckConfig::GatewayChain { retry } => Ok(Self::GatewayChain(
                GatewayChainCheck::new(gateway_config, retry.clone())?,
            )),
            GwCiphertextCheckConfig::CoprocessorAttestations {
                retry,
                head_timeout_ms,
                request_timeout_ms,
                registry_refresh_ms,
                max_concurrent_handles,
                gateway_config_address,
            } => Ok(Self::CoprocessorAttestations(
                CoprocessorAttestationCheck::new(
                    gateway_config,
                    retry.clone(),
                    *head_timeout_ms,
                    *request_timeout_ms,
                    *registry_refresh_ms,
                    *max_concurrent_handles,
                    gateway_config_address,
                    cancel_token,
                )
                .await?,
            )),
        }
    }

    /// Checks public decryption readiness.
    ///
    /// `extra_data` is forwarded to the on-chain Gateway check. The off-chain Coprocessor
    /// attestation check ignores it: the consensus verdict is a property of the ciphertext
    /// material alone.
    pub async fn check_public_decryption_readiness(
        &self,
        job_id: &JobId,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        match self {
            Self::GatewayChain(c) => {
                c.check_public_decryption_readiness(job_id, handles, extra_data)
                    .await
            }
            Self::CoprocessorAttestations(a) => {
                a.check_public_decryption_readiness(job_id, handles).await
            }
        }
    }

    /// Checks user decryption readiness, accepting core `HandleContractPair` types.
    ///
    /// `extra_data` is forwarded to the on-chain Gateway check. The off-chain Coprocessor
    /// attestation check ignores it: the consensus verdict is a property of the ciphertext
    /// material alone.
    pub async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        pairs: &[HandleContractPair],
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        match self {
            Self::GatewayChain(c) => {
                c.check_user_decryption_readiness(job_id, pairs, extra_data)
                    .await
            }
            Self::CoprocessorAttestations(a) => {
                a.check_user_decryption_readiness(job_id, pairs).await
            }
        }
    }
}

/// The on-chain Gateway check: asks the Gateway chain (`isPublicDecryptionReady` /
/// `isUserDecryptionReady_1`).
pub struct GatewayChainCheck {
    retry_config: RetrySettings,
    gw_decryption: GatewayDecryption,
}

impl GatewayChainCheck {
    fn new(
        gateway_config: &GatewayConfig,
        retry: RetrySettings,
    ) -> Result<Self, EventProcessingError> {
        let decryption_address = Address::from_str(&gateway_config.contracts.decryption_address)
            .map_err(|_| {
                EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                    "contracts.decryption_address".to_owned(),
                ))
            })?;

        let provider = build_gateway_provider(&gateway_config.blockchain_rpc.read_http_url)
            .map_err(|e| EventProcessingError::ValidationFailed {
                field: "blockchain_rpc_url".to_string(),
                reason: e.to_string(),
            })?;

        let gw_decryption = Decryption::new(decryption_address, provider);

        Ok(Self {
            retry_config: retry,
            gw_decryption,
        })
    }

    async fn check_public_decryption_readiness(
        &self,
        job_id: &JobId,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        info!(
            step = %ReadinessStep::Started,
            int_job_id = %job_id,
            "Starting public decryption gateway ciphertext check"
        );

        let result = self
            .retry_until_ready(job_id, || {
                let decryption = self.gw_decryption.clone();
                let handles = handles.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isPublicDecryptionReady(handles, extra_data)
                        .call()
                        .await
                }
            })
            .await;

        match &result {
            Ok(()) => info!(
                step = %ReadinessStep::Passed,
                int_job_id = %job_id,
                "Public decryption gateway ciphertext check passed"
            ),
            Err(e) => error!(
                step = %ReadinessStep::Failed,
                int_job_id = %job_id,
                error = ?e,
                "Public decryption gateway ciphertext check failed"
            ),
        }

        result
    }

    /// Check user decryption readiness, accepting core `HandleContractPair` types.
    /// Converts to gateway binding types internally.
    ///
    /// All three request kinds (legacy direct, legacy delegated and unified
    /// EIP-712) route through the same `isUserDecryptionReady_1((bytes32,
    /// address)[], bytes)` overload: the contract only checks that ciphertext
    /// material exists for each handle, so per-pair contract addresses and the
    /// requesting user/delegator address play no role here.
    async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        pairs: &[HandleContractPair],
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        info!(
            step = %ReadinessStep::Started,
            int_job_id = %job_id,
            "Starting user decryption gateway ciphertext check"
        );

        let contract_pairs: Vec<Decryption::CtHandleContractPair> = pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        let result = self
            .retry_until_ready(job_id, || {
                let decryption = self.gw_decryption.clone();
                let pairs = contract_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isUserDecryptionReady_1(pairs, extra_data)
                        .call()
                        .await
                }
            })
            .await;

        match &result {
            Ok(()) => info!(
                step = %ReadinessStep::Passed,
                int_job_id = %job_id,
                "User decryption gateway ciphertext check passed"
            ),
            Err(e) => error!(
                step = %ReadinessStep::Failed,
                int_job_id = %job_id,
                error = ?e,
                "User decryption gateway ciphertext check failed"
            ),
        }

        result
    }

    async fn retry_until_ready<F, Fut>(
        &self,
        job_id: &JobId,
        check_fn: F,
    ) -> Result<(), ReadinessCheckError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<bool, alloy::contract::Error>>,
    {
        let max_retries = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);
        let mut retries = 0;
        let mut last_error: Option<alloy::contract::Error>;

        loop {
            match check_fn().await {
                Ok(is_ready) => {
                    last_error = None;
                    if is_ready {
                        return Ok(());
                    } else {
                        info!(int_job_id = %job_id, "Gateway not ready, will retry");
                    }
                }
                Err(err) => {
                    error!(int_job_id = %job_id, error = %redact_alloy_error(&err), "Contract call failed, will retry");
                    last_error = Some(err);
                }
            }

            retries += 1;
            if retries >= max_retries {
                warn!(
                    int_job_id = %job_id,
                    max_retries = max_retries,
                    retry_interval_ms = self.retry_config.retry_interval_ms,
                    "Max retries reached for readiness check"
                );
                return if let Some(err) = last_error {
                    Err(ReadinessCheckError::GwContractError(err))
                } else {
                    Err(ReadinessCheckError::GwTimeout)
                };
            }

            warn!(
                step = %ReadinessStep::Retrying,
                int_job_id = %job_id,
                attempt = retries,
                max_attempts = max_retries,
                "Retrying readiness check"
            );
            tokio::time::sleep(retry_interval).await;
        }
    }
}

/// The off-chain Coprocessor attestation check: evaluates attestation consensus, per handle
/// (RFC 023).
pub struct CoprocessorAttestationCheck {
    retry_config: RetrySettings,
    registry: CoprocessorRegistry<Arc<Provider>, AnyNetwork>,
    http_client: BoundedClient,
    request_timeout: Duration,
    max_concurrent_handles: NonZeroUsize,
}

impl CoprocessorAttestationCheck {
    /// Mirrors the Coprocessor registry from `GatewayConfig` and starts its refresh task.
    ///
    /// `cancel_token` stops the refresh task and nothing else — a critically failed refresh is
    /// answered per request, see [`Self::check_handles_with_retry`].
    #[allow(clippy::too_many_arguments)]
    async fn new(
        gateway_config: &GatewayConfig,
        retry: RetrySettings,
        head_timeout_ms: u64,
        request_timeout_ms: u64,
        registry_refresh_ms: u64,
        max_concurrent_handles: NonZeroUsize,
        gateway_config_address: &str,
        cancel_token: CancellationToken,
    ) -> Result<Self, EventProcessingError> {
        let gateway_config_address = Address::from_str(gateway_config_address).map_err(|_| {
            EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                "gw_ciphertext_check.gateway_config_address".to_owned(),
            ))
        })?;

        let provider = build_gateway_provider(&gateway_config.blockchain_rpc.read_http_url)
            .map_err(|e| EventProcessingError::ValidationFailed {
                field: "blockchain_rpc_url".to_string(),
                reason: e.to_string(),
            })?;

        let registry = CoprocessorRegistry::connect(
            provider,
            gateway_config_address,
            Duration::from_millis(registry_refresh_ms),
            cancel_token,
        )
        .await
        .map_err(|e| EventProcessingError::ContractCallFailed(e.to_string()))?;

        Ok(Self {
            retry_config: retry,
            registry,
            // A handle issues at most one `HEAD` per bucket, so with `max_concurrent_handles`
            // handles in flight, per-bucket concurrency is that same number — whatever the
            // registry size. The outer bound is therefore the per-bucket ceiling too.
            http_client: BoundedClient::for_attestations_only(
                Client::new(),
                max_concurrent_handles,
                Duration::from_millis(head_timeout_ms),
                COPROCESSOR_CONTEXT_ID_V1,
            ),
            request_timeout: Duration::from_millis(request_timeout_ms),
            max_concurrent_handles,
        })
    }

    async fn check_public_decryption_readiness(
        &self,
        job_id: &JobId,
        handles: Vec<FixedBytes<32>>,
    ) -> Result<(), ReadinessCheckError> {
        info!(
            step = %ReadinessStep::Started,
            int_job_id = %job_id,
            "Starting public decryption ciphertext attestation check"
        );

        let result = self.check_handles_with_retry(job_id, &handles).await;

        match &result {
            Ok(()) => info!(
                step = %ReadinessStep::Passed,
                int_job_id = %job_id,
                "Public decryption ciphertext attestation check passed"
            ),
            Err(e) => error!(
                step = %ReadinessStep::Failed,
                int_job_id = %job_id,
                error = ?e,
                "Public decryption ciphertext attestation check failed"
            ),
        }

        result
    }

    /// Checks user decryption readiness, accepting core `HandleContractPair` types.
    ///
    /// All three request kinds (legacy direct, legacy delegated and unified EIP-712) collapse to
    /// the same per-handle check: consensus is a property of the ciphertext material alone, so the
    /// per-pair contract address and the requesting user or delegator play no role. The same
    /// reasoning is why the on-chain Gateway check routes all three through a single
    /// `isUserDecryptionReady` overload.
    async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        pairs: &[HandleContractPair],
    ) -> Result<(), ReadinessCheckError> {
        info!(
            step = %ReadinessStep::Started,
            int_job_id = %job_id,
            "Starting user decryption ciphertext attestation check"
        );

        let handles: Vec<FixedBytes<32>> = pairs.iter().map(|pair| pair.ct_handle.into()).collect();
        let result = self.check_handles_with_retry(job_id, &handles).await;

        match &result {
            Ok(()) => info!(
                step = %ReadinessStep::Passed,
                int_job_id = %job_id,
                "User decryption ciphertext attestation check passed"
            ),
            Err(e) => error!(
                step = %ReadinessStep::Failed,
                int_job_id = %job_id,
                error = ?e,
                "User decryption ciphertext attestation check failed"
            ),
        }

        result
    }

    /// Retries the whole handle set while attestations are still missing, under an overall
    /// wall-clock budget, and gives up immediately once the Coprocessors have disagreed.
    ///
    /// Two independent limits, whichever is reached first: `retry.max_attempts` bounds how many
    /// times the handle set is re-probed, and `request_timeout_ms` bounds how long that may take in
    /// total. The attempt count alone is not a time bound — an attempt waits out `head_timeout` per
    /// unresponsive bucket, so a hanging Coprocessor would otherwise stretch the nominal budget by
    /// an order of magnitude while holding a readiness throttler permit the whole time.
    async fn check_handles_with_retry(
        &self,
        job_id: &JobId,
        handles: &[FixedBytes<32>],
    ) -> Result<(), ReadinessCheckError> {
        // Before `consensus_reachable`: a snapshot that failed to refresh can still report `true`.
        if self.registry.last_refresh_failed_critically() {
            return Err(ReadinessCheckError::RegistryStale);
        }

        let snapshot = self.registry.snapshot();
        if !snapshot.consensus_reachable() {
            return Err(ReadinessCheckError::ConsensusUnreachable {
                registered: snapshot.coprocessors.len(),
                threshold: snapshot.threshold.get(),
            });
        }

        match tokio::time::timeout(
            self.request_timeout,
            self.retry_while_missing(job_id, handles),
        )
        .await
        {
            Ok(result) => result,
            Err(_elapsed) => {
                // Same verdict as exhausting the attempts — retriable, `readiness_check_timed_out`
                // — but a distinct log line, because the remedy differs: attempts exhausted means
                // the attestations are late, whereas the budget expiring means the probes
                // themselves are slow and a bucket is likely unresponsive.
                warn!(
                    step = %ReadinessStep::Failed,
                    int_job_id = %job_id,
                    request_timeout_ms = self.request_timeout.as_millis(),
                    handles = handles.len(),
                    "Ciphertext attestation check exceeded its overall request budget"
                );
                Err(ReadinessCheckError::GwTimeout)
            }
        }
    }

    /// Re-probes the handle set until it succeeds, becomes unreachable, or runs out of attempts.
    /// Bounded in time only by its caller — see [`Self::check_handles_with_retry`].
    async fn retry_while_missing(
        &self,
        job_id: &JobId,
        handles: &[FixedBytes<32>],
    ) -> Result<(), ReadinessCheckError> {
        let max_attempts = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);
        let mut attempts = 0;
        let started = Instant::now();

        loop {
            match self.require_consensus_once(handles).await {
                Ok(()) => return Ok(()),
                Err(ConsensusCheckError::Unreachable(round)) => {
                    error!(
                        step = %ReadinessStep::Failed,
                        int_job_id = %job_id,
                        handle = %round.handle,
                        ?round,
                        "Coprocessors did not agree on the ciphertext material"
                    );
                    return Err(ReadinessCheckError::NoAttestationConsensus { round });
                }
                Err(ConsensusCheckError::MissedThisRound(round)) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        let elapsed = started.elapsed();
                        error!(
                            int_job_id = %job_id,
                            handle = %round.handle,
                            attempts,
                            elapsed_ms = elapsed.as_millis(),
                            ?round,
                            "Max retries reached for ciphertext attestation check"
                        );
                        return Err(ReadinessCheckError::AttestationsNotReady {
                            attempts,
                            elapsed,
                            last_round: round,
                        });
                    }

                    warn!(
                        step = %ReadinessStep::Retrying,
                        int_job_id = %job_id,
                        attempt = attempts,
                        max_attempts,
                        handle = %round.handle,
                        ?round,
                        "Retrying ciphertext attestation check"
                    );
                    tokio::time::sleep(retry_interval).await;
                }
            }
        }
    }

    /// Evaluates consensus for every handle once. A request is only as ready as its least-ready
    /// handle, so any handle without consensus fails the whole request.
    ///
    /// An `Unreachable` verdict has to win wherever it sits in the set, and short-circuits the
    /// round. The reported `MissedThisRound` is the one belonging to the lowest-indexed failing
    /// handle, so the message does not vary with the order replies happen to arrive in.
    async fn require_consensus_once(
        &self,
        handles: &[FixedBytes<32>],
    ) -> Result<(), ConsensusCheckError> {
        let snapshot = self.registry.snapshot();

        // Handles are copied out of the slice rather than borrowed: a borrow taken from the
        // iterator would tie the closure to one anonymous lifetime, and the resulting future is
        // not general enough for the `run_consumer` callers to hold across an await.
        let mut verdicts = stream::iter(handles.iter().copied().enumerate())
            .map(|(index, handle)| {
                let snapshot = snapshot.clone();
                async move {
                    let outcome = fetch_attestations_and_check_consensus(
                        &self.http_client,
                        handle,
                        &snapshot,
                    )
                    .await;
                    (index, outcome)
                }
            })
            .buffer_unordered(self.max_concurrent_handles.get());

        let mut first_missed: Option<(usize, ConsensusCheckError)> = None;

        while let Some((index, outcome)) = verdicts.next().await {
            match outcome {
                Ok(_) => (),
                Err(e @ ConsensusCheckError::Unreachable { .. }) => return Err(e),
                Err(e @ ConsensusCheckError::MissedThisRound { .. }) => {
                    if first_missed.as_ref().is_none_or(|(at, _)| index < *at) {
                        first_missed = Some((index, e));
                    }
                }
            }
        }

        match first_missed {
            Some((_, e)) => Err(e),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A `GatewayChainCheck` whose `gw_decryption` is never dialed: `retry_until_ready`
    /// only reads `retry_config`, and the retry loop is driven entirely by the injected
    /// `check_fn`, so the provider and address here are placeholders.
    fn test_gateway_chain_check(retry_config: RetrySettings) -> GatewayChainCheck {
        let provider = build_gateway_provider("http://localhost:1").expect("dummy rpc url parses");
        let gw_decryption = Decryption::new(Address::ZERO, provider);
        GatewayChainCheck {
            retry_config,
            gw_decryption,
        }
    }

    fn dummy_contract_error() -> alloy::contract::Error {
        alloy::contract::Error::AbiError(alloy::dyn_abi::Error::SolTypes(
            alloy::sol_types::Error::Other("simulated transient RPC blip".into()),
        ))
    }

    /// A transient RPC error on the first attempt, followed only by clean `Ok(false)` polls
    /// until retries are exhausted, must not surface as a terminal `GwContractError` (HTTP 500).
    /// `last_error` has to be cleared on every `Ok`, otherwise the stale error from the very
    /// first attempt leaks through as the final verdict instead of the retryable `GwTimeout`
    /// (HTTP 503).
    #[tokio::test]
    async fn transient_error_cleared_by_later_ok_yields_gw_timeout() {
        let check = test_gateway_chain_check(RetrySettings {
            max_attempts: 3,
            retry_interval_ms: 0,
        });
        let attempt = AtomicUsize::new(0);

        let result = check
            .retry_until_ready(&JobId::ZERO, || {
                let this_attempt = attempt.fetch_add(1, Ordering::SeqCst);
                async move {
                    if this_attempt == 0 {
                        Err(dummy_contract_error())
                    } else {
                        Ok(false)
                    }
                }
            })
            .await;

        assert!(
            matches!(result, Err(ReadinessCheckError::GwTimeout)),
            "expected GwTimeout, got {result:?}"
        );
    }
}
