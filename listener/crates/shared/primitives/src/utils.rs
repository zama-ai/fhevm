use alloy_primitives::Address;

/// Convert an optional [`Address`] to its EIP-55 checksummed string representation.
pub fn checksum_optional_address(addr: &Option<Address>) -> Option<String> {
    addr.map(|a| a.to_checksum(None))
}

pub fn chain_id_to_namespace(chain_id: u64) -> String {
    format!("chain-id-{}", chain_id)
}

/// Converts a u64 to i64, clamping to i64::MAX if the value exceeds it.
/// Prevents silent wrapping to negative values which could cause
/// destructive SQL behavior (e.g., deleting all rows instead of keeping N).
pub fn saturating_u64_to_i64(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturating_u64_to_i64_zero() {
        assert_eq!(saturating_u64_to_i64(0), 0);
    }

    #[test]
    fn saturating_u64_to_i64_normal_value() {
        assert_eq!(saturating_u64_to_i64(1000), 1000);
    }

    #[test]
    fn saturating_u64_to_i64_at_i64_max() {
        assert_eq!(saturating_u64_to_i64(i64::MAX as u64), i64::MAX);
    }

    #[test]
    fn saturating_u64_to_i64_above_i64_max() {
        assert_eq!(saturating_u64_to_i64(i64::MAX as u64 + 1), i64::MAX);
    }

    #[test]
    fn saturating_u64_to_i64_at_u64_max() {
        assert_eq!(saturating_u64_to_i64(u64::MAX), i64::MAX);
    }
}
