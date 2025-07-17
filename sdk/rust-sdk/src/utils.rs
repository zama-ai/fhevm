use crate::FhevmError;
use crate::Result;
use alloy::primitives::{Address, Bytes};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use tfhe::safe_serialization::{safe_deserialize, safe_deserialize_conformant, safe_serialize};
use tfhe::zk::CompactPkeCrs;
use tfhe::{ClientKey, CompactPublicKey};

use tracing::{debug, info};

// Add validation helper
pub fn validate_address(addr: &Address) -> Result<()> {
    // Check for zero address
    if addr.is_zero() {
        return Err(FhevmError::InvalidParams(
            "Zero address is not allowed".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_address_from_str(addr_str: &str) -> Result<Address> {
    // Check for empty string
    if addr_str.trim().is_empty() {
        return Err(FhevmError::InvalidParams(
            "Address string cannot be empty".to_string(),
        ));
    }
    debug!("Parsing address: {}", addr_str);

    let address = Address::from_str(addr_str.trim()).map_err(|e| {
        FhevmError::InvalidParams(format!("Invalid address format '{}': {}", addr_str, e))
    })?;

    debug!("Parsed address: {}", address);

    // Validate the parsed address
    validate_address(&address)?;

    Ok(address)
}

/// Helper function to parse hex strings with or without 0x prefix
pub fn parse_hex_string(hex_str: &str, field_name: &str) -> Result<Bytes> {
    let cleaned = hex_str.strip_prefix("0x").unwrap_or(hex_str);

    let bytes = hex::decode(cleaned).map_err(|e| {
        FhevmError::InvalidParams(format!("Invalid hex string for {}: {}", field_name, e))
    })?;

    Ok(Bytes::from(bytes))
}

pub fn generate_fhe_keyset(output_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(output_dir)
        .map_err(|e| FhevmError::FileError(format!("Failed to create output directory: {}", e)))?;

    let params = tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    // Indicate which parameters to use for the Compact Public Key encryption
    let cpk_params = tfhe::shortint::parameters::PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    // And parameters allowing to keyswitch/cast to the computation parameters.
    let casting_params =
        tfhe::shortint::parameters::PARAM_KEYSWITCH_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    // Enable the dedicated parameters on the config
    let config = tfhe::ConfigBuilder::with_custom_parameters(params)
        .use_dedicated_compact_public_key_parameters((cpk_params, casting_params))
        .build();

    // The CRS should be generated in an offline phase then shared to all clients and the server
    let crs = CompactPkeCrs::from_config(config, 512).unwrap();

    // Then use TFHE-rs as usual
    let client_key = tfhe::ClientKey::generate(config);
    let server_key = tfhe::ServerKey::new(&client_key);
    let public_key = tfhe::CompactPublicKey::try_new(&client_key).unwrap();

    let mut serialized_pub_key = Vec::new();
    safe_serialize(&public_key, &mut serialized_pub_key, 1 << 30).unwrap();

    let mut serialized_client_key = Vec::new();
    safe_serialize(&client_key, &mut serialized_client_key, 1 << 30).unwrap();

    let mut serialized_server_key = Vec::new();
    safe_serialize(&server_key, &mut serialized_server_key, 1 << 30).unwrap();

    let mut serialized_crs = Vec::new();
    safe_serialize(&crs, &mut serialized_crs, 1 << 30).unwrap();

    std::fs::write(output_dir.join("public_key.bin"), &serialized_pub_key)
        .map_err(|e| FhevmError::FileError(format!("Failed to write public key: {}", e)))?;

    std::fs::write(output_dir.join("crs.bin"), &serialized_crs)
        .map_err(|e| FhevmError::FileError(format!("Failed to write CRS: {}", e)))?;

    let mut client_key_file = File::create(output_dir.join("client_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to create client key file: {}", e)))?;

    client_key_file
        .write_all(&serialized_client_key)
        .map_err(|e| FhevmError::FileError(format!("Failed to write client key: {}", e)))?;

    let mut server_key_file = File::create(output_dir.join("server_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to create server key file: {}", e)))?;

    server_key_file
        .write_all(&serialized_server_key)
        .map_err(|e| FhevmError::FileError(format!("Failed to write server key: {}", e)))?;

    info!(
        "FHE keyset generated and saved successfully to: {}",
        output_dir.display()
    );

    // Print file sizes for information
    info!("File sizes:");
    info!("  Public key: {} bytes", serialized_pub_key.len());
    info!("  Client key: {} bytes", serialized_client_key.len());
    info!("  Server key: {} bytes", serialized_server_key.len());
    info!("  CRS: {} bytes", serialized_crs.len());

    Ok(())
}
pub fn load_fhe_keyset(
    input_dir: &Path,
) -> Result<(
    tfhe::CompactPublicKey,
    tfhe::ClientKey,
    tfhe::ServerKey,
    CompactPkeCrs,
)> {
    info!("Loading FHE keyset from: {}", input_dir.display());

    // Indicate which parameters to use for the Compact Public Key encryption
    let cpk_params = tfhe::shortint::parameters::PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    // And parameters allowing to keyswitch/cast to the computation parameters.

    // Read public key
    let pub_key_data = std::fs::read(input_dir.join("public_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read public key: {}", e)))?;

    let public_key: CompactPublicKey =
        safe_deserialize_conformant(pub_key_data.as_slice(), 1 << 30, &cpk_params).map_err(
            |e| FhevmError::EncryptionError(format!("Failed to deserialize public key: {}", e)),
        )?;

    // Read client key
    let client_key_data = std::fs::read(input_dir.join("client_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read client key: {}", e)))?;

    let client_key: ClientKey =
        safe_deserialize(client_key_data.as_slice(), 1 << 30).map_err(|e| {
            FhevmError::EncryptionError(format!("Failed to deserialize client key: {}", e))
        })?;

    // Read server key
    let server_key_data = std::fs::read(input_dir.join("server_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read server key: {}", e)))?;

    let server_key = safe_deserialize(server_key_data.as_slice(), 1 << 32).map_err(|e| {
        FhevmError::EncryptionError(format!("Failed to deserialize server key: {}", e))
    })?;

    // Read CRS
    let crs_data = std::fs::read(input_dir.join("crs.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read CRS: {}", e)))?;

    let crs = safe_deserialize(crs_data.as_slice(), 1 << 30)
        .map_err(|e| FhevmError::EncryptionError(format!("Failed to deserialize CRS: {}", e)))?;

    info!("FHE keyset loaded successfully");

    Ok((public_key, client_key, server_key, crs))
}

/// Convert a chain ID to a standardized 32-byte representation
/// This matches the Node.js implementation: fromHexString(chainId.toString(16).padStart(64, '0'))
pub fn chain_id_to_bytes(chain_id: u64) -> [u8; 32] {
    let mut buffer = [0u8; 32];

    // Direct big-endian placement
    let chain_id_bytes = chain_id.to_be_bytes(); // 8 bytes for u64

    debug!("chain_id_bytes length: {}", chain_id_bytes.len());
    debug!("chain_id_bytes hex: {}", hex::encode(chain_id_bytes));

    let start_idx = buffer.len().saturating_sub(chain_id_bytes.len());
    debug_assert!(start_idx + chain_id_bytes.len() <= buffer.len());

    buffer[start_idx..].copy_from_slice(&chain_id_bytes);

    debug!("final buffer hex: {}", hex::encode(buffer));
    debug!("final buffer length: {}", buffer.len());

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_id_to_bytes_different_sizes() {
        // Test case 1: Small chain ID (1 - Ethereum Mainnet)
        let small_chain_id = 1;
        let small_bytes = chain_id_to_bytes(small_chain_id);

        // For chain ID 1, the result should be 31 zeros followed by 0x01
        let expected_small_hex = "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(
            hex::encode(small_bytes),
            expected_small_hex,
            "Small chain ID (1) conversion failed"
        );

        // Test case 2: Medium chain ID (42161 - Arbitrum One)
        let medium_chain_id = 42161; // 0xA4B1 in hex
        let medium_bytes = chain_id_to_bytes(medium_chain_id);

        // For chain ID 42161, the result should have 0xA4B1 at the end
        let expected_medium_hex =
            "000000000000000000000000000000000000000000000000000000000000a4b1";
        assert_eq!(
            hex::encode(medium_bytes),
            expected_medium_hex,
            "Medium chain ID (42161) conversion failed"
        );

        // Test case 3: Large chain ID (2^32 - 1 = 4294967295)
        let large_chain_id = 4294967295; // 0xFFFFFFFF in hex (fits in 4 bytes)
        let large_bytes = chain_id_to_bytes(large_chain_id);

        // For chain ID 4294967295, the result should have 0xFFFFFFFF at the end
        let expected_large_hex = "00000000000000000000000000000000000000000000000000000000ffffffff";
        assert_eq!(
            hex::encode(large_bytes),
            expected_large_hex,
            "Large chain ID (4294967295) conversion failed"
        );

        // Test case 4: Very large chain ID (u64::MAX = 18446744073709551615)
        let max_chain_id = u64::MAX; // 0xFFFFFFFFFFFFFFFF in hex (fits in 8 bytes)
        let max_bytes = chain_id_to_bytes(max_chain_id);

        // For max chain ID, the result should have 0xFFFFFFFFFFFFFFFF at the end
        // Corrected expected hex to match the actual output (64 characters)
        let expected_max_hex = "000000000000000000000000000000000000000000000000ffffffffffffffff";

        assert_eq!(
            hex::encode(max_bytes),
            expected_max_hex,
            "Maximum chain ID (u64::MAX) conversion failed"
        );
    }

    #[test]
    fn test_chain_id_in_auxiliary_data() {
        // Test using chain ID bytes in auxiliary data context

        // Create mock data for auxiliary data
        let contract_address = [1u8; 20]; // Mock contract address
        let user_address = [2u8; 20]; // Mock user address
        let acl_address = [3u8; 20]; // Mock ACL address

        // Test with a realistic chain ID: Avalanche C-Chain (43114)
        let chain_id = 43114;

        // Create auxiliary data manually
        let mut aux_data = Vec::with_capacity(92);
        aux_data.extend_from_slice(&contract_address);
        aux_data.extend_from_slice(&user_address);
        aux_data.extend_from_slice(&acl_address);
        aux_data.extend_from_slice(&chain_id_to_bytes(chain_id));

        // Verify the total length is correct (20 + 20 + 20 + 32 = 92 bytes)
        assert_eq!(aux_data.len(), 92, "Auxiliary data should be 92 bytes");

        // Verify chain ID bytes are correctly placed at the end
        let chain_id_portion = &aux_data[60..92];
        assert_eq!(
            hex::encode(chain_id_portion),
            "000000000000000000000000000000000000000000000000000000000000a86a", // 43114 = 0xA86A in hex
            "Chain ID bytes in auxiliary data are incorrect"
        );
    }
}
