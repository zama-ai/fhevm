//! Handle metadata only; no TFHE parsing or semantic validation.

pub const HANDLE_LEN: usize = 32;

/// Byte 21 marker for handles produced by FHE operations or by the bridge,
/// distinguishing them from input-verification handles which encode the input
/// index in this position (0..=254). Matches the `0xff` literal used by the
/// FHEVMExecutor and the bridge contracts in `host-contracts/`.
pub const COMPUTED_HANDLE_INDEX_MARKER: u8 = 0xff;

/// Byte 31 of every handle. Matches the contract-side `HANDLE_VERSION`
/// constant in `host-contracts/contracts/shared/Constants.sol`.
pub const HANDLE_VERSION: u8 = 0;

/// Highest variant in `host-contracts/contracts/shared/FheType.sol` on this
/// source baseline (`Int248`). Keep this synchronized when that enum grows.
/// This checks an event's declared handle metadata, not TFHE type semantics.
pub const MAX_FHE_TYPE_BYTE: u8 = 83;

/// The executor's `_mintHandle` path appends these four fields to every
/// computation result. A malformed RPC log must not become a durable output.
pub fn validate_computed_handle_metadata(
    handle: &[u8; HANDLE_LEN],
    chain_id: u64,
) -> Result<(), &'static str> {
    if handle[21] != COMPUTED_HANDLE_INDEX_MARKER {
        return Err("computed result handle is missing its marker");
    }
    if handle[22..30] != chain_id.to_be_bytes() {
        return Err("computed result handle has the wrong chain id");
    }
    if handle[30] > MAX_FHE_TYPE_BYTE {
        return Err("computed result handle has an unknown FheType");
    }
    if handle[31] != HANDLE_VERSION {
        return Err("computed result handle has an unknown version");
    }
    Ok(())
}

/// Application-level bounds, shared with the computation metadata adapter.
pub const MAX_MULTI_OUTPUT_ARITY: usize = u8::MAX as usize;

pub fn is_valid_multi_output_arity(outputs: usize) -> bool {
    outputs > 0 && outputs <= MAX_MULTI_OUTPUT_ARITY
}

pub fn get_ct_type(handle: &[u8]) -> Option<i16> {
    (handle.len() == HANDLE_LEN).then(|| i16::from(handle[30]))
}

#[cfg(test)]
mod computed_metadata_tests {
    use super::*;
    #[test]
    fn computed_footer_rejects_each_invalid_metadata_field() {
        let mut valid = [0_u8; HANDLE_LEN];
        valid[21] = COMPUTED_HANDLE_INDEX_MARKER;
        valid[22..30].copy_from_slice(&42_u64.to_be_bytes());
        valid[30] = MAX_FHE_TYPE_BYTE;
        assert!(validate_computed_handle_metadata(&valid, 42).is_ok());
        for (index, byte) in [(21, 254), (29, 43), (30, MAX_FHE_TYPE_BYTE + 1), (31, HANDLE_VERSION + 1)] {
            let mut invalid = valid;
            invalid[index] = byte;
            assert!(validate_computed_handle_metadata(&invalid, 42).is_err());
        }
    }
}
