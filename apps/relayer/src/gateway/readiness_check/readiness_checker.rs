use crate::{
    config::settings::{AppConfigError, GatewayConfig, RetrySettings},
    core::{
        errors::EventProcessingError,
        event::{DelegatedUserDecryptRequest, UserDecryptRequest},
        job_id::JobId,
    },
    gateway::{
        arbitrum::bindings::Decryption,
        readiness_check::host_acl_checker::{HostAclChecker, HostAclError},
    },
};
use alloy::{
    primitives::{Address, Bytes, FixedBytes},
    providers::{fillers::FillProvider, ProviderBuilder, RootProvider},
};
use fhevm_gateway_bindings::decryption::Decryption::DecryptionInstance;
use reqwest::Url;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// Steps for readiness checker operations
#[derive(Debug, Clone, Copy)]
pub enum ReadinessStep {
    Started,
    Passed,
    Failed,
    Retrying,
}

impl fmt::Display for ReadinessStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Started => write!(f, "readiness_started"),
            Self::Passed => write!(f, "readiness_passed"),
            Self::Failed => write!(f, "readiness_failed"),
            Self::Retrying => write!(f, "readiness_retrying"),
        }
    }
}

#[derive(Debug)]
pub enum ReadinessCheckError {
    GwTimeout,
    GwContractError(alloy::contract::Error),
    NotAllowedOnHostAcl(HostAclError),
    HostAclFailed(HostAclError),
}

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
struct GwCiphertextChecker {
    gw_retry_config: RetrySettings,
    gw_decryption: GatewayDecryption,
}

impl GwCiphertextChecker {
    fn new(gateway_config: &GatewayConfig) -> Result<Self, EventProcessingError> {
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

        let gw_decryption = Decryption::new(decryption_address, provider);

        Ok(Self {
            gw_retry_config: gateway_config
                .readiness_checker
                .gw_ciphertext_check
                .retry
                .clone(),
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

    async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        user_address: Address,
        contract_pairs: Vec<Decryption::CtHandleContractPair>,
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        info!(
            step = %ReadinessStep::Started,
            int_job_id = %job_id,
            "Starting user decryption gateway ciphertext check"
        );

        let result = self
            .check_readiness_internal(job_id, || {
                let decryption = self.gw_decryption.clone();
                let pairs = contract_pairs.clone();
                let extra_data = extra_data.clone();
                async move {
                    decryption
                        .isUserDecryptionReady(user_address, pairs, extra_data)
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

    async fn check_readiness_internal<F, Fut>(
        &self,
        job_id: &JobId,
        check_fn: F,
    ) -> Result<(), ReadinessCheckError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<bool, alloy::contract::Error>>,
    {
        let max_retries = self.gw_retry_config.max_attempts;
        let retry_interval = Duration::from_millis(self.gw_retry_config.retry_interval_ms);
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
                    error!(int_job_id = %job_id, error = %err, "Contract call failed, will retry");
                    last_error = Some(err);
                }
            }

            retries += 1;
            if retries >= max_retries {
                warn!(
                    int_job_id = %job_id,
                    max_retries = max_retries,
                    retry_interval_ms = self.gw_retry_config.retry_interval_ms,
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

/// Combined readiness checker that orchestrates host ACL checks and gateway
/// ciphertext readiness checks.
pub struct ReadinessChecker {
    host_acl: HostAclChecker,
    gw_ciphertext: GwCiphertextChecker,
}

impl ReadinessChecker {
    pub fn new(
        host_acl: HostAclChecker,
        gateway_config: &GatewayConfig,
    ) -> Result<Self, EventProcessingError> {
        let gw_ciphertext = GwCiphertextChecker::new(gateway_config)?;
        Ok(Self {
            host_acl,
            gw_ciphertext,
        })
    }

    pub async fn check_host_acl_public_decrypt(
        &self,
        job_id: &JobId,
        handles: &[[u8; 32]],
    ) -> Result<(), ReadinessCheckError> {
        self.host_acl
            .check_public_decrypt(job_id, handles)
            .await
            .map_err(|e| match &e {
                HostAclError::NotAllowed { .. } => ReadinessCheckError::NotAllowedOnHostAcl(e),
                _ => ReadinessCheckError::HostAclFailed(e),
            })
    }

    pub async fn check_public_decryption_readiness(
        &self,
        job_id: &JobId,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        self.gw_ciphertext
            .check_public_decryption_readiness(job_id, handles, extra_data)
            .await
    }

    pub async fn check_host_acl_user_decrypt(
        &self,
        job_id: &JobId,
        request: &UserDecryptRequest,
    ) -> Result<(), ReadinessCheckError> {
        self.host_acl
            .check_user_decrypt(
                job_id,
                &request.ct_handle_contract_pairs,
                request.user_address,
            )
            .await
            .map_err(|e| match &e {
                HostAclError::NotAllowed { .. } => ReadinessCheckError::NotAllowedOnHostAcl(e),
                _ => ReadinessCheckError::HostAclFailed(e),
            })
    }

    pub async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        user_address: Address,
        contract_pairs: Vec<Decryption::CtHandleContractPair>,
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        self.gw_ciphertext
            .check_user_decryption_readiness(job_id, user_address, contract_pairs, extra_data)
            .await
    }

    pub async fn check_host_acl_delegated_user_decrypt(
        &self,
        job_id: &JobId,
        request: &DelegatedUserDecryptRequest,
    ) -> Result<(), ReadinessCheckError> {
        self.host_acl
            .check_delegated_user_decrypt(
                job_id,
                &request.ct_handle_contract_pairs,
                request.delegator_address,
                request.delegate_address,
            )
            .await
            .map_err(|e| match &e {
                HostAclError::NotAllowed { .. } => ReadinessCheckError::NotAllowedOnHostAcl(e),
                _ => ReadinessCheckError::HostAclFailed(e),
            })
    }
}
