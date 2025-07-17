use gateway_sdk::{FhevmError, FhevmSdkBuilder, utils::parse_hex_string};
use std::path::PathBuf;
use tracing::{Level, info};

fn main() -> Result<(), FhevmError> {
    // Initialize logging
    gateway_sdk::logging::init(Level::INFO);

    info!("Starting Public Decryption Example");

    // 1. Create SDK instance
    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory(PathBuf::from("./keys"))
        .with_gateway_chain_id(43113) // Avalanche testnet
        .with_host_chain_id(11155111) // Ethereum Sepolia
        .with_decryption_contract("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    let handle: Vec<u8> = parse_hex_string(
        "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400",
        "handle",
    )?
    .into();

    let calldata = sdk
        .create_public_decrypt_request_builder()
        .with_handles_from_bytes(&[handle])?
        .build_and_generate_calldata()?;

    info!(
        "Calldata for public decryption request len: {}",
        calldata.len()
    );

    info!("Public Decryption Example completed successfully");
    Ok(())
}
