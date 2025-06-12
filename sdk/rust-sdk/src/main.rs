use std::path;

use gateway_sdk::utils::validate_address_from_str;
use gateway_sdk::{EncryptedInput, FhevmError, FhevmSdk, FhevmSdkBuilder};

use alloy::primitives::address;
use gateway_sdk::EncryptedInputBuilder;
use gateway_sdk::logging::{self, LogConfig, LogFormat};
use tracing::{error, info, warn};

use tracing::Level;

fn main() -> Result<(), FhevmError> {
    // Initialize logging (if needed)

    let config = LogConfig {
        level: Level::INFO,
        show_file_line: true,
        show_thread_ids: false,
        format: LogFormat::Compact,
    };

    logging::init_with_config(config);

    info!("FHEVM SDK Demo");

    // APPROACH 1: Using the builder pattern
    info!("=== Builder Pattern Approach ===");
    let mut sdk_from_builder = create_sdk_with_builder()?;
    demo_sdk_functionality(&mut sdk_from_builder)?;

    info!("FHEVM SDK Demo completed");
    Ok(())
}

/// Create an SDK instance using the builder pattern
fn create_sdk_with_builder() -> Result<FhevmSdk, FhevmError> {
    info!("Creating SDK using builder pattern");

    let sdk = create_sample_builder().build()?;
    info!("SDK successfully created with builder");

    Ok(sdk)
}

/// Create a sample builder with test configuration
fn create_sample_builder() -> FhevmSdkBuilder {
    FhevmSdkBuilder::new()
        .with_keys_directory(path::PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111) // Example: Ethereum Sepolia
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
}

/// Demonstrate SDK functionality
fn demo_sdk_functionality(sdk: &mut FhevmSdk) -> Result<(), FhevmError> {
    // Set up test addresses
    let contract_address = address!("0x7777777777777777777777777777777777777777");
    let user_address = address!("0x8888888888888888888888888888888888888888");

    // Example: Encrypt a value
    info!("Encrypting value 18446744073709550042");

    // Create an input builder and explicitly type it
    let mut builder: EncryptedInputBuilder = match sdk.create_input_builder() {
        Ok(b) => b,
        Err(e) => {
            info!("Error creating input builder: {}", e);
            return Err(e);
        }
    };

    // Add a value
    if let Err(e) = builder.add_u64(18446744073709550042) {
        info!("Error adding value: {}", e);
        return Err(e);
    }

    // Encrypt for a specific contract and user
    let encrypted: EncryptedInput =
        builder.encrypt_and_prove_for(contract_address, user_address)?;
    info!("Encryption successful!");
    info!("  - Handles: {}", encrypted.handles.len());
    info!("  - Ciphertext size: {} bytes", encrypted.ciphertext.len());

    let handle_vecs: Vec<Vec<u8>> = encrypted
        .handles
        .iter()
        .map(|handle| handle.to_vec())
        .collect();

    // Example: Generate EIP-712 signature
    info!("Generating EIP-712 hash");

    // Message parameters
    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";

    let contract_addresses = vec![validate_address_from_str(
        "0x56a24bcaE11890353726596fD6f5cABb5a126Df9",
    )?];
    let start_timestamp = 1748252823;
    let duration_days = 10;

    let wallet_private_key = "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f";

    // Example 1: Generate EIP-712 hash only (no signing)
    info!("--- Example 1: Hash Only ---");
    match sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contract_addresses,
        start_timestamp,
        duration_days,
        None, // No wallet key
        None, // No verification
    ) {
        Ok(result) => {
            info!("✅ EIP-712 hash generated successfully");
            info!("   Hash: {}", result.hash);
            info!("   Signed: {}", result.is_signed());
            info!("   Verification status: {}", result.verification_status());
        }
        Err(e) => error!("❌ Hash generation error: {}", e),
    }

    // Example 2: Generate hash and sign (no verification)
    info!("--- Example 2: Hash + Sign (Fast) ---");
    match sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contract_addresses,
        start_timestamp,
        duration_days,
        Some(wallet_private_key), // With wallet key
        None,                     // No verification (default, fast)
    ) {
        Ok(result) => {
            info!("✅ EIP-712 hash and signature generated successfully");
            info!("   Hash: {}", result.hash);
            info!("   Signed: {}", result.is_signed());
            info!("   Signer: {}", result.signer.unwrap_or_default());
            info!("   Verification status: {}", result.verification_status());

            if let Ok(signature) = result.require_signature() {
                info!("   Signature: 0x{}", hex::encode(signature));
            }
        }
        Err(e) => error!("❌ Signing error: {}", e),
    }

    // Example 3: Generate, sign, and verify (full process)
    info!("--- Example 3: Hash + Sign + Verify (Full) ---");
    match sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contract_addresses,
        start_timestamp,
        duration_days,
        Some(wallet_private_key), // With wallet key
        Some(true),               // With verification
    ) {
        Ok(result) => {
            info!("✅ Full EIP-712 process completed");
            info!("   Hash: {}", result.hash);
            info!("   Signed: {}", result.is_signed());
            info!("   Signer: {}", result.signer.unwrap_or_default());
            info!(
                "   Verification attempted: {}",
                result.was_verification_attempted()
            );
            info!("   Verification status: {}", result.verification_status());

            if result.is_verified() {
                info!("   🎉 Signature verified successfully!");
            } else if result.was_verification_attempted() {
                warn!("   ⚠️ Signature verification failed");
            }

            // Demonstrate error handling for verification
            match result.ensure_verified() {
                Ok(()) => info!("   ✅ Verification check passed"),
                Err(e) => warn!("   ⚠️ Verification check failed: {}", e),
            }
        }
        Err(e) => error!("❌ Full process error: {}", e),
    }

    // Generate user decrypt calldata using the builder pattern
    info!("Generating user decrypt calldata");

    let signature = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12";

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
            info!("✅ Calldata generated: {} bytes", calldata.len());
            info!(
                "   First 32 bytes: 0x{}",
                hex::encode(&calldata[..32.min(calldata.len())])
            );
        }
        Err(e) => info!("❌ Calldata generation error: {}", e),
    }

    Ok(())
}
