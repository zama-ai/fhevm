use alloy::primitives::U256;

/// Version `0x01`: context_id only (RFC 003).
const EXTRA_DATA_V1_VERSION: u8 = 0x01;
/// Minimum length of v1 extra_data (version + context_id).
const EXTRA_DATA_V1_LENGTH: usize = 33;

/// Version `0x02`: context_id + epoch_id (RFC 005).
const EXTRA_DATA_V2_VERSION: u8 = 0x02;
/// Minimum length of v2 extra_data (version + context_id + epoch_id).
const EXTRA_DATA_V2_LENGTH: usize = 65;

/// Parse the context ID from extra_data bytes.
///
/// Format v1: `[0x01 | context_id (32 bytes)]`
/// Format v2: `[0x02 | context_id (32 bytes) | epoch_id (32 bytes)]`
///
/// Returns `Ok(U256::ZERO)` when extra_data is empty or version is `0x00`,
/// signalling "use static config default" (context ID 0 is invalid).
///
/// Returns `Err` for malformed input (unsupported version, truncated data).
pub fn parse_context_id_from_extra_data(extra_data: &[u8]) -> Result<U256, ExtraDataError> {
    let Some(&version) = extra_data.first() else {
        return Ok(U256::ZERO);
    };

    match version {
        // 0x00 is the legacy/default marker — use static threshold
        0x00 => Ok(U256::ZERO),
        EXTRA_DATA_V1_VERSION | EXTRA_DATA_V2_VERSION => {
            let min_len = if version == EXTRA_DATA_V1_VERSION {
                EXTRA_DATA_V1_LENGTH
            } else {
                EXTRA_DATA_V2_LENGTH
            };
            if extra_data.len() < min_len {
                return Err(ExtraDataError::TooShort {
                    version,
                    len: extra_data.len(),
                    expected: min_len,
                });
            }
            let bytes: [u8; 32] = extra_data[1..33]
                .try_into()
                .expect("slice is exactly 32 bytes");
            Ok(U256::from_be_bytes(bytes))
        }
        _ => Err(ExtraDataError::UnsupportedVersion(version)),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExtraDataError {
    #[error("Unsupported extra_data version: 0x{0:02x}")]
    UnsupportedVersion(u8),

    #[error("extra_data too short for v{version:#04x}: {len} bytes, expected at least {expected}")]
    TooShort {
        version: u8,
        len: usize,
        expected: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_zero() {
        assert_eq!(parse_context_id_from_extra_data(&[]).unwrap(), U256::ZERO);
    }

    #[test]
    fn single_zero_byte_returns_zero() {
        assert_eq!(
            parse_context_id_from_extra_data(&[0x00]).unwrap(),
            U256::ZERO
        );
    }

    #[test]
    fn valid_v1_returns_context_id() {
        let context_id = U256::from(42u64);
        let mut data = vec![EXTRA_DATA_V1_VERSION];
        data.extend_from_slice(&context_id.to_be_bytes::<32>());

        assert_eq!(parse_context_id_from_extra_data(&data).unwrap(), context_id);
    }

    #[test]
    fn valid_v2_returns_context_id() {
        let context_id = U256::from(99u64);
        let epoch_id = U256::from(7u64);
        let mut data = vec![EXTRA_DATA_V2_VERSION];
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        data.extend_from_slice(&epoch_id.to_be_bytes::<32>());

        assert_eq!(parse_context_id_from_extra_data(&data).unwrap(), context_id);
    }

    #[test]
    fn v2_with_trailing_bytes_works() {
        let context_id = U256::from(1u64);
        let mut data = vec![EXTRA_DATA_V2_VERSION];
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        data.extend_from_slice(&U256::from(2u64).to_be_bytes::<32>());
        data.extend_from_slice(&[0xff; 10]);

        assert_eq!(parse_context_id_from_extra_data(&data).unwrap(), context_id);
    }

    #[test]
    fn v1_too_short_returns_error() {
        let mut data = vec![EXTRA_DATA_V1_VERSION];
        data.extend_from_slice(&[0u8; 10]);
        assert!(parse_context_id_from_extra_data(&data).is_err());
    }

    #[test]
    fn v2_too_short_returns_error() {
        let mut data = vec![EXTRA_DATA_V2_VERSION];
        data.extend_from_slice(&[0u8; 32]); // only context_id, missing epoch_id
        assert!(parse_context_id_from_extra_data(&data).is_err());
    }

    #[test]
    fn unknown_version_returns_error() {
        let mut data = vec![0x03];
        data.extend_from_slice(&[0u8; 64]);
        assert!(parse_context_id_from_extra_data(&data).is_err());
    }

    #[test]
    fn large_context_id() {
        // KMS_CONTEXT_COUNTER_BASE + 1 = 0x07 << 248 | 1
        let mut bytes = [0u8; 32];
        bytes[0] = 0x07;
        bytes[31] = 0x01;
        let context_id = U256::from_be_bytes(bytes);
        let mut data = vec![EXTRA_DATA_V1_VERSION];
        data.extend_from_slice(&context_id.to_be_bytes::<32>());

        assert_eq!(parse_context_id_from_extra_data(&data).unwrap(), context_id);
    }
}
