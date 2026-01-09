use alloy::primitives::{Address, FixedBytes, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::{Signature, SignerSync};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolStruct};

sol! {
    struct InputVerificationAttestation {
        uint256 requestId;
        bytes32 commitment;
        bytes32[] handles;
        uint256 contractChainId;
        address contractAddress;
        address userAddress;
        uint256 epochId;
    }

    struct CiphertextResponseAttestation {
        bytes32 handle;
        uint256 keyId;
        bytes32 ciphertextDigest;
        uint256 epochId;
    }
}

pub fn sign_input_verification_attestation(
    signer: &PrivateKeySigner,
    domain: &Eip712Domain,
    request_id: U256,
    commitment: FixedBytes<32>,
    handles: Vec<FixedBytes<32>>,
    contract_chain_id: U256,
    contract_address: Address,
    user_address: Address,
    epoch_id: U256,
) -> anyhow::Result<Signature> {
    let attestation = InputVerificationAttestation {
        requestId: request_id,
        commitment,
        handles,
        contractChainId: contract_chain_id,
        contractAddress: contract_address,
        userAddress: user_address,
        epochId: epoch_id,
    };

    let signing_hash = attestation.eip712_signing_hash(domain);
    let signature = signer.sign_hash_sync(&signing_hash)?;
    Ok(signature)
}

pub fn sign_ciphertext_response(
    signer: &PrivateKeySigner,
    domain: &Eip712Domain,
    handle: FixedBytes<32>,
    key_id: U256,
    ciphertext_digest: FixedBytes<32>,
    epoch_id: U256,
) -> anyhow::Result<Signature> {
    let response = CiphertextResponseAttestation {
        handle,
        keyId: key_id,
        ciphertextDigest: ciphertext_digest,
        epochId: epoch_id,
    };

    let signing_hash = response.eip712_signing_hash(domain);
    let signature = signer.sign_hash_sync(&signing_hash)?;
    Ok(signature)
}
