use alloy::primitives::U256;

// Contract error selectors (4-byte function signatures)
// These are used for parsing and classifying contract revert errors

/// EnforcedPause - OpenZeppelin Pausable contract is paused
pub const SELECTOR_ENFORCED_PAUSE: &str = "0xd93c0665";

/// ERC20InsufficientBalance - OpenZeppelin ERC20 insufficient balance
pub const SELECTOR_INSUFFICIENT_BALANCE: &str = "0xe450d38c";

/// ERC20InsufficientAllowance - OpenZeppelin ERC20 insufficient allowance
pub const SELECTOR_INSUFFICIENT_ALLOWANCE: &str = "0xfb8f41b2";

/// InvalidUserSignature - Custom error from contracts/interfaces/IDecryption.sol
pub const SELECTOR_INVALID_SIGNATURE: &str = "0x2a873d27";

// ============================================================================
// Revert Parser
// ============================================================================
// Pure parsing logic for contract revert errors from RPC messages
// NO dependencies on HTTP or metrics modules

/// Revert reasons we care about for metrics and alerting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RevertReason {
    /// ERC20 insufficient balance - ALERT on this (relayer wallet issue)
    InsufficientBalance,

    /// ERC20 insufficient allowance - ALERT on this (relayer wallet issue)
    InsufficientAllowance,

    /// Contract paused - ALERT on this
    ContractPaused,

    /// Invalid user signature - track but don't alert
    InvalidSignature,

    /// Unknown revert
    Unknown,
}

/// Extract error selector (0xXXXXXXXX) from RPC message
///
/// Looks for patterns like "execution reverted: 0xd93c0665" or "0xd93c0665"
pub fn extract_revert_selector(message: &str) -> Option<String> {
    let message_lower = message.to_lowercase();

    // Look for 0x + 8 hex chars
    for (i, _) in message_lower.match_indices("0x") {
        if i + 10 <= message_lower.len() {
            let potential = &message_lower[i..i + 10];
            if potential[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(potential.to_string());
            }
        }
    }

    // Also try to find 8 hex chars without 0x prefix
    let chars: Vec<char> = message_lower.chars().collect();
    for window in chars.windows(8) {
        if window.iter().all(|c| c.is_ascii_hexdigit()) {
            let selector: String = window.iter().collect();
            return Some(format!("0x{}", selector));
        }
    }

    None
}

/// Classify error selector to revert reason
///
/// Recognizes specific error selectors:
/// - 0xd93c0665: EnforcedPause (OpenZeppelin Pausable)
/// - 0xe450d38c: ERC20InsufficientBalance (OpenZeppelin ERC20)
/// - 0xfb8f41b2: ERC20InsufficientAllowance (OpenZeppelin ERC20)
/// - 0x2a873d27: InvalidUserSignature (contracts/interfaces/IDecryption.sol)
pub fn classify_revert_selector(selector: &str) -> RevertReason {
    match selector {
        SELECTOR_ENFORCED_PAUSE => RevertReason::ContractPaused,
        SELECTOR_INSUFFICIENT_BALANCE => RevertReason::InsufficientBalance,
        SELECTOR_INSUFFICIENT_ALLOWANCE => RevertReason::InsufficientAllowance,
        SELECTOR_INVALID_SIGNATURE => RevertReason::InvalidSignature,
        _ => RevertReason::Unknown,
    }
}

// ============================================================================
// U256 Conversions
// ============================================================================

/// Converts U256 to i64 for database storage, returns error if value exceeds i64::MAX.
pub fn u256_to_i64(v: U256) -> Result<i64, &'static str> {
    if v > U256::from(i64::MAX) {
        return Err("U256 value too large for i64");
    }
    Ok(v.as_limbs()[0] as i64)
}

/// Converts U256 to i32 for database storage, returns error if value exceeds i32::MAX.
pub fn u256_to_i32(v: U256) -> Result<i32, &'static str> {
    if v > U256::from(i32::MAX) {
        return Err("U256 value too large for i32");
    }
    Ok(v.as_limbs()[0] as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    // ========================================================================
    // Revert Parser Tests
    // ========================================================================

    #[test]
    fn test_extract_revert_selector_with_prefix() {
        assert_eq!(
            extract_revert_selector("execution reverted: 0xd93c0665"),
            Some(SELECTOR_ENFORCED_PAUSE.to_string())
        );
    }

    #[test]
    fn test_extract_revert_selector_without_prefix() {
        assert_eq!(
            extract_revert_selector("execution reverted: d93c0665"),
            Some(SELECTOR_ENFORCED_PAUSE.to_string())
        );
    }

    #[test]
    fn test_extract_revert_selector_uppercase() {
        assert_eq!(
            extract_revert_selector("0xD93C0665"),
            Some(SELECTOR_ENFORCED_PAUSE.to_string())
        );
    }

    #[test]
    fn test_extract_revert_selector_no_match() {
        assert_eq!(extract_revert_selector("generic error"), None);
    }

    #[test]
    fn test_classify_revert_enforced_pause() {
        assert_eq!(
            classify_revert_selector(SELECTOR_ENFORCED_PAUSE),
            RevertReason::ContractPaused
        );
    }

    #[test]
    fn test_classify_revert_insufficient_balance() {
        assert_eq!(
            classify_revert_selector(SELECTOR_INSUFFICIENT_BALANCE),
            RevertReason::InsufficientBalance
        );
    }

    #[test]
    fn test_classify_revert_insufficient_allowance() {
        assert_eq!(
            classify_revert_selector(SELECTOR_INSUFFICIENT_ALLOWANCE),
            RevertReason::InsufficientAllowance
        );
    }

    #[test]
    fn test_classify_revert_invalid_signature() {
        assert_eq!(
            classify_revert_selector(SELECTOR_INVALID_SIGNATURE),
            RevertReason::InvalidSignature
        );
    }

    #[test]
    fn test_classify_revert_unknown_selector() {
        assert_eq!(
            classify_revert_selector("0x12345678"),
            RevertReason::Unknown
        );
    }

    // ========================================================================
    // U256 Conversion Tests
    // ========================================================================

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

    #[test]
    fn test_u256_to_i32_small_values() {
        let small = U256::from(123);
        assert_eq!(u256_to_i32(small).unwrap(), 123i32);
    }

    #[test]
    fn test_u256_to_i32_max_i32() {
        let max_i32 = U256::from(i32::MAX);
        assert_eq!(u256_to_i32(max_i32).unwrap(), i32::MAX);
    }

    #[test]
    fn test_u256_to_i32_overflow() {
        let too_large = U256::from(i32::MAX) + U256::from(1);
        assert!(u256_to_i32(too_large).is_err());
    }

    #[test]
    fn test_u256_to_i32_zero() {
        let zero = U256::ZERO;
        assert_eq!(u256_to_i32(zero).unwrap(), 0i32);
    }
}
