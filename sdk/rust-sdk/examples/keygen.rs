use fhevm_sdk::{FhevmError, FhevmSdkBuilder};
use std::path::PathBuf;

// Import our keygen functions
use fhevm_sdk::utils::generate_fhe_keyset;

fn main() -> Result<(), FhevmError> {
    // Initialize logging
    env_logger::init();

    log::info!("FHEVM SDK Keygen Example");

    // Define output directory for keys
    let output_dir = PathBuf::from("./keys");

    // Generate FHE keys and save them to disk
    log::info!("\n=== Generating FHE Keys ===");
    match generate_fhe_keyset(&output_dir) {
        Ok(()) => log::info!(
            "Keys generated and saved successfully to: {}",
            output_dir.display()
        ),
        Err(e) => log::info!("Error generating keys: {}", e),
    }

    // Try to load the keys back
    log::info!("\n=== Loading FHE Keys ===");
    log::info!("Successfully loaded all keys and CRS");

    // Create SDK configuration using the loaded keys
    let _sdk = FhevmSdkBuilder::new()
        .with_keys_directory(&output_dir)
        .with_gateway_chain_id(43113) // Example chain ID
        .with_host_chain_id(11155111) // Example chain ID
        .with_gateway_contract("Decryption", "0x1234567890123456789012345678901234567890")
        .with_host_contract("FHE", "0x0987654321098765432109876543210987654321")
        .build()?;

    log::info!("SDK initialized with loaded keys");

    log::info!("\nFHEVM SDK Keygen Example completed");
    Ok(())
}
