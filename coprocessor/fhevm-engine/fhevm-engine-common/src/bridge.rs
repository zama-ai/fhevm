//! Confidential bridge primitives (see RFC 008).

use sha3::{Digest, Keccak256};

use crate::types::{COMPUTED_HANDLE_INDEX_MARKER, HANDLE_VERSION};

/// Must match `BRIDGE_DERIVATION_DOMAIN_SEPARATOR` in `HandlesReceiver.sol`.
pub const BRIDGE_DERIVATION_DOMAIN_SEPARATOR: &[u8; 8] = b"FHE_brdg";

/// Mirror of `HandlesReceiver._deriveDstHandle`.
///
/// Computes
/// ```text
///   h = keccak256(BRIDGE_DERIVATION_DOMAIN_SEPARATOR
///                 || src_handle
///                 || acl_address
///                 || u256(dst_chain_id)
///                 || prev_block_hash
///                 || u256(block_timestamp))
/// ```
/// then overwrites bytes 21-31:
/// - byte 21 = `COMPUTED_HANDLE_INDEX_MARKER`
/// - bytes 22-29 = `dst_chain_id` big-endian
/// - byte 30 = `src_handle[30]` (preserves the source `FheType`)
/// - byte 31 = `HANDLE_VERSION`
pub fn derive_dst_handle(
    src_handle: &[u8; 32],
    acl_address: &[u8; 20],
    dst_chain_id: u64,
    prev_block_hash: &[u8; 32],
    block_timestamp: u64,
) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(BRIDGE_DERIVATION_DOMAIN_SEPARATOR);
    hasher.update(src_handle);
    hasher.update(acl_address);
    hasher.update(u256_be_from_u64(dst_chain_id));
    hasher.update(prev_block_hash);
    hasher.update(u256_be_from_u64(block_timestamp));

    let mut result: [u8; 32] = hasher.finalize().into();
    result[21] = COMPUTED_HANDLE_INDEX_MARKER;
    result[22..30].copy_from_slice(&dst_chain_id.to_be_bytes());
    result[30] = src_handle[30];
    result[31] = HANDLE_VERSION;
    result
}

pub fn chain_id_from_handle(handle: &[u8; 32]) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&handle[22..30]);
    u64::from_be_bytes(bytes)
}

// 32-byte big-endian encoding of a `u64`, matching Solidity's packed `uint256`.
fn u256_be_from_u64(v: u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[24..].copy_from_slice(&v.to_be_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_bytes_are_correct() {
        let mut src = [0u8; 32];
        src[30] = 0x04;
        let dst_chain_id: u64 = 12_345;

        let result = derive_dst_handle(&src, &[0xcd; 20], dst_chain_id, &[0xef; 32], 1_700_000_000);

        assert_eq!(result[21], COMPUTED_HANDLE_INDEX_MARKER);
        assert_eq!(&result[22..30], &dst_chain_id.to_be_bytes());
        assert_eq!(result[30], src[30]);
        assert_eq!(result[31], HANDLE_VERSION);
    }

    #[test]
    fn deterministic_for_same_inputs() {
        let src = [1u8; 32];
        let acl = [3u8; 20];
        let prev = [4u8; 32];
        assert_eq!(
            derive_dst_handle(&src, &acl, 7, &prev, 100),
            derive_dst_handle(&src, &acl, 7, &prev, 100)
        );
    }

    #[test]
    fn each_input_changes_hash_prefix() {
        // Only bytes 0..21 come from the hash; verify each input perturbs them.
        let handle = [1u8; 32];
        let acl = [3u8; 20];
        let prev = [4u8; 32];
        let chain = 7u64;
        let ts = 100u64;
        let baseline = derive_dst_handle(&handle, &acl, chain, &prev, ts);

        let mut other_handle = handle;
        other_handle[0] ^= 0xff;
        assert_ne!(
            &baseline[..21],
            &derive_dst_handle(&other_handle, &acl, chain, &prev, ts)[..21]
        );

        let mut other_acl = acl;
        other_acl[0] ^= 0xff;
        assert_ne!(
            &baseline[..21],
            &derive_dst_handle(&handle, &other_acl, chain, &prev, ts)[..21]
        );

        assert_ne!(
            &baseline[..21],
            &derive_dst_handle(&handle, &acl, chain + 1, &prev, ts)[..21]
        );

        let mut other_prev = prev;
        other_prev[0] ^= 0xff;
        assert_ne!(
            &baseline[..21],
            &derive_dst_handle(&handle, &acl, chain, &other_prev, ts)[..21]
        );

        assert_ne!(
            &baseline[..21],
            &derive_dst_handle(&handle, &acl, chain, &prev, ts + 1)[..21]
        );
    }

    #[test]
    fn chain_id_round_trips_through_handle() {
        let chain = 0xdead_beef_0000_0001u64;
        let h = derive_dst_handle(&[0u8; 32], &[0u8; 20], chain, &[0u8; 32], 0);
        assert_eq!(chain_id_from_handle(&h), chain);
    }
}
