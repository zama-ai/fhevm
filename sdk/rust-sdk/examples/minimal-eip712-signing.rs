//! # Minimal EIP-712 Signing
//!
//! Generate an EIP-712 signature for user decryption.
//!
//! ```bash
//! cargo run --example minimal-eip712-signing
//! ```

use gateway_sdk::{FhevmSdkBuilder, Result};

fn main() -> Result<()> {
    println!("✍️ Generating EIP-712 signature...");

    // Setup SDK
    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory("./keys")
        .with_gateway_chain_id(54321)
        .with_host_chain_id(12345)
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    // Generate signature

    let result = sdk
        .create_eip712_signature_builder()
        .public_key(
            "2000000000000000750f4e54713eae622dfeb01809290183a447e2b277e89d2c6a681af1aa5b2c2b",
        )
        .add_contract("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46")?
        .validity_period(1748870511, 10)
        .sign_with("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .generate_and_sign()?;

    println!("✅ EIP-712 signature generated!");
    println!("   Hash: 0x{}", hex::encode(result.hash));
    if let Ok(sig) = result.require_signature() {
        println!("   Signature: 0x{}", hex::encode(sig));
    }

    Ok(())
}
