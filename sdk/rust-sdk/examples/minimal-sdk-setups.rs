//! # Minimal SDK Setup
//!
//! The simplest possible FHEVM SDK setup example.
//!
//! ```bash
//! cargo run --example minimal-sdk-setup
//! ```

use gateway_sdk::{FhevmSdkBuilder, Result};

fn main() -> Result<()> {
    println!("🚀 Setting up FHEVM SDK...");

    let _sdk = FhevmSdkBuilder::new()
        .with_keys_directory("./keys")
        .with_gateway_chain_id(54321)
        .with_host_chain_id(12345)
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    println!("✅ SDK ready!");
    Ok(())
}
