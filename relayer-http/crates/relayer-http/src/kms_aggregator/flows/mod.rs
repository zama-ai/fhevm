//! What differs between user decrypt and public decrypt: DTOs, route, response checks, counting, output shape.
//! The checks mirror what the gateway `Decryption.sol` did, minus KMS signer recovery (see `docs.md`, "Trust boundary").

pub mod public_decrypt;
pub mod user_decrypt;

use alloy::primitives::B256;
use serde::{Serialize, de::DeserializeOwned};

/// Why a `200` response does not count.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RejectReason {
    /// The SDK rejects any share whose signature is not 65 bytes: one malformed node must not break the answer.
    #[error("signature is {0} bytes, expected 65")]
    BadSignature(usize),
    /// Same signature bytes as an already accepted response: the gateway counted one response per KMS signer.
    #[error("duplicate of an accepted response")]
    Duplicate,
}

/// A decryption flow. Stateless: plain functions over the connector DTOs.
pub trait Flow: Send + Sync + 'static {
    type Request: Serialize + Send + Sync + 'static;
    type Response: DeserializeOwned + Send + 'static;
    type Output: Serialize + Send + 'static;
    const NAME: &'static str;
    const ROUTE: &'static str;
    /// The connector's content-derived `decryptionId` (the same on every node). Logging only.
    fn decryption_id(request: &Self::Request) -> B256;
    /// Ciphertext handles, for logs.
    fn handles(request: &Self::Request) -> Vec<B256>;
    /// Whether this response can be accepted next to the already accepted ones.
    fn check(accepted: &[Self::Response], response: &Self::Response) -> Result<(), RejectReason>;
    /// How many accepted responses count toward the threshold.
    fn counted(accepted: &[Self::Response]) -> usize;
    /// The relayer's answer, once `counted >= threshold`. `None` only if `accepted` is empty.
    fn output(accepted: Vec<Self::Response>) -> Option<Self::Output>;
}

/// 65-byte signature and not a byte-for-byte copy of an accepted one.
pub(crate) fn check_signature<'a>(
    mut accepted: impl Iterator<Item = &'a [u8]>,
    signature: &[u8],
) -> Result<(), RejectReason> {
    if signature.len() != 65 {
        return Err(RejectReason::BadSignature(signature.len()));
    }
    if accepted.any(|a| a == signature) {
        return Err(RejectReason::Duplicate);
    }
    Ok(())
}

/// Hex without `0x`, as the current relayer answers.
pub(crate) fn hex(bytes: &[u8]) -> String {
    alloy::hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_signature_rules() {
        let good = [7u8; 65];
        let other = [8u8; 65];
        assert_eq!(check_signature([].into_iter(), &good), Ok(()));
        assert_eq!(
            check_signature([other.as_slice()].into_iter(), &good),
            Ok(())
        );
        assert_eq!(
            check_signature([good.as_slice()].into_iter(), &good),
            Err(RejectReason::Duplicate)
        );
        assert_eq!(
            check_signature([].into_iter(), &good[..64]),
            Err(RejectReason::BadSignature(64))
        );
        assert_eq!(
            check_signature([].into_iter(), &[]),
            Err(RejectReason::BadSignature(0))
        );
    }

    #[test]
    fn hex_has_no_prefix() {
        assert_eq!(hex(&[0x00, 0xab, 0xff]), "00abff");
        assert_eq!(hex(&[]), "");
    }
}
