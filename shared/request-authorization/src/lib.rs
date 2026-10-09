//! `FhevmSig`: an HTTP request signed by an Ethereum key for one recipient, as RFC 038 specifies
//! its request authentication.
//!
//! The caller signs the EIP-712 hash of `Request { path, bodyDigest, expires, audience }` and sends
//! the signature in the `Authorization` header:
//!
//! ```text
//! Authorization: FhevmSig expires=1790950060, sig=0x<65 bytes: r, s, v>
//! ```
//!
//! `path` starts at the version segment, as sent on the wire. A query string is not signed, so a
//! route that accepts this scheme refuses a request that carries one. `bodyDigest` is the
//! keccak-256 of the exact body bytes, so a server checks what it received without re-encoding it.
//! `audience` is the recipient's address: the server rebuilds the request with its own, so a
//! header signed for another recipient recovers an unrelated address. The server recovers the
//! signer and decides whether that address may call.

use alloy_primitives::{Address, B256, Signature, hex, keccak256};
use alloy_signer::Signer;
use alloy_sol_types::{Eip712Domain, SolStruct, eip712_domain, sol};

sol! {
    /// The signed fields of a request.
    struct Request {
        string path;
        bytes32 bodyDigest;
        uint64 expires;
        address audience;
    }
}

/// The `Authorization` scheme.
pub const SCHEME: &str = "FhevmSig";

/// The longest validity a server accepts, in seconds: `MAX_AUTH_VALIDITY`.
pub const MAX_AUTH_VALIDITY_SECS: u64 = 300;

/// The clock skew a server tolerates on both bounds of `expires`, in seconds: `CLOCK_SKEW`.
pub const CLOCK_SKEW_SECS: u64 = 30;

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
            name: "fhevm-http-auth",
            version: "1",
            chain_id: self.chain_id,
            verifying_contract: self.contract,
        }
    }

    /// The hash a caller signs for `body` sent to `path` at `audience`, valid until `expires`
    /// (Unix seconds).
    fn signing_hash(&self, path: &str, body: &[u8], expires: u64, audience: Address) -> B256 {
        Request {
            path: path.to_owned(),
            bodyDigest: keccak256(body),
            expires,
            audience,
        }
        .eip712_signing_hash(&self.domain())
    }
}

/// Signs `body` sent to `path` at `audience`, valid until `expires` (Unix seconds), and returns the
/// `Authorization` header value.
pub async fn authorize<S: Signer + ?Sized>(
    signer: &S,
    registry: &KeyRegistry,
    path: &str,
    body: &[u8],
    expires: u64,
    audience: Address,
) -> alloy_signer::Result<String> {
    let signature = signer
        .sign_hash(&registry.signing_hash(path, body, expires, audience))
        .await?;
    Ok(header_value(expires, &signature))
}

fn header_value(expires: u64, signature: &Signature) -> String {
    format!(
        "{SCHEME} expires={expires}, sig=0x{}",
        hex::encode(signature.as_bytes())
    )
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthorizationError {
    #[error("missing or malformed {SCHEME} authorization")]
    Malformed,
    #[error("the authorization expired at {expires}, more than {CLOCK_SKEW_SECS} s before {now}")]
    Expired { expires: u64, now: u64 },
    #[error(
        "the authorization expires at {expires}, more than {MAX_AUTH_VALIDITY_SECS} s and the \
         {CLOCK_SKEW_SECS} s skew after {now}"
    )]
    TooLongLived { expires: u64, now: u64 },
    #[error("the signature recovers no signer")]
    BadSignature,
}

/// An `Authorization` header within its validity, and who signed it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Authorization {
    pub signer: Address,
    /// What the signer signed: one hash per registry, path, body, expiry and audience, whichever
    /// of a signature's two encodings carried it.
    pub signing_hash: B256,
    /// Unix seconds.
    pub expires: u64,
}

impl Authorization {
    /// The last Unix second a server accepts this authorization, skew included.
    pub fn accepted_until(&self) -> u64 {
        self.expires.saturating_add(CLOCK_SKEW_SECS)
    }
}

/// Recovers who signed the `Authorization` header `header` for `body` sent to `path` at
/// `audience`, the server's own address, at Unix time `now`. A signature over other fields
/// recovers another address, so the caller must check the signer against the addresses it accepts.
pub fn recover_authorization(
    registry: &KeyRegistry,
    header: &str,
    path: &str,
    body: &[u8],
    audience: Address,
    now: u64,
) -> Result<Authorization, AuthorizationError> {
    let (expires, signature) = parse(header)?;
    if expires.saturating_add(CLOCK_SKEW_SECS) < now {
        return Err(AuthorizationError::Expired { expires, now });
    }
    if expires > now.saturating_add(MAX_AUTH_VALIDITY_SECS + CLOCK_SKEW_SECS) {
        return Err(AuthorizationError::TooLongLived { expires, now });
    }
    let signing_hash = registry.signing_hash(path, body, expires, audience);
    let signer = signature
        .recover_address_from_prehash(&signing_hash)
        .map_err(|_| AuthorizationError::BadSignature)?;
    Ok(Authorization {
        signer,
        signing_hash,
        expires,
    })
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
        .strip_prefix("sig=0x")
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
    const PATH: &str = "/v1/example";
    const BODY: &[u8] = b"{\"leaves\":[]}";
    const AUDIENCE: Address = Address::repeat_byte(0xA0);
    const NOW: u64 = 1_790_950_000;

    fn signer() -> PrivateKeySigner {
        "0x3f45b129a7fd099146e9fe63851a71646231f7743c712695f3b2d2bf0e41c774"
            .parse()
            .unwrap()
    }

    fn header(expires: u64) -> String {
        let signature = signer()
            .sign_hash_sync(&REGISTRY.signing_hash(PATH, BODY, expires, AUDIENCE))
            .unwrap();
        header_value(expires, &signature)
    }

    fn recover(header: &str) -> Result<Authorization, AuthorizationError> {
        recover_authorization(&REGISTRY, header, PATH, BODY, AUDIENCE, NOW)
    }

    #[test]
    fn the_signer_is_recovered_within_the_validity() {
        let address = signer().address();
        for expires in [
            NOW - CLOCK_SKEW_SECS,
            NOW,
            NOW + 60,
            NOW + MAX_AUTH_VALIDITY_SECS + CLOCK_SKEW_SECS,
        ] {
            assert_eq!(
                recover(&header(expires)).map(|authorization| authorization.signer),
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
        for (registry, path, body, audience) in [
            (other_chain, PATH, BODY, AUDIENCE),
            (other_contract, PATH, BODY, AUDIENCE),
            (REGISTRY, "/v1/other", BODY, AUDIENCE),
            (REGISTRY, PATH, b"{\"leaves\":[1]}".as_slice(), AUDIENCE),
            (REGISTRY, PATH, BODY, Address::repeat_byte(0xA1)),
        ] {
            assert_ne!(
                recover_authorization(&registry, &header, path, body, audience, NOW)
                    .map(|authorization| authorization.signer),
                Ok(address)
            );
        }
    }

    #[test]
    fn an_expired_or_too_long_lived_authorization_is_refused() {
        let expires = NOW - CLOCK_SKEW_SECS - 1;
        assert_eq!(
            recover(&header(expires)),
            Err(AuthorizationError::Expired { expires, now: NOW })
        );
        let expires = NOW + MAX_AUTH_VALIDITY_SECS + CLOCK_SKEW_SECS + 1;
        assert_eq!(
            recover(&header(expires)),
            Err(AuthorizationError::TooLongLived { expires, now: NOW })
        );
    }

    #[test]
    fn a_malformed_header_is_refused() {
        let valid = header(NOW + 60);
        let (_, signature) = valid.split_once("sig=0x").unwrap();
        for malformed in [
            String::new(),
            "Bearer secret".to_owned(),
            valid.replacen(SCHEME, "fhevmsig", 1),
            valid.replacen(", ", ",", 1),
            valid.replacen("expires=", "expires=+", 1),
            valid.replacen("sig=", "signature=", 1),
            format!("{SCHEME} expires=, sig=0x{signature}"),
            format!("{SCHEME} expires={}, sig=0x{}", NOW + 60, &signature[2..]),
            format!("{SCHEME} expires={}, sig={signature}", NOW + 60),
        ] {
            assert_eq!(
                recover(&malformed),
                Err(AuthorizationError::Malformed),
                "{malformed}"
            );
        }
    }

    #[tokio::test]
    async fn authorize_signs_what_recover_authorization_checks() {
        let header = authorize(&signer(), &REGISTRY, PATH, BODY, NOW + 60, AUDIENCE)
            .await
            .unwrap();
        let authorization = recover(&header).unwrap();
        assert_eq!(
            authorization,
            Authorization {
                signer: signer().address(),
                signing_hash: REGISTRY.signing_hash(PATH, BODY, NOW + 60, AUDIENCE),
                expires: NOW + 60,
            }
        );
        assert_eq!(authorization.accepted_until(), NOW + 60 + CLOCK_SKEW_SECS);
    }

    /// The other encoding of one ECDSA signature, `s` mirrored to `n - s` with the parity flipped,
    /// recovers the same signer over the same hash, so a server keying repeats by the hash sees
    /// it as the same request.
    #[test]
    fn both_encodings_of_a_signature_name_one_signing_hash() {
        let hash = REGISTRY.signing_hash(PATH, BODY, NOW + 60, AUDIENCE);
        let signature = signer().sign_hash_sync(&hash).unwrap();
        let mirrored = Signature::new(
            signature.r(),
            alloy_primitives::U256::from_be_bytes(SECP256K1N) - signature.s(),
            !signature.v(),
        );
        let first = recover(&header_value(NOW + 60, &signature));
        let second = recover(&header_value(NOW + 60, &mirrored));
        assert_eq!(first, second);
        assert_eq!(first.unwrap().signer, signer().address());
    }

    /// The secp256k1 group order.
    const SECP256K1N: [u8; 32] =
        alloy_primitives::hex!("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");

    /// Pins the wire format to values computed with viem 2 alone, independently of this crate:
    ///
    /// ```js
    /// const typed = {
    ///   domain: { name: "fhevm-http-auth", version: "1", chainId: 12345,
    ///             verifyingContract: "0xc0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0" },
    ///   types: { Request: [{ name: "path", type: "string" }, { name: "bodyDigest", type: "bytes32" },
    ///                      { name: "expires", type: "uint64" }, { name: "audience", type: "address" }] },
    ///   primaryType: "Request",
    ///   message: { path: "/v1/example", bodyDigest: keccak256(toBytes('{"leaves":[]}')),
    ///              expires: 1790950060n, audience: "0xa0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0" },
    /// };
    /// hashTypedData(typed);
    /// privateKeyToAccount("0x3f45…c774").signTypedData(typed);
    /// ```
    #[test]
    fn the_wire_format_matches_viem() {
        assert_eq!(
            signer().address(),
            "0x31De9c8ac5ECD5EacEddDdEE531e9BaD8AC9c2A5"
                .parse::<Address>()
                .unwrap()
        );
        assert_eq!(
            REGISTRY
                .signing_hash(PATH, BODY, NOW + 60, AUDIENCE)
                .to_string(),
            "0xb7cd940895f56b35d0ccc114e375919c7b734805663a44209413fa906b160e4a"
        );
        assert_eq!(
            header(NOW + 60),
            "FhevmSig expires=1790950060, sig=0x221e0fdb37848dea9211b3ec72a1ece532dcc4b519099098b3e2bf938dfb214146a40971c7fdb3174f85b5abaa844eb080769b18d7a5cc620d6cd71c6954da4b1c"
        );
    }
}
