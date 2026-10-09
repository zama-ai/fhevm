//! Server-side verification.

use crate::{
    DEFAULT_CLOCK_SKEW, DEFAULT_MAX_AUTH_VALIDITY, FhevmSigHeader, signing_hash, unix_now,
};
use alloy::{primitives::Address, sol_types::Eip712Domain};
use std::{
    collections::HashSet,
    sync::{Arc, PoisonError, RwLock},
    time::Duration,
};
use thiserror::Error;

/// Why a request failed authentication..
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

/// Which signers an endpoint accepts in [`FhevmSigVerifier::verify`].
pub trait SignerAllowlist {
    fn allows(&self, signer: Address) -> bool;
}

/// A fixed set of signers.
impl SignerAllowlist for HashSet<Address> {
    fn allows(&self, signer: Address) -> bool {
        self.contains(&signer)
    }
}

/// A set of signers refreshed at runtime, e.g. from on-chain state.
#[derive(Clone, Debug, Default)]
pub struct RefreshableSigners(Arc<RwLock<HashSet<Address>>>);

impl RefreshableSigners {
    pub fn new(signers: impl IntoIterator<Item = Address>) -> Self {
        Self(Arc::new(RwLock::new(signers.into_iter().collect())))
    }

    /// Swaps the whole set at once.
    pub fn replace(&self, signers: impl IntoIterator<Item = Address>) {
        let signers = signers.into_iter().collect();
        // A panic while holding the lock can't leave the set half-written, so poisoning is ignored.
        *self.0.write().unwrap_or_else(PoisonError::into_inner) = signers;
    }
}

impl SignerAllowlist for RefreshableSigners {
    fn allows(&self, signer: Address) -> bool {
        self.0
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .contains(&signer)
    }
}

/// Server-side verification for one recipient.
#[derive(Clone, Debug)]
pub struct FhevmSigVerifier<P = HashSet<Address>> {
    domain: Eip712Domain,
    audience: Address,
    allowlist: P,
    max_validity: Duration,
    clock_skew: Duration,
}

impl<P: SignerAllowlist> FhevmSigVerifier<P> {
    pub fn new(domain: Eip712Domain, audience: Address, allowlist: P) -> Self {
        Self {
            domain,
            audience,
            allowlist,
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

    /// The signers [`Self::verify`] accepts, e.g. to clone a [`RefreshableSigners`] for its refresh task.
    pub fn allowlist(&self) -> &P {
        &self.allowlist
    }

    /// Recovers the signer address from the signed request, without checking it is allowed.
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

    /// Returns the signer of the request if its [`SignerAllowlist`] allows it.
    pub fn verify(
        &self,
        authorization: Option<&[u8]>,
        path: &str,
        body: &[u8],
    ) -> Result<Address, AuthError> {
        let signer = self.recover(authorization, path, body, unix_now())?;
        if self.allowlist.allows(signer) {
            Ok(signer)
        } else {
            Err(AuthError::UnknownSigner(signer))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use std::collections::HashSet;

    #[tokio::test]
    async fn round_trip_recovers_the_signer() {
        let (signer, domain, audience) = setup();
        let header = fresh_header(&signer, &domain, audience).await;
        let allowed = HashSet::from([signer.address()]);
        let verifier = FhevmSigVerifier::new(domain, audience, allowed);
        assert_eq!(
            verifier.verify(Some(header.as_bytes()), PATH, BODY),
            Ok(signer.address())
        );
    }

    #[tokio::test]
    async fn unknown_signer_reports_the_recovered_address() {
        let (signer, domain, audience) = setup();
        let header = fresh_header(&signer, &domain, audience).await;
        let others = HashSet::from([Address::repeat_byte(0x01)]);
        let verifier = FhevmSigVerifier::new(domain, audience, others);
        assert_eq!(
            verifier.verify(Some(header.as_bytes()), PATH, BODY),
            Err(AuthError::UnknownSigner(signer.address()))
        );
    }

    #[tokio::test]
    async fn refreshing_the_signers_applies_to_the_verifier() {
        let (signer, domain, audience) = setup();
        let header = fresh_header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience, RefreshableSigners::default());
        let verify = || verifier.verify(Some(header.as_bytes()), PATH, BODY);
        assert_eq!(verify(), Err(AuthError::UnknownSigner(signer.address())));

        // A clone, as held by a refresh task, updates the verifier's set.
        let refresher = verifier.allowlist().clone();
        refresher.replace([signer.address()]);
        assert_eq!(verify(), Ok(signer.address()));
        refresher.replace([]);
        assert_eq!(verify(), Err(AuthError::UnknownSigner(signer.address())));
    }

    #[test]
    fn missing_header_is_rejected() {
        let (_, domain, audience) = setup();
        let verifier = FhevmSigVerifier::new(domain, audience, HashSet::new());
        assert_eq!(
            verifier.recover(None, PATH, BODY, NOW),
            Err(AuthError::Missing)
        );
    }

    #[tokio::test]
    async fn expired_header_is_rejected() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience, HashSet::new());
        let late = NOW + 60 + DEFAULT_CLOCK_SKEW.as_secs() + 1;
        let err = verifier
            .recover(Some(header.as_bytes()), PATH, BODY, late)
            .unwrap_err();
        assert_eq!(err, AuthError::Expired);
    }

    #[tokio::test]
    async fn too_long_lived_header_is_rejected() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain.clone(), audience, HashSet::new());
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
        let strict = FhevmSigVerifier::new(domain, audience, HashSet::new())
            .with_max_validity(Duration::from_secs(10))
            .with_clock_skew(Duration::ZERO);
        assert_eq!(
            strict.recover(Some(header.as_bytes()), PATH, BODY, NOW),
            Err(AuthError::TooLongLived)
        );
    }

    #[tokio::test]
    async fn verifying_rejects_a_query_string() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let verifier = FhevmSigVerifier::new(domain, audience, HashSet::new());
        for target in ["/v1/user-decrypt?a=1", "/v1/user-decrypt?"] {
            assert_eq!(
                verifier.recover(Some(header.as_bytes()), target, BODY, NOW),
                Err(AuthError::QueryString),
                "{target}"
            );
        }
    }

    #[tokio::test]
    async fn tampering_recovers_another_address() {
        let (signer, domain, audience) = setup();
        let header = header(&signer, &domain, audience).await;
        let recover = |domain: &Eip712Domain, audience, path, body| {
            FhevmSigVerifier::new(domain.clone(), audience, HashSet::new())
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
        let other_chain = crate::domain(2, Address::repeat_byte(0xc0));
        assert_ne!(
            recover(&other_chain, audience, PATH, BODY),
            signer.address()
        );
        let other_contract = crate::domain(1, Address::repeat_byte(0xc1));
        assert_ne!(
            recover(&other_contract, audience, PATH, BODY),
            signer.address()
        );
    }
}
