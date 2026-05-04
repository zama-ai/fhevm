use crate::types::handle::extract_chain_id_from_handle;
use alloy::{
    primitives::Address,
    providers::{ProviderBuilder, RootProvider, mock::Asserter},
};
use fhevm_host_bindings::acl::ACL::{self, ACLInstance};
use std::collections::HashMap;
use tracing::info;

/// 32-byte ABI-encoded ERC-1271 magic value `0x1626ba7e` (`bytes4` left-aligned in a 32-byte
/// word). Push as a host-chain mock response to make the RFC-012 `isValidSignature` check pass
/// in `DecryptionProcessor::check_user_decryption_request_v2`.
pub fn erc1271_magic_response() -> Vec<u8> {
    let mut word = vec![0u8; 32];
    word[..4].copy_from_slice(&[0x16, 0x26, 0xba, 0x7e]);
    word
}

/// Inits a mock host chain ACL contracts map.
///
/// Accepts ABI-encoded responses, allowing callers to mix responses with different types.
pub fn init_host_chains_acl_contracts_mock(
    ct_handle: &[u8],
    responses: Vec<Vec<u8>>,
) -> HashMap<u64, ACLInstance<RootProvider>> {
    let asserter = Asserter::new();
    for response in responses {
        asserter.push_success(&response);
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
