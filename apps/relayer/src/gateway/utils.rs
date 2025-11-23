use alloy::primitives::U256;

/// Converts U256 to i64 for database storage, returns error if value exceeds i64::MAX.
pub fn u256_to_i64(v: U256) -> Result<i64, &'static str> {
    if v > U256::from(i64::MAX) {
        return Err("U256 value too large for i64");
    }
    Ok(v.as_limbs()[0] as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    #[test]
    fn test_u256_to_i64_small_values() {
        let small = U256::from(123);
        assert_eq!(u256_to_i64(small).unwrap(), 123i64);
    }

    #[test]
    fn test_u256_to_i64_max_i64() {
        let max_i64 = U256::from(i64::MAX);
        assert_eq!(u256_to_i64(max_i64).unwrap(), i64::MAX);
    }

    #[test]
    fn test_u256_to_i64_overflow() {
        let too_large = U256::from(i64::MAX) + U256::from(1);
        assert!(u256_to_i64(too_large).is_err());
    }

    #[test]
    fn test_u256_to_i64_zero() {
        let zero = U256::ZERO;
        assert_eq!(u256_to_i64(zero).unwrap(), 0i64);
    }
}
