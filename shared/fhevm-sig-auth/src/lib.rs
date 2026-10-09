//! `FhevmSig` HTTP request authentication.
//!
//! The sender signs an EIP-712 `Request { path, bodyDigest, expires, audience }` and sends
//! `Authorization: FhevmSig expires=<unix seconds>, sig=0x<65-byte signature>`. The server rebuilds
//! the struct from the request it actually received and its own address as `audience`, recovers
//! the signer with `ecrecover` and checks it against the endpoint's [`SignerAllowlist`].
//!
//! The crate is framework-agnostic.

mod header;
mod sign;
mod verify;

pub use header::FhevmSigHeader;
pub use sign::{SignError, sign, sign_valid_for};
pub use verify::{AuthError, FhevmSigVerifier, RefreshableSigners, SignerAllowlist};

use alloy::{
    primitives::{Address, B256, U256, keccak256},
    sol_types::{Eip712Domain, SolStruct},
};
use std::{
    borrow::Cow,
    time::{Duration, SystemTime},
};

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

/// Fixtures shared by the modules' tests.
#[cfg(test)]
mod test_utils {
    pub(crate) use super::*;
    pub(crate) use alloy::signers::local::PrivateKeySigner;

    pub(crate) const NOW: u64 = 1_800_000_000;
    pub(crate) const PATH: &str = "/v1/user-decrypt";
    pub(crate) const BODY: &[u8] = br#"{"payload":1}"#;

    pub(crate) fn setup() -> (PrivateKeySigner, Eip712Domain, Address) {
        let domain = domain(1, Address::repeat_byte(0xc0));
        (
            PrivateKeySigner::random(),
            domain,
            Address::repeat_byte(0xa1),
        )
    }

    /// A header expiring at `NOW + 60`.
    pub(crate) async fn header(
        signer: &PrivateKeySigner,
        domain: &Eip712Domain,
        audience: Address,
    ) -> String {
        sign(signer, domain, audience, PATH, BODY, NOW + 60)
            .await
            .unwrap()
            .to_string()
    }

    /// A header valid for 60 s from the real clock, for [`FhevmSigVerifier::verify`].
    pub(crate) async fn fresh_header(
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
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use alloy::primitives::b256;

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
}
