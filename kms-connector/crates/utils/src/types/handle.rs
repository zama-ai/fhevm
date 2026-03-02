use anyhow::anyhow;
use tfhe::FheTypes;

// Current handle format:
// [21 first random bytes from hashing] | index_21 | chainID_22...29 | fheType_30 | version_31
// Source: https://github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/libraries/HandleOps.sol#L6-L11

/// Extracts the FHE type from a ciphertext's handle.
pub fn extract_fhe_type_from_handle(handle: &[u8]) -> anyhow::Result<FheTypes> {
    let err_prefix = "Failed to extract fhe_type from handle.";
    if handle.len() >= 32 {
        let type_byte = handle[30]; // FHE type is at index 30
        FheTypes::try_from(type_byte as i32).map_err(|e| anyhow!("{err_prefix} {e}"))
    } else {
        Err(anyhow!(
            "{} Handle too short: {} bytes, expected 32 bytes",
            err_prefix,
            handle.len()
        ))
    }
}

/// Extracts the chain id from a ciphertext's handle.
pub fn extract_chain_id_from_handle(handle: &[u8]) -> anyhow::Result<u64> {
    let err_prefix = "Failed to extract chain_id from handle.";
    if handle.len() >= 32 {
        let chain_id_bytes = handle[22..30]
            .try_into()
            .map_err(|e| anyhow!("{err_prefix} {e}"))?;
        Ok(u64::from_be_bytes(chain_id_bytes))
    } else {
        Err(anyhow!(
            "{} Handle too short: {} bytes, expected 32 bytes",
            err_prefix,
            handle.len()
        ))
    }
}
