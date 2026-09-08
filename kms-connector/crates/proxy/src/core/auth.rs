//! Single pre-shared API key authentication of the sender.

use alloy::primitives::B256;
use http::HeaderValue;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

/// Verifies `Authorization: Bearer <api-key>` headers against the SHA-256 digest of the key.
///
/// Only the digest is held in memory, not the plaintext key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiKeyVerifier {
    api_key_digest: B256,
}

impl ApiKeyVerifier {
    pub fn new(api_key_digest: B256) -> Self {
        Self { api_key_digest }
    }

    /// Returns `true` iff the header is a well-formed bearer token whose SHA-256 matches the
    /// configured digest.
    pub fn verify(&self, authorization: Option<&HeaderValue>) -> bool {
        let Some(token) = authorization.and_then(bearer_token) else {
            return false;
        };
        let digest = Sha256::digest(token.as_bytes());
        digest
            .as_slice()
            .ct_eq(self.api_key_digest.as_slice())
            .into()
    }
}

/// Extracts the token of a `Bearer <token>` credential.
fn bearer_token(header: &HeaderValue) -> Option<&str> {
    let value = header.to_str().ok()?;
    let (scheme, token) = value.trim().split_once(char::is_whitespace)?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let token = token.trim();
    (!token.is_empty()).then_some(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "test";
    // sha256("test")
    const DIGEST: &str = "0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";

    fn verifier() -> ApiKeyVerifier {
        ApiKeyVerifier::new(DIGEST.parse().unwrap())
    }

    fn header(value: &str) -> HeaderValue {
        HeaderValue::from_str(value).unwrap()
    }

    #[test]
    fn accepts_matching_bearer_token() {
        assert!(verifier().verify(Some(&header(&format!("Bearer {KEY}")))));
        assert!(verifier().verify(Some(&header(&format!("bearer {KEY}")))));
        assert!(verifier().verify(Some(&header(&format!("Bearer   {KEY} ")))));
    }

    #[test]
    fn rejects_bad_tokens() {
        let verifier = verifier();
        assert!(!verifier.verify(None));
        assert!(!verifier.verify(Some(&header("Bearer wrong"))));
        assert!(!verifier.verify(Some(&header("Bearer"))));
        assert!(!verifier.verify(Some(&header("Bearer "))));
        assert!(!verifier.verify(Some(&header(&format!("Basic {KEY}")))));
        assert!(!verifier.verify(Some(&header(KEY))));
        assert!(!verifier.verify(Some(&header(&format!("Bearer {KEY}x")))));
    }
}
