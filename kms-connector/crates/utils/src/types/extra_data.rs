use alloy::primitives::U256;
use anyhow::anyhow;

/// The expected length of the `extra_data` bytes.
const EXPECTED_EXTRA_DATA_LENGTH: usize = 33;

/// The version of the unified `extra_data` format.
const EXTRA_DATA_VERSION: u8 = 0x01;

/// Parses the `extra_data` bytes to extract an optional context ID.
///
/// Format:
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

/// Parses the optional ordered list of native identities from the `extra_data` bytes.
///
/// Unified format:
/// - Byte 0: version (`0x01`)
/// - Bytes 1..33: context ID
/// - Byte 33: identity count
/// - Bytes 34..: identities (`count * 32` bytes)
///
/// The exact interpretation of the identities depends on the caller:
/// - user decryption: `[user_id, contract_id...]`
/// - delegated user decryption: `[delegator_id, delegate_id, contract_id...]`
///
/// The format optionally supports a trailing auth signer:
/// - Byte 0: version (`0x01`)
/// - Bytes 1..33: context ID
/// - Byte 33: identity count
/// - Bytes 34..: identities (`count * 32` bytes)
/// - Optional next byte: auth signer byte length
/// - Optional remaining bytes: auth signer (`verifier || key`)
pub fn parse_extra_data_identities(extra_data: &[u8]) -> anyhow::Result<Option<Vec<[u8; 32]>>> {
    if extra_data.is_empty() || extra_data == [0x00] {
        return Ok(None);
    }

    if extra_data[0] != EXTRA_DATA_VERSION {
        return Err(anyhow!(
            "Unsupported extra_data version: 0x{:02x}, expected 0x{:02x}",
            extra_data[0],
            EXTRA_DATA_VERSION
        ));
    }

    if extra_data.len() == EXPECTED_EXTRA_DATA_LENGTH {
        return Ok(None);
    }

    if extra_data.len() < 34 {
        return Err(anyhow!(
            "extra_data too short for identities: {} bytes, expected at least 34 bytes",
            extra_data.len()
        ));
    }
    let count = extra_data[33] as usize;
    let identities_offset = 34usize;
    let base_len = identities_offset + count * 32;
    let expected_len = if extra_data.len() == base_len {
        base_len
    } else {
        if extra_data.len() < base_len + 1 {
            return Err(anyhow!(
                "extra_data too short for auth signer: {} bytes, expected at least {} bytes",
                extra_data.len(),
                base_len + 1
            ));
        }
        let auth_signer_len = extra_data[base_len] as usize;
        if auth_signer_len < 20 {
            return Err(anyhow!(
                "invalid auth signer length: expected at least 20 bytes, got {}",
                auth_signer_len
            ));
        }
        base_len + 1 + auth_signer_len
    };

    if extra_data.len() != expected_len {
        return Err(anyhow!(
            "invalid extra_data length: {} bytes, expected {} bytes for {} identities",
            extra_data.len(),
            expected_len,
            count
        ));
    }

    let mut identities = Vec::with_capacity(count);
    for index in 0..count {
        let start = identities_offset + index * 32;
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
        let mut data = vec![0x04]; // wrong version
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
    fn valid_context_and_identities() {
        let mut data = vec![0x01];
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
    fn valid_context_and_identities_with_auth_signer() {
        let mut data = vec![0x01];
        let context_id = U256::from(69u64);
        data.extend_from_slice(&context_id.to_be_bytes::<32>());
        data.push(2);
        data.extend_from_slice(&[0x11; 32]);
        data.extend_from_slice(&[0x22; 32]);
        data.push(20);
        data.extend_from_slice(&[0x44; 20]);

        assert_eq!(parse_extra_data_context(&data).unwrap(), Some(context_id));
        assert_eq!(
            parse_extra_data_identities(&data).unwrap(),
            Some(vec![[0x11; 32], [0x22; 32]])
        );
    }

    #[test]
    fn context_only_has_no_identities() {
        let mut data = vec![0x01];
        data.extend_from_slice(&U256::from(69u64).to_be_bytes::<32>());
        assert_eq!(parse_extra_data_identities(&data).unwrap(), None);
    }

    #[test]
    fn invalid_identity_length_errors() {
        let mut data = vec![0x01];
        data.extend_from_slice(&U256::from(69u64).to_be_bytes::<32>());
        data.push(1);

        let err = parse_extra_data_identities(&data).unwrap_err();
        assert!(
            err.to_string().contains("extra_data too short for auth signer"),
            "Unexpected error: {err}"
        );
    }
}
