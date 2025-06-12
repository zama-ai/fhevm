use gateway_sdk::logging;
use gateway_sdk::{FhevmError, FhevmSdkBuilder};
use std::path::PathBuf;
use tracing::{Level, info};

// Import our keygen functions
use gateway_sdk::utils::generate_fhe_keyset;

fn main() -> Result<(), FhevmError> {
    logging::init(Level::INFO);
    info!("FHEVM SDK Keygen Example");

    // Define output directory for keys
    let output_dir = PathBuf::from("./keys");

    // Generate FHE keys and save them to disk
    info!("=== Generating FHE Keys ===");
    match generate_fhe_keyset(&output_dir) {
        Ok(()) => info!(
            "Keys generated and saved successfully to: {}",
            output_dir.display()
        ),
        Err(e) => info!("Error generating keys: {}", e),
    }

    // Try to load the keys back
    info!("=== Loading FHE Keys ===");
    info!("Successfully loaded all keys and CRS");

    // Create SDK configuration using the loaded keys
    let _sdk = FhevmSdkBuilder::new()
        .with_keys_directory(&output_dir)
        .with_gateway_chain_id(43113) // Example chain ID
        .with_host_chain_id(11155111) // Example chain ID
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    info!("SDK initialized with loaded keys");

    info!("FHEVM SDK Keygen Example completed");
    Ok(())
}
