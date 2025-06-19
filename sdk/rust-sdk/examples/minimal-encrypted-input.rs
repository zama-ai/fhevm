//! # Minimal Encrypted Input
//!
//! Encrypt a single value with the associated ZkPok
//!
//! ```bash
//! cargo run --example minimal-encrypted-input
//! ```

use gateway_sdk::{FhevmSdkBuilder, Result, utils::validate_address_from_str};

fn main() -> Result<()> {
    println!("🔐 Creating encrypted input...");

    // Setup SDK
    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory("./keys")
        .with_gateway_chain_id(54321)
        .with_host_chain_id(12345)
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    // Create encrypted input
    let mut builder = sdk.create_input_builder()?;
    builder.add_u8(42)?;

    let encrypted = builder.encrypt_and_prove_for(
        validate_address_from_str("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46")?,
        validate_address_from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80")?,
    )?;

    println!("✅ Value 42 encrypted!");
    println!("   Handle: 0x{}", hex::encode(&encrypted.handles[0]));
    println!("   Ciphertext: {} bytes", encrypted.ciphertext.len());

    Ok(())
}
