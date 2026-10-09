//! The `Authorization: FhevmSig ...` header value.

use crate::{AuthError, SCHEME};
use alloy::primitives::{Signature, hex};
use std::{fmt, str::FromStr};

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

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

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
