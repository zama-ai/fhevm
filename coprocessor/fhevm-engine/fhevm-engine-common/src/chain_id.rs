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

/// High byte of the eight-byte chain-id field (handle bytes 22–29).
pub const EVM_CHAIN_TYPE: u8 = 0x00;
pub const SOLANA_CHAIN_TYPE: u8 = 0x01;
const CHAIN_TYPE_SHIFT: u32 = 56;
pub const CLUSTER_TAG_MASK: u64 = 0x00ff_ffff_ffff_ffff;

pub const fn chain_type_byte(chain_id: u64) -> u8 {
    (chain_id >> CHAIN_TYPE_SHIFT) as u8
}

pub const fn is_evm_host_chain_id(chain_id: u64) -> bool {
    chain_type_byte(chain_id) == EVM_CHAIN_TYPE
}

pub const fn is_solana_host_chain_id(chain_id: u64) -> bool {
    chain_type_byte(chain_id) == SOLANA_CHAIN_TYPE
}

pub const fn solana_host_chain_id(cluster_tag: u64) -> u64 {
    ((SOLANA_CHAIN_TYPE as u64) << CHAIN_TYPE_SHIFT) | (cluster_tag & CLUSTER_TAG_MASK)
}

impl ChainId {
    /// Returns the inner value as `i64` (for database operations).
    ///
    /// Type-byte Solana ids sit below `i64::MAX` and store as a positive BIGINT.
    /// [`Self::from_canonical_u64`] still bitcasts any `u64`, including values
    /// with bit 63 set.
    #[inline]
    pub fn as_i64(self) -> i64 {
        self.0
    }

    /// Returns the canonical u64 chain id (for blockchain APIs and handle
    /// derivation). The type byte is preserved verbatim.
    #[inline]
    pub fn as_u64(self) -> u64 {
        self.0 as u64
    }

    /// Builds a chain id from a canonical u64 host identifier. Unlike
    /// `TryFrom<u64>` (which rejects values above `i64::MAX`), this bitcasts the
    /// full 64-bit identity so a value with bit 63 set still round-trips through
    /// the BIGINT column. Type-byte Solana ids do not need that path.
    #[inline]
    pub fn from_canonical_u64(value: u64) -> Self {
        ChainId(value as i64)
    }

    /// True when the high byte is the Solana type byte `0x01`.
    #[inline]
    pub fn is_solana_host(self) -> bool {
        is_solana_host_chain_id(self.as_u64())
    }

    /// True when the high byte is the EVM type byte `0x00`.
    #[inline]
    pub fn is_evm_host(self) -> bool {
        is_evm_host_chain_id(self.as_u64())
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
        assert!(id.is_evm_host());
    }

    #[test]
    fn unknown_type_byte_is_neither_evm_nor_solana() {
        let id = ChainId::from_canonical_u64(0x0200_0000_0000_3039);
        assert!(!id.is_solana_host());
        assert!(!id.is_evm_host());
        let leftover_bit63 = ChainId::from_canonical_u64(0x8000_0000_0000_3039);
        assert!(!leftover_bit63.is_solana_host());
        assert!(!leftover_bit63.is_evm_host());
    }

    #[test]
    fn canonical_u64_round_trips_solana_type_byte() {
        let canonical = solana_host_chain_id(12345);
        let id = ChainId::from_canonical_u64(canonical);

        assert_eq!(id.as_u64(), canonical);
        assert!(id.as_i64() > 0);
        assert!(id.is_solana_host());
        assert_eq!(ChainId::try_from(canonical).unwrap(), id);
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
