//! An HTTP request authorized by an Ethereum key.
//!
//! The caller signs the EIP-712 hash of `RequestAuthorization { path, bodyDigest, expires }` and
//! sends the signature in the `Authorization` header:
//!
//! ```text
//! Authorization: Zama-EIP712 expires=1790950000, signature=0x<65 bytes: r, s, v>
//! ```
//!
//! `bodyDigest` is the keccak-256 of the exact body bytes, so a server checks what it received
//! without re-encoding it. The server recovers the signer and decides whether that address may
//! call. No recipient is signed: one signature covers every server the caller sends the same
//! request to, so a server can replay it to the others until it expires. That suits requests that
//! every recipient answers the same way and that start no new work.

use alloy_primitives::{Address, B256, Signature, hex, keccak256};
use alloy_sol_types::{Eip712Domain, SolStruct, eip712_domain, sol};

sol! {
    /// The signed fields of a request.
    struct RequestAuthorization {
        string path;
        bytes32 bodyDigest;
        uint64 expires;
    }
}

/// The `Authorization` scheme.
pub const SCHEME: &str = "Zama-EIP712";

/// How far ahead of the server's clock `expires` may be, in seconds. A caller picks a shorter
/// validity; the margin absorbs clock skew.
pub const MAX_VALIDITY_SECS: u64 = 300;

/// The signing domain. `chain_id` is the canonical chain that registers the callers' keys, so a
/// signature for one network is invalid on another.
pub fn domain(chain_id: u64) -> Eip712Domain {
    eip712_domain! {
        name: "zama-request-authorization",
        version: "1",
        chain_id: chain_id,
    }
}

/// The hash a caller signs for `body` sent to `path`, valid until `expires` (Unix seconds).
pub fn signing_hash(chain_id: u64, path: &str, body: &[u8], expires: u64) -> B256 {
    RequestAuthorization {
        path: path.to_owned(),
        bodyDigest: keccak256(body),
        expires,
    }
    .eip712_signing_hash(&domain(chain_id))
}

/// The `Authorization` header value for `signature` over a request valid until `expires`.
pub fn header_value(expires: u64, signature: &Signature) -> String {
    format!(
        "{SCHEME} expires={expires}, signature=0x{}",
        hex::encode(signature.as_bytes())
    )
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthorizationError {
    #[error("missing or malformed {SCHEME} authorization")]
    Malformed,
    #[error("the authorization expired at {expires}, before {now}")]
    Expired { expires: u64, now: u64 },
    #[error("the authorization expires at {expires}, more than {MAX_VALIDITY_SECS} s after {now}")]
    TooFarAhead { expires: u64, now: u64 },
    #[error("the signature recovers no signer")]
    BadSignature,
}

/// Recovers who signed the `Authorization` header `header` for `body` sent to `path`, at Unix time
/// `now`. A signature over other fields recovers another address, so the caller must check the
/// signer against the addresses it accepts.
pub fn recover_signer(
    chain_id: u64,
    header: &str,
    path: &str,
    body: &[u8],
    now: u64,
) -> Result<Address, AuthorizationError> {
    let (expires, signature) = parse(header)?;
    if expires < now {
        return Err(AuthorizationError::Expired { expires, now });
    }
    if expires > now.saturating_add(MAX_VALIDITY_SECS) {
        return Err(AuthorizationError::TooFarAhead { expires, now });
    }
    signature
        .recover_address_from_prehash(&signing_hash(chain_id, path, body, expires))
        .map_err(|_| AuthorizationError::BadSignature)
}

fn parse(header: &str) -> Result<(u64, Signature), AuthorizationError> {
    let malformed = || AuthorizationError::Malformed;
    let parameters = header
        .strip_prefix(SCHEME)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or_else(malformed)?;
    let (expires, signature) = parameters.split_once(", ").ok_or_else(malformed)?;
    let expires = expires
        .strip_prefix("expires=")
        .filter(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|digits| digits.parse().ok())
        .ok_or_else(malformed)?;
    let signature = signature
        .strip_prefix("signature=0x")
        .and_then(|hex_bytes| hex::decode(hex_bytes).ok())
        .and_then(|bytes| Signature::try_from(bytes.as_slice()).ok())
        .ok_or_else(malformed)?;
    Ok((expires, signature))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_signer::SignerSync;
    use alloy_signer_local::PrivateKeySigner;

    const CHAIN_ID: u64 = 12345;
    const PATH: &str = "/v1/solana/merkle-proofs";
    const BODY: &[u8] = b"{\"leaves\":[]}";
    const NOW: u64 = 1_790_950_000;

    fn signer() -> PrivateKeySigner {
        "0x3f45b129a7fd099146e9fe63851a71646231f7743c712695f3b2d2bf0e41c774"
            .parse()
            .unwrap()
    }

    fn header(expires: u64) -> String {
        let signature = signer()
            .sign_hash_sync(&signing_hash(CHAIN_ID, PATH, BODY, expires))
            .unwrap();
        header_value(expires, &signature)
    }

    #[test]
    fn the_signer_is_recovered_within_the_validity() {
        let address = signer().address();
        for expires in [NOW, NOW + 60, NOW + MAX_VALIDITY_SECS] {
            assert_eq!(
                recover_signer(CHAIN_ID, &header(expires), PATH, BODY, NOW),
                Ok(address)
            );
        }
    }

    #[test]
    fn other_fields_recover_another_signer() {
        let address = signer().address();
        let header = header(NOW + 60);
        for (chain_id, path, body) in [
            (CHAIN_ID + 1, PATH, BODY),
            (CHAIN_ID, "/v1/other", BODY),
            (CHAIN_ID, PATH, b"{\"leaves\":[1]}".as_slice()),
        ] {
            assert_ne!(
                recover_signer(chain_id, &header, path, body, NOW),
                Ok(address)
            );
        }
    }

    #[test]
    fn an_expired_or_far_future_authorization_is_refused() {
        assert_eq!(
            recover_signer(CHAIN_ID, &header(NOW - 1), PATH, BODY, NOW),
            Err(AuthorizationError::Expired {
                expires: NOW - 1,
                now: NOW
            })
        );
        let expires = NOW + MAX_VALIDITY_SECS + 1;
        assert_eq!(
            recover_signer(CHAIN_ID, &header(expires), PATH, BODY, NOW),
            Err(AuthorizationError::TooFarAhead { expires, now: NOW })
        );
    }

    #[test]
    fn a_malformed_header_is_refused() {
        let valid = header(NOW + 60);
        let (_, signature) = valid.split_once("signature=0x").unwrap();
        for malformed in [
            String::new(),
            "Bearer secret".to_owned(),
            valid.replacen(SCHEME, "zama-eip712", 1),
            valid.replacen(", ", ",", 1),
            valid.replacen("expires=", "expires=+", 1),
            format!("{SCHEME} expires=, signature=0x{signature}"),
            format!(
                "{SCHEME} expires={}, signature=0x{}",
                NOW + 60,
                &signature[2..]
            ),
            format!("{SCHEME} expires={}, signature={signature}", NOW + 60),
        ] {
            assert_eq!(
                recover_signer(CHAIN_ID, &malformed, PATH, BODY, NOW),
                Err(AuthorizationError::Malformed),
                "{malformed}"
            );
        }
    }

    /// Pins the wire format. viem's `hashTypedData` and `signTypedData` produce the same values.
    #[test]
    fn the_wire_format_is_pinned() {
        assert_eq!(
            signing_hash(CHAIN_ID, PATH, BODY, NOW + 60).to_string(),
            "0x98d689106ab8952c9c7d3e50943f45752750c846203ef8cbec27ca0e71df6bb2"
        );
        assert_eq!(
            header(NOW + 60),
            "Zama-EIP712 expires=1790950060, signature=0x1abede313062938323cae28925ae88625946020b7a6926e1d5a6e1bf219f59d37ba4926d26af15b5692a45fbb7f2753959409658a795c7a4c984c58161c173fd1b"
        );
    }
}
