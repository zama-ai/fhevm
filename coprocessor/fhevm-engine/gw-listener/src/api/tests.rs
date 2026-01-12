use std::str::FromStr;

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::eip712_domain;

use super::signing::{sign_ciphertext_response, sign_input_verification_attestation};

fn test_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str(
        "0x59c6995e998f97a5a0044966f094538a1b5c3b5d6e8cf2f6f7c6a8b2f1e5a7d3",
    )
    .expect("valid test key")
}

fn test_input_domain() -> alloy::sol_types::Eip712Domain {
    eip712_domain! {
        name: "InputVerification",
        version: "1",
        chain_id: 1u64,
        verifying_contract: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
    }
}

#[test]
fn sign_input_verification_attestation_returns_signature() {
    let signer = test_signer();
    let domain = test_input_domain();
    let handles = vec![FixedBytes::from([1u8; 32]), FixedBytes::from([2u8; 32])];

    let signature = sign_input_verification_attestation(
        &signer,
        &domain,
        handles,
        Address::from_str("0x0000000000000000000000000000000000000003").unwrap(),
        Address::from_str("0x0000000000000000000000000000000000000002").unwrap(),
        U256::from(42161u64),
        alloy::primitives::Bytes::from(vec![0u8]),
    )
    .expect("signature");

    assert_eq!(signature.as_bytes().len(), 65);
}

#[test]
fn sign_ciphertext_response_returns_signature() {
    let signer = test_signer();
    let domain = eip712_domain! {
        name: "FHEVM",
        version: "1",
        chain_id: 1u64,
        verifying_contract: Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
    };

    let signature = sign_ciphertext_response(
        &signer,
        &domain,
        FixedBytes::from([4u8; 32]),
        U256::from(5u64),
        FixedBytes::from([6u8; 32]),
        U256::from(7u64),
    )
    .expect("signature");

    assert_eq!(signature.as_bytes().len(), 65);
}
