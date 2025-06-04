//! Example of creating and using encrypted inputs with FHEVM

use std::sync::Arc;

use alloy::primitives::address;
use fhevm_sdk::{
    FhevmError, Result,
    encryption::{
        EncryptedInputBuilder, input::get_default_encryption_parameters,
        primitives::create_encryption_parameters,
    },
    logging,
};
use serde_json::json;

/// Example: Basic encryption of a boolean value
fn example_encrypt_bool() -> Result<()> {
    log::info!("Example 1: Encrypting a boolean value");

    // Set up test addresses
    let contract_address = address!("0x7777777777777777777777777777777777777777");
    let user_address = address!("0x8888888888888888888888888888888888888888");
    let acl_address = address!("0x9999999999999999999999999999999999999999");
    let chain_id = 1;

    // Create or load encryption parameters
    let (public_key, _, _, crs) = get_default_encryption_parameters().unwrap();

    // Create input builder
    let mut input_builder = EncryptedInputBuilder::new(
        acl_address,
        Arc::new(public_key.clone()),
        Arc::new(crs.clone()),
        chain_id,
    );

    // Add a boolean value
    input_builder.add_bool(true)?;

    // Generate the encrypted value with proof
    let encrypted_value = input_builder.encrypt_for(contract_address, user_address)?;

    log::info!(
        "  Encrypted boolean value (size: {} bytes)",
        encrypted_value.ciphertext.len()
    );

    Ok(())
}

fn example_encrypt_u64() -> Result<()> {
    log::info!("Example 1: Encrypting a u64 value");

    // Set up test addresses
    let contract_address = address!("0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C");
    let user_address = address!("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80");
    let acl_address = address!("0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c");
    let chain_id = 12345u64; // Ethereum mainnet

    // Create or load encryption parameters
    let default_path = std::path::PathBuf::from("./keys");
    let (public_key, _, _, crs) = create_encryption_parameters(&default_path).unwrap();

    // Create input builder
    let mut input_builder = EncryptedInputBuilder::new(
        acl_address,
        Arc::new(public_key.clone()),
        Arc::new(crs.clone()),
        chain_id,
    );

    // Add a u64 value
    input_builder.add_u64(18446744073709550042)?;

    // Generate the encrypted value with proof
    let encrypted_value = input_builder.encrypt_for(contract_address, user_address)?;

    log::info!("Encryption successful!");
    log::info!("  - Handles: {}", encrypted_value.handles.len());
    log::info!(
        "  - Ciphertext size: {} bytes",
        encrypted_value.ciphertext.len()
    );

    let payload = json!({
        "contractAddress": format!("{:#x}", contract_address),
        "userAddress": format!("{:#x}", user_address),
        "ciphertextWithInputVerification": format!("0x{}", hex::encode(&encrypted_value.ciphertext)),
        "contractChainId": format!("0x{:x}", chain_id)
    });

    // Generate a curl command
    let relayer_url = "http://localhost:3000/v1/input-proof";

    let curl_file_command = format!(
        "curl -X POST {} \\\n  -H \"Content-Type: application/json\" \\\n  -d @payload.json",
        relayer_url
    );

    log::info!("\n Curl command using the saved payload file:");
    log::info!("{}", curl_file_command);

    // If you want to save the payload to a file for later use
    let json_string = match serde_json::to_string_pretty(&payload) {
        Ok(s) => s,
        Err(e) => {
            return Err(FhevmError::InvalidParams(format!(
                "JSON serialization failed: {}",
                e
            )));
        }
    };
    std::fs::write("payload.json", json_string)?;

    log::info!("\nPayload saved to payload.json");

    Ok(())
}

/// Example: Creating an encrypted input with multiple values
fn example_create_complex_input() -> Result<()> {
    log::info!("Example 2: Creating an encrypted input with multiple values");

    // Set up test addresses
    let contract_address = address!("0x7777777777777777777777777777777777777777");
    let user_address = address!("0x8888888888888888888888888888888888888888");
    let acl_address = address!("0x9999999999999999999999999999999999999999");
    let chain_id = 1; // Ethereum mainnet

    // Create or load encryption parameters
    let (public_key, _, _, crs) = get_default_encryption_parameters().unwrap();

    // Create input builder
    let mut input_builder = EncryptedInputBuilder::new(
        acl_address,
        Arc::new(public_key.clone()),
        Arc::new(crs.clone()),
        chain_id,
    );

    // Add multiple values of different types
    input_builder.add_bool(true)?;
    input_builder.add_u8(123)?;
    input_builder.add_u64(9999999)?;
    input_builder.add_address("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")?;

    // Get the bit widths for reference
    let bit_widths = input_builder.get_bits();
    log::info!("  Added values with bit widths: {:?}", bit_widths);

    // Generate the encrypted value with proof
    let encrypted_value = input_builder.encrypt_for(contract_address, user_address)?;

    log::info!(
        "  Encrypted complex input (size: {} bytes)",
        encrypted_value.ciphertext.len()
    );

    Ok(())
}

/// Main function (required for examples directory)
fn main() -> Result<()> {
    logging::init_from_env(log::LevelFilter::Info);
    // Run all examples
    log::info!("FHEVM SDK Encryption Examples");
    log::info!("==============================");
    log::info!("Example: Encrypt a boolean value");
    example_encrypt_bool()?;

    log::info!("Example: Encrypt a complex value");
    example_create_complex_input()?;

    // Example: Encrypt a u64 value with a curl command ready to
    // be sent to the relayer
    log::info!("Example: Encrypt a u64 value, with curl command example to relayer");
    example_encrypt_u64()?;

    Ok(())
}
