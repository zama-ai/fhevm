use alloy::{hex, primitives::U256};
use anyhow::anyhow;
use tfhe::FheTypes;

/// Extracts the FHE type from handle bytes.
pub fn extract_fhe_type_from_handle(bytes: &[u8]) -> anyhow::Result<FheTypes> {
    // Format: keccak256(keccak256(bundleCiphertext)+index)[0:29] + index + type + version
    // - Last byte (31): Version (currently 0)
    // - Second-to-last byte (30): FHE Type
    // - Third-to-last byte (29): Handle index
    // - Rest (0-28): Hash data
    if bytes.len() >= 32 {
        let type_byte = bytes[30]; // FHE type is at index 30
        FheTypes::try_from(type_byte as i32).map_err(anyhow::Error::from)
    } else {
        Err(anyhow!(
            "Handle too short: {} bytes, expected 32 bytes",
            bytes.len()
        ))
    }
}

/// Converts a U256 request ID to a valid hex format that KMS Core expects.
///
/// The KMS Core expects a hex string that decodes to exactly 32 bytes.
pub fn format_request_id(request_id: U256) -> String {
    // Convert U256 to big-endian bytes
    let bytes = request_id.to_be_bytes::<32>();
    // Encode as hex string
    hex::encode(bytes)
}
