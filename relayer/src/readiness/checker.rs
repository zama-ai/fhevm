use crate::{
    config::settings::GatewayConfig,
    core::{
        errors::EventProcessingError,
        event::{HandleContractPair, UserDecryptRequest},
        job_id::JobId,
    },
    gateway::ciphertext_checker::CiphertextChecker,
    host::{HostAclChecker, HostAclError},
};
use alloy::primitives::{Bytes, FixedBytes};
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

    /// Host ACL pre-check covering all three attestation types. The shape
    /// of the check is dictated by the request variant:
    /// - `LegacyDirect`: per-pair `isAllowed(handle, user) +
    ///   isAllowed(handle, contract)`.
    /// - `LegacyDelegated`: per-pair `isHandleDelegatedForUserDecryption`
    ///   for the single (delegator, delegate) couple.
    /// - `Eip712UnifiedV1`: per-`HandleEntry` direct vs delegated split
    ///   based on `owner_address == user_address`.
    pub async fn check_host_acl_user_decrypt(
        &self,
        job_id: &JobId,
        request: &UserDecryptRequest,
    ) -> Result<(), ReadinessCheckError> {
        let result = match request {
            UserDecryptRequest::LegacyDirect {
                ct_handle_contract_pairs,
                user_address,
                ..
            } => {
                self.host_acl
                    .check_user_decrypt(job_id, ct_handle_contract_pairs, *user_address)
                    .await
            }
            UserDecryptRequest::LegacyDelegated {
                ct_handle_contract_pairs,
                delegator_address,
                delegate_address,
                ..
            } => {
                self.host_acl
                    .check_delegated_user_decrypt(
                        job_id,
                        ct_handle_contract_pairs,
                        *delegator_address,
                        *delegate_address,
                    )
                    .await
            }
            UserDecryptRequest::Eip712UnifiedV1 {
                handles,
                user_address,
                ..
            } => {
                self.host_acl
                    .check_unified_user_decrypt(job_id, handles, *user_address)
                    .await
            }
            // RFC-021 Solana: the host-chain ACL is enforced off-gateway by the KMS Connector
            // (`solana_acl` reads the on-chain Solana ACL at confirmed commitment), so the relayer
            // performs no EVM-style host ACL check here.
            UserDecryptRequest::SolanaUnifiedV1 { .. } => Ok(()),
        };
        result.map_err(|e| match &e {
            HostAclError::NotAllowed { .. } => ReadinessCheckError::NotAllowedOnHostAcl(e),
            _ => ReadinessCheckError::HostAclFailed(e),
        })
    }

    pub async fn check_user_decryption_readiness(
        &self,
        job_id: &JobId,
        pairs: &[HandleContractPair],
        extra_data: Bytes,
    ) -> Result<(), ReadinessCheckError> {
        self.ciphertext
            .check_user_decryption_readiness(job_id, pairs, extra_data)
            .await
    }
}
