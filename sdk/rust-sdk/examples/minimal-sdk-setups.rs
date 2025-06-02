//! # Minimal SDK Setup
//!
//! The simplest possible FHEVM SDK setup example.
//!
//! ```bash
//! cargo run --example minimal-sdk-setup
//! ```

use fhevm_sdk::{FhevmSdkBuilder, Result};

fn main() -> Result<()> {
    println!("🚀 Setting up FHEVM SDK...");

    let _sdk = FhevmSdkBuilder::new()
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

    println!("✅ SDK ready!");
    Ok(())
}
