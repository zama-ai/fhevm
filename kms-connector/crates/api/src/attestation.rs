//! The `attestationType` discriminant of `v1/user-decrypt`.

use strum::{Display, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

/// The signature schemes a `v1/user-decrypt` `signature` can be produced with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, EnumIter, IntoStaticStr)]
pub enum AttestationType {
    /// Unified EIP-712 user decryption: the signature is the user's EIP-712 (or ERC-1271)
    /// signature over the `payload` (except `handles`, which are excluded from the signature).
    #[strum(serialize = "eip712-unified-user-decrypt-v1")]
    Eip712UnifiedUserDecryptV1,
}

impl AttestationType {
    /// The `attestationType` values this crate version supports.
    pub fn supported() -> impl Iterator<Item = &'static str> {
        Self::iter().map(Self::into)
    }
}
