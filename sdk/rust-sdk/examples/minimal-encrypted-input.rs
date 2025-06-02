//! # Minimal Encrypted Input
//!
//! Encrypt a single value for a smart contract.
//!
//! ```bash
//! cargo run --example minimal-encrypted-input
//! ```

use fhevm_sdk::{FhevmSdkBuilder, Result, utils::validate_address_from_str};

fn main() -> Result<()> {
    println!("🔐 Creating encrypted input...");

    // Setup SDK
    let mut sdk = FhevmSdkBuilder::new()
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

    // Create encrypted input
    let mut builder = sdk.create_input_builder()?;
    builder.add_u8(42)?;

    let encrypted = builder.encrypt_for(
        validate_address_from_str("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46")?,
        validate_address_from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80")?,
    )?;

    println!("✅ Value 42 encrypted!");
    println!("   Handle: 0x{}", hex::encode(&encrypted.handles[0]));
    println!("   Ciphertext: {} bytes", encrypted.ciphertext.len());

    Ok(())
}
