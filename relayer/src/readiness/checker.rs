use crate::{
    config::settings::GatewayConfig,
    core::{
        errors::EventProcessingError,
        event::{DelegatedUserDecryptRequest, UserDecryptRequest},
        job_id::JobId,
    },
    gateway::ciphertext_checker::CiphertextChecker,
    host::{HostAclChecker, HostAclError},
};
use alloy::primitives::{Address, Bytes, FixedBytes};
use std::fmt;

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

/// Combined readiness checker that orchestrates host ACL checks and gateway
/// ciphertext readiness checks.
pub struct ReadinessChecker {
    host_acl: HostAclChecker,
    ciphertext: CiphertextChecker,
}

impl ReadinessChecker {
    pub fn new(
        host_acl: HostAclChecker,
        gateway_config: &GatewayConfig,
    ) -> Result<Self, EventProcessingError> {
        let ciphertext = CiphertextChecker::new(gateway_config)?;
        Ok(Self {
            host_acl,
            ciphertext,
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
        self.ciphertext
            .check_public_decryption_readiness(job_id, handles, extra_data)
            .await
    }

    pub async fn check_host_acl_user_decrypt(
        &self,
        job_id: &JobId,
        request: &UserDecryptRequest,
    ) -> Result<(), ReadinessCheckError> {
        self.host_acl
            .check_user_decrypt(job_id, request)
            .await
            .map_err(|e| match &e {
                HostAclError::NotAllowed { .. } => ReadinessCheckError::NotAllowedOnHostAcl(e),
                _ => ReadinessCheckError::HostAclFailed(e),
            })
    }

    pub async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        address: Address,
        pairs: &[crate::core::event::HandleContractPair],
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        self.ciphertext
            .check_user_decryption_readiness(job_id, address, pairs, extra_data)
            .await
    }

    pub async fn check_host_acl_delegated_user_decrypt(
        &self,
        job_id: &JobId,
        request: &DelegatedUserDecryptRequest,
    ) -> Result<(), ReadinessCheckError> {
        self.host_acl
            .check_delegated_user_decrypt(job_id, request)
            .await
            .map_err(|e| match &e {
                HostAclError::NotAllowed { .. } => ReadinessCheckError::NotAllowedOnHostAcl(e),
                _ => ReadinessCheckError::HostAclFailed(e),
            })
    }
}
