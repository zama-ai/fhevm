use alloy::primitives::U256;
use anyhow::anyhow;

/// The expected length of the `extra_data` bytes.
const EXPECTED_EXTRA_DATA_LENGTH: usize = 33;

/// The version of the `extra_data` format.
const EXTRA_DATA_VERSION: u8 = 0x01;

/// Parses the `extra_data` bytes to extract an optional context ID.
///
/// Format (v1):
/// - Byte 0: version (`0x01`)
/// - Bytes 1..33: context ID (32 bytes, big-endian U256)
/// - Bytes 33..: optional additional data (ignored)
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
    if version != EXTRA_DATA_VERSION {
        return Err(anyhow!(
            "Unsupported extra_data version: 0x{:02x}, expected 0x{:02x}",
            version,
            EXTRA_DATA_VERSION
        ));
    }

    let context_id_bytes: [u8; 32] = extra_data[1..33]
        .try_into()
        .map_err(|e| anyhow!("Failed to extract context_id from extra_data: {e}"))?;

    Ok(Some(U256::from_be_bytes(context_id_bytes)))
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
        let mut data = vec![0x02]; // wrong version
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
}
