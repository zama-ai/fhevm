use alloy::primitives::U256;
use anyhow::anyhow;

/// The expected length of the `extra_data` bytes.
const EXPECTED_EXTRA_DATA_LENGTH: usize = 33;

/// The version of the v1 `extra_data` format.
const EXTRA_DATA_VERSION_V1: u8 = 0x01;
/// The version of the v2 `extra_data` format.
const EXTRA_DATA_VERSION_V2: u8 = 0x02;

/// Parses the `extra_data` bytes to extract an optional context ID.
///
/// Format (v1):
/// - Byte 0: version (`0x01`)
/// - Bytes 1..33: context ID (32 bytes, big-endian U256)
/// - Bytes 33..: optional additional data (ignored)
///
/// Format (v2):
/// - Byte 0: version (`0x02`)
/// - Bytes 1..33: context ID (32 bytes, big-endian U256)
/// - Byte 33: contract ID count
/// - Bytes 34..: contract IDs (`count * 32` bytes)
///
/// Special cases for backward compatibility:
/// - Empty `extra_data` → `Ok(None)`
/// - `extra_data == [0x00]` → `Ok(None)` (legacy sentinel)
pub fn parse_extra_data_context(extra_data: &[u8]) -> anyhow::Result<Option<U256>> {
    if extra_data.is_empty() {
        return Ok(None);
    }

    if extra_data == [0x00] {
        return Ok(None);
    }

    if extra_data.len() < EXPECTED_EXTRA_DATA_LENGTH {
        return Err(anyhow!(
            "extra_data too short: {} bytes, expected at least {} bytes",
            extra_data.len(),
            EXPECTED_EXTRA_DATA_LENGTH
        ));
    }

    let version = extra_data[0];
    if version != EXTRA_DATA_VERSION_V1 && version != EXTRA_DATA_VERSION_V2 {
        return Err(anyhow!(
            "Unsupported extra_data version: 0x{:02x}, expected 0x{:02x} or 0x{:02x}",
            version,
            EXTRA_DATA_VERSION_V1,
            EXTRA_DATA_VERSION_V2
        ));
    }

    let context_id_bytes: [u8; 32] = extra_data[1..33]
        .try_into()
        .map_err(|e| anyhow!("Failed to extract context_id from extra_data: {e}"))?;

    Ok(Some(U256::from_be_bytes(context_id_bytes)))
}

/// Parses the optional ordered list of native identities from the `extra_data` bytes.
///
/// V2 format:
/// - Byte 0: version (`0x02`)
/// - Bytes 1..33: context ID
/// - Byte 33: identity count
/// - Bytes 34..: identities (`count * 32` bytes)
///
/// The exact interpretation of the identities depends on the caller:
/// - user decryption V2: `[user_id, contract_id...]`
/// - delegated user decryption V2: `[delegator_id, delegate_id, contract_id...]`
/// - legacy v2 callers may still use `[contract_id...]`
pub fn parse_extra_data_identities(extra_data: &[u8]) -> anyhow::Result<Option<Vec<[u8; 32]>>> {
    if extra_data.is_empty() || extra_data == [0x00] {
        return Ok(None);
    }

    if extra_data[0] == EXTRA_DATA_VERSION_V1 {
        return Ok(None);
    }

    if extra_data[0] != EXTRA_DATA_VERSION_V2 {
        return Err(anyhow!(
            "Unsupported extra_data version: 0x{:02x}, expected 0x{:02x} or 0x{:02x}",
            extra_data[0],
            EXTRA_DATA_VERSION_V1,
            EXTRA_DATA_VERSION_V2
        ));
    }

    if extra_data.len() < 34 {
        return Err(anyhow!(
            "extra_data too short for v2 identities: {} bytes, expected at least 34 bytes",
            extra_data.len()
        ));
    }

    let count = extra_data[33] as usize;
    let expected_len = 34 + count * 32;
    if extra_data.len() != expected_len {
        return Err(anyhow!(
            "invalid v2 extra_data length: {} bytes, expected {} bytes for {} contract ids",
            extra_data.len(),
            expected_len,
            count
        ));
    }

    let mut identities = Vec::with_capacity(count);
    for index in 0..count {
        let start = 34 + index * 32;
        let end = start + 32;
        let identity: [u8; 32] = extra_data[start..end]
            .try_into()
            .map_err(|e| anyhow!("Failed to extract identity[{index}] from extra_data: {e}"))?;
        identities.push(identity);
    }

    Ok(Some(identities))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_none() {
        assert_eq!(parse_extra_data_context(&[]).unwrap(), None);
    }

    #[test]
    fn single_zero_byte_returns_none() {
        assert_eq!(parse_extra_data_context(&[0x00]).unwrap(), None);
    }

    #[test]
    fn valid_v1_exactly() {
        let mut data = vec![0x01]; // version
        let context_id = U256::from(69u64);
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        assert_eq!(data.len(), 33);

        let result = parse_extra_data_context(&data).unwrap();
        assert_eq!(result, Some(U256::from(69u64)));
    }

    #[test]
    fn wrong_version_byte_errors() {
        let mut data = vec![0x03]; // wrong version
        data.extend_from_slice(&[0u8; 32]);

        let err = parse_extra_data_context(&data).unwrap_err();
        assert!(
            err.to_string().contains("Unsupported extra_data version"),
            "Unexpected error: {err}"
        );
    }

    #[test]
    fn too_short_error() {
        let mut data = vec![0x01]; // version

        // Add 10 bytes: not empty, not [0x00], but < 33
        data.extend_from_slice(&[0u8; 10]);

        let err = parse_extra_data_context(&data).unwrap_err();
        assert!(
            err.to_string().contains("extra_data too short"),
            "Unexpected error: {err}"
        );
    }

    #[test]
    fn valid_v2_context_and_identities() {
        let mut data = vec![0x02];
        let context_id = U256::from(69u64);
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        data.push(2);
        data.extend_from_slice(&[0x11; 32]);
        data.extend_from_slice(&[0x22; 32]);

        assert_eq!(parse_extra_data_context(&data).unwrap(), Some(context_id));
        assert_eq!(
            parse_extra_data_identities(&data).unwrap(),
            Some(vec![[0x11; 32], [0x22; 32]])
        );
    }

    #[test]
    fn v1_has_no_identities() {
        let mut data = vec![0x01];
        data.extend_from_slice(&U256::from(69u64).to_be_bytes::<32>());
        assert_eq!(parse_extra_data_identities(&data).unwrap(), None);
    }

    #[test]
    fn invalid_v2_identity_length_errors() {
        let mut data = vec![0x02];
        data.extend_from_slice(&U256::from(69u64).to_be_bytes::<32>());
        data.push(1);

        let err = parse_extra_data_identities(&data).unwrap_err();
        assert!(
            err.to_string().contains("invalid v2 extra_data length"),
            "Unexpected error: {err}"
        );
    }
}
