//! # Minimal User Decryption
//!
//! Decrypt an encrypted value using KMS client api.
//! This example is taken from a centralized KMS configuration.
//!
//! ```bash
//! cargo run --example minimal-user-decryption-request
//! ```

use alloy::primitives::{U256, address};
use gateway_sdk::{
    FhevmSdkBuilder, Result, blockchain::bindings::Decryption::CtHandleContractPair,
    utils::validate_address_from_str,
};
use std::{path, str::FromStr};
fn main() -> Result<()> {
    println!("🔓 Processing user decryption...");

    let handle_pair = CtHandleContractPair {
        ctHandle: U256::from_str(
            "0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200",
        )?
        .into(),
        contractAddress: validate_address_from_str("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46")?,
    };

    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory(path::PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111) // Example: Ethereum Sepolia
        .with_decryption_contract("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    let signature = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12";
    let user_address = address!("0x8888888888888888888888888888888888888888");
    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";

    let contract_addresses = vec![validate_address_from_str(
        "0x56a24bcaE11890353726596fD6f5cABb5a126Df9",
    )?];
    let start_timestamp = 1748252823;
    let duration_days = 10;

    let handle_vecs: Vec<Vec<u8>> = vec![handle_pair.ctHandle.to_vec()];

    match sdk
        .create_user_decrypt_request_builder()
        .add_handles_from_bytes(&handle_vecs, &contract_addresses)?
        .user_address_from_str(&user_address.to_string())?
        .signature_from_hex(signature)?
        .public_key_from_hex(&public_key)?
        .validity(start_timestamp, duration_days)?
        .build_and_generate_calldata()
    {
        Ok(calldata) => {
            println!("✅ Calldata generated: {} bytes", calldata.len());
            println!(
                "   First 32 bytes: 0x{}",
                hex::encode(&calldata[..32.min(calldata.len())])
            );
        }
        Err(e) => eprintln!("❌ Calldata generation error: {}", e),
    }

    Ok(())
}
