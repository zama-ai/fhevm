//! `FhevmSig` (RFC-038) signing of the connector calls: one header per node, for that node's audience.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::signers::Signer;
use alloy::signers::aws::AwsSigner;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::Eip712Domain;
use fhevm_sig_auth::PathAndQuery;
use reqwest::header::HeaderValue;

use super::config::{AuthConfig, ConfigError, SignerConfig};

/// Signs `Authorization: FhevmSig ...` headers with the relayer's key (local or AWS KMS).
pub struct RequestSigner {
    signer: Arc<dyn Signer + Send + Sync>,
    domain: Eip712Domain,
    validity: Duration,
}

impl RequestSigner {
    /// Loads the key: from the env var named in the config, or from AWS KMS. The key is never logged.
    pub async fn from_config(cfg: &AuthConfig) -> Result<Self, ConfigError> {
        let signer: Arc<dyn Signer + Send + Sync> = match &cfg.signer {
            SignerConfig::PrivateKey { value_env } => {
                let key = std::env::var(value_env).map_err(|_| {
                    ConfigError(format!("auth.signer: env var {value_env} is not set"))
                })?;
                let signer: PrivateKeySigner = key.trim().parse().map_err(|_| {
                    ConfigError(format!(
                        "auth.signer: {value_env} does not hold a valid private key"
                    ))
                })?;
                Arc::new(signer)
            }
            SignerConfig::AwsKms {
                key_id,
                region,
                endpoint,
            } => {
                let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
                    .region(aws_config::Region::new(region.clone()));
                if let Some(endpoint) = endpoint {
                    loader = loader.endpoint_url(endpoint);
                }
                let client = aws_sdk_kms::Client::new(&loader.load().await);
                let signer = AwsSigner::new(client, key_id.clone(), None)
                    .await
                    .map_err(|e| ConfigError(format!("auth.signer: AWS KMS key: {e}")))?;
                Arc::new(signer)
            }
        };
        Ok(Self::new(
            signer,
            fhevm_sig_auth::domain(cfg.chain_id, cfg.protocol_config),
            cfg.validity,
        ))
    }

    pub fn new(
        signer: Arc<dyn Signer + Send + Sync>,
        domain: Eip712Domain,
        validity: Duration,
    ) -> Self {
        Self {
            signer,
            domain,
            validity,
        }
    }

    /// The relayer's address: what the connectors must whitelist.
    pub fn address(&self) -> Address {
        self.signer.address()
    }

    /// The header for one call to `route` with `body`, valid for `validity` from now. Marked sensitive.
    pub async fn sign(
        &self,
        audience: Address,
        route: &str,
        body: &[u8],
    ) -> Result<HeaderValue, String> {
        let route = PathAndQuery::try_from(route).map_err(|e| e.to_string())?;
        let header = fhevm_sig_auth::sign_valid_for(
            &*self.signer,
            &self.domain,
            audience,
            &route,
            body,
            self.validity,
        )
        .await
        .map_err(|e| e.to_string())?;
        Ok(header.to_header_value())
    }
}

impl fmt::Debug for RequestSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestSigner")
            .field("address", &self.address())
            .field("validity", &self.validity)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use fhevm_sig_auth::FhevmSigVerifier;
    use std::collections::HashSet;

    /// A local signer with a fixed key, for chain 1 and `ProtocolConfig` `0xc0c0…c0`.
    pub(crate) fn test_signer() -> Arc<RequestSigner> {
        let key: PrivateKeySigner =
            "0x0101010101010101010101010101010101010101010101010101010101010101"
                .parse()
                .unwrap();
        Arc::new(RequestSigner::new(
            Arc::new(key),
            test_domain(),
            Duration::from_secs(60),
        ))
    }

    pub(crate) fn test_domain() -> Eip712Domain {
        fhevm_sig_auth::domain(1, Address::repeat_byte(0xc0))
    }

    #[tokio::test]
    async fn header_verifies_for_its_audience_only() {
        let signer = test_signer();
        let audience = Address::repeat_byte(0xa1);
        let header = signer
            .sign(audience, "/v1/user-decrypt", b"{}")
            .await
            .unwrap();
        assert!(header.is_sensitive());

        let now = fhevm_sig_auth::unix_now();
        let recover = |audience| {
            FhevmSigVerifier::new(test_domain(), audience, HashSet::new())
                .recover(
                    Some(header.as_bytes()),
                    &PathAndQuery::from_static("/v1/user-decrypt"),
                    now,
                )
                .map(|(signer, _)| signer)
        };
        assert_eq!(recover(audience), Ok(signer.address()));
        assert_ne!(recover(Address::repeat_byte(0xb2)), Ok(signer.address()));
    }

    #[tokio::test]
    async fn private_key_comes_from_the_named_env_var() {
        let cfg = |value_env: &str| AuthConfig {
            chain_id: 1,
            protocol_config: Address::repeat_byte(0xc0),
            validity: Duration::from_secs(60),
            signer: SignerConfig::PrivateKey {
                value_env: value_env.to_owned(),
            },
        };
        let e = RequestSigner::from_config(&cfg("RELAYER_HTTP_TEST_UNSET_KEY"))
            .await
            .unwrap_err();
        assert!(
            e.0.contains("RELAYER_HTTP_TEST_UNSET_KEY is not set"),
            "{e}"
        );

        // `PATH` is always set and never a private key.
        let e = RequestSigner::from_config(&cfg("PATH")).await.unwrap_err();
        assert!(e.0.contains("does not hold a valid private key"), "{e}");
        assert!(!e.0.contains(&std::env::var("PATH").unwrap()), "{e}");
    }
}
