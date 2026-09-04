use crate::core::event_processor::RequestCheckError;
use alloy::{
    primitives::{Address, B256, U256},
    providers::Provider,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use tracing::warn;
use user_decryption_signature::{ERC1271_GAS_WARN_THRESHOLD, Erc1271Accepted, Erc1271GasBudget};

/// A client for one host chain's RPC node, tagged with its chain id so every error it produces
/// carries the host chain identity.
#[derive(Clone)]
pub struct HostRpcClient<P> {
    chain_id: u64,
    acl_contract: ACLInstance<P>,
}

impl<P: Provider> HostRpcClient<P> {
    pub fn new(chain_id: u64, acl_contract: ACLInstance<P>) -> Self {
        Self {
            chain_id,
            acl_contract,
        }
    }

    pub async fn is_allowed_for_decryption(&self, handle: B256) -> Result<bool, RequestCheckError> {
        self.acl_contract
            .isAllowedForDecryption(handle)
            .call()
            .await
            .map_err(|e| self.acl_err(e))
    }

    pub async fn is_allowed(
        &self,
        handle: B256,
        account: Address,
    ) -> Result<bool, RequestCheckError> {
        self.acl_contract
            .isAllowed(handle, account)
            .call()
            .await
            .map_err(|e| self.acl_err(e))
    }

    pub async fn is_handle_delegated_for_user_decryption(
        &self,
        delegator: Address,
        delegate: Address,
        contract: Address,
        handle: B256,
    ) -> Result<bool, RequestCheckError> {
        self.acl_contract
            .isHandleDelegatedForUserDecryption(delegator, delegate, contract, handle)
            .call()
            .await
            .map_err(|e| self.acl_err(e))
    }

    pub async fn decryption_signature_invalidated_before(
        &self,
        account: Address,
    ) -> Result<U256, RequestCheckError> {
        self.acl_contract
            .decryptionSignatureInvalidatedBefore(account)
            .call()
            .await
            .map_err(|e| self.acl_err(e))
    }

    /// RFC-012 EIP-712/ERC-1271 signature verification, on this host chain.
    pub async fn verify_signature(
        &self,
        claimed_signer: Address,
        digest: B256,
        signature: &[u8],
        gas_limit: u64,
    ) -> Result<(), RequestCheckError> {
        let accepted = user_decryption_signature::verify_signature(
            self.acl_contract.provider(),
            claimed_signer,
            digest,
            signature,
            Erc1271GasBudget {
                limit: gas_limit,
                warn_above: ERC1271_GAS_WARN_THRESHOLD,
            },
        )
        .await
        .map_err(|e| {
            RequestCheckError::from(e).context(format!("on host chain {}", self.chain_id))
        })?;

        if accepted == Erc1271Accepted::AboveWarnThreshold {
            warn!(
                signer = %claimed_signer,
                chain_id = self.chain_id,
                warn_above = ERC1271_GAS_WARN_THRESHOLD,
                gas_limit,
                "ERC-1271 verification needed more gas than expected"
            );
        }
        Ok(())
    }

    fn acl_err(&self, e: impl Into<anyhow::Error>) -> RequestCheckError {
        RequestCheckError::network(e).context(format!(
            "on host chain {} (ACL contract {})",
            self.chain_id,
            self.acl_contract.address()
        ))
    }
}
