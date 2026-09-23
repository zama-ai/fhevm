use alloy::primitives::FixedBytes;
use anyhow::anyhow;
use tfhe::FheTypes;

// Current handle format:
// [21 first random bytes from hashing] | index_21 | chainID_22...29 | fheType_30 | version_31
// Source: https://github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/libraries/HandleOps.sol#L6-L11

/// Extracts the FHE type from a ciphertext's handle.
pub fn extract_fhe_type_from_handle(handle: &FixedBytes<32>) -> anyhow::Result<FheTypes> {
    FheTypes::try_from(handle[30] as i32)
        .map_err(|e| anyhow!("Failed to extract fhe_type from handle: {e}"))
}

/// Extracts the chain id from a ciphertext's handle.
pub fn extract_chain_id_from_handle(handle: &FixedBytes<32>) -> anyhow::Result<u64> {
    let chain_id_bytes = handle[22..30]
        .try_into()
        .map_err(|e| anyhow!("Failed to extract chain_id from handle: {e}"))?;
    Ok(u64::from_be_bytes(chain_id_bytes))
}
