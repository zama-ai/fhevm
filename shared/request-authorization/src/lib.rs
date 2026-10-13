//! `FhevmSig`: an HTTP request signed by an Ethereum key for one recipient, as RFC 038 specifies
//! its request authentication.
//!
//! The caller signs the EIP-712 hash of `Request { path, bodyDigest, expires, audience }` and sends
//! the signature in the `Authorization` header, exactly these three parameters in this order:
//!
//! ```text
//! Authorization: FhevmSig expires=1790950060, digest=0x<32 bytes>, sig=0x<65 bytes: r, s, v>
//! ```
//!
//! `path` starts at the version segment, as sent on the wire. A query string is not signed, so a
//! route that accepts this scheme refuses a request that carries one. `bodyDigest` is the
//! keccak-256 of the exact body bytes, `keccak256("")` for an empty body, so a server checks what
//! it received without re-encoding it. The header carries it as `digest`, so a server recovers the
//! signer before it reads the body, then checks the body against the digest
//! ([`Authorization::check_body`]).
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

    /// The hash a caller signs for a body of keccak-256 `body_digest` sent to `path` at
    /// `audience`, valid until `expires` (Unix seconds).
    fn signing_hash(&self, path: &str, body_digest: B256, expires: u64, audience: Address) -> B256 {
        Request {
            path: path.to_owned(),
            bodyDigest: body_digest,
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
    let body_digest = keccak256(body);
    let signature = signer
        .sign_hash(&registry.signing_hash(path, body_digest, expires, audience))
        .await?;
    Ok(header_value(expires, body_digest, &signature))
}

fn header_value(expires: u64, body_digest: B256, signature: &Signature) -> String {
    format!(
        "{SCHEME} expires={expires}, digest={body_digest}, sig=0x{}",
        hex::encode(signature.as_bytes())
    )
}

/// Why a server refuses a request, one variant per verification step.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthorizationError {
    #[error("not a {SCHEME} authorization of exactly `expires`, `digest` and `sig`, in this order")]
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

/// A body other than the one signed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("the body's keccak-256 {received} is not the signed digest {signed}")]
pub struct BodyMismatch {
    pub signed: B256,
    pub received: B256,
}

/// An `Authorization` header within its validity, and who signed it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Authorization {
    pub signer: Address,
    /// What the signer signed: one hash per registry, path, body, expiry and audience, whichever
    /// of a signature's two encodings carried it.
    pub signing_hash: B256,
    /// The keccak-256 of the body the signer signed.
    pub body_digest: B256,
    /// Unix seconds.
    pub expires: u64,
}

impl Authorization {
    /// The last Unix second a server accepts this authorization, skew included.
    pub fn accepted_until(&self) -> u64 {
        self.expires.saturating_add(CLOCK_SKEW_SECS)
    }

    /// Refuses a body other than the one signed: a valid header replayed with another body
    /// within its validity.
    pub fn check_body(&self, body: &[u8]) -> Result<(), BodyMismatch> {
        let received = keccak256(body);
        if received == self.body_digest {
            Ok(())
        } else {
            Err(BodyMismatch {
                signed: self.body_digest,
                received,
            })
        }
    }
}

/// Recovers who signed the `Authorization` header `header` for `path` at `audience`, the server's
/// own address, at Unix time `now`, from the header alone, before the body is read. A signature
/// over other fields recovers another address, so the caller checks the signer against the
/// addresses it accepts, then the body with [`Authorization::check_body`].
pub fn recover_authorization(
    registry: &KeyRegistry,
    header: &str,
    path: &str,
    audience: Address,
    now: u64,
) -> Result<Authorization, AuthorizationError> {
    let (expires, body_digest, signature) = parse(header)?;
    if expires.saturating_add(CLOCK_SKEW_SECS) < now {
        return Err(AuthorizationError::Expired { expires, now });
    }
    if expires > now.saturating_add(MAX_AUTH_VALIDITY_SECS + CLOCK_SKEW_SECS) {
        return Err(AuthorizationError::TooLongLived { expires, now });
    }
    let signing_hash = registry.signing_hash(path, body_digest, expires, audience);
    let signer = signature
        .recover_address_from_prehash(&signing_hash)
        .map_err(|_| AuthorizationError::BadSignature)?;
    Ok(Authorization {
        signer,
        signing_hash,
        body_digest,
        expires,
    })
}

fn parse(header: &str) -> Result<(u64, B256, Signature), AuthorizationError> {
    let malformed = || AuthorizationError::Malformed;
    let mut parameters = header
        .strip_prefix(SCHEME)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or_else(malformed)?
        .split(", ");
    let mut parameter = |name: &str| {
        parameters
            .next()
            .and_then(|parameter| parameter.strip_prefix(name))
            .ok_or_else(malformed)
    };
    let expires = Some(parameter("expires=")?)
        .filter(|digits| digits.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|digits| digits.parse().ok())
        .ok_or_else(malformed)?;
    // `hex::decode` also strips a second `0x`.
    let hex_bytes = |digits: &str| {
        Some(digits)
            .filter(|digits| digits.bytes().all(|b| b.is_ascii_hexdigit()))
            .and_then(|digits| hex::decode(digits).ok())
    };
    let body_digest = hex_bytes(parameter("digest=0x")?)
        .and_then(|bytes| B256::try_from(bytes.as_slice()).ok())
        .ok_or_else(malformed)?;
    let signature = hex_bytes(parameter("sig=0x")?)
        .and_then(|bytes| Signature::try_from(bytes.as_slice()).ok())
        .ok_or_else(malformed)?;
    if parameters.next().is_some() {
        return Err(malformed());
    }
    Ok((expires, body_digest, signature))
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
        let digest = keccak256(BODY);
        let signature = signer()
            .sign_hash_sync(&REGISTRY.signing_hash(PATH, digest, expires, AUDIENCE))
            .unwrap();
        header_value(expires, digest, &signature)
    }

    fn recover(header: &str) -> Result<Authorization, AuthorizationError> {
        recover_authorization(&REGISTRY, header, PATH, AUDIENCE, NOW)
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
        for (registry, path, audience) in [
            (other_chain, PATH, AUDIENCE),
            (other_contract, PATH, AUDIENCE),
            (REGISTRY, "/v1/other", AUDIENCE),
            (REGISTRY, PATH, Address::repeat_byte(0xA1)),
        ] {
            assert_ne!(
                recover_authorization(&registry, &header, path, audience, NOW)
                    .map(|authorization| authorization.signer),
                Ok(address)
            );
        }
    }

    /// The server recovers the signer from the header's digest, so a digest changed on the way
    /// recovers another address, and the signer is known before the body is read.
    #[test]
    fn another_digest_in_the_header_recovers_another_signer() {
        let header = header(NOW + 60);
        let other_digest = keccak256(b"{\"leaves\":[1]}");
        let edited = header.replacen(&keccak256(BODY).to_string(), &other_digest.to_string(), 1);
        assert_ne!(edited, header);
        assert_ne!(
            recover(&edited).map(|authorization| authorization.signer),
            Ok(signer().address())
        );
    }

    /// A valid header with another body recovers its signer, and the body check refuses the body.
    #[test]
    fn only_the_signed_body_passes_the_body_check() {
        let authorization = recover(&header(NOW + 60)).unwrap();
        assert_eq!(authorization.signer, signer().address());
        assert_eq!(authorization.check_body(BODY), Ok(()));
        let other = b"{\"leaves\":[1]}";
        assert_eq!(
            authorization.check_body(other),
            Err(BodyMismatch {
                signed: keccak256(BODY),
                received: keccak256(other),
            })
        );
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

    /// Exactly `expires`, `digest` and `sig`, in this order: a header of the earlier two-parameter
    /// form, a reordered one, or one with a parameter more is refused.
    #[test]
    fn a_malformed_header_is_refused() {
        let valid = header(NOW + 60);
        let expires = NOW + 60;
        let digest = keccak256(BODY).to_string();
        let digest = digest.strip_prefix("0x").unwrap();
        let (_, signature) = valid.split_once("sig=0x").unwrap();
        for malformed in [
            String::new(),
            "Bearer secret".to_owned(),
            valid.replacen(SCHEME, "fhevmsig", 1),
            valid.replacen(", ", ",", 1),
            valid.replacen("expires=", "expires=+", 1),
            valid.replacen("sig=", "signature=", 1),
            valid.replacen("digest=", "bodyDigest=", 1),
            format!("{valid}, extra=1"),
            format!("{SCHEME} expires={expires}, sig=0x{signature}"),
            format!("{SCHEME} digest=0x{digest}, expires={expires}, sig=0x{signature}"),
            format!("{SCHEME} expires={expires}, sig=0x{signature}, digest=0x{digest}"),
            format!("{SCHEME} expires=, digest=0x{digest}, sig=0x{signature}"),
            format!(
                "{SCHEME} expires={expires}, digest=0x{}, sig=0x{signature}",
                &digest[2..]
            ),
            format!("{SCHEME} expires={expires}, digest=0x{digest}00, sig=0x{signature}"),
            format!("{SCHEME} expires={expires}, digest={digest}, sig=0x{signature}"),
            format!(
                "{SCHEME} expires={expires}, digest=0x{digest}, sig=0x{}",
                &signature[2..]
            ),
            format!("{SCHEME} expires={expires}, digest=0x{digest}, sig={signature}"),
            valid.replacen("digest=0x", "digest=0x0x", 1),
            valid.replacen("sig=0x", "sig=0x0x", 1),
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
                signing_hash: REGISTRY.signing_hash(PATH, keccak256(BODY), NOW + 60, AUDIENCE),
                body_digest: keccak256(BODY),
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
        let digest = keccak256(BODY);
        let hash = REGISTRY.signing_hash(PATH, digest, NOW + 60, AUDIENCE);
        let signature = signer().sign_hash_sync(&hash).unwrap();
        let mirrored = Signature::new(
            signature.r(),
            alloy_primitives::U256::from_be_bytes(SECP256K1N) - signature.s(),
            !signature.v(),
        );
        let first = recover(&header_value(NOW + 60, digest, &signature));
        let second = recover(&header_value(NOW + 60, digest, &mirrored));
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
    /// const sig = await privateKeyToAccount("0x3f45…c774").signTypedData(typed);
    /// `FhevmSig expires=${typed.message.expires}, digest=${typed.message.bodyDigest}, sig=${sig}`;
    /// // A request without a body signs keccak256 of no bytes.
    /// const empty = { ...typed, message: { ...typed.message, bodyDigest: keccak256(new Uint8Array()) } };
    /// ```
    #[tokio::test]
    async fn the_wire_format_matches_viem() {
        assert_eq!(
            signer().address(),
            "0x31De9c8ac5ECD5EacEddDdEE531e9BaD8AC9c2A5"
                .parse::<Address>()
                .unwrap()
        );
        let header_for = |body: &'static [u8]| async move {
            authorize(&signer(), &REGISTRY, PATH, body, NOW + 60, AUDIENCE)
                .await
                .unwrap()
        };
        assert_eq!(
            recover(&header_for(BODY).await)
                .unwrap()
                .signing_hash
                .to_string(),
            "0xb7cd940895f56b35d0ccc114e375919c7b734805663a44209413fa906b160e4a"
        );
        assert_eq!(
            header_for(BODY).await,
            "FhevmSig expires=1790950060, \
             digest=0xa29ece6ee767951fdf51b1975b836dd42daaa6b5acb1462ba02bf549965c5ac8, \
             sig=0x221e0fdb37848dea9211b3ec72a1ece532dcc4b519099098b3e2bf938dfb214146a40971c7fdb3174f85b5abaa844eb080769b18d7a5cc620d6cd71c6954da4b1c"
        );
        assert_eq!(
            recover(&header_for(b"").await)
                .unwrap()
                .signing_hash
                .to_string(),
            "0x7fdba7612e3c0568d59b1274bebb86b6040758b926ce94f52c74d6d33c4b2a2a"
        );
        assert_eq!(
            header_for(b"").await,
            "FhevmSig expires=1790950060, \
             digest=0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470, \
             sig=0x762f9d2eb58a04a7ff1388c95f44b6b6256e67ae3b2c9da490bb9ea79b56b9e7701a51c190c28d56b436ed29965a644bb475b54f1967a582119725dbd05d33361b"
        );
    }
}
