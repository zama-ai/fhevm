use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::{Signature, SignerSync};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolStruct};

sol! {
    struct CiphertextVerification {
        bytes32[] ctHandles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
        bytes extraData;
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
    handles: Vec<FixedBytes<32>>,
    user_address: Address,
    contract_address: Address,
    contract_chain_id: U256,
    extra_data: Bytes,
) -> anyhow::Result<Signature> {
    let attestation = CiphertextVerification {
        ctHandles: handles,
        userAddress: user_address,
        contractAddress: contract_address,
        contractChainId: contract_chain_id,
        extraData: extra_data,
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
