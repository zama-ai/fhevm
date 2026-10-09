//! Client-side signing.

use crate::{FhevmSigHeader, signing_hash, unix_now};
use alloy::{primitives::Address, signers::Signer, sol_types::Eip712Domain};
use std::time::Duration;
use thiserror::Error;

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

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    #[tokio::test]
    async fn signing_refuses_query_strings() {
        let (signer, domain, audience) = setup();
        for target in ["/v1/x?a=1", "/v1/x?"] {
            let query = sign(&signer, &domain, audience, target, BODY, NOW).await;
            assert!(matches!(query, Err(SignError::QueryString)), "{target}");
        }
    }
}
