use std::vec::IntoIter;

use crate::core::event_processor::RequestCheckError;
use alloy::{
    primitives::{Address, B256, Bytes},
    providers::Provider,
    sol_types::SolCall,
};
use anyhow::anyhow;
use fhevm_host_bindings::acl::ACL::ACLInstance;

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

    /// Runs `calls` against the ACL contract in a single `eth_call` through `ACL.multicall`, so
    /// that all reads are served from the same block.
    ///
    /// Each element of `calls` must be an ABI-encoded ACL call. The returned results must be
    /// decoded in the same order with [`AclMulticallResults::decode_next`].
    pub async fn acl_multicall(
        &self,
        calls: Vec<Bytes>,
    ) -> Result<AclMulticallResults<'_, P>, RequestCheckError> {
        let calls_len = calls.len();
        let results = self
            .acl_contract
            .multicall(calls)
            .call()
            .await
            .map_err(|e| self.acl_err(e))?;
        if results.len() != calls_len {
            return Err(self.acl_err(anyhow!(
                "ACL multicall returned {} results for {calls_len} calls",
                results.len()
            )));
        }
        Ok(AclMulticallResults {
            client: self,
            results: results.into_iter(),
        })
    }

    /// RFC-012 EIP-712/ERC-1271 signature verification, on this host chain.
    pub async fn verify_signature(
        &self,
        claimed_signer: Address,
        digest: B256,
        signature: &[u8],
        gas_limit: u64,
    ) -> Result<(), RequestCheckError> {
        user_decryption_signature::verify_signature(
            self.acl_contract.provider(),
            claimed_signer,
            digest,
            signature,
            gas_limit,
        )
        .await
        .map_err(|e| RequestCheckError::from(e).context(format!("on host chain {}", self.chain_id)))
    }

    fn acl_err(&self, e: impl Into<anyhow::Error>) -> RequestCheckError {
        RequestCheckError::network(e).context(format!(
            "on host chain {} (ACL contract {})",
            self.chain_id,
            self.acl_contract.address()
        ))
    }
}

/// Results of a [`HostRpcClient::acl_multicall`], consumed in the order the calls were made.
pub struct AclMulticallResults<'a, P> {
    client: &'a HostRpcClient<P>,
    results: IntoIter<Bytes>,
}

impl<P: Provider> AclMulticallResults<'_, P> {
    /// ABI-decodes the next result as the return data of a `C` call, tagging errors with the
    /// host chain's context.
    pub fn decode_next<C: SolCall>(&mut self) -> Result<C::Return, RequestCheckError> {
        let result = self.results.next().ok_or_else(|| {
            self.client.acl_err(anyhow!(
                "ACL multicall returned fewer results than expected"
            ))
        })?;
        C::abi_decode_returns(&result).map_err(|e| self.client.acl_err(e))
    }
}
