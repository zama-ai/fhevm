//! `FhevmSig` HTTP request authentication.
//!
//! The sender signs an EIP-712 `Request { path, bodyDigest, expires, audience }` and sends
//! `Authorization: FhevmSig expires=<unix seconds>, sig=0x<65-byte signature>`. The server rebuilds
//! the struct from the request it actually received and its own address as `audience`, recovers
//! the signer with `ecrecover` and checks it against the link's whitelist.
//!
//! The crate is framework-agnostic.

use alloy::{
    primitives::{Address, B256, Signature, U256, hex, keccak256},
    signers::Signer,
    sol_types::{Eip712Domain, SolStruct},
};
use std::{
    borrow::Cow,
    fmt,
    str::FromStr,
    time::{Duration, SystemTime},
};
use thiserror::Error;

// In a private module, so the generic `Request` name stays out of consumers' namespace.
mod eip712 {
    alloy::sol! {
        /// The signed message. Field names and order are part of the EIP-712 type hash.
        struct Request {
            string path;
            bytes32 bodyDigest;
            uint64 expires;
            address audience;
        }
    }
}
use eip712::Request;

/// The EIP-712 domain `name`.
pub const DOMAIN_NAME: &str = "fhevm-http-auth";
/// The EIP-712 domain `version`.
pub const DOMAIN_VERSION: &str = "1";
/// The `Authorization` scheme.
pub const SCHEME: &str = "FhevmSig";
/// The longest lifetime a server accepts for a header, unless set with [`FhevmSigVerifier::with_max_validity`].
pub const DEFAULT_MAX_AUTH_VALIDITY: Duration = Duration::from_secs(5 * 60);
/// The clock difference tolerated between sender and server, unless set with [`FhevmSigVerifier::with_clock_skew`].
pub const DEFAULT_CLOCK_SKEW: Duration = Duration::from_secs(30);

/// The current Unix time in seconds. A clock set before 1970 reads as `0`.
pub fn unix_now() -> u64 {
    SystemTime::UNIX_EPOCH
        .elapsed()
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

/// A parsed or freshly signed `Authorization: FhevmSig ...` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FhevmSigHeader {
    pub expires: u64,
    pub signature: Signature,
}

impl FhevmSigHeader {
    /// Parses a raw header value.
    pub fn parse(value: &[u8]) -> Result<Self, AuthError> {
        std::str::from_utf8(value)
            .map_err(|_| AuthError::Malformed("not UTF-8"))?
            .parse()
    }

    /// The header value, marked sensitive so it is never logged by `http`-based clients.
    #[cfg(feature = "http")]
    pub fn to_header_value(&self) -> http::HeaderValue {
        let mut value = http::HeaderValue::try_from(self.to_string())
            .expect("an FhevmSig header is always visible ASCII");
        value.set_sensitive(true);
        value
    }
}

// `FhevmSig expires=<decimal>, sig=0x<130 lowercase hex chars>`.
impl fmt::Display for FhevmSigHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{SCHEME} expires={}, sig=0x{}",
            self.expires,
            hex::encode(self.signature.as_bytes())
        )
    }
}

impl FromStr for FhevmSigHeader {
    type Err = AuthError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (scheme, params) = value
            .split_once(' ')
            .ok_or(AuthError::Malformed("missing parameters"))?;
        if !scheme.eq_ignore_ascii_case(SCHEME) {
            return Err(AuthError::Malformed("wrong scheme"));
        }
        let (expires, sig) = params
            .split_once(", ")
            .ok_or(AuthError::Malformed("expected `expires=..., sig=...`"))?;
        let expires = expires
            .strip_prefix("expires=")
            .ok_or(AuthError::Malformed("first parameter must be `expires=`"))?
            .parse()
            .map_err(|_| AuthError::Malformed("`expires` is not a u64"))?;
        let signature = sig
            .strip_prefix("sig=")
            .ok_or(AuthError::Malformed("second parameter must be `sig=`"))?
            .parse()
            .map_err(|e| AuthError::InvalidSignature(format!("`sig`: {e}")))?;
        Ok(Self { expires, signature })
    }
}

/// The EIP-712 domain of a deployment: the Ethereum chain id and the `ProtocolConfig` address.
pub fn domain(chain_id: u64, protocol_config: Address) -> Eip712Domain {
    Eip712Domain::new(
        Some(Cow::Borrowed(DOMAIN_NAME)),
        Some(Cow::Borrowed(DOMAIN_VERSION)),
        Some(U256::from(chain_id)),
        Some(protocol_config),
        None,
    )
}

/// The EIP-712 digest that is signed and recovered.
pub fn signing_hash(
    domain: &Eip712Domain,
    audience: Address,
    path: &str,
    body: &[u8],
    expires: u64,
) -> B256 {
    Request {
        path: path.to_owned(),
        bodyDigest: keccak256(body),
        expires,
        audience,
    }
    .eip712_signing_hash(domain)
}

/// Why signing failed.
#[derive(Debug, Error)]
pub enum SignError {
    #[error("path must not contain a query string")]
    QueryString,
    #[error("signer failed: {0}")]
    Signer(#[from] alloy::signers::Error),
}

/// Signs `path` and `body` for `audience`, valid until `expires` (Unix seconds).
pub async fn sign<S: Signer + Send + Sync + ?Sized>(
    signer: &S,
    domain: &Eip712Domain,
    audience: Address,
    path: &str,
    body: &[u8],
    expires: u64,
) -> Result<FhevmSigHeader, SignError> {
    if path.contains('?') {
        return Err(SignError::QueryString);
    }
    let hash = signing_hash(domain, audience, path, body, expires);
    let signature = signer.sign_hash(&hash).await?;
    Ok(FhevmSigHeader { expires, signature })
}

/// [`sign`] with `expires = now + validity`.
pub async fn sign_valid_for<S: Signer + Send + Sync + ?Sized>(
    signer: &S,
    domain: &Eip712Domain,
    audience: Address,
    path: &str,
    body: &[u8],
    validity: Duration,
) -> Result<FhevmSigHeader, SignError> {
    let expires = unix_now().saturating_add(validity.as_secs());
    sign(signer, domain, audience, path, body, expires).await
}

/// Why a request failed authentication. Every variant maps to a `401`.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("missing Authorization header")]
    Missing,
    #[error("malformed FhevmSig header: {0}")]
    Malformed(&'static str),
    #[error("path carries a query string")]
    QueryString,
    #[error("FhevmSig expired")]
    Expired,
    #[error("FhevmSig lives longer than accepted")]
    TooLongLived,
    #[error("invalid FhevmSig signature: {0}")]
    InvalidSignature(String),
    #[error("signer {0} is not allowed")]
    UnknownSigner(Address),
}

impl AuthError {
    /// The stable `auth_*` code naming the failing step.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Missing => "auth_missing",
            Self::Malformed(_) => "auth_malformed",
            Self::QueryString => "auth_query_string",
            Self::Expired => "auth_expired",
            Self::TooLongLived => "auth_too_long_lived",
            Self::InvalidSignature(_) => "auth_invalid_signature",
            Self::UnknownSigner(_) => "auth_unknown_signer",
        }
    }
}

/// Server-side verification for one recipient.
#[derive(Clone, Debug)]
pub struct FhevmSigVerifier {
    domain: Eip712Domain,
    audience: Address,
    max_validity: Duration,
    clock_skew: Duration,
}

impl FhevmSigVerifier {
    pub fn new(domain: Eip712Domain, audience: Address) -> Self {
        Self {
            domain,
            audience,
            max_validity: DEFAULT_MAX_AUTH_VALIDITY,
            clock_skew: DEFAULT_CLOCK_SKEW,
        }
    }

    pub fn with_max_validity(mut self, max_validity: Duration) -> Self {
        self.max_validity = max_validity;
        self
    }

    pub fn with_clock_skew(mut self, clock_skew: Duration) -> Self {
        self.clock_skew = clock_skew;
        self
    }

    /// Recovers the signer address from the signed request.
    pub fn recover(
        &self,
        auth_header: Option<&[u8]>,
        path: &str,
        body: &[u8],
        now: u64,
    ) -> Result<Address, AuthError> {
        let header = FhevmSigHeader::parse(auth_header.ok_or(AuthError::Missing)?)?;
        if path.contains('?') {
            return Err(AuthError::QueryString);
        }
        let skew = self.clock_skew.as_secs();
        if header.expires < now.saturating_sub(skew) {
            return Err(AuthError::Expired);
        }
        let latest = now
            .saturating_add(self.max_validity.as_secs())
            .saturating_add(skew);
        if header.expires > latest {
            return Err(AuthError::TooLongLived);
        }
        let hash = signing_hash(&self.domain, self.audience, path, body, header.expires);
        header
            .signature
            .recover_address_from_prehash(&hash)
            .map_err(|e| AuthError::InvalidSignature(e.to_string()))
    }

    /// Verifies the signer of the request is allowed by the endpoint.
    pub fn verify(
        &self,
        authorization: Option<&[u8]>,
        path: &str,
        body: &[u8],
        is_allowed: impl Fn(Address) -> bool,
    ) -> Result<Address, AuthError> {
        let signer = self.recover(authorization, path, body, unix_now())?;
        if is_allowed(signer) {
            Ok(signer)
        } else {
            Err(AuthError::UnknownSigner(signer))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{primitives::b256, signers::local::PrivateKeySigner};

    const NOW: u64 = 1_800_000_000;
    const PATH: &str = "/v1/user-decrypt";
    const BODY: &[u8] = br#"{"payload":1}"#;

    fn setup() -> (PrivateKeySigner, Eip712Domain, Address) {
        let domain = domain(1, Address::repeat_byte(0xc0));
        (
            PrivateKeySigner::random(),
            domain,
            Address::repeat_byte(0xa1),
        )
    }

    /// A header expiring at `NOW + 60`.
    async fn header(signer: &PrivateKeySigner, domain: &Eip712Domain, audience: Address) -> String {
        sign(signer, domain, audience, PATH, BODY, NOW + 60)
            .await
            .unwrap()
            .to_string()
    }

    /// A header valid for 60 s from the real clock, for [`FhevmSigVerifier::verify`].
    async fn fresh_header(
        signer: &PrivateKeySigner,
        domain: &Eip712Domain,
        audience: Address,
    ) -> String {
        sign_valid_for(
            signer,
            domain,
            audience,
            PATH,
            BODY,
            Duration::from_secs(60),
        )
        .await
        .unwrap()
        .to_string()
    }

    #[tokio::test]
    async fn round_trip_recovers_the_signer() {
        let (signer, domain, audience) = setup();
        let header = fresh_header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience);
        let allowed = |a| a == signer.address();
        assert_eq!(
            verifier.verify(Some(header.as_bytes()), PATH, BODY, allowed),
            Ok(signer.address())
        );
    }

    #[tokio::test]
    async fn display_and_parse_round_trip() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        assert!(header.starts_with("FhevmSig expires=1800000060, sig=0x"));
        assert_eq!(
            header.parse::<FhevmSigHeader>().unwrap().to_string(),
            header
        );
    }

    #[tokio::test]
    async fn malformed_headers_are_rejected() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let sig = header.split_once("sig=0x").unwrap().1;
        let cases = [
            (String::new(), "missing parameters"),
            ("FhevmSig".to_owned(), "missing parameters"),
            (format!("Bearer expires=1, sig=0x{sig}"), "wrong scheme"),
            (
                format!("FhevmSig sig=0x{sig}, expires=1"),
                "first parameter must be `expires=`",
            ),
            (
                "FhevmSig expires=1".to_owned(),
                "expected `expires=..., sig=...`",
            ),
            (
                format!("FhevmSig expires=1,sig=0x{sig}"),
                "expected `expires=..., sig=...`",
            ),
            (
                format!("FhevmSig expires=x, sig=0x{sig}"),
                "`expires` is not a u64",
            ),
            (
                format!("FhevmSig expires=-1, sig=0x{sig}"),
                "`expires` is not a u64",
            ),
            (
                format!("FhevmSig expires=18446744073709551616, sig=0x{sig}"),
                "`expires` is not a u64",
            ),
            (
                format!("FhevmSig expires=1, sign=0x{sig}"),
                "second parameter must be `sig=`",
            ),
        ];
        for (value, reason) in cases {
            assert_eq!(
                FhevmSigHeader::parse(value.as_bytes()),
                Err(AuthError::Malformed(reason)),
                "{value}"
            );
        }

        let short_sig = format!("FhevmSig expires=1, sig=0x{}", &sig[2..]);
        assert!(matches!(
            FhevmSigHeader::parse(short_sig.as_bytes()),
            Err(AuthError::InvalidSignature(_))
        ));
        assert_eq!(
            FhevmSigHeader::parse(b"FhevmSig expires=1, sig=0x\xff"),
            Err(AuthError::Malformed("not UTF-8"))
        );
        // The scheme is case-insensitive (RFC 9110).
        assert!(FhevmSigHeader::parse(header.to_lowercase().as_bytes()).is_ok());
    }

    /// Freezes the EIP-712 encoding: the `Request` type, the domain and `bodyDigest = keccak256(body)`.
    /// A change here breaks every deployed sender or verifier.
    #[test]
    fn signing_hash_is_stable() {
        let domain = domain(1, Address::repeat_byte(0xc0));
        let hash = signing_hash(&domain, Address::repeat_byte(0xa1), PATH, BODY, NOW + 60);
        assert_eq!(
            hash,
            b256!("0xa06d35ef211b66e99d7e1165552319faf87480e17354294b6428e0b13dc7fc7b")
        );
    }

    #[tokio::test]
    async fn unknown_signer_reports_the_recovered_address() {
        let (signer, domain, audience) = setup();
        let header = fresh_header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience);
        assert_eq!(
            verifier.verify(Some(header.as_bytes()), PATH, BODY, |_| false),
            Err(AuthError::UnknownSigner(signer.address()))
        );
    }

    #[tokio::test]
    async fn expired_header_is_rejected() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience);
        let late = NOW + 60 + DEFAULT_CLOCK_SKEW.as_secs() + 1;
        let err = verifier
            .recover(Some(header.as_bytes()), PATH, BODY, late)
            .unwrap_err();
        assert_eq!(err, AuthError::Expired);
    }

    #[tokio::test]
    async fn signing_refuses_query_strings() {
        let (signer, domain, audience) = setup();
        for target in ["/v1/x?a=1", "/v1/x?"] {
            let query = sign(&signer, &domain, audience, target, BODY, NOW).await;
            assert!(matches!(query, Err(SignError::QueryString)), "{target}");
        }
    }

    #[tokio::test]
    async fn verifying_rejects_a_query_string() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience);
        for target in ["/v1/user-decrypt?a=1", "/v1/user-decrypt?"] {
            assert_eq!(
                verifier.recover(Some(header.as_bytes()), target, BODY, NOW),
                Err(AuthError::QueryString),
                "{target}"
            );
        }
    }

    #[test]
    fn missing_header_is_rejected() {
        let (_, domain, audience) = setup();
        let verifier = FhevmSigVerifier::new(domain, audience);
        assert_eq!(
            verifier.recover(None, PATH, BODY, NOW),
            Err(AuthError::Missing)
        );
    }

    #[tokio::test]
    async fn too_long_lived_header_is_rejected() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain.clone(), audience);
        let max = DEFAULT_MAX_AUTH_VALIDITY.as_secs() + DEFAULT_CLOCK_SKEW.as_secs();
        // `expires = NOW + 60` is exactly at the limit, then one second past it.
        let at_limit = NOW + 60 - max;
        assert!(
            verifier
                .recover(Some(header.as_bytes()), PATH, BODY, at_limit)
                .is_ok()
        );
        assert_eq!(
            verifier.recover(Some(header.as_bytes()), PATH, BODY, at_limit - 1),
            Err(AuthError::TooLongLived)
        );

        // A lowered `max_validity` is enforced.
        let strict = FhevmSigVerifier::new(domain, audience)
            .with_max_validity(Duration::from_secs(10))
            .with_clock_skew(Duration::ZERO);
        assert_eq!(
            strict.recover(Some(header.as_bytes()), PATH, BODY, NOW),
            Err(AuthError::TooLongLived)
        );
    }

    #[tokio::test]
    async fn tampering_recovers_another_address() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let recover = |domain: &Eip712Domain, audience, path, body| {
            FhevmSigVerifier::new(domain.clone(), audience)
                .recover(Some(header.as_bytes()), path, body, NOW)
                .unwrap()
        };
        assert_eq!(recover(&domain, audience, PATH, BODY), signer.address());
        let other_audience = Address::repeat_byte(0xb2);
        assert_ne!(
            recover(&domain, other_audience, PATH, BODY),
            signer.address()
        );
        assert_ne!(
            recover(&domain, audience, "/v1/public-decrypt", BODY),
            signer.address()
        );
        assert_ne!(
            recover(&domain, audience, PATH, br#"{"payload":2}"#),
            signer.address()
        );
        let other_chain = super::domain(2, Address::repeat_byte(0xc0));
        assert_ne!(
            recover(&other_chain, audience, PATH, BODY),
            signer.address()
        );
        let other_contract = super::domain(1, Address::repeat_byte(0xc1));
        assert_ne!(
            recover(&other_contract, audience, PATH, BODY),
            signer.address()
        );
    }

    #[cfg(feature = "http")]
    #[tokio::test]
    async fn header_value_is_sensitive_and_parses_back() {
        let (signer, domain, audience) = setup();
        let header = sign(&signer, &domain, audience, PATH, BODY, NOW + 60)
            .await
            .unwrap();
        let value = header.to_header_value();
        assert!(value.is_sensitive());
        assert_eq!(FhevmSigHeader::parse(value.as_bytes()), Ok(header));
    }
}
