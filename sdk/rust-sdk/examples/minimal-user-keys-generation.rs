//! # Minimal Key Generation
//!
//! Generate a cryptobox keypair for user decryption.
//!
//! ```bash
//! cargo run --example minimal-key-generation
//! ```

use gateway_sdk::{Result, signature::generate_keypair};

fn main() -> Result<()> {
    println!("🔑 Generating keypair...");

    let keypair = generate_keypair()?;

    println!("✅ Keypair generated:");
    println!("   Public:  {}", keypair.public_key);
    println!("   Private: {}", keypair.private_key);

    Ok(())
}
