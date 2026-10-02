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
use alloy_signer::Signer;
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

/// The contract that registers the callers' keys, and its chain: the EIP-712 domain's
/// `verifyingContract` and `chainId`. A signature is valid for this registry only, so two networks
/// on one chain do not accept each other's requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyRegistry {
    pub chain_id: u64,
    pub contract: Address,
}

impl KeyRegistry {
    fn domain(&self) -> Eip712Domain {
        eip712_domain! {
            name: "zama-request-authorization",
            version: "1",
            chain_id: self.chain_id,
            verifying_contract: self.contract,
        }
    }

    /// The hash a caller signs for `body` sent to `path`, valid until `expires` (Unix seconds).
    fn signing_hash(&self, path: &str, body: &[u8], expires: u64) -> B256 {
        RequestAuthorization {
            path: path.to_owned(),
            bodyDigest: keccak256(body),
            expires,
        }
        .eip712_signing_hash(&self.domain())
    }
}

/// Signs `body` sent to `path`, valid until `expires` (Unix seconds), and returns the
/// `Authorization` header value.
pub async fn authorize<S: Signer + ?Sized>(
    signer: &S,
    registry: &KeyRegistry,
    path: &str,
    body: &[u8],
    expires: u64,
) -> alloy_signer::Result<String> {
    let signature = signer
        .sign_hash(&registry.signing_hash(path, body, expires))
        .await?;
    Ok(header_value(expires, &signature))
}

fn header_value(expires: u64, signature: &Signature) -> String {
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
    registry: &KeyRegistry,
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
        .recover_address_from_prehash(&registry.signing_hash(path, body, expires))
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
        .filter(|digits| digits.bytes().all(|b| b.is_ascii_digit()))
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

    const REGISTRY: KeyRegistry = KeyRegistry {
        chain_id: 12345,
        contract: Address::repeat_byte(0xC0),
    };
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
            .sign_hash_sync(&REGISTRY.signing_hash(PATH, BODY, expires))
            .unwrap();
        header_value(expires, &signature)
    }

    #[test]
    fn the_signer_is_recovered_within_the_validity() {
        let address = signer().address();
        for expires in [NOW, NOW + 60, NOW + MAX_VALIDITY_SECS] {
            assert_eq!(
                recover_signer(&REGISTRY, &header(expires), PATH, BODY, NOW),
                Ok(address)
            );
        }
    }

    #[test]
    fn other_fields_recover_another_signer() {
        let address = signer().address();
        let header = header(NOW + 60);
        let other_chain = KeyRegistry {
            chain_id: REGISTRY.chain_id + 1,
            ..REGISTRY
        };
        let other_contract = KeyRegistry {
            contract: Address::repeat_byte(0xC1),
            ..REGISTRY
        };
        for (registry, path, body) in [
            (other_chain, PATH, BODY),
            (other_contract, PATH, BODY),
            (REGISTRY, "/v1/other", BODY),
            (REGISTRY, PATH, b"{\"leaves\":[1]}".as_slice()),
        ] {
            assert_ne!(
                recover_signer(&registry, &header, path, body, NOW),
                Ok(address)
            );
        }
    }

    #[test]
    fn an_expired_or_far_future_authorization_is_refused() {
        assert_eq!(
            recover_signer(&REGISTRY, &header(NOW - 1), PATH, BODY, NOW),
            Err(AuthorizationError::Expired {
                expires: NOW - 1,
                now: NOW
            })
        );
        let expires = NOW + MAX_VALIDITY_SECS + 1;
        assert_eq!(
            recover_signer(&REGISTRY, &header(expires), PATH, BODY, NOW),
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
                recover_signer(&REGISTRY, &malformed, PATH, BODY, NOW),
                Err(AuthorizationError::Malformed),
                "{malformed}"
            );
        }
    }

    #[tokio::test]
    async fn authorize_signs_what_recover_signer_checks() {
        let header = authorize(&signer(), &REGISTRY, PATH, BODY, NOW + 60)
            .await
            .unwrap();
        assert_eq!(
            recover_signer(&REGISTRY, &header, PATH, BODY, NOW),
            Ok(signer().address())
        );
    }

    /// Pins the wire format. viem's `hashTypedData` and `signTypedData` produce the same values.
    #[test]
    fn the_wire_format_is_pinned() {
        assert_eq!(
            REGISTRY.signing_hash(PATH, BODY, NOW + 60).to_string(),
            "0x4e470f835159e2e36b00c1b761bf3ec15f71d13f87cf44ee9dea7bf0c4e7a02c"
        );
        assert_eq!(
            header(NOW + 60),
            "Zama-EIP712 expires=1790950060, signature=0x8d81390596a2072fb5f3028e0ed67aec83eaab655969be9422e1b5c371897a896a3abbdad8acb39bc0215c511c506a4f934700ab2cb2de7c73d853188311a2ef1b"
        );
    }
}
