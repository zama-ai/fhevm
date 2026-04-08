//! Utility functions for FHEVM SDK.
//!
//! Re-exports platform-agnostic utilities from [`fhevm_client_core`]
//! and adds filesystem-dependent key generation and loading.

// Re-export core utilities
pub use fhevm_client_core::utils::{
    chain_id_to_bytes, parse_hex_string, validate_address, validate_address_from_str,
};

use crate::{FhevmError, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tfhe::safe_serialization::{safe_deserialize, safe_deserialize_conformant, safe_serialize};
use tfhe::zk::CompactPkeCrs;
use tfhe::{ClientKey, CompactPublicKey};
use tracing::info;

pub fn generate_fhe_keyset(output_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(output_dir)
        .map_err(|e| FhevmError::FileError(format!("Failed to create output directory: {e}")))?;

    let params = tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    let cpk_params = tfhe::shortint::parameters::PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    let casting_params =
        tfhe::shortint::parameters::PARAM_KEYSWITCH_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    let config = tfhe::ConfigBuilder::with_custom_parameters(params)
        .use_dedicated_compact_public_key_parameters((cpk_params, casting_params))
        .build();

    let crs = CompactPkeCrs::from_config(config, 512)
        .map_err(|e| FhevmError::KeyGenerationError(format!("Failed to generate CRS: {e}")))?;

    let client_key = tfhe::ClientKey::generate(config);
    let server_key = tfhe::ServerKey::new(&client_key);
    let public_key = tfhe::CompactPublicKey::try_new(&client_key)
        .map_err(|e| FhevmError::KeyGenerationError(format!("Failed to generate public key: {e}")))?;

    let mut serialized_pub_key = Vec::new();
    safe_serialize(&public_key, &mut serialized_pub_key, 1 << 30)
        .map_err(|e| FhevmError::KeyGenerationError(format!("Failed to serialize public key: {e}")))?;

    let mut serialized_client_key = Vec::new();
    safe_serialize(&client_key, &mut serialized_client_key, 1 << 30)
        .map_err(|e| FhevmError::KeyGenerationError(format!("Failed to serialize client key: {e}")))?;

    let mut serialized_server_key = Vec::new();
    safe_serialize(&server_key, &mut serialized_server_key, 1 << 30)
        .map_err(|e| FhevmError::KeyGenerationError(format!("Failed to serialize server key: {e}")))?;

    let mut serialized_crs = Vec::new();
    safe_serialize(&crs, &mut serialized_crs, 1 << 30)
        .map_err(|e| FhevmError::KeyGenerationError(format!("Failed to serialize CRS: {e}")))?;

    std::fs::write(output_dir.join("public_key.bin"), &serialized_pub_key)
        .map_err(|e| FhevmError::FileError(format!("Failed to write public key: {e}")))?;

    std::fs::write(output_dir.join("crs.bin"), &serialized_crs)
        .map_err(|e| FhevmError::FileError(format!("Failed to write CRS: {e}")))?;

    let mut client_key_file = File::create(output_dir.join("client_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to create client key file: {e}")))?;

    client_key_file
        .write_all(&serialized_client_key)
        .map_err(|e| FhevmError::FileError(format!("Failed to write client key: {e}")))?;

    let mut server_key_file = File::create(output_dir.join("server_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to create server key file: {e}")))?;

    server_key_file
        .write_all(&serialized_server_key)
        .map_err(|e| FhevmError::FileError(format!("Failed to write server key: {e}")))?;

    info!(
        "FHE keyset generated and saved successfully to: {}",
        output_dir.display()
    );

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

    let cpk_params = tfhe::shortint::parameters::PARAM_PKE_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;

    let pub_key_data = std::fs::read(input_dir.join("public_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read public key: {e}")))?;

    let public_key: CompactPublicKey =
        safe_deserialize_conformant(pub_key_data.as_slice(), 1 << 30, &cpk_params).map_err(
            |e| FhevmError::EncryptionError(format!("Failed to deserialize public key: {e}")),
        )?;

    let client_key_data = std::fs::read(input_dir.join("client_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read client key: {e}")))?;

    let client_key: ClientKey =
        safe_deserialize(client_key_data.as_slice(), 1 << 30).map_err(|e| {
            FhevmError::EncryptionError(format!("Failed to deserialize client key: {e}"))
        })?;

    let server_key_data = std::fs::read(input_dir.join("server_key.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read server key: {e}")))?;

    let server_key = safe_deserialize(server_key_data.as_slice(), 1 << 32).map_err(|e| {
        FhevmError::EncryptionError(format!("Failed to deserialize server key: {e}"))
    })?;

    let crs_data = std::fs::read(input_dir.join("crs.bin"))
        .map_err(|e| FhevmError::FileError(format!("Failed to read CRS: {e}")))?;

    let crs = safe_deserialize(crs_data.as_slice(), 1 << 30)
        .map_err(|e| FhevmError::EncryptionError(format!("Failed to deserialize CRS: {e}")))?;

    info!("FHE keyset loaded successfully");

    Ok((public_key, client_key, server_key, crs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;

    #[test]
    fn test_chain_id_to_bytes_different_sizes() {
        let bytes = chain_id_to_bytes(1);
        assert_eq!(
            hex::encode(bytes),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );

        let bytes = chain_id_to_bytes(42161);
        assert_eq!(
            hex::encode(bytes),
            "000000000000000000000000000000000000000000000000000000000000a4b1"
        );

        let bytes = chain_id_to_bytes(4294967295);
        assert_eq!(
            hex::encode(bytes),
            "00000000000000000000000000000000000000000000000000000000ffffffff"
        );

        let bytes = chain_id_to_bytes(u64::MAX);
        assert_eq!(
            hex::encode(bytes),
            "000000000000000000000000000000000000000000000000ffffffffffffffff"
        );
    }

    #[test]
    fn test_chain_id_in_auxiliary_data() {
        let contract_address = [1u8; 20];
        let user_address = [2u8; 20];
        let acl_address = [3u8; 20];

        let chain_id = 43114;

        let mut aux_data = Vec::with_capacity(92);
        aux_data.extend_from_slice(&contract_address);
        aux_data.extend_from_slice(&user_address);
        aux_data.extend_from_slice(&acl_address);
        aux_data.extend_from_slice(&chain_id_to_bytes(chain_id));

        assert_eq!(aux_data.len(), 92, "Auxiliary data should be 92 bytes");

        let chain_id_portion = &aux_data[60..92];
        assert_eq!(
            hex::encode(chain_id_portion),
            "000000000000000000000000000000000000000000000000000000000000a86a",
        );
    }
}
