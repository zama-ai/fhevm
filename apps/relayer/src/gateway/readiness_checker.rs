use crate::{
    config::settings::{AppConfigError, GatewayConfig, RetrySettings},
    core::errors::EventProcessingError,
    gateway::arbitrum::bindings::Decryption,
};
use alloy::{
    primitives::{Address, Bytes, FixedBytes},
    providers::{fillers::FillProvider, ProviderBuilder, RootProvider},
};
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

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

/// A generic readiness checker for gateway operations.
///
/// This struct handles the common pattern of checking if the gateway is ready
/// for a specific operation, with configurable retry logic.
pub struct ReadinessChecker {
    retry_config: RetrySettings,
    decryption_address: Address,
    provider: Arc<Provider>,
}

impl ReadinessChecker {
    /// Creates a new ReadinessChecker with the given gateway configuration.
    pub fn new(gateway_config: &GatewayConfig) -> Result<Self, EventProcessingError> {
        // Get decryption address
        let decryption_address = Address::from_str(&gateway_config.contracts.decryption_address)
            .map_err(|_| {
                EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                    "contracts.decryption_address".to_owned(),
                ))
            })?;

        // Create provider once
        let url = Url::parse(&gateway_config.blockchain_rpc.http_url)
            .map_err(|e| EventProcessingError::HandlerError(format!("Invalid URL: {}", e)))?;

        let provider = Arc::new(
            ProviderBuilder::new()
                .network::<alloy::network::AnyNetwork>()
                .connect_http(url),
        );

        Ok(Self {
            retry_config: gateway_config.readiness_checker.retry.clone(),
            decryption_address,
            provider,
        })
    }

    /// Checks if the gateway is ready for public decryption, with retry logic.
    ///
    /// # Arguments
    /// * `handles` - Vector of handles to decrypt
    /// * `extra_data` - Extra data for the decryption
    ///
    /// # Returns
    /// * `Ok(())` if the gateway is ready
    /// * `Err(EventProcessingError)` if the gateway is not ready after all retries
    pub async fn check_public_decryption_readiness(
        &self,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<(), EventProcessingError> {
        // Use the stored provider
        let decryption = Decryption::new(self.decryption_address, self.provider.clone());

        let operation_desc = format!("public decryption for handles: {:?}", handles);

        self.check_readiness_internal(
            || {
                let decryption = decryption.clone();
                let handles = handles.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isPublicDecryptionReady(handles, extra_data)
                        .call()
                        .await
                }
            },
            &operation_desc,
        )
        .await
        .map_err(|_| EventProcessingError::ReadinessCheckFailed)
    }

    /// Checks if the gateway is ready for user decryption, with retry logic.
    ///
    /// # Arguments
    /// * `user_address` - User's address
    /// * `contract_pairs` - Contract pairs for decryption
    /// * `extra_data` - Extra data for the decryption
    ///
    /// # Returns
    /// * `Ok(())` if the gateway is ready
    /// * `Err(EventProcessingError)` if the gateway is not ready after all retries
    pub async fn check_user_decryption_readiness(
        &self,
        user_address: Address,
        contract_pairs: Vec<Decryption::CtHandleContractPair>,
        extra_data: Bytes,
    ) -> Result<(), EventProcessingError> {
        // Use the stored provider
        let decryption = Decryption::new(self.decryption_address, self.provider.clone());

        let operation_desc = format!("user decryption for address: {:?}", user_address);

        info!(
            "Checking if the decryption manager is ready for user address: {:?} and contract pairs: {:?}",
            user_address, contract_pairs
        );

        self.check_readiness_internal(
            || {
                let decryption = decryption.clone();
                let pairs = contract_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isUserDecryptionReady(user_address, pairs, extra_data)
                        .call()
                        .await
                }
            },
            &operation_desc,
        )
        .await
        .map_err(|_| EventProcessingError::ReadinessCheckFailed)
    }

    /// Internal helper for checking readiness with retry logic.
    async fn check_readiness_internal<F, Fut>(
        &self,
        check_fn: F,
        operation_desc: &str,
    ) -> Result<(), String>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<bool, alloy::contract::Error>>,
    {
        let max_retries = self.retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.retry_config.retry_interval_ms);

        let mut retries = 0;

        loop {
            match check_fn().await {
                Ok(is_ready) => {
                    if is_ready {
                        info!("Gateway is ready for operation: {}", operation_desc);
                        return Ok(());
                    } else {
                        info!("Gateway not ready for {}, retrying...", operation_desc);
                    }
                }
                Err(err) => {
                    error!(
                        "Readiness check for {} failed with error: {}, retrying...",
                        operation_desc, err
                    );
                }
            }

            retries += 1;
            if retries >= max_retries {
                warn!("Max retries reached for {} readiness check", operation_desc);
                return Err(format!(
                    "Gateway not ready for {} after {} retries",
                    operation_desc, max_retries
                ));
            }

            info!(
                "Retrying {} readiness check (attempt {}/{})",
                operation_desc, retries, max_retries
            );
            tokio::time::sleep(retry_interval).await;
        }
    }
}
