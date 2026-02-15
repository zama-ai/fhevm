use alloy::primitives::U256;
use std::fmt;

/// A validated, non-negative chain identifier.
///
/// Internally stored as `i64` (matching PostgreSQL BIGINT), but guaranteed
/// to be non-negative (>= 0) so it can safely round-trip between i64 and u64.
///
/// Construction is fallible — use `TryFrom<u64>`, `TryFrom<i64>`, or
/// `TryFrom<U256>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChainId(i64);

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid chain id: {value} (must be non-negative and fit in i64)")]
pub struct InvalidChainId {
    value: String,
}

impl ChainId {
    /// Returns the inner value as `i64` (for database operations).
    #[inline]
    pub fn as_i64(self) -> i64 {
        self.0
    }

    /// Returns the inner value as `u64` (for blockchain APIs).
    /// Safe because the invariant guarantees 0 <= self.0 <= i64::MAX.
    #[inline]
    pub fn as_u64(self) -> u64 {
        self.0 as u64
    }
}

impl TryFrom<i64> for ChainId {
    type Error = InvalidChainId;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(ChainId(value))
        } else {
            Err(InvalidChainId {
                value: value.to_string(),
            })
        }
    }
}

impl TryFrom<u64> for ChainId {
    type Error = InvalidChainId;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if let Ok(value) = i64::try_from(value) {
            Ok(ChainId(value))
        } else {
            Err(InvalidChainId {
                value: value.to_string(),
            })
        }
    }
}

impl TryFrom<U256> for ChainId {
    type Error = InvalidChainId;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value > U256::from(i64::MAX as u64) {
            return Err(InvalidChainId {
                value: value.to_string(),
            });
        }
        Ok(ChainId(value.to::<i64>()))
    }
}

impl From<ChainId> for U256 {
    fn from(id: ChainId) -> Self {
        U256::from(id.as_u64())
    }
}

impl fmt::Display for ChainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_i64() {
        let id = ChainId::try_from(1_i64).unwrap();
        assert_eq!(id.as_i64(), 1);
        assert_eq!(id.as_u64(), 1);
    }

    #[test]
    fn valid_u64() {
        let id = ChainId::try_from(12345_u64).unwrap();
        assert_eq!(id.as_i64(), 12345);
        assert_eq!(id.as_u64(), 12345);
    }

    #[test]
    fn zero_is_valid() {
        let id = ChainId::try_from(0_i64).unwrap();
        assert_eq!(id.as_i64(), 0);
        assert_eq!(id.as_u64(), 0);

        let id = ChainId::try_from(0_u64).unwrap();
        assert_eq!(id.as_i64(), 0);

        let id = ChainId::try_from(U256::ZERO).unwrap();
        assert_eq!(id.as_i64(), 0);
    }

    #[test]
    fn max_i64() {
        let id = ChainId::try_from(i64::MAX).unwrap();
        assert_eq!(id.as_i64(), i64::MAX);
        assert_eq!(id.as_u64(), i64::MAX as u64);
    }

    #[test]
    fn rejects_negative_i64() {
        assert!(ChainId::try_from(-1_i64).is_err());
    }

    #[test]
    fn rejects_overflow_u64() {
        assert!(ChainId::try_from(u64::MAX).is_err());
        assert!(ChainId::try_from(i64::MAX as u64 + 1).is_err());
    }

    #[test]
    fn valid_u256() {
        let id = ChainId::try_from(U256::from(42)).unwrap();
        assert_eq!(id.as_i64(), 42);
    }

    #[test]
    fn rejects_overflow_u256() {
        assert!(ChainId::try_from(U256::from(i64::MAX as u64 + 1)).is_err());
    }

    #[test]
    fn into_u256() {
        let id = ChainId::try_from(99_u64).unwrap();
        let u: U256 = id.into();
        assert_eq!(u, U256::from(99));
    }

    #[test]
    fn display() {
        let id = ChainId::try_from(12345_u64).unwrap();
        assert_eq!(format!("{id}"), "12345");
    }
}
