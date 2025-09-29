use alloy::primitives::address;
use gateway_sdk::logging::{self, LogConfig, LogFormat};
use gateway_sdk::{FhevmSdk, FhevmSdkBuilder, Result};
use std::path;
use tracing::{Level, info, warn};

fn main() -> Result<()> {
    // Initialize logging
    logging::init_with_config(LogConfig {
        level: Level::INFO,
        show_file_line: true,
        show_thread_ids: false,
        format: LogFormat::Compact,
    });

    info!("🚀 FHEVM SDK Demo");

    // Create SDK and run demos
    let mut sdk = create_sdk()?;
    info!("✅ SDK created successfully\n");

    // Show configuration
    println!("{}", sdk.configuration_status());
    println!("\n{}", "=".repeat(60));

    // Run all demos
    demo_encryption(&mut sdk)?;
    println!("\n{}", "=".repeat(60));

    demo_eip712_signatures(&sdk)?;
    println!("\n{}", "=".repeat(60));

    demo_decrypt_calldata(&mut sdk)?;

    info!("\n✅ All demos completed successfully!");
    Ok(())
}

fn create_sdk() -> Result<FhevmSdk> {
    FhevmSdkBuilder::new()
        .with_keys_directory(path::PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111)
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()
}

/// Demo 1: Encryption with real data
fn demo_encryption(sdk: &mut FhevmSdk) -> Result<()> {
    info!("\n📦 Demo 1: Encrypting Data");

    let contract_address = address!("0x7777777777777777777777777777777777777777");
    let user_address = address!("0x8888888888888888888888888888888888888888");

    // Create encrypted input
    let mut builder = sdk.create_input_builder()?;
    builder.add_u64(18446744073709550042)?; // Large u64 value
    builder.add_bool(true)?;
    builder.add_u32(42)?;

    let encrypted = builder.encrypt_and_prove_for(contract_address, user_address)?;

    info!("✅ Encryption successful!");
    info!("   - Values encrypted: {}", encrypted.handles.len());
    info!("   - Ciphertext size: {} bytes", encrypted.ciphertext.len());
    info!("   - Contract: {}", contract_address);
    info!("   - User: {}", user_address);

    // Show first handle as example
    if let Some(first_handle) = encrypted.handles_as_hex().first() {
        info!(
            "   - Example handle: {}...{}",
            &first_handle[..10],
            &first_handle[first_handle.len() - 6..]
        );
    }

    Ok(())
}

/// Demo 2: EIP-712 Signatures (all three modes)
fn demo_eip712_signatures(sdk: &FhevmSdk) -> Result<()> {
    info!("\n🔏 Demo 2: EIP-712 Signatures");

    // Test data
    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
    let contract_addresses_str = "0x56a24bcaE11890353726596fD6f5cABb5a126Df9";
    let wallet_private_key = "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f";
    let (start_timestamp, duration_days) = (1748252823, 10);

    // Mode 1: Hash only
    info!("\n   Mode 1: Hash Only (No Signing)");
    let hash_only = sdk
        .eip712_builder()
        .with_public_key(public_key)
        .with_contract(contract_addresses_str)?
        .with_validity_period(start_timestamp, duration_days)
        .generate_hash()?;

    info!("   ✅ Hash: {}", hash_only);

    // Mode 2: Hash + Sign (fast, no verification)
    info!("\n   Mode 2: Hash + Sign (Fast)");
    let signed_only = sdk
        .eip712_builder()
        .with_public_key(public_key)
        .with_contract(contract_addresses_str)?
        .with_validity_period(start_timestamp, duration_days)
        .with_private_key(wallet_private_key)
        .generate_and_sign_only()?;

    info!(
        "   ✅ Signed by: {}",
        signed_only.signer.unwrap_or_default()
    );
    if let Ok(sig) = signed_only.require_signature() {
        info!(
            "   ✅ Signature: 0x{}...{}",
            &hex::encode(&sig[..4]),
            &hex::encode(&sig[sig.len() - 4..])
        );
    }

    // Mode 3: Hash + Sign + Verify (complete)
    info!("\n   Mode 3: Hash + Sign + Verify (Full)");
    let verified = sdk
        .eip712_builder()
        .with_public_key(public_key)
        .with_contract(contract_addresses_str)?
        .with_validity_period(start_timestamp, duration_days)
        .with_private_key(wallet_private_key)
        .with_verification(true)
        .generate_and_sign()?;

    if verified.is_verified() {
        info!("   ✅ Signature verified successfully!");
    } else {
        warn!("   ⚠️  Signature verification failed");
    }

    Ok(())
}

/// Demo 3: Generate Decrypt Calldata
fn demo_decrypt_calldata(sdk: &mut FhevmSdk) -> Result<()> {
    info!("\n🔐 Demo 3: Decrypt Calldata Generation");

    // Create some test handles
    let handles = vec![vec![1u8; 32], vec![2u8; 32]];
    let contract_addresses = vec![address!("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1")];
    let user_address = "0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1";

    // Public decrypt calldata
    info!("\n   Public Decrypt:");
    let public_calldata = sdk.generate_public_decrypt_calldata(&handles)?;
    info!("   ✅ Calldata size: {} bytes", public_calldata.len());
    info!(
        "   ✅ Function selector: 0x{}",
        hex::encode(&public_calldata[..4])
    );

    // User decrypt calldata (with real encryption)
    info!("\n   User Decrypt (with real data):");

    // First, create real encrypted data
    let mut builder = sdk.create_input_builder()?;
    builder.add_u32(123)?;
    builder.add_bool(true)?;

    let encrypted = builder.encrypt_and_prove_for(
        contract_addresses[0],
        address!("0x8888888888888888888888888888888888888888"),
    )?;

    // Convert handles to Vec<Vec<u8>>
    let handle_vecs: Vec<Vec<u8>> = encrypted.handles.iter().map(|h| h.to_vec()).collect();

    // Create user decrypt request
    let public_key =
        "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
    let signature = "0x".to_owned() + &"ab".repeat(65);

    let user_calldata = sdk
        .create_user_decrypt_request_builder()
        .with_handles_from_bytes(&handle_vecs, &contract_addresses)?
        .with_user_address_from_str(user_address)?
        .with_signature_from_hex(&signature)?
        .with_public_key_from_hex(public_key)?
        .with_validity(1640995200, 30)?
        .build_and_generate_calldata()?;

    info!("   ✅ Calldata size: {} bytes", user_calldata.len());
    info!(
        "   ✅ Function selector: 0x{}",
        hex::encode(&user_calldata[..4])
    );
    info!("   ✅ Handles included: {}", handle_vecs.len());

    // Verify proof calldata
    info!("\n   Verify Proof:");
    let verify_calldata = sdk.generate_verify_proof_calldata(&encrypted)?;
    info!("   ✅ Calldata size: {} bytes", verify_calldata.len());
    info!("   ✅ For contract: {}", encrypted.contract_address);

    Ok(())
}
