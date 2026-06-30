use alloy::primitives::U256;
use anyhow::anyhow;

/// Version `0x01`: context_id only (RFC 003).
pub const EXTRA_DATA_V1_VERSION: u8 = 0x01;

/// The expected length of v1 `extra_data` (version + context_id).
pub const EXTRA_DATA_V1_LENGTH: usize = 33;

/// Version `0x02`: context_id + epoch_id (RFC 005).
pub const EXTRA_DATA_V2_VERSION: u8 = 0x02;

/// The expected length of v2 `extra_data` (version + context_id + epoch_id).
pub const EXTRA_DATA_V2_LENGTH: usize = 65;

/// Parsed extra_data contents.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtraData {
    pub context_id: Option<U256>,
    pub epoch_id: Option<U256>,
}

/// Parses the `extra_data` bytes to extract a context ID and an optional epoch ID.
///
/// Versions `0x01` and `0x02` are accepted (rolling compatibility window).
/// Empty extra_data and `0x00` are legacy default-context markers.
///
/// Format (v1, RFC 003):
/// - Byte 0: version (`0x01`)
/// - Bytes 1..33: context ID (32 bytes, big-endian U256)
/// - Bytes 33..: optional additional data (ignored)
///
/// Format (v2, RFC 005):
/// - Byte 0: version (`0x02`)
/// - Bytes 1..33: context ID (32 bytes, big-endian U256)
/// - Bytes 33..65: epoch ID (32 bytes, big-endian U256)
/// - Bytes 65..: optional additional data (ignored)
///
/// Version `0x01` → epoch_id is `None` (caller should fall back to DEFAULT_EPOCH_ID).
/// Empty or `[0x00]` → no explicit context and no epoch.
pub fn parse_extra_data(extra_data: &[u8]) -> anyhow::Result<ExtraData> {
    if extra_data.is_empty() || extra_data == [0x00] {
        return Ok(ExtraData {
            context_id: None,
            epoch_id: None,
        });
    }

    match extra_data[0] {
        EXTRA_DATA_V1_VERSION => {
            if extra_data.len() < EXTRA_DATA_V1_LENGTH {
                return Err(anyhow!(
                    "extra_data too short for v1: {} bytes, expected at least {} bytes",
                    extra_data.len(),
                    EXTRA_DATA_V1_LENGTH
                ));
            }

            let context_id_bytes: [u8; 32] = extra_data[1..33]
                .try_into()
                .map_err(|e| anyhow!("Failed to extract context_id from extra_data: {e}"))?;

            Ok(ExtraData {
                context_id: Some(U256::from_be_bytes(context_id_bytes)),
                epoch_id: None,
            })
        }
        EXTRA_DATA_V2_VERSION => {
            if extra_data.len() < EXTRA_DATA_V2_LENGTH {
                return Err(anyhow!(
                    "extra_data too short for v2: {} bytes, expected at least {} bytes",
                    extra_data.len(),
                    EXTRA_DATA_V2_LENGTH
                ));
            }

            let context_id_bytes: [u8; 32] = extra_data[1..33]
                .try_into()
                .map_err(|e| anyhow!("Failed to extract context_id from extra_data: {e}"))?;

            let epoch_id_bytes: [u8; 32] = extra_data[33..65]
                .try_into()
                .map_err(|e| anyhow!("Failed to extract epoch_id from extra_data: {e}"))?;

            Ok(ExtraData {
                context_id: Some(U256::from_be_bytes(context_id_bytes)),
                epoch_id: Some(U256::from_be_bytes(epoch_id_bytes)),
            })
        }
        _ => Err(anyhow!(
            "Unsupported extra_data version: 0x{:02x}, expected 0x00, 0x{:02x}, or 0x{:02x}",
            extra_data[0],
            EXTRA_DATA_V1_VERSION,
            EXTRA_DATA_V2_VERSION
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_default_context() {
        assert_eq!(
            parse_extra_data(&[]).unwrap(),
            ExtraData {
                context_id: None,
                epoch_id: None
            }
        );
    }

    #[test]
    fn single_zero_byte_returns_default_context() {
        assert_eq!(
            parse_extra_data(&[0x00]).unwrap(),
            ExtraData {
                context_id: None,
                epoch_id: None
            }
        );
    }

    #[test]
    fn valid_v1_returns_context_only() {
        let mut data = vec![EXTRA_DATA_V1_VERSION];
        let context_id = U256::from(69u64);
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        data.extend_from_slice(&U256::from(2u64).to_be_bytes::<32>());

        let result = parse_extra_data(&data).unwrap();
        assert_eq!(
            result,
            ExtraData {
                context_id: Some(U256::from(69u64)),
                epoch_id: None
            }
        );
    }

    #[test]
    fn valid_v2_returns_context_and_epoch() {
        let mut data = vec![EXTRA_DATA_V2_VERSION];
        let context_id = U256::from(42u64);
        let epoch_id = U256::from(7u64);
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        data.extend_from_slice(&epoch_id.to_be_bytes::<32>());

        let result = parse_extra_data(&data).unwrap();
        assert_eq!(
            result,
            ExtraData {
                context_id: Some(U256::from(42u64)),
                epoch_id: Some(U256::from(7u64))
            }
        );
    }

    #[test]
    fn v2_with_trailing_bytes_ignored() {
        let mut data = vec![EXTRA_DATA_V2_VERSION];
        data.extend_from_slice(&U256::from(1u64).to_be_bytes::<32>());
        data.extend_from_slice(&U256::from(2u64).to_be_bytes::<32>());
        data.extend_from_slice(&[0xff; 10]); // extra trailing bytes

        let result = parse_extra_data(&data).unwrap();
        assert_eq!(
            result,
            ExtraData {
                context_id: Some(U256::from(1u64)),
                epoch_id: Some(U256::from(2u64))
            }
        );
    }

    #[test]
    fn wrong_version_byte_errors() {
        let mut data = vec![0x03];
        data.extend_from_slice(&[0u8; 64]);

        let err = parse_extra_data(&data).unwrap_err();
        assert!(
            err.to_string().contains("Unsupported extra_data version"),
            "Unexpected error: {err}"
        );
    }

    #[test]
    fn v1_too_short_error() {
        let mut data = vec![EXTRA_DATA_V1_VERSION];
        data.extend_from_slice(&[0u8; 10]);

        let err = parse_extra_data(&data).unwrap_err();
        assert!(
            err.to_string().contains("extra_data too short for v1"),
            "Unexpected error: {err}"
        );
    }

    #[test]
    fn v2_too_short_error() {
        let mut data = vec![EXTRA_DATA_V2_VERSION];
        // 32 bytes for context_id but missing epoch_id
        data.extend_from_slice(&[0u8; EXTRA_DATA_V1_LENGTH]);

        let err = parse_extra_data(&data).unwrap_err();
        assert!(
            err.to_string().contains("extra_data too short for v2"),
            "Unexpected error: {err}"
        );
    }
}
