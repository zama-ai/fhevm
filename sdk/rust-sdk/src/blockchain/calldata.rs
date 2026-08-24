//! Calldata module for FHEVM SDK

use crate::Result;
use crate::decryption::user::UserDecryptRequest;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::sol_types::SolCall;
// The legacy overload — `(CtHandleContractPair[], RequestValidity, ContractsInfo, address,
// bytes, bytes, bytes)` — is the one this SDK builds. The alias is pinned by the selector test
// below rather than by the `_N` suffix alone: alloy numbers overloads by their position in the
// generated bindings, so adding or removing any `userDecryptionRequest` overload renumbers the
// rest. A silently renumbered alias still compiles whenever two overloads happen to accept the
// same argument shape, and would then encode calldata for the wrong function.
use fhevm_gateway_bindings::decryption::Decryption::{
    publicDecryptionRequestCall, userDecryptionRequest_2Call as userDecryptionRequestCall,
};
use fhevm_gateway_bindings::decryption::IDecryption::ContractsInfo;
use fhevm_gateway_bindings::input_verification::InputVerification;
use tracing::info;

pub fn public_decryption_req(handles: Vec<FixedBytes<32>>) -> Result<Bytes> {
    info!("Generating public decryption request calldata");
    let extra_data = Bytes::new(); // Empty extra_data for now
    let calldata = publicDecryptionRequestCall::new((handles, extra_data)).abi_encode();
    Ok(Bytes::from(calldata))
}

/// Generates calldata for user decryption.
pub fn user_decryption_req(
    user_decrypt_request: UserDecryptRequest,
    contracts_chain_id: u64,
) -> Result<Bytes> {
    info!("Generating user decryption request calldata");

    let extra_data = Bytes::new(); // Empty extra_data for now
    let call = userDecryptionRequestCall::new((
        user_decrypt_request.ct_handle_contract_pairs,
        user_decrypt_request.request_validity,
        ContractsInfo {
            chainId: U256::from(contracts_chain_id),
            addresses: user_decrypt_request.contract_addresses,
        },
        user_decrypt_request.user_address,
        user_decrypt_request.public_key,
        user_decrypt_request.signature,
        extra_data,
    ));

    let calldata = userDecryptionRequestCall::abi_encode(&call);

    Ok(Bytes::from(calldata))
}

/// Computes calldata for verifyProofRequest function
///
/// # Arguments
/// * `contract_chain_id` - Chain ID where the contract is deployed
/// * `contract_address` - Address of the contract
/// * `user_address` - Address of the user
/// * `ciphertext_with_zkproof` - Combined ciphertext and ZK proof data
pub fn verify_proof_req(
    contract_chain_id: u64,
    contract_address: Address,
    user_address: Address,
    ciphertext_with_zkproof: Bytes,
) -> Result<Bytes> {
    info!("Generating verification proof request calldata");
    let request_call = InputVerification::verifyProofRequestCall {
        contractChainId: U256::from(contract_chain_id),
        contractAddress: contract_address,
        userAddress: user_address,
        ciphertextWithZKProof: ciphertext_with_zkproof,
        extraData: Bytes::new(), // Empty extra_data for now
    };
    let calldata = request_call.abi_encode();
    Ok(Bytes::from(calldata))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gateway has three `userDecryptionRequest` overloads, and alloy tells them apart only
    /// by a positional `_N` suffix. This SDK targets the legacy one; the suffix that names it
    /// moves whenever an overload is added or removed upstream, and a moved suffix does not
    /// necessarily break the build — two overloads can accept argument shapes close enough to
    /// still typecheck. What cannot silently move is the selector, so that is what is pinned.
    ///
    /// The value below is the one in `gateway-contracts/selectors.txt` for
    /// `userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)`.
    /// A mismatch here means the alias now names a different function and the SDK is encoding
    /// calldata nobody will execute.
    #[test]
    fn user_decryption_alias_names_the_legacy_overload() {
        assert_eq!(
            userDecryptionRequestCall::SELECTOR,
            [0xf1, 0xb5, 0x7a, 0xdb],
            "the userDecryptionRequest alias no longer selects the legacy overload"
        );
    }

    /// The same pin for the public-decryption call, which is not overloaded today but shares the
    /// generated-bindings surface and would be renamed by the same class of upstream change.
    #[test]
    fn public_decryption_call_selector_is_pinned() {
        assert_eq!(
            publicDecryptionRequestCall::SELECTOR,
            [0xd8, 0x99, 0x8f, 0x45],
            "the publicDecryptionRequest selector changed"
        );
    }
}
