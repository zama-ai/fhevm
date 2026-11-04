//! Zama relayer over HTTP. Reading order: `kms_aggregator/docs.md`, then `kms_aggregator/aggregator.rs`.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic
    )
)]

pub mod config;
pub mod endpoint;
pub mod kms_aggregator;
pub mod logging;

use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::config::{HttpConfig, RelayerConfig};
use kms_aggregator::{
    Aggregator, Caller, ConfigError, HttpClient, PublicDecrypt, RequestSigner, UserDecrypt,
};

/// Everything a request handler needs: the one shared state of the process. Built once in `main`, cloned per
/// request (Arcs only). Both aggregators share one `Caller`: one HTTP client, one call semaphore.
#[derive(Clone)]
pub struct App {
    pub user_decrypt: Arc<Aggregator<UserDecrypt>>,
    pub public_decrypt: Arc<Aggregator<PublicDecrypt>>,
    /// Bind address, body limit, supported chain ids.
    pub http: Arc<HttpConfig>,
    /// Cancelled on SIGINT/SIGTERM: `/healthz` answers 503 and every running aggregation ends.
    pub shutdown: CancellationToken,
}

impl App {
    /// Loads the signing key (env var or AWS KMS), builds the shared HTTP client, semaphore and both aggregators.
    pub async fn new(
        config: &RelayerConfig,
        shutdown: CancellationToken,
    ) -> Result<Self, ConfigError> {
        let kms = &config.kms_aggregator;
        let client = Arc::new(HttpClient::new(kms)?);
        let signer = Arc::new(RequestSigner::from_config(&kms.auth).await?);
        tracing::info!(address = %signer.address(), "FhevmSig signer loaded");
        let caller = Arc::new(Caller::new(kms, client, signer)?);
        Ok(Self {
            user_decrypt: Arc::new(Aggregator::new(
                caller.clone(),
                kms.user_decrypt.threshold,
                kms.user_decrypt.checks,
                shutdown.clone(),
            )),
            public_decrypt: Arc::new(Aggregator::new(
                caller,
                kms.public_decrypt.threshold,
                (),
                shutdown.clone(),
            )),
            http: Arc::new(config.http.clone()),
            shutdown,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kms_aggregator::config::SignerConfig;

    const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../config/config.yaml");

    #[tokio::test]
    async fn app_needs_the_signing_key() {
        let mut config = RelayerConfig::load(EXAMPLE).unwrap();
        config.kms_aggregator.auth.signer = SignerConfig::PrivateKey {
            value_env: "RELAYER_HTTP_TEST_UNSET_KEY".to_owned(),
        };
        let e = App::new(&config, CancellationToken::new())
            .await
            .err()
            .unwrap();
        assert!(
            e.0.contains("RELAYER_HTTP_TEST_UNSET_KEY is not set"),
            "{e}"
        );
    }

    /// Tests against an AWS emulator (Floci), run only by `make -C dev test-aws`, which starts it:
    /// `cargo nextest run -p relayer-http --run-ignored only -E 'test(/aws_kms_emulator/)'`.
    mod aws_kms_emulator {
        use super::*;
        use alloy::signers::Signer;
        use alloy::signers::aws::AwsSigner;
        use aws_sdk_kms::types::{KeySpec, KeyUsageType};
        use fhevm_sig_auth::{FhevmSigVerifier, PathAndQuery};
        use kms_connector_api::USER_DECRYPTION_ROUTE;
        use std::collections::{HashMap, HashSet};

        /// The AWS KMS signer as deployed: `key_id` and `address` come in as `APP_*` overrides; region, endpoint and
        /// credentials from the AWS environment (IRSA's role and token file, as on EKS).
        #[tokio::test]
        #[ignore = "needs an AWS emulator: make -C dev test-aws"]
        async fn signs_as_deployed() {
            let aws = aws_config::defaults(aws_config::BehaviorVersion::latest())
                .load()
                .await;
            let kms = aws_sdk_kms::Client::new(&aws);
            let key_id = kms
                .create_key()
                .key_spec(KeySpec::EccSecgP256K1)
                .key_usage(KeyUsageType::SignVerify)
                .send()
                .await
                .unwrap()
                .key_metadata
                .unwrap()
                .key_id;
            let address = AwsSigner::new(kms, key_id.clone(), None)
                .await
                .unwrap()
                .address();

            // The example config with an AWS KMS signer whose key id is left to the environment.
            let line = "signer: { type: private_key, value_env: RELAYER_SIGNER_KEY }";
            let yaml = std::fs::read_to_string(EXAMPLE).unwrap();
            assert!(yaml.contains(line));
            let path = std::env::temp_dir().join(format!("relayer-http-aws-{key_id}.yaml"));
            std::fs::write(
                &path,
                yaml.replace(line, r#"signer: { type: aws_kms, key_id: "from-env" }"#),
            )
            .unwrap();
            let env = ::config::Environment::with_prefix("APP").source(Some(HashMap::from([
                (
                    "APP_KMS_AGGREGATOR__AUTH__SIGNER__KEY_ID".to_owned(),
                    key_id,
                ),
                (
                    "APP_KMS_AGGREGATOR__AUTH__ADDRESS".to_owned(),
                    address.to_string(),
                ),
            ])));
            let config = RelayerConfig::load_with(path.to_str().unwrap(), env);
            std::fs::remove_file(&path).unwrap();
            let config = config.unwrap();

            // The startup: the key loads, has `auth.address` and signs the test hash.
            App::new(&config, CancellationToken::new()).await.unwrap();

            // A connector call's header verifies as a connector proxy verifies it.
            let auth = &config.kms_aggregator.auth;
            let audience = config.kms_aggregator.endpoints[0].audience;
            let signer = RequestSigner::from_config(auth).await.unwrap();
            let header = signer
                .sign(audience, USER_DECRYPTION_ROUTE, b"{}")
                .await
                .unwrap();
            let verifier = FhevmSigVerifier::new(
                fhevm_sig_auth::domain(auth.chain_id, auth.protocol_config),
                audience,
                HashSet::from([address]),
            );
            assert_eq!(
                verifier.verify(
                    Some(header.as_bytes()),
                    &PathAndQuery::from_static(USER_DECRYPTION_ROUTE),
                    b"{}"
                ),
                Ok(address)
            );
        }
    }
}
