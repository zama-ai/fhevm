use crate::{
    config::settings::{AppConfigError, GatewayConfig, RetrySettings},
    core::{errors::EventProcessingError, event::HandleContractPair, job_id::JobId},
    gateway::arbitrum::bindings::{Decryption, DecryptionNative, NativeCtHandleContractPair},
    host::redact_alloy_error,
    readiness::ReadinessCheckError,
};
use alloy::{
    primitives::{Address, Bytes, FixedBytes},
    providers::{fillers::FillProvider, ProviderBuilder, RootProvider},
};
use fhevm_gateway_bindings::decryption::Decryption::DecryptionInstance;
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

use crate::readiness::ReadinessStep;

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

type GatewayDecryption = DecryptionInstance<Arc<Provider>, alloy::network::AnyNetwork>;

/// Checks gateway ciphertext readiness (isPublicDecryptionReady / isUserDecryptionReady).
pub struct CiphertextChecker {
    retry_config: RetrySettings,
    decryption_address: Address,
    provider: Arc<Provider>,
    gw_decryption: GatewayDecryption,
}

impl CiphertextChecker {
    pub fn new(gateway_config: &GatewayConfig) -> Result<Self, EventProcessingError> {
        let decryption_address = Address::from_str(&gateway_config.contracts.decryption_address)
            .map_err(|_| {
                EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                    "contracts.decryption_address".to_owned(),
                ))
            })?;

        let url = Url::parse(&gateway_config.blockchain_rpc.read_http_url).map_err(|e| {
            EventProcessingError::ValidationFailed {
                field: "blockchain_rpc_url".to_string(),
                reason: format!("invalid URL: {}", e),
            }
        })?;

        let provider = Arc::new(
            ProviderBuilder::new()
                .network::<alloy::network::AnyNetwork>()
                .connect_http(url),
        );

        let gw_decryption = Decryption::new(decryption_address, provider.clone());

        Ok(Self {
            decryption_address,
            provider,
            retry_config: gateway_config
                .readiness_checker
                .gw_ciphertext_check
                .retry
                .clone(),
            gw_decryption,
        })
    }

    pub async fn check_public_decryption_readiness(
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
            .check_readiness_internal(job_id, || {
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
    pub async fn check_user_decryption_readiness(
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

        let result = if is_native_pairs(pairs) {
            let native_pairs: Vec<NativeCtHandleContractPair> = pairs
                .iter()
                .map(|pair| NativeCtHandleContractPair {
                    ctHandle: pair.ct_handle.into(),
                    contractId: pair.contract_id.expect("native user decrypt requires contractId"),
                })
                .collect();

            self.check_readiness_internal(job_id, || {
                let decryption_address = self.decryption_address;
                let provider = self.provider.clone();
                let pairs = native_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    let native_decryption = DecryptionNative::new(decryption_address, provider);
                    native_decryption
                        .isUserDecryptionReadyNative(pairs, extra_data)
                        .call()
                        .await
                }
            })
            .await
        } else {
            let contract_pairs: Vec<Decryption::CtHandleContractPair> = pairs
                .iter()
                .map(Decryption::CtHandleContractPair::from)
                .collect();

            self.check_readiness_internal(job_id, || {
                let decryption = self.gw_decryption.clone();
                let pairs = contract_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isUserDecryptionReady_0(pairs, extra_data)
                        .call()
                        .await
                }
            })
            .await
        };

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

    pub async fn check_delegated_user_decryption_readiness(
        &self,
        job_id: &JobId,
        pairs: &[HandleContractPair],
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        info!(
            step = %ReadinessStep::Started,
            int_job_id = %job_id,
            "Starting delegated user decryption gateway ciphertext check"
        );

        let result = if is_native_pairs(pairs) {
            let native_pairs: Vec<NativeCtHandleContractPair> = pairs
                .iter()
                .map(|pair| NativeCtHandleContractPair {
                    ctHandle: pair.ct_handle.into(),
                    contractId: pair
                        .contract_id
                        .expect("native delegated user decrypt requires contractId"),
                })
                .collect();

            self.check_readiness_internal(job_id, || {
                let decryption_address = self.decryption_address;
                let provider = self.provider.clone();
                let pairs = native_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    let native_decryption = DecryptionNative::new(decryption_address, provider);
                    native_decryption
                        .isDelegatedUserDecryptionReadyNative(pairs, extra_data)
                        .call()
                        .await
                }
            })
            .await
        } else {
            let contract_pairs: Vec<Decryption::CtHandleContractPair> = pairs
                .iter()
                .map(Decryption::CtHandleContractPair::from)
                .collect();

            self.check_readiness_internal(job_id, || {
                let decryption = self.gw_decryption.clone();
                let pairs = contract_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isDelegatedUserDecryptionReady(pairs, extra_data)
                        .call()
                        .await
                }
            })
            .await
        };

        match &result {
            Ok(()) => info!(
                step = %ReadinessStep::Passed,
                int_job_id = %job_id,
                "Delegated user decryption gateway ciphertext check passed"
            ),
            Err(e) => error!(
                step = %ReadinessStep::Failed,
                int_job_id = %job_id,
                error = ?e,
                "Delegated user decryption gateway ciphertext check failed"
            ),
        }

        result
    }

    async fn check_readiness_internal<F, Fut>(
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
        let mut last_error: Option<alloy::contract::Error> = None;

        loop {
            match check_fn().await {
                Ok(is_ready) => {
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

fn is_native_pairs(pairs: &[HandleContractPair]) -> bool {
    !pairs.is_empty()
        && pairs
            .iter()
            .all(|pair| pair.contract_address.is_none() && pair.contract_id.is_some())
}
