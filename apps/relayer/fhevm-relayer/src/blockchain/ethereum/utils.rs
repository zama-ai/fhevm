use crate::core::errors::EventProcessingError;
use alloy::primitives::FixedBytes;
use alloy::rpc::types::Log;
use alloy::signers::local::PrivateKeySigner;
use eyre::eyre;
use std::str::FromStr;

pub fn extract_event_signature(log: &Log) -> Result<&FixedBytes<32>, EventProcessingError> {
    log.inner
        .data
        .topics()
        .first()
        .ok_or(EventProcessingError::MissingTopic)
}

/// Parse a private key string, handling 0x prefix
///
/// # Arguments
/// * `key` - The private key as a string (with or without 0x prefix)
///
/// # Returns
/// * `Ok(PrivateKeySigner)` - Successfully parsed private key
/// * `Err(eyre::Report)` - If the key format is invalid
pub fn parse_private_key(key: &str) -> eyre::Result<PrivateKeySigner> {
    // Remove 0x prefix if present using strip_prefix
    let key_without_prefix = key.strip_prefix("0x").unwrap_or(key);

    // Validate key length
    if key_without_prefix.len() != 64 {
        return Err(eyre!(
            "Private key has invalid length ({} chars), expected 64 hex chars or 66 with 0x prefix",
            key.len()
        ));
    }

    // Parse key to signer
    PrivateKeySigner::from_str(key_without_prefix)
        .map_err(|e| eyre!("Failed to parse private key: {}", e))
}
