use std::collections::HashMap;

use alloy::{
    primitives::{Address, FixedBytes, U256},
    providers::Provider,
    sol,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::GatewayConfigInstance;
use tracing::warn;

sol! {
    #[sol(rpc)]
    interface ACL {
        function isAllowed(bytes32 handle, address account) external view returns (bool);
        function isAllowedForDecryption(bytes32 handle) external view returns (bool);
    }
}

#[derive(Clone)]
pub struct AclChecker<P: Provider + Clone> {
    host_chain_acls: HashMap<u64, Address>,
    host_chain_providers: HashMap<u64, P>,
    fallback_provider: P,
}

impl<P> AclChecker<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    pub fn with_host_chains(
        host_chain_acls: HashMap<u64, Address>,
        host_chain_providers: HashMap<u64, P>,
        fallback_provider: P,
    ) -> Self {
        Self {
            host_chain_acls,
            host_chain_providers,
            fallback_provider,
        }
    }

    pub async fn new(
        gateway_config_contract: GatewayConfigInstance<P>,
        host_chain_providers: HashMap<u64, P>,
        fallback_provider: P,
    ) -> anyhow::Result<Self> {
        let response = gateway_config_contract.getHostChains().call().await?;
        let mut host_chain_acls = HashMap::new();

        for host_chain in response {
            let chain_id = u256_to_u64(host_chain.chainId)?;
            host_chain_acls.insert(chain_id, host_chain.aclAddress);
        }

        Ok(Self::with_host_chains(
            host_chain_acls,
            host_chain_providers,
            fallback_provider,
        ))
    }

    pub async fn check_user_access(
        &self,
        chain_id: u64,
        handle: FixedBytes<32>,
        user_address: Address,
    ) -> anyhow::Result<bool> {
        let resolved_chain_id = self.resolve_chain_id(chain_id, handle);
        let acl_address = self
            .host_chain_acls
            .get(&resolved_chain_id)
            .copied()
            .ok_or_else(|| anyhow!("No ACL configured for host chain {resolved_chain_id}"))?;
        let provider = self.provider_for_chain(resolved_chain_id);
        let acl_contract = ACL::new(acl_address, provider.clone());
        acl_contract
            .isAllowed(handle, user_address)
            .call()
            .await
            .map_err(Into::into)
    }

    pub async fn check_public_access(
        &self,
        chain_id: u64,
        handle: FixedBytes<32>,
    ) -> anyhow::Result<bool> {
        let resolved_chain_id = self.resolve_chain_id(chain_id, handle);
        let acl_address = self
            .host_chain_acls
            .get(&resolved_chain_id)
            .copied()
            .ok_or_else(|| anyhow!("No ACL configured for host chain {resolved_chain_id}"))?;
        let provider = self.provider_for_chain(resolved_chain_id);
        let acl_contract = ACL::new(acl_address, provider.clone());
        acl_contract
            .isAllowedForDecryption(handle)
            .call()
            .await
            .map_err(Into::into)
    }

    fn resolve_chain_id(&self, chain_id: u64, handle: FixedBytes<32>) -> u64 {
        if self.host_chain_acls.contains_key(&chain_id) {
            return chain_id;
        }

        let derived_chain_id = extract_chain_id_from_handle(handle);
        if self.host_chain_acls.contains_key(&derived_chain_id) {
            warn!(
                "Chain id {chain_id} not configured for ACL; derived {derived_chain_id} from handle"
            );
            return derived_chain_id;
        }

        chain_id
    }

    fn provider_for_chain(&self, chain_id: u64) -> &P {
        if let Some(provider) = self.host_chain_providers.get(&chain_id) {
            return provider;
        }

        warn!("No host chain RPC configured for chain {chain_id}; using gateway provider");
        &self.fallback_provider
    }
}

fn u256_to_u64(value: U256) -> anyhow::Result<u64> {
    let bytes = value.to_be_bytes::<32>();
    if bytes[..24].iter().any(|b| *b != 0) {
        return Err(anyhow!("Value does not fit in u64: {value}"));
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[24..]);
    Ok(u64::from_be_bytes(buf))
}

fn extract_chain_id_from_handle(handle: FixedBytes<32>) -> u64 {
    let bytes: &[u8; 32] = handle.as_ref();
    let mut buf = [0u8; 8];
    // Handle format encodes chain id in bytes 22-29 (see host contracts handle metadata).
    buf.copy_from_slice(&bytes[22..30]);
    u64::from_be_bytes(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };

    #[tokio::test]
    async fn test_check_user_access_uses_host_chain_provider() {
        let chain_id = 1u64;
        let acl_address = Address::repeat_byte(0x11);

        let host_asserter = Asserter::new();
        host_asserter.push_success(&true.abi_encode());
        let host_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(host_asserter);

        let fallback_asserter = Asserter::new();
        let fallback_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(fallback_asserter);

        let mut host_chain_acls = HashMap::new();
        host_chain_acls.insert(chain_id, acl_address);

        let mut host_chain_providers = HashMap::new();
        host_chain_providers.insert(chain_id, host_provider);

        let checker =
            AclChecker::with_host_chains(host_chain_acls, host_chain_providers, fallback_provider);

        let allowed = checker
            .check_user_access(chain_id, FixedBytes::<32>::ZERO, Address::ZERO)
            .await
            .unwrap();
        assert!(allowed);
    }

    #[tokio::test]
    async fn test_check_public_access_uses_fallback_provider() {
        let chain_id = 10u64;
        let acl_address = Address::repeat_byte(0x22);

        let fallback_asserter = Asserter::new();
        fallback_asserter.push_success(&false.abi_encode());
        let fallback_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(fallback_asserter);

        let mut host_chain_acls = HashMap::new();
        host_chain_acls.insert(chain_id, acl_address);

        let checker =
            AclChecker::with_host_chains(host_chain_acls, HashMap::new(), fallback_provider);

        let allowed = checker
            .check_public_access(chain_id, FixedBytes::<32>::ZERO)
            .await
            .unwrap();
        assert!(!allowed);
    }
}
