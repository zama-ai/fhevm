use std::path;

use fhevm_sdk::signature::validate_address_from_str;
use fhevm_sdk::{EncryptedInput, FhevmError, FhevmSdk, FhevmSdkBuilder};

use alloy::primitives::address;
use fhevm_sdk::EncryptedInputBuilder;
use fhevm_sdk::logging;

fn main() -> Result<(), FhevmError> {
    // Initialize logging (if needed)
    logging::init_from_env(log::LevelFilter::Info);

    log::info!("FHEVM SDK Demo");

    // APPROACH 1: Using the builder pattern
    log::info!("=== Builder Pattern Approach ===");
    let mut sdk_from_builder = create_sdk_with_builder()?;
    demo_sdk_functionality(&mut sdk_from_builder)?;

    log::info!("FHEVM SDK Demo completed");
    Ok(())
}

/// Create an SDK instance using the builder pattern
fn create_sdk_with_builder() -> Result<FhevmSdk, FhevmError> {
    log::info!("Creating SDK using builder pattern");

    let sdk = create_sample_builder().build()?;
    log::info!("SDK successfully created with builder");

    Ok(sdk)
}

/// Create a sample builder with test configuration
fn create_sample_builder() -> FhevmSdkBuilder {
    FhevmSdkBuilder::new()
        .with_keys_directory(path::PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111) // Example: Ethereum Sepolia
        .with_gateway_contract("Decryption", "0x1234567890123456789012345678901234567bbb")
        .with_gateway_contract(
            "input-verifier",
            "0x1234567890123456789012345678901234567aaa",
        )
        .with_host_contract("acl", "0x0987654321098765432109876543210987654321")
}

/// Demonstrate SDK functionality
fn demo_sdk_functionality(sdk: &mut FhevmSdk) -> Result<(), FhevmError> {
    // Set up test addresses
    let contract_address = address!("0x7777777777777777777777777777777777777777");
    let user_address = address!("0x8888888888888888888888888888888888888888");

    // Example: Encrypt a value
    log::info!("Encrypting value 18446744073709550042");

    // Create an input builder and explicitly type it
    let mut builder: EncryptedInputBuilder = match sdk.create_input_builder() {
        Ok(b) => b,
        Err(e) => {
            log::info!("Error creating input builder: {}", e);
            return Err(e);
        }
    };

    // Add a value
    if let Err(e) = builder.add_u64(18446744073709550042) {
        log::info!("Error adding value: {}", e);
        return Err(e);
    }

    // Encrypt for a specific contract and user
    let encrypted: EncryptedInput = builder.encrypt_for(contract_address, user_address)?;
    log::info!("Encryption successful!");
    log::info!("  - Handles: {}", encrypted.handles.len());
    log::info!("  - Ciphertext size: {} bytes", encrypted.ciphertext.len());

    let handle_vecs: Vec<Vec<u8>> = encrypted
        .handles
        .iter()
        .map(|handle| handle.to_vec())
        .collect();

    log::info!("Generating user decrypt calldata");
    match sdk.generate_user_decrypt_calldata(&handle_vecs, &user_address.to_string()) {
        Ok(calldata) => log::info!("Calldata generated: {} bytes", calldata.len()),
        Err(e) => log::info!("Calldata generation error: {}", e),
    }

    // Example: Generate EIP-712 signature
    log::info!("Generating EIP-712 hash");

    // Message parameters
    let public_key = hex::decode(
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331",
    )
    .unwrap();
    let contract_addresses = vec![validate_address_from_str(
        "0x56a24bcaE11890353726596fD6f5cABb5a126Df9",
    )?];
    let start_timestamp = 1748252823;
    let duration_days = 10;
    match sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contract_addresses,
        start_timestamp,
        duration_days,
    ) {
        Ok(hash) => log::info!("Hash generated: {} bytes", hash.len()),
        Err(e) => log::info!("Hash generation error: {}", e),
    }

    Ok(())
}
