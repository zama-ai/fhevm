//! Stateless checks shared by the routes, before any fan-out: the connector's cheap handle rules (it would reject
//! these requests anyway; failing here saves n calls) and the `extraData` format. Nothing here touches a chain.

use std::fmt::Display;

use alloy::primitives::B256;

use crate::config::HttpConfig;

/// The connector's budget for one decryption request, in plaintext bits.
const MAX_DECRYPTION_BITS: u32 = 2048;

/// Which field is wrong, and how. Rendered as the message of a `400 malformed`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{field}: {issue}")]
pub struct Invalid {
    pub field: String,
    pub issue: String,
    /// The offending value, for the DEBUG `request invalid` line only: never in the answer.
    pub value: Option<String>,
}

impl Invalid {
    pub fn new(field: impl Into<String>, issue: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            issue: issue.into(),
            value: None,
        }
    }

    /// The same, naming the offending value.
    #[must_use]
    pub fn with_value(mut self, value: impl Display) -> Self {
        self.value = Some(value.to_string());
        self
    }
}

/// `value` must equal `expected`.
pub fn expect(field: &str, value: &str, expected: &str) -> Result<(), Invalid> {
    if value == expected {
        Ok(())
    } else {
        Err(Invalid::new(field, format!("must be \"{expected}\"")))
    }
}

/// The chain id and the plaintext size a handle carries.
/// Handle layout: `[0..21] hash | 21 index | 22..30 chain id (big-endian u64) | 30 FHE type | 31 version`.
pub fn decode_handle(handle: &B256) -> Result<(u64, u32), &'static str> {
    let [.., c0, c1, c2, c3, c4, c5, c6, c7, fhe_type, _version] = handle.0;
    let bits = match fhe_type {
        0 => 2, // ebool
        2 => 8,
        3 => 16,
        4 => 32,
        5 => 64,
        6 => 128,
        7 => 160, // eaddress
        8 => 256,
        _ => return Err("FHE type cannot be decrypted"),
    };
    Ok((u64::from_be_bytes([c0, c1, c2, c3, c4, c5, c6, c7]), bits))
}

/// Non-empty, at most `http.max_handles_per_decryption`, decryptable types, one chain id of
/// `http.supported_chain_ids`, at most 2048 bits in total.
pub fn handles<'a>(
    field: &str,
    handles: impl ExactSizeIterator<Item = &'a B256>,
    http: &HttpConfig,
) -> Result<(), Invalid> {
    let (chain_ids, max) = (&http.supported_chain_ids, http.max_handles_per_decryption);
    if handles.len() > max {
        return Err(Invalid::new(field, format!("at most {max} handles"))
            .with_value(format_args!("{} handles", handles.len())));
    }
    let (mut chain, mut bits, mut count) = (None, 0u32, 0usize);
    for (i, handle) in handles.enumerate() {
        count += 1;
        let (chain_id, size) = decode_handle(handle)
            .map_err(|issue| Invalid::new(format!("{field}[{i}]"), issue).with_value(handle))?;
        if *chain.get_or_insert(chain_id) != chain_id {
            return Err(Invalid::new(
                format!("{field}[{i}]"),
                "all handles must be on the same chain",
            )
            .with_value(handle));
        }
        if !chain_ids.contains(&chain_id) {
            return Err(Invalid::new(
                format!("{field}[{i}]"),
                format!("chain id {chain_id} is not supported"),
            )
            .with_value(handle));
        }
        bits = bits.saturating_add(size);
    }
    if count == 0 {
        return Err(Invalid::new(field, "must not be empty"));
    }
    if bits > MAX_DECRYPTION_BITS {
        return Err(Invalid::new(
            field,
            format!("total size {bits} bits exceeds {MAX_DECRYPTION_BITS}"),
        )
        .with_value(format_args!("{count} handles")));
    }
    Ok(())
}

/// `0x02` + 32-byte context id + 32-byte epoch id (v2) only; v0 (empty or `0x00`) and v1 are rejected.
/// Trailing bytes are allowed, as the connector allows them.
pub fn extra_data(field: &str, bytes: &[u8]) -> Result<(), Invalid> {
    let ok = match bytes.first() {
        Some(2) => bytes.len() >= 65,
        _ => false,
    };
    if ok {
        Ok(())
    } else {
        Err(Invalid::new(field, "must be 0x02 + context id + epoch id"))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A handle carrying `chain_id` and `fhe_type`, the rest zero.
    pub(crate) fn handle(chain_id: u64, fhe_type: u8) -> B256 {
        let mut bytes = [0u8; 32];
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        bytes[30] = fhe_type;
        bytes[31] = 1;
        B256::from(bytes)
    }

    #[test]
    fn decode_handle_table() {
        for (fhe_type, bits) in [
            (0, 2),
            (2, 8),
            (3, 16),
            (4, 32),
            (5, 64),
            (6, 128),
            (7, 160),
            (8, 256),
        ] {
            assert_eq!(decode_handle(&handle(137, fhe_type)), Ok((137, bits)));
        }
        for bad in [1, 9, 12, 255] {
            assert!(decode_handle(&handle(1, bad)).is_err(), "{bad}");
        }
        assert_eq!(decode_handle(&handle(u64::MAX, 4)), Ok((u64::MAX, 32)));
    }

    #[test]
    fn handles_rules() {
        let ok = [handle(1, 4), handle(1, 0)];
        assert_eq!(
            handles("h", ok.iter(), &HttpConfig::for_tests(&[1, 137])),
            Ok(())
        );
        assert_eq!(
            handles("h", [].iter(), &HttpConfig::for_tests(&[1])),
            Err(Invalid::new("h", "must not be empty"))
        );
        assert_eq!(
            handles(
                "h",
                [handle(1, 4), handle(1, 9)].iter(),
                &HttpConfig::for_tests(&[1])
            ),
            Err(Invalid::new("h[1]", "FHE type cannot be decrypted").with_value(handle(1, 9)))
        );
        assert_eq!(
            handles(
                "h",
                [handle(1, 4), handle(137, 4)].iter(),
                &HttpConfig::for_tests(&[1, 137])
            ),
            Err(
                Invalid::new("h[1]", "all handles must be on the same chain")
                    .with_value(handle(137, 4))
            )
        );
        assert_eq!(
            handles(
                "h",
                [handle(31337, 4)].iter(),
                &HttpConfig::for_tests(&[1, 137])
            ),
            Err(Invalid::new("h[0]", "chain id 31337 is not supported")
                .with_value(handle(31337, 4)))
        );
        let eight = vec![handle(1, 8); 8];
        assert_eq!(
            handles("h", eight.iter(), &HttpConfig::for_tests(&[1])),
            Ok(())
        );
        let too_many = vec![handle(1, 0); 51];
        assert_eq!(
            handles("h", too_many.iter(), &HttpConfig::for_tests(&[1])),
            Err(Invalid::new("h", "at most 50 handles").with_value("51 handles"))
        );
        let nine = vec![handle(1, 8); 9];
        assert_eq!(
            handles("h", nine.iter(), &HttpConfig::for_tests(&[1])),
            Err(Invalid::new("h", "total size 2304 bits exceeds 2048").with_value("9 handles"))
        );
    }

    #[test]
    fn extra_data_rules() {
        let rejected = Err(Invalid::new("x", "must be 0x02 + context id + epoch id"));
        assert_eq!(extra_data("x", &[]), rejected);
        assert_eq!(extra_data("x", &[0]), rejected);
        assert_eq!(extra_data("x", &[1; 33]), rejected);
        assert_eq!(extra_data("x", &[2; 65]), Ok(()));
        assert_eq!(extra_data("x", &[2; 70]), Ok(()));
        for bad in [vec![1; 32], vec![2; 64], vec![3; 33], vec![0xff]] {
            assert!(extra_data("x", &bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn expect_names_the_field() {
        assert_eq!(expect("version", "2.0", "2.0"), Ok(()));
        assert_eq!(
            expect("version", "1.0", "2.0"),
            Err(Invalid::new("version", "must be \"2.0\""))
        );
        assert_eq!(Invalid::new("a.b[0]", "bad").to_string(), "a.b[0]: bad");
    }
}
