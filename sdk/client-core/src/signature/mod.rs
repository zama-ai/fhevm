//! Signature module for FHEVM client core.
//!
//! Provides EIP-712 signature generation, verification, and key utilities.
//! Uses synchronous signing only — no tokio dependency.

use crate::{ClientCoreError, Result};
use alloy::primitives::{Address, B256, Bytes};
use alloy::signers::local::PrivateKeySigner;
use serde::{Deserialize, Serialize};

pub mod eip712;

pub use self::eip712::{Eip712Config, Eip712Result, Eip712SignatureBuilder};

/// Keypair for EIP-712 signing operations.
///
/// Construct via [`Keypair::new`] which validates both keys.
/// The private key is redacted from `Debug` output and excluded from serialization.
#[derive(Clone, Serialize, Deserialize)]
pub struct Keypair {
    public_key: String,
    #[serde(skip_serializing)]
    private_key: String,
}

impl std::fmt::Debug for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keypair")
            .field("public_key", &self.public_key)
            .field("private_key", &"[REDACTED]")
            .finish()
    }
}

impl Keypair {
    /// Create a new keypair with validation.
    ///
    /// Both keys must be valid hex strings (with optional 0x prefix).
    /// The private key must be exactly 64 hex characters.
    pub fn new(public_key: String, private_key: String) -> Result<Self> {
        if public_key.is_empty() {
            return Err(ClientCoreError::KeyError(
                "Public key cannot be empty".to_string(),
            ));
        }
        let cleaned_pub = public_key.strip_prefix("0x").unwrap_or(&public_key);
        if !cleaned_pub.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ClientCoreError::KeyError(
                "Public key contains non-hex characters".to_string(),
            ));
        }
        validate_private_key_format(&private_key)?;
        Ok(Self {
            public_key,
            private_key,
        })
    }

    /// Get the public key (hex-encoded).
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// Get the private key (hex-encoded).
    pub fn private_key(&self) -> &str {
        &self.private_key
    }
}

/// Validate private key format (64 hex characters, optionally 0x-prefixed).
pub fn validate_private_key_format(private_key: &str) -> Result<()> {
    if private_key.is_empty() {
        return Err(ClientCoreError::InvalidParams(
            "Private key cannot be empty".to_string(),
        ));
    }

    let cleaned_key = private_key.strip_prefix("0x").unwrap_or(private_key);

    if cleaned_key.len() != 64 {
        return Err(ClientCoreError::InvalidParams(
            "Invalid private key format (must be 64 hex characters)".to_string(),
        ));
    }

    if !cleaned_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ClientCoreError::InvalidParams(
            "Invalid private key format (contains non-hex characters)".to_string(),
        ));
    }

    Ok(())
}

/// Sign an EIP-712 hash with a private key (synchronous — no tokio).
pub(crate) fn sign_eip712_hash(hash: B256, private_key: &str) -> Result<Bytes> {
    use alloy::signers::SignerSync;
    use std::str::FromStr;

    let private_key_str = private_key.strip_prefix("0x").unwrap_or(private_key);

    let signer = PrivateKeySigner::from_str(private_key_str)
        .map_err(|e| ClientCoreError::SignatureError(format!("Invalid private key: {e}")))?;

    let signature = signer
        .sign_hash_sync(&hash)
        .map_err(|e| ClientCoreError::SignatureError(format!("Failed to sign: {e}")))?;

    Ok(Bytes::from(signature.as_bytes().to_vec()))
}

/// Derive Ethereum address from a private key.
pub fn derive_address_from_private_key(private_key: &str) -> Result<Address> {
    use std::str::FromStr;

    let private_key_str = private_key.strip_prefix("0x").unwrap_or(private_key);

    let signer = PrivateKeySigner::from_str(private_key_str)
        .map_err(|e| ClientCoreError::SignatureError(format!("Invalid private key: {e}")))?;

    Ok(signer.address())
}
