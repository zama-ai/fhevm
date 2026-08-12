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
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::readiness::ReadinessStep;

/// Checks that every handle has Coprocessor attestation consensus.
pub struct CiphertextChecker {
    retry_config: RetrySettings,
    registry: CoprocessorRegistry<Arc<Provider>, AnyNetwork>,
    http_client: Client,
    head_timeout: Duration,
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

    /// Retries the whole handle set while attestations are starved, and gives up immediately once
    /// the Coprocessors have split.
    ///
    /// Coprocessors upload attestations asynchronously, so a first-attempt `Starved` is the
    /// normal case and worth retrying. `Split` is different in kind: every registered Coprocessor
    /// has already answered and their signers disagree, so re-reading the same objects cannot
    /// change that verdict.
    async fn check_handles_with_retry(
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
    /// retriable-vs-terminal and a `Split` has to win wherever it sits in the list. Stopping at the
    /// first `Starved` would hide a split handle behind a retry budget that can never satisfy it,
    /// and would make the user-facing verdict depend on the order the handles happen to arrive in —
    /// the same request reported as a 503 timeout or a 500 no-consensus depending on position. A
    /// `Split` is returned as soon as it is seen: it is terminal, so there is nothing left to learn.
    ///
    /// Handles are checked sequentially; each one already fans out concurrently across every
    /// Coprocessor bucket.
    async fn check_handles_once(
        &self,
        handles: &[FixedBytes<32>],
    ) -> Result<(), ConsensusCheckError> {
        let snapshot = self.registry.snapshot();
        let mut retriable: Option<ConsensusCheckError> = None;

        for handle in handles {
            match fetch_attestations_and_check_consensus(
                &self.http_client,
                *handle,
                &snapshot,
                self.head_timeout,
                COPROCESSOR_CONTEXT_ID_V1,
            )
            .await
            {
                Ok(_) => (),
                Err(e @ ConsensusCheckError::Split { .. }) => return Err(e),
                Err(e @ ConsensusCheckError::Starved { .. }) => retriable = retriable.or(Some(e)),
            }
        }

        match retriable {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
