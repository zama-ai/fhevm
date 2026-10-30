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

/// Signed at startup to check the key; not an EIP-712 digest, so the signature authorizes nothing.
const PROBE: &[u8] = b"relayer-http startup signing check";

/// Signs `Authorization: FhevmSig ...` headers with the relayer's key (local or AWS KMS).
pub struct RequestSigner {
    signer: Arc<dyn Signer + Send + Sync>,
    domain: Eip712Domain,
    validity: Duration,
}

impl RequestSigner {
    /// Loads the key from the env var or AWS KMS.
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
            SignerConfig::AwsKms { key_id } => {
                let aws = aws_config::defaults(aws_config::BehaviorVersion::latest())
                    .load()
                    .await;
                let client = aws_sdk_kms::Client::new(&aws);
                let signer = AwsSigner::new(client, key_id.clone(), None)
                    .await
                    .map_err(|e| ConfigError(format!("auth.signer: AWS KMS key: {e}")))?;
                Arc::new(signer)
            }
        };
        let signer = Self::checked(cfg, signer)?;
        signer.probe().await?;
        Ok(signer)
    }

    /// Fails startup when the key cannot sign (AWS KMS without `kms:Sign`).
    async fn probe(&self) -> Result<(), ConfigError> {
        let hash = alloy::primitives::keccak256(PROBE);
        let signature = self.signer.sign_hash(&hash).await.map_err(|e| {
            ConfigError(format!(
                "auth.signer: the startup test signature failed: {e}"
            ))
        })?;
        match signature.recover_address_from_prehash(&hash) {
            Ok(address) if address == self.address() => Ok(()),
            _ => Err(ConfigError(format!(
                "auth.signer: the startup test signature does not recover to {}",
                self.address()
            ))),
        }
    }

    /// Refuses a key other than `auth.address`.
    fn checked(
        cfg: &AuthConfig,
        signer: Arc<dyn Signer + Send + Sync>,
    ) -> Result<Self, ConfigError> {
        if signer.address() != cfg.address {
            return Err(ConfigError(format!(
                "kms_aggregator.auth.address is {} but the signing key's address is {}",
                cfg.address,
                signer.address()
            )));
        }
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

    /// The address connectors must allow.
    pub fn address(&self) -> Address {
        self.signer.address()
    }

    /// The `FhevmSig` header for one call, marked sensitive.
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

    /// An `AuthConfig` for chain 7 and `ProtocolConfig` `0xc1…c1`, unlike `test_domain()`, expecting `address`.
    fn auth_config(address: Address) -> AuthConfig {
        AuthConfig {
            chain_id: 7,
            protocol_config: Address::repeat_byte(0xc1),
            validity: Duration::from_secs(60),
            address,
            signer: SignerConfig::PrivateKey {
                value_env: "RELAYER_SIGNER_KEY".to_owned(),
            },
        }
    }

    #[tokio::test]
    async fn domain_comes_from_the_config() {
        let key = Arc::new(PrivateKeySigner::random());
        let signer = RequestSigner::checked(&auth_config(key.address()), key.clone()).unwrap();
        let audience = Address::repeat_byte(0xa1);
        let header = signer
            .sign(audience, "/v1/user-decrypt", b"{}")
            .await
            .unwrap();
        let verifier = FhevmSigVerifier::new(
            fhevm_sig_auth::domain(7, Address::repeat_byte(0xc1)),
            audience,
            HashSet::from([key.address()]),
        );
        let route = PathAndQuery::from_static("/v1/user-decrypt");
        assert_eq!(
            verifier.verify(Some(header.as_bytes()), &route, b"{}"),
            Ok(key.address())
        );
    }

    #[test]
    fn a_key_other_than_auth_address_is_refused() {
        let key = Arc::new(PrivateKeySigner::random());
        let expected = Address::repeat_byte(0xee);
        let e = RequestSigner::checked(&auth_config(expected), key.clone()).unwrap_err();
        assert!(
            e.0.contains(&format!(
                "auth.address is {expected} but the signing key's address is {}",
                key.address()
            )),
            "{e}"
        );
    }

    /// Claims `address`, signs with `inner`, or fails every signature when `inner` is `None` (an AWS KMS key without
    /// `kms:Sign`).
    struct FaultySigner {
        inner: Option<PrivateKeySigner>,
        address: Address,
    }

    #[async_trait::async_trait]
    impl Signer for FaultySigner {
        async fn sign_hash(
            &self,
            hash: &alloy::primitives::B256,
        ) -> alloy::signers::Result<alloy::primitives::Signature> {
            match &self.inner {
                Some(inner) => inner.sign_hash(hash).await,
                None => Err(alloy::signers::Error::other("AccessDeniedException")),
            }
        }

        fn address(&self) -> Address {
            self.address
        }

        fn chain_id(&self) -> Option<alloy::primitives::ChainId> {
            None
        }

        fn set_chain_id(&mut self, _chain_id: Option<alloy::primitives::ChainId>) {}
    }

    #[tokio::test]
    async fn probe_passes_for_a_key_that_signs() {
        test_signer().probe().await.unwrap();
    }

    #[tokio::test]
    async fn probe_fails_when_the_key_cannot_sign_or_is_not_its_address() {
        let address = Address::repeat_byte(0xee);
        let probe = |inner| async move {
            RequestSigner::new(
                Arc::new(FaultySigner { inner, address }),
                test_domain(),
                Duration::from_secs(60),
            )
            .probe()
            .await
            .unwrap_err()
            .0
        };
        let e = probe(None).await;
        assert!(
            e.contains("startup test signature failed: ") && e.contains("AccessDeniedException"),
            "{e}"
        );
        let e = probe(Some(PrivateKeySigner::random())).await;
        assert!(e.contains(&format!("does not recover to {address}")), "{e}");
    }

    #[tokio::test]
    async fn private_key_comes_from_the_named_env_var() {
        let cfg = |value_env: &str| AuthConfig {
            chain_id: 1,
            protocol_config: Address::repeat_byte(0xc0),
            validity: Duration::from_secs(60),
            address: Address::repeat_byte(0xee),
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

    #[tokio::test]
    async fn sign_rejects_a_route_with_a_query_string() {
        let e = test_signer()
            .sign(Address::repeat_byte(0xa1), "/v1/user-decrypt?a=1", b"{}")
            .await
            .unwrap_err();
        assert!(e.contains("query string"), "{e}");
    }

    #[tokio::test]
    async fn expiry_is_now_plus_validity() {
        let signer = test_signer();
        let before = fhevm_sig_auth::unix_now();
        let header = signer
            .sign(Address::repeat_byte(0xa1), "/v1/user-decrypt", b"{}")
            .await
            .unwrap();
        let after = fhevm_sig_auth::unix_now();
        let expires = fhevm_sig_auth::FhevmSigHeader::parse(header.as_bytes())
            .unwrap()
            .expires;
        assert!((before + 60..=after + 60).contains(&expires), "{expires}");
    }
}
