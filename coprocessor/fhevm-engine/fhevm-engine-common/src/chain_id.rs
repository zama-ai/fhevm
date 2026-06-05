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

/// RFC-021 reserves the high bit of the u64 chain id as the host `chain_type`
/// marker: when set, the host chain is Solana rather than an EVM chain. The
/// remaining 63 bits carry the logical chain id.
pub const SOLANA_CHAIN_TYPE_BIT: u64 = 1 << 63;

impl ChainId {
    /// Returns the inner value as `i64` (for database operations).
    ///
    /// For a Solana host id (chain-type high bit set) this is the negative
    /// two's-complement bit pattern of the canonical u64; the BIGINT column
    /// stores that pattern so the value round-trips back through [`Self::as_u64`].
    #[inline]
    pub fn as_i64(self) -> i64 {
        self.0
    }

    /// Returns the canonical u64 chain id (for blockchain APIs and handle
    /// derivation). The chain-type high bit is preserved verbatim.
    #[inline]
    pub fn as_u64(self) -> u64 {
        self.0 as u64
    }

    /// Builds a chain id from a canonical u64 host identifier, accepting the
    /// RFC-021 `chain_type` high bit. Unlike `TryFrom<u64>` (which is strict and
    /// rejects values above `i64::MAX` for EVM safety), this preserves the full
    /// 64-bit identity by storing its two's-complement bit pattern, so a Solana
    /// host id survives the round-trip through the i64-backed BIGINT column.
    #[inline]
    pub fn from_canonical_u64(value: u64) -> Self {
        ChainId(value as i64)
    }

    /// True when the chain-type high bit marks this as a Solana host chain.
    #[inline]
    pub fn is_solana_host(self) -> bool {
        self.as_u64() & SOLANA_CHAIN_TYPE_BIT != 0
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
        if i64::try_from(value).is_ok() {
            Ok(ChainId(value as i64))
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

    #[test]
    fn evm_chain_is_not_solana_host() {
        let id = ChainId::try_from(12345_u64).unwrap();
        assert!(!id.is_solana_host());
    }

    #[test]
    fn canonical_u64_round_trips_solana_high_bit() {
        let canonical = SOLANA_CHAIN_TYPE_BIT | 12345;
        let id = ChainId::from_canonical_u64(canonical);

        // Recovered verbatim as u64, even though the i64 storage is negative.
        assert_eq!(id.as_u64(), canonical);
        assert!(id.as_i64() < 0);
        assert!(id.is_solana_host());

        // Storing the i64 bit pattern and reading it back preserves identity,
        // matching how the BIGINT column round-trips the value.
        assert_eq!(ChainId::from_canonical_u64(id.as_i64() as u64), id);
    }

    #[test]
    fn canonical_u64_preserves_evm_ids() {
        let id = ChainId::from_canonical_u64(12345);
        assert_eq!(id.as_u64(), 12345);
        assert_eq!(id.as_i64(), 12345);
        assert!(!id.is_solana_host());
    }
}
