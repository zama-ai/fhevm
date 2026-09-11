use alloy::primitives::B256;
use solana_sdk::signature::Signature;
use std::fmt;

/// Native transaction identity; synthetic computation batches use a hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransactionId {
    Hash(B256),
    /// Solana identifies a transaction by its first 64-byte signature.
    SolanaSignature(Signature),
}

impl TransactionId {
    pub const ZERO: Self = Self::Hash(B256::ZERO);

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Hash(hash) => hash.as_slice(),
            Self::SolanaSignature(signature) => signature.as_ref(),
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl Default for TransactionId {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<B256> for TransactionId {
    fn from(hash: B256) -> Self {
        Self::Hash(hash)
    }
}

impl From<[u8; 32]> for TransactionId {
    fn from(hash: [u8; 32]) -> Self {
        Self::Hash(hash.into())
    }
}

impl TryFrom<&[u8]> for TransactionId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes.len() {
            32 => Ok(Self::Hash(B256::from_slice(bytes))),
            64 => Ok(Self::SolanaSignature(Signature::try_from(bytes)?)),
            length => anyhow::bail!(
                "transaction ID must contain 32 or 64 bytes, received {length}"
            ),
        }
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hash(hash) => write!(f, "{hash:#x}"),
            Self::SolanaSignature(signature) => write!(f, "{signature}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn native_ids_round_trip_and_display_without_losing_bytes() {
        let hash = TransactionId::Hash(B256::repeat_byte(7));
        let mut signature = [7; 64];
        let first = TransactionId::SolanaSignature(Signature::from(signature));
        signature[63] = 8;
        let second = TransactionId::SolanaSignature(Signature::from(signature));
        let ids = [hash, first, second];
        assert_eq!(ids.into_iter().collect::<HashSet<_>>().len(), 3);
        for id in ids {
            assert_eq!(TransactionId::try_from(id.as_slice()).unwrap(), id);
        }
        assert_eq!(hash.to_string(), format!("{:#x}", B256::repeat_byte(7)));
        assert_eq!(second.to_string(), bs58::encode(signature).into_string());
    }

    #[test]
    fn rejects_invalid_database_id_lengths() {
        for length in [0, 1, 31, 33, 63, 65] {
            assert!(
                TransactionId::try_from(vec![0; length].as_slice()).is_err()
            );
        }
    }
}
