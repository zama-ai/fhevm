use alloy::{
    primitives::{Address, Bytes, U256},
    signers::{Signer, local::PrivateKeySigner},
    sol,
    sol_types::{SolStruct, eip712_domain},
};
use anyhow::anyhow;
use std::str::FromStr;
// The RFC-016 unified signing struct is reused from the KMS connector's off-chain verifier so the
// EIP-712 `typeHash` can never drift. Aliased because the legacy struct below shares its Solidity
// name (with different fields).
use user_decryption_signature::UserDecryptRequestVerification as UserDecryptRequestVerificationV2;

// `UserDecryptRequestVerification` is only used off-chain to compute the EIP-712 signature
// (it's internal to the `Decryption` contract, so it isn't part of `fhevm_gateway_bindings`).
sol! {
    struct UserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }
}

/// Generates the EIP-712 signature of a `UserDecryptRequestVerification` message.
#[allow(clippy::too_many_arguments)]
pub async fn user_decrypt_eip712_signature(
    decryption_contract: Address,
    contracts_chain_id: u64,
    public_key: &str,
    allowed_contract: Address,
    start_timestamp: u64,
    duration_days: u64,
    extra_data: Vec<u8>,
    private_key: &str,
) -> anyhow::Result<Bytes> {
    let domain = eip712_domain! {
        name: "Decryption",
        version: "1",
        chain_id: contracts_chain_id,
        verifying_contract: decryption_contract,
    };
    let message = UserDecryptRequestVerification {
        publicKey: Bytes::from(alloy::hex::decode(
            public_key.strip_prefix("0x").unwrap_or(public_key),
        )?),
        contractAddresses: vec![allowed_contract],
        startTimestamp: U256::from(start_timestamp),
        durationDays: U256::from(duration_days),
        extraData: extra_data.into(),
    };
    let hash = message.eip712_signing_hash(&domain);

    let signer = PrivateKeySigner::from_str(private_key.strip_prefix("0x").unwrap_or(private_key))
        .map_err(|e| anyhow!("Invalid private key: {e}"))?;
    let signature = signer
        .sign_hash(&hash)
        .await
        .map_err(|e| anyhow!("Failed to sign: {e}"))?;

    Ok(Bytes::from(signature.as_bytes().to_vec()))
}

/// Generates the EIP-712 signature of a RFC-016 `UserDecryptRequestVerificationV2` message.
#[allow(clippy::too_many_arguments)]
pub async fn user_decrypt_v2_eip712_signature(
    decryption_contract: Address,
    contracts_chain_id: u64,
    user_address: Address,
    public_key: &str,
    allowed_contracts: Vec<Address>,
    start_timestamp: u64,
    duration_seconds: u64,
    extra_data: Vec<u8>,
    private_key: &str,
) -> anyhow::Result<Bytes> {
    let domain = eip712_domain! {
        name: "Decryption",
        version: "1",
        chain_id: contracts_chain_id,
        verifying_contract: decryption_contract,
    };
    let message = UserDecryptRequestVerificationV2 {
        userAddress: user_address,
        publicKey: Bytes::from(alloy::hex::decode(
            public_key.strip_prefix("0x").unwrap_or(public_key),
        )?),
        allowedContracts: allowed_contracts,
        startTimestamp: U256::from(start_timestamp),
        durationSeconds: U256::from(duration_seconds),
        extraData: extra_data.into(),
    };
    let hash = message.eip712_signing_hash(&domain);

    let signer = PrivateKeySigner::from_str(private_key.strip_prefix("0x").unwrap_or(private_key))
        .map_err(|e| anyhow!("Invalid private key: {e}"))?;
    let signature = signer
        .sign_hash(&hash)
        .await
        .map_err(|e| anyhow!("Failed to sign: {e}"))?;

    Ok(Bytes::from(signature.as_bytes().to_vec()))
}
