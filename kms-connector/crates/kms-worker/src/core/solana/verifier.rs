//! Whether a Solana decryption request is authorized: through the host program's accounts and the
//! coprocessors' leaf proofs. Every check reads live host state and runs on every worker attempt.
//! EVM requests are authorized by `DecryptionProcessor`, as on main.

use crate::core::{
    config::{Config, HostSettings},
    event_processor::{RequestCheckError, RequestCheckKind},
    solana::{
        SolanaHost,
        pipeline::{AuthorizationContext, authorize_request},
        proof::CoprocessorProofClient,
        public_decrypt::check_public_decrypt,
        snapshot::SolanaRpcClient,
    },
};
use anyhow::anyhow;
use connector_utils::types::solana_request::{
    SolanaPublicDecryptionRequest, SolanaUserDecryptionRequestV1,
};
use kms_connector_api::ErrorCode;
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use tracing::info;

/// Decides whether a Solana decryption request is authorized on the host chain its handles name.
#[derive(Clone)]
pub struct SolanaDecryptionVerifier {
    /// The Solana host chains, by chain id.
    hosts: HashMap<u64, SolanaHost>,
}

impl SolanaDecryptionVerifier {
    pub fn new(hosts: HashMap<u64, SolanaHost>) -> Self {
        Self { hosts }
    }

    /// Builds a reader for every Solana host chain in `config`.
    pub fn connect(config: &Config) -> anyhow::Result<Self> {
        // The workspace `reqwest`, not alloy's re-export: alloy now vendors a different major, and
        // the Solana readers are typed against the workspace crate.
        let proof_client = ::reqwest::Client::builder()
            .connect_timeout(config.host_rpc_call_timeout)
            .timeout(config.host_rpc_call_timeout)
            .build()?;
        let hosts = config
            .host_chains
            .iter()
            .filter_map(|host_chain| {
                let HostSettings::Solana(solana) = &host_chain.host else {
                    return None;
                };
                let host = SolanaHost {
                    program_id: solana.host_program_id,
                    reader: SolanaRpcClient::new(
                        host_chain.url.clone(),
                        config.host_rpc_call_timeout,
                        config.host_rpc_max_concurrent_calls,
                    ),
                    proofs: CoprocessorProofClient::new(&solana.proof_routes, proof_client.clone()),
                };
                Some((host_chain.chain_id, host))
            })
            .collect();
        Ok(Self::new(hosts))
    }

    /// A chain this verifier does not hold is retried, as `DecryptionProcessor` retries a chain its
    /// ACL map lacks.
    fn host(&self, chain_id: u64) -> Result<&SolanaHost, RequestCheckError> {
        self.hosts.get(&chain_id).ok_or_else(|| {
            RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::UpstreamTransient,
                anyhow!("No Solana host chain configured for chain id {chain_id}"),
            )
        })
    }

    /// Proves each handle's public-decrypt leaf against the encrypted store the request names for
    /// it.
    #[tracing::instrument(skip_all)]
    pub async fn check_public_decryption(
        &self,
        request: &SolanaPublicDecryptionRequest,
    ) -> Result<(), RequestCheckError> {
        check_public_decrypt(self.host(request.chain_id())?, request).await?;
        info!(
            "Solana public decryption check passed for {} handles!",
            request.handles().len()
        );
        Ok(())
    }

    /// Rechecks the permit and the host authorization state.
    #[tracing::instrument(skip_all)]
    pub async fn check_user_decryption(
        &self,
        request: &SolanaUserDecryptionRequestV1,
    ) -> Result<(), RequestCheckError> {
        let host = self.host(request.permit().chain_id())?;
        let context = AuthorizationContext {
            program_id: host.program_id,
            now_unix_seconds: Utc::now().timestamp() as u64,
        };
        authorize_request(&host.reader, &host.proofs, context, request).await?;
        info!(
            "Solana user decryption check passed for {} handles!",
            request.handles().len()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::{ApiKey, HostChainConfig, ProofRoute, SolanaHostSettings, solana_host_chain_id},
        event_processor::{ProcessingError, ProcessingErrorKind},
        solana::{
            proof::{HostProofReader, LeafKind, LeafQuery},
            snapshot::HostStateReader,
        },
    };
    use alloy::primitives::{B256, U256};
    use connector_utils::tests::rand::rand_handle;
    use solana_pubkey::Pubkey;
    use std::time::Duration;
    use tokio::{net::TcpListener, time::timeout};

    fn solana_chain(cluster_tag: u64, endpoint: &str) -> HostChainConfig {
        HostChainConfig {
            url: endpoint.parse().unwrap(),
            chain_id: solana_host_chain_id(cluster_tag),
            host: HostSettings::Solana(SolanaHostSettings {
                host_program_id: Pubkey::new_from_array([7; 32]),
                proof_routes: vec![ProofRoute {
                    url: endpoint.parse().unwrap(),
                    api_key: ApiKey::from("test-key".to_owned()),
                }],
            }),
        }
    }

    fn handle_on(chain_id: u64) -> B256 {
        let mut bytes = *rand_handle();
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        bytes.into()
    }

    fn public_request(handle: B256) -> SolanaPublicDecryptionRequest {
        SolanaPublicDecryptionRequest::new(U256::ONE, &[handle], &[B256::ZERO], vec![]).unwrap()
    }

    fn user_request(handle: B256) -> SolanaUserDecryptionRequestV1 {
        connector_utils::tests::rand::solana_user_decryption_request(U256::ONE, handle)
    }

    /// Both request families, checked against `verifier`, as the worker records the result.
    async fn check_both(
        verifier: &SolanaDecryptionVerifier,
        handle: B256,
    ) -> [Result<(), ProcessingError>; 2] {
        [
            verifier
                .check_public_decryption(&public_request(handle))
                .await
                .map_err(RequestCheckError::record),
            verifier
                .check_user_decryption(&user_request(handle))
                .await
                .map_err(RequestCheckError::record),
        ]
    }

    #[tokio::test]
    async fn connect_keeps_only_the_solana_host_chains() {
        let config = Config {
            host_chains: vec![
                Config::default().host_chains.remove(0),
                solana_chain(2, "http://coprocessor.test:8080"),
            ],
            ..Default::default()
        };

        let verifier = SolanaDecryptionVerifier::connect(&config).unwrap();

        assert_eq!(verifier.hosts.len(), 1);
        assert_eq!(
            verifier.hosts[&solana_host_chain_id(2)].program_id,
            Pubkey::new_from_array([7; 32])
        );
    }

    #[tokio::test]
    async fn connected_hosts_bound_rpc_and_proof_requests() {
        // The listening socket permits TCP connections but never answers HTTP requests.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let config = Config {
            host_chains: vec![solana_chain(2, &endpoint)],
            host_rpc_call_timeout: Duration::from_millis(250),
            ..Default::default()
        };
        let verifier = SolanaDecryptionVerifier::connect(&config).unwrap();
        let host = &verifier.hosts[&solana_host_chain_id(2)];
        let keys = [Pubkey::new_from_array([1; 32])];
        let queries = [LeafQuery {
            encrypted_store: Pubkey::new_from_array([1; 32]),
            handle: B256::new([2; 32]),
            kind: LeafKind::Public,
        }];

        let (rpc, proofs) = tokio::join!(
            timeout(
                Duration::from_secs(3),
                host.reader.read_accounts(&keys, None)
            ),
            timeout(Duration::from_secs(3), host.proofs.read_proofs(0, &queries)),
        );

        assert!(rpc.expect("the RPC client must time out").is_err());
        assert!(proofs.expect("the proof client must time out").is_err());
        drop(listener);
    }

    #[tokio::test]
    async fn a_request_for_a_chain_without_a_solana_host_is_retried() {
        let verifier = SolanaDecryptionVerifier::new(HashMap::new());

        for handle in [handle_on(solana_host_chain_id(12345)), handle_on(1)] {
            for result in check_both(&verifier, handle).await {
                match result {
                    Err(error) if error.kind == ProcessingErrorKind::Recoverable => assert!(
                        error
                            .source
                            .to_string()
                            .contains("No Solana host chain configured")
                    ),
                    other => panic!("expected a recoverable unknown-host error, got {other:?}"),
                }
            }
        }
    }
}
