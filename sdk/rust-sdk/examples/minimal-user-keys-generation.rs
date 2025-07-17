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
    println!(
        "   Public:  {}\n   Len: {},",
        keypair.public_key[..32].to_string() + "...",
        (keypair.public_key.bytes().len() / 2 - 1)
    );
    println!(
        "   Private: {}\n   Len: {},",
        keypair.private_key[..32].to_string() + "...",
        (keypair.private_key.bytes().len() / 2 - 1)
    );

    Ok(())
}
