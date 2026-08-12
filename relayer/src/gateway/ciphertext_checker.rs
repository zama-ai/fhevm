//! Off-chain ciphertext readiness: per-handle Coprocessor attestation consensus.
//!
//! Before a decryption request is forwarded to the Gateway, every handle it names must be backed
//! by ciphertext material that a majority of registered Coprocessors attest to. That verdict is
//! computed off-chain (RFC 023): the Coprocessor registry is mirrored from `GatewayConfig`, each
//! bucket is probed with an S3 `HEAD` request, and the attestations are validated and counted
//! against the on-chain majority threshold.
//!
//! The check fails closed — a handle without consensus rejects the whole request before any
//! Gateway transaction is sent.

use crate::{
    config::settings::{AppConfigError, GatewayConfig, RetrySettings},
    core::{errors::EventProcessingError, event::HandleContractPair, job_id::JobId},
    host::provider::{build_gateway_provider, Provider},
    readiness::ReadinessCheckError,
};
use alloy::{
    network::AnyNetwork,
    primitives::{Address, FixedBytes},
    transports::http::Client,
};
use ciphertext_attestation_client::{
    fetch_attestations_and_check_consensus, ConsensusCheckError, CoprocessorRegistry,
    COPROCESSOR_CONTEXT_ID_V1,
};
use futures::stream::{self, StreamExt};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::readiness::ReadinessStep;

/// Handles probed concurrently within one attempt.
///
/// Each handle already fans out one `HEAD` per Coprocessor bucket, so requests in flight peak at
/// this many times the registry size. Bounded rather than unbounded so a request naming a large
/// handle set cannot spike that product, and fixed rather than configurable to keep the
/// `gw_ciphertext_check` subtree to the knobs an operator actually tunes.
const MAX_CONCURRENT_HANDLES: usize = 8;

/// Checks that every handle has Coprocessor attestation consensus.
pub struct CiphertextChecker {
    retry_config: RetrySettings,
    registry: CoprocessorRegistry<Arc<Provider>, AnyNetwork>,
    http_client: Client,
    head_timeout: Duration,
    request_timeout: Duration,
}

impl CiphertextChecker {
    /// Mirrors the Coprocessor registry from `GatewayConfig` and starts its refresh task.
    ///
    /// `cancel_token` is the relayer-wide shutdown token: the registry cancels it if a refresh
    /// hits a condition no healthy protocol can produce (e.g. an invalid on-chain threshold),
    /// since continuing would mean gating decryptions on a registry known to be wrong.
    pub async fn new(
        gateway_config: &GatewayConfig,
        cancel_token: CancellationToken,
    ) -> Result<Self, EventProcessingError> {
        let gateway_config_address =
            Address::from_str(&gateway_config.contracts.gateway_config_address).map_err(|_| {
                EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                    "contracts.gateway_config_address".to_owned(),
                ))
            })?;

        let provider = build_gateway_provider(&gateway_config.blockchain_rpc.read_http_url)
            .map_err(|e| EventProcessingError::ValidationFailed {
                field: "blockchain_rpc_url".to_string(),
                reason: e.to_string(),
            })?;

        let check_config = &gateway_config.readiness_checker.gw_ciphertext_check;
        let registry = CoprocessorRegistry::connect(
            provider,
            gateway_config_address,
            Duration::from_millis(check_config.registry_refresh_ms),
            cancel_token,
        )
        .await
        .map_err(|e| EventProcessingError::ContractCallFailed(e.to_string()))?;

        Ok(Self {
            retry_config: check_config.retry.clone(),
            registry,
            http_client: Client::new(),
            head_timeout: Duration::from_millis(check_config.head_timeout_ms),
            request_timeout: Duration::from_millis(check_config.request_timeout_ms),
        })
    }

    pub async fn check_public_decryption_readiness(
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
    /// reasoning is why the on-chain path this replaces routed all three through a single
    /// `isUserDecryptionReady` overload.
    pub async fn check_user_decryption_readiness(
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

    /// Retries the whole handle set while attestations are starved, under an overall wall-clock
    /// budget, and gives up immediately once the Coprocessors have split.
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
        match tokio::time::timeout(
            self.request_timeout,
            self.retry_while_starved(job_id, handles),
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

    /// Re-probes the handle set until it succeeds, splits, or runs out of attempts. Bounded in time
    /// only by its caller — see [`Self::check_handles_with_retry`].
    async fn retry_while_starved(
        &self,
        job_id: &JobId,
        handles: &[FixedBytes<32>],
    ) -> Result<(), ReadinessCheckError> {
        let max_attempts = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);
        let mut attempts = 0;

        loop {
            match self.check_handles_once(handles).await {
                Ok(()) => return Ok(()),
                Err(e @ ConsensusCheckError::Split { .. }) => {
                    error!(
                        step = %ReadinessStep::Failed,
                        int_job_id = %job_id,
                        "Coprocessors did not agree on the ciphertext material: {e}"
                    );
                    return Err(ReadinessCheckError::NoAttestationConsensus(e.to_string()));
                }
                Err(e @ ConsensusCheckError::Starved { .. }) => {
                    info!(int_job_id = %job_id, "Ciphertext not attested yet: {e}");

                    attempts += 1;
                    if attempts >= max_attempts {
                        warn!(
                            int_job_id = %job_id,
                            max_attempts,
                            retry_interval_ms = self.retry_config.retry_interval_ms,
                            "Max retries reached for ciphertext attestation check"
                        );
                        return Err(ReadinessCheckError::GwTimeout);
                    }

                    warn!(
                        step = %ReadinessStep::Retrying,
                        int_job_id = %job_id,
                        attempt = attempts,
                        max_attempts,
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
    /// A failing handle does not end the round, because the *kind* of failure decides
    /// retriable-vs-terminal and a `Split` has to win wherever it sits in the set. Reporting the
    /// first failure seen would hide a split handle behind a retry budget that can never satisfy it,
    /// and would make the user-facing verdict depend on which handle happened to fail first — the
    /// same request reported as a 503 timeout or a 500 no-consensus depending on position or timing.
    /// A `Split` short-circuits the round: it is terminal, so there is nothing left to learn.
    ///
    /// Handles are probed concurrently, up to [`MAX_CONCURRENT_HANDLES`] at a time, so an attempt
    /// costs about one `head_timeout` rather than one per handle. The reported `Starved` is the one
    /// belonging to the lowest-indexed failing handle, so the message does not vary with the order
    /// replies happen to arrive in.
    async fn check_handles_once(
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
                        self.head_timeout,
                        COPROCESSOR_CONTEXT_ID_V1,
                    )
                    .await;
                    (index, outcome)
                }
            })
            .buffer_unordered(MAX_CONCURRENT_HANDLES);

        let mut first_starved: Option<(usize, ConsensusCheckError)> = None;

        while let Some((index, outcome)) = verdicts.next().await {
            match outcome {
                Ok(_) => (),
                Err(e @ ConsensusCheckError::Split { .. }) => return Err(e),
                Err(e @ ConsensusCheckError::Starved { .. }) => {
                    if first_starved.as_ref().is_none_or(|(at, _)| index < *at) {
                        first_starved = Some((index, e));
                    }
                }
            }
        }

        match first_starved {
            Some((_, e)) => Err(e),
            None => Ok(()),
        }
    }
}
