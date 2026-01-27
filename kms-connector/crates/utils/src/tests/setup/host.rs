use crate::types::fhe::extract_chain_id_from_handle;
use alloy::{
    primitives::Address,
    providers::{ProviderBuilder, RootProvider, mock::Asserter},
    sol_types::SolValue,
};
use fhevm_host_bindings::acl::ACL::{self, ACLInstance};
use std::collections::HashMap;
use tracing::info;

pub fn init_host_chains_acl_contracts_mock(
    ct_handle: &[u8],
    responses: Vec<bool>,
) -> HashMap<u64, ACLInstance<RootProvider>> {
    let asserter = Asserter::new();
    for response in responses {
        asserter.push_success(&response.abi_encode());
    }
    let host_mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter.clone());

    let acl_contracts_mock = HashMap::from([(
        extract_chain_id_from_handle(ct_handle).unwrap(),
        ACL::new(Address::default(), host_mock_provider),
    )]);
    info!("Host chain mock started!");

    acl_contracts_mock
}
