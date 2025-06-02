//! # Minimal EIP-712 Signing
//!
//! Generate an EIP-712 signature for user decryption.
//!
//! ```bash
//! cargo run --example minimal-eip712-signing
//! ```

use fhevm_sdk::{FhevmSdkBuilder, Result, utils::validate_address_from_str};

fn main() -> Result<()> {
    println!("✍️ Generating EIP-712 signature...");

    // Setup SDK
    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory("./keys")
        .with_gateway_chain_id(54321)
        .with_host_chain_id(12345)
        .with_gateway_contract("Decryption", "0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
        .with_gateway_contract(
            "input-verifier",
            "0xc9bAE822fE6793e3B456144AdB776D5A318CB71e",
        )
        .with_host_contract("acl", "0x9999999999999999999999999999999999999999")
        .build()?;

    // Generate signature
    let result = sdk.generate_eip712_for_user_decrypt(
        "2000000000000000750f4e54713eae622dfeb01809290183a447e2b277e89d2c6a681af1aa5b2c2b",
        &[validate_address_from_str(
            "0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46",
        )?],
        1748870511,
        10,
        Some("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"),
        None,
    )?;

    println!("✅ EIP-712 signature generated!");
    println!("   Hash: 0x{}", hex::encode(result.hash));
    if let Ok(sig) = result.require_signature() {
        println!("   Signature: 0x{}", hex::encode(sig));
    }

    Ok(())
}
