//! Calldata module for FHEVM SDK

use crate::Result;
use crate::decryption::user::UserDecryptRequest;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::sol_types::SolCall;
use fhevm_gateway_rust_bindings::decryption::Decryption::{
    publicDecryptionRequestCall, userDecryptionRequestCall,
};
use fhevm_gateway_rust_bindings::decryption::IDecryption::ContractsInfo;
use fhevm_gateway_rust_bindings::input_verification::InputVerification;
use tracing::info;

pub fn public_decryption_req(handles: Vec<FixedBytes<32>>) -> Result<Bytes> {
    info!("Generating public decryption request calldata");
    let extra_data = Bytes::new(); // Empty extra_data for now
    let calldata = publicDecryptionRequestCall::new((handles, extra_data)).abi_encode();
    Ok(Bytes::from(calldata))
}

/// Generate calldata for user decryptÒ
pub fn user_decryption_req(user_decrypt_request: UserDecryptRequest) -> Result<Bytes> {
    info!("Generating user decryption request calldata");

    let extra_data = Bytes::new(); // Empty extra_data for now
    let call = userDecryptionRequestCall::new((
        user_decrypt_request.ct_handle_contract_pairs,
        user_decrypt_request.request_validity,
        ContractsInfo {
            chainId: U256::from(user_decrypt_request.contracts_chain_id),
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
