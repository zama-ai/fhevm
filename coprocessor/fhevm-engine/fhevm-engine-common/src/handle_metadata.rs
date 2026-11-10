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

/// Metadata guaranteed by a successful `InputVerifier.verifyInput` call.
/// Proof signatures and ciphertext contents are deliberately outside this
/// application-layer check. The proof's byte layout is bounded by two u8
/// counts, so every offset below is safe after the length check.
pub fn validate_verified_input_metadata(
    handle: &[u8; HANDLE_LEN],
    result: &[u8; HANDLE_LEN],
    input_type: u8,
    proof: &[u8],
    chain_id: u64,
) -> Result<(), &'static str> {
    let index = usize::from(handle[21]);
    if index == usize::from(COMPUTED_HANDLE_INDEX_MARKER) {
        return Err("verified input handle index is out of range");
    }
    if handle[22..30] != chain_id.to_be_bytes() {
        return Err("verified input handle has the wrong chain id");
    }
    if handle[30] > MAX_FHE_TYPE_BYTE || handle[30] != input_type {
        return Err("verified input handle has an invalid type");
    }
    if handle[31] != HANDLE_VERSION {
        return Err("verified input handle has an unknown version");
    }
    if result != handle {
        return Err("verified input result differs from its input handle");
    }
    if proof.len() < 2 {
        return Err("verified input proof has no count header");
    }
    let count = usize::from(proof[0]);
    let signers = usize::from(proof[1]);
    if count <= index {
        return Err("verified input proof does not contain its index");
    }
    let minimum_len = 2 + HANDLE_LEN * count + 65 * signers;
    if proof.len() < minimum_len {
        return Err("verified input proof is truncated");
    }
    if proof[2 + HANDLE_LEN * index..2 + HANDLE_LEN * (index + 1)] != handle[..] {
        return Err("verified input proof selects a different handle");
    }
    if proof[2..2 + HANDLE_LEN * count]
        .chunks_exact(HANDLE_LEN)
        .any(|entry| entry[31] != HANDLE_VERSION)
    {
        return Err("verified input proof contains an unknown handle version");
    }
    Ok(())
}

/// Persistent ACL grants may refer to a verified input or a computed/bridged
/// handle. Both forms retain their originating host chain, FheType and version.
/// Their producer identity is resolved separately because a missed earlier
/// event may still arrive through catch-up or replay.
pub fn validate_acl_handle_metadata(
    handle: &[u8; HANDLE_LEN],
    chain_id: u64,
) -> Result<(), &'static str> {
    if handle[22..30] != chain_id.to_be_bytes() {
        return Err("ACL handle has the wrong chain id");
    }
    if handle[30] > MAX_FHE_TYPE_BYTE {
        return Err("ACL handle has an unknown FheType");
    }
    if handle[31] != HANDLE_VERSION {
        return Err("ACL handle has an unknown version");
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
        for (index, byte) in [
            (21, 254),
            (29, 43),
            (30, MAX_FHE_TYPE_BYTE + 1),
            (31, HANDLE_VERSION + 1),
        ] {
            let mut invalid = valid;
            invalid[index] = byte;
            assert!(validate_computed_handle_metadata(&invalid, 42).is_err());
        }
    }
}

#[cfg(test)]
mod verified_metadata_tests {
    use super::*;
    #[test]
    fn verified_input_checks_index_selection_counts_and_versions() {
        let mut handle = [0_u8; HANDLE_LEN];
        handle[22..30].copy_from_slice(&42_u64.to_be_bytes());
        handle[30] = 5;
        let mut proof = vec![1, 1];
        proof.extend_from_slice(&handle);
        proof.extend_from_slice(&[0_u8; 65]);
        assert!(validate_verified_input_metadata(&handle, &handle, 5, &proof, 42).is_ok());
        assert!(validate_verified_input_metadata(
            &handle,
            &handle,
            5,
            &proof[..proof.len() - 1],
            42
        )
        .is_err());
        assert!(validate_verified_input_metadata(&handle, &handle, 6, &proof, 42).is_err());
        assert!(validate_verified_input_metadata(&handle, &handle, 5, &proof, 43).is_err());
        let mut result = handle;
        result[0] = 1;
        assert!(validate_verified_input_metadata(&handle, &result, 5, &proof, 42).is_err());
        let mut invalid = handle;
        invalid[21] = 255;
        assert!(validate_verified_input_metadata(&invalid, &invalid, 5, &proof, 42).is_err());
        let mut other_version = proof.clone();
        other_version[33] = HANDLE_VERSION + 1;
        assert!(validate_verified_input_metadata(&handle, &handle, 5, &other_version, 42).is_err());
        let mut bad_selection = proof.clone();
        bad_selection[2] = 1;
        assert!(validate_verified_input_metadata(&handle, &handle, 5, &bad_selection, 42).is_err());
        let mut empty_count = proof;
        empty_count[0] = 0;
        assert!(validate_verified_input_metadata(&handle, &handle, 5, &empty_count, 42).is_err());
    }
}

#[cfg(test)]
mod acl_metadata_tests {
    use super::*;
    #[test]
    fn grants_accept_input_or_computed_handles_with_valid_footer() {
        let mut handle = [0_u8; HANDLE_LEN];
        handle[22..30].copy_from_slice(&42_u64.to_be_bytes());
        handle[30] = MAX_FHE_TYPE_BYTE;
        for index in [0, 254, COMPUTED_HANDLE_INDEX_MARKER] {
            handle[21] = index;
            assert!(validate_acl_handle_metadata(&handle, 42).is_ok());
        }
        for (index, byte) in [
            (29, 43),
            (30, MAX_FHE_TYPE_BYTE + 1),
            (31, HANDLE_VERSION + 1),
        ] {
            let mut invalid = handle;
            invalid[index] = byte;
            assert!(validate_acl_handle_metadata(&invalid, 42).is_err());
        }
    }
}
