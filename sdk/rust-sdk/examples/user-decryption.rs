use alloy::primitives::address;
use gateway_sdk::utils::validate_address_from_str;
use gateway_sdk::{FhevmError, FhevmSdk, FhevmSdkBuilder};
use serde_json::json;
use std::path::PathBuf;

/// Complete user decrypt example - from encrypted input to curl command
///
/// This example demonstrates the full workflow:
/// 1. Create encrypted inputs (handles)
/// 2. Generate EIP-712 signature for user decrypt
/// 3. Generate user decrypt calldata
/// 4. Prepare curl command for relayer
fn main() -> Result<(), FhevmError> {
    // Initialize logging
    gateway_sdk::logging::init_from_env(log::LevelFilter::Info);

    log::info!("🚀 Complete User Decrypt Example");

    // Create SDK instance with proper configuration
    let mut sdk = create_configured_sdk()?;

    // Step 1: Create some encrypted inputs to get handles
    log::info!("=== Step 1: Creating Encrypted Inputs ===");
    let handles = create_sample_encrypted_inputs(&mut sdk)?;

    // Step 2: Generate EIP-712 signature for user decrypt
    log::info!("=== Step 2: Generating EIP-712 Signature ===");
    let eip712_result = generate_user_decrypt_signature(&sdk)?;

    // Step 3: Generate user decrypt calldata
    log::info!("=== Step 3: Generating User Decrypt Calldata ===");
    let calldata = generate_user_decrypt_calldata(&sdk, &handles, &eip712_result)?;
    log::info!(
        "Generated user decrypt calldata ({} bytes): 0x{}",
        calldata.len(),
        hex::encode(&calldata[..std::cmp::min(64, calldata.len())])
    );

    // Step 4: Prepare curl command for relayer
    log::info!("=== Step 4: Preparing Relayer Request ===");
    prepare_relayer_curl_command(&handles, &eip712_result)?;

    log::info!("🎉 Complete user decrypt example finished successfully!");
    Ok(())
}

/// Create a properly configured SDK instance
fn create_configured_sdk() -> Result<FhevmSdk, FhevmError> {
    log::info!("Creating SDK with configuration...");

    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory(PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111)
        .with_gateway_contract("Decryption", "0x1234567890123456789012345678901234567890")
        .with_gateway_contract(
            "input-verifier",
            "0xc9bAE822fE6793e3B456144AdB776D5A318CB71e",
        )
        .with_host_contract("ACL", "0x0987654321098765432109876543210987654321")
        .build()?;

    log::info!("✅ SDK configured successfully");
    Ok(sdk)
}

/// Create sample encrypted inputs and return their handles
fn create_sample_encrypted_inputs(sdk: &mut FhevmSdk) -> Result<Vec<Vec<u8>>, FhevmError> {
    log::info!("Creating encrypted inputs...");

    // Set up addresses
    let contract_address = validate_address_from_str("0x7777777777777777777777777777777777777777")?;
    let user_address = validate_address_from_str("0x8888888888888888888888888888888888888888")?;

    // Create input builder
    let mut builder = sdk.create_input_builder()?;

    // Add some sample encrypted values
    builder.add_bool(true)?;
    builder.add_u32(42)?;
    builder.add_u64(18446744073709550042)?;

    // Encrypt for the specific contract and user
    let encrypted_input = builder.encrypt_for(contract_address, user_address)?;

    // Convert handles to Vec<Vec<u8>> format
    let handles: Vec<Vec<u8>> = encrypted_input
        .handles
        .iter()
        .map(|handle| handle.to_vec())
        .collect();

    log::info!("✅ Created {} encrypted handles", handles.len());
    for (i, handle) in handles.iter().enumerate() {
        log::info!("   Handle {}: 0x{}", i, hex::encode(handle));
    }

    Ok(handles)
}

/// Generate EIP-712 signature for user decrypt (matching JS pattern)
fn generate_user_decrypt_signature(
    sdk: &FhevmSdk,
) -> Result<gateway_sdk::signature::Eip712Result, FhevmError> {
    log::info!("Generating EIP-712 signature for user decrypt...");

    // Test parameters (matching the JS example pattern)
    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";

    let contract_addresses = vec![
        validate_address_from_str("0x56a24bcaE11890353726596fD6f5cABb5a126Df9")?,
        validate_address_from_str("0x7777777777777777777777777777777777777777")?,
    ];

    let start_timestamp = 1748252823u64;
    let duration_days = 10u64;

    // Private key for signing (test key - never use in production!)
    let wallet_private_key = "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f";

    // Generate EIP-712 signature with verification
    let eip712_result = sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contract_addresses,
        start_timestamp,
        duration_days,
        Some(wallet_private_key), // With signing
        Some(true),               // With verification
    )?;

    // Validate the result
    if !eip712_result.is_signed() {
        return Err(FhevmError::SignatureError(
            "Failed to generate signature".to_string(),
        ));
    }

    if !eip712_result.is_verified() {
        log::warn!("⚠️ Signature was not verified successfully");
    } else {
        log::info!("✅ Signature generated and verified successfully");
    }

    log::info!("EIP-712 Results:");
    log::info!("   Hash: {}", eip712_result.hash);
    log::info!("   Signer: {}", eip712_result.signer.unwrap_or_default());
    log::info!("   Verification: {}", eip712_result.verification_status());

    if let Ok(signature) = eip712_result.require_signature() {
        log::info!("   Signature: 0x{}", hex::encode(signature));
    }

    Ok(eip712_result)
}

/// Generate user decrypt calldata using the signature
fn generate_user_decrypt_calldata(
    sdk: &FhevmSdk,
    handles: &[Vec<u8>],
    eip712_result: &gateway_sdk::signature::Eip712Result,
) -> Result<Vec<u8>, FhevmError> {
    log::info!("Generating user decrypt calldata...");

    // Parameters for calldata generation
    let user_address = "0xfCefe53c7012a075b8a711df391100d9c431c468"; // Expected signer address
    let contract_addresses = vec![
        address!("0x56a24bcaE11890353726596fD6f5cABb5a126Df9"),
        address!("0x7777777777777777777777777777777777777777"),
    ];

    // Get signature and public key as hex strings
    let signature_bytes = eip712_result.require_signature()?;
    let signature_hex = hex::encode(signature_bytes);

    let public_key_hex =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";

    let start_timestamp = 1748252823u64;
    let duration_days = 10u64;
    match sdk
        .create_user_decrypt_request_builder()
        .add_handles_from_bytes(&handles, &contract_addresses)?
        .user_address_from_str(&user_address.to_string())?
        .signature_from_hex(&signature_hex)?
        .public_key_from_hex(&public_key_hex)?
        .validity(start_timestamp, duration_days)?
        .build_and_generate_calldata()
    {
        Ok(calldata) => {
            log::info!("✅ Calldata generated: {} bytes", calldata.len());
            log::info!(
                "   First 32 bytes: 0x{}",
                hex::encode(&calldata[..32.min(calldata.len())])
            );
            return Ok(calldata);
        }
        Err(e) => {
            log::error!("❌ Calldata generation error: {}", e);
            return Err(e);
        }
    }
}

/// Prepare curl command for relayer (matching JS userDecrypt call)
fn prepare_relayer_curl_command(
    handles: &[Vec<u8>],
    eip712_result: &gateway_sdk::signature::Eip712Result,
) -> Result<(), FhevmError> {
    log::info!("Preparing relayer curl command...");

    // Prepare handle-contract pairs (matching JS HandleContractPairs format)
    let handle_contract_pairs: Vec<serde_json::Value> = handles
        .iter()
        .enumerate()
        .map(|(i, handle)| {
            json!({
                "ctHandle": format!("0x{}", hex::encode(handle)),
                "contractAddress": if i == 0 {
                    "0x56a24bcaE11890353726596fD6f5cABb5a126Df9"
                } else {
                    "0x7777777777777777777777777777777777777777"
                }
            })
        })
        .collect();

    // Get signature as hex string
    let signature_bytes = eip712_result.require_signature()?;
    let signature_hex = hex::encode(signature_bytes);

    // Prepare the request payload (matching JS userDecrypt API)
    let payload = json!({
        "handleContractPairs": handle_contract_pairs,
        "requestValidity": {
            "startTimestamp": "1748252823",
            "durationDays": "10"
        },
        "contractsChainId": "11155111", // Ethereum Sepolia
        "contractAddresses": [
            "0x56a24bcaE11890353726596fD6f5cABb5a126Df9",
            "0x7777777777777777777777777777777777777777"
        ],
        "userAddress": "0xfCefe53c7012a075b8a711df391100d9c431c468",
        "signature": signature_hex,
        "publicKey": "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331"
    });

    // Also show compact version
    let compact_payload = serde_json::to_string(&payload).unwrap();
    let compact_curl = format!(
        r#"curl -X POST 'http://localhost:3000/v1/user-decrypt' -H 'Content-Type: application/json' -d '{}'"#,
        compact_payload
    );

    log::info!("📋 Compact version:");
    println!("{}", compact_curl);
    log::info!("");

    // Show payload details
    log::info!("📊 Payload details:");
    log::info!("   Handle pairs: {}", handle_contract_pairs.len());
    log::info!(
        "   Contract addresses: {}",
        payload["contractAddresses"].as_array().unwrap().len()
    );
    log::info!("   User address: {}", payload["userAddress"]);
    log::info!("   Signature length: {} chars", signature_hex.len());
    log::info!(
        "   Public key length: {} chars",
        payload["publicKey"].as_str().unwrap().len()
    );

    // Validation check
    if signature_hex.len() != 130 {
        // 65 bytes * 2 hex chars
        log::warn!(
            "⚠️ Signature length is unexpected: {} (expected 130)",
            signature_hex.len()
        );
    }

    if payload["publicKey"].as_str().unwrap().len() != 80 {
        // 40 bytes * 2 hex chars
        log::warn!(
            "⚠️ Public key length is unexpected: {} (expected 64)",
            payload["publicKey"].as_str().unwrap().len()
        );
    }

    log::info!("✅ Relayer curl command prepared successfully");

    Ok(())
}

/// Demonstrate error handling scenarios
#[allow(dead_code)]
fn demonstrate_error_scenarios(sdk: &FhevmSdk) -> Result<(), FhevmError> {
    log::info!("=== Error Scenarios Demo ===");

    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
    let contracts = vec![validate_address_from_str(
        "0x56a24bcaE11890353726596fD6f5cABb5a126Df9",
    )?];

    // Scenario 1: Try to verify without wallet key
    log::info!("Testing verification without wallet key...");
    match sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contracts,
        1748252823,
        10,
        None,
        Some(true),
    ) {
        Ok(_) => log::error!("❌ Should have failed"),
        Err(e) => log::info!("✅ Correctly caught error: {}", e),
    }

    Ok(())
}

/// Performance comparison between different approaches
#[allow(dead_code)]
fn performance_comparison(sdk: &FhevmSdk) -> Result<(), FhevmError> {
    log::info!("=== Performance Comparison ===");

    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
    let contracts = vec![validate_address_from_str(
        "0x56a24bcaE11890353726596fD6f5cABb5a126Df9",
    )?];
    let wallet_key = "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f";

    // Benchmark: Hash only
    let start = std::time::Instant::now();
    let _ =
        sdk.generate_eip712_for_user_decrypt(&public_key, &contracts, 1748252823, 10, None, None)?;
    let hash_time = start.elapsed();

    // Benchmark: Hash + Sign
    let start = std::time::Instant::now();
    let _ = sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contracts,
        1748252823,
        10,
        Some(wallet_key),
        None,
    )?;
    let sign_time = start.elapsed();

    // Benchmark: Hash + Sign + Verify
    let start = std::time::Instant::now();
    let _ = sdk.generate_eip712_for_user_decrypt(
        &public_key,
        &contracts,
        1748252823,
        10,
        Some(wallet_key),
        Some(true),
    )?;
    let verify_time = start.elapsed();

    log::info!("⚡ Performance Results:");
    log::info!("   Hash only:        {:?}", hash_time);
    log::info!(
        "   Hash + Sign:      {:?} (+ {:?})",
        sign_time,
        sign_time.saturating_sub(hash_time)
    );
    log::info!(
        "   Hash + Sign + Verify: {:?} (+ {:?})",
        verify_time,
        verify_time.saturating_sub(sign_time)
    );

    Ok(())
}
