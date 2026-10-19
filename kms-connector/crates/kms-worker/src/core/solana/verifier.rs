//! Whether a Solana decryption request is authorized: through the host program's accounts and the
//! coprocessors' Merkle proofs. Every check reads live host state and runs on every worker attempt.
//! EVM requests are authorized by `DecryptionProcessor`, as on main.

use crate::core::{
    config::{Config, HostSettings},
    event_processor::{RequestCheckError, RequestCheckKind},
    solana::{
        SolanaHost,
        membership::KmsContextMembers,
        pipeline::{AuthorizationContext, authorize_request},
        proof::{CoprocessorProofClient, proof_http_client},
        public_decrypt::check_public_decrypt,
        snapshot::SolanaRpcClient,
    },
};
use alloy::providers::DynProvider;
use anyhow::anyhow;
use connector_utils::{
    config::KmsWallet,
    types::{
        extra_data::parse_extra_data,
        solana_request::{SolanaPublicDecryptionRequest, SolanaUserDecryptionRequestV1},
    },
};
use kms_connector_api::ErrorCode;
use request_authorization::KeyRegistry;
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use tracing::info;

/// Decides whether a Solana decryption request is authorized on the host chain its handles name.
#[derive(Clone)]
pub struct SolanaDecryptionVerifier {
    /// The Solana host chains, by chain id.
    hosts: HashMap<u64, SolanaHost>,
    /// The nodes of the KMS contexts, one of which a request's own node must be.
    members: KmsContextMembers,
}

impl SolanaDecryptionVerifier {
    pub fn new(hosts: HashMap<u64, SolanaHost>, members: KmsContextMembers) -> Self {
        Self { hosts, members }
    }

    /// Builds a reader for every Solana host chain in `config`. The proof requests are signed by
    /// the configured tx-sender wallet, which a Solana host chain therefore requires. The KMS
    /// contexts are read from `config`'s `ProtocolConfig` through `ethereum_provider`.
    pub async fn connect(config: &Config, ethereum_provider: DynProvider) -> anyhow::Result<Self> {
        let members =
            KmsContextMembers::new(config.protocol_config_contract.address, ethereum_provider);
        let solana_chains: Vec<_> = config
            .host_chains
            .iter()
            .filter_map(|host_chain| match &host_chain.host {
                HostSettings::Solana(solana) => Some((host_chain, solana)),
                HostSettings::Evm { .. } => None,
            })
            .collect();
        if solana_chains.is_empty() {
            return Ok(Self::new(HashMap::new(), members));
        }
        let wallet = KmsWallet::from_config(
            config.private_key.as_ref(),
            config.aws_kms_config.as_ref(),
            None,
        )
        .await?;
        let proof_client = proof_http_client(config.host_rpc_call_timeout)?;
        let hosts = solana_chains
            .into_iter()
            .map(|(host_chain, solana)| {
                let host = SolanaHost {
                    program_id: solana.host_program_id,
                    reader: SolanaRpcClient::new(
                        host_chain.url.clone(),
                        config.host_rpc_call_timeout,
                        config.host_rpc_max_concurrent_calls,
                    ),
                    proofs: CoprocessorProofClient::new(
                        &solana.proof_servers,
                        proof_client.clone(),
                        wallet.clone(),
                        KeyRegistry {
                            chain_id: config.ethereum_chain_id,
                            contract: config.protocol_config_contract.address,
                        },
                        config.host_rpc_call_timeout,
                    ),
                };
                (host_chain.chain_id, host)
            })
            .collect();
        Ok(Self::new(hosts, members))
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

    /// Refuses a request unless this connector's tx-sender, which signs its proof requests, is a
    /// node of the KMS context the request names. It runs before any host or proof read.
    async fn check_member(
        &self,
        host: &SolanaHost,
        extra_data: &[u8],
    ) -> Result<(), RequestCheckError> {
        let extra_data = parse_extra_data(extra_data).map_err(|e| {
            RequestCheckError::irrecoverable(
                RequestCheckKind::KmsContext,
                ErrorCode::Unprocessable,
                e,
            )
        })?;
        let Some(context_id) = extra_data.context_id else {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::KmsContext,
                ErrorCode::Unprocessable,
                anyhow!("A Solana decryption request names no KMS context"),
            ));
        };
        self.members
            .check(context_id, host.proofs.tx_sender())
            .await
    }

    /// Proves each handle's public-decrypt leaf against the encrypted store the request names for
    /// it.
    #[tracing::instrument(skip_all)]
    pub async fn check_public_decryption(
        &self,
        request: &SolanaPublicDecryptionRequest,
    ) -> Result<(), RequestCheckError> {
        let host = self.host(request.chain_id())?;
        self.check_member(host, request.extra_data()).await?;
        check_public_decrypt(host, request).await?;
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
        let host = self.host(request.request.permit().chain_id())?;
        self.check_member(host, &request.extra_data()).await?;
        let context = AuthorizationContext {
            program_id: host.program_id,
            now_unix_seconds: Utc::now().timestamp() as u64,
        };
        authorize_request(&host.reader, &host.proofs, context, request).await?;
        info!(
            "Solana user decryption check passed for {} handles!",
            request.request.entries().len()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::{HostChainConfig, ProofServer, SolanaHostSettings},
        event_processor::{ProcessingError, ProcessingErrorKind},
        solana::{
            proof::{HostProofReader, LeafKind, LeafQuery},
            snapshot::HostStateReader,
        },
    };
    use alloy::{
        primitives::{Address, B256, U256},
        providers::{Provider, ProviderBuilder, mock::Asserter},
    };
    use connector_utils::tests::rand::rand_handle;
    use solana_pubkey::Pubkey;
    use std::time::Duration;
    use tokio::{net::TcpListener, time::timeout};
    use zama_solana_acl::host_chain::solana_host_chain_id;

    fn solana_chain(cluster_tag: u64, endpoint: &str) -> HostChainConfig {
        HostChainConfig {
            url: endpoint.parse().unwrap(),
            chain_id: solana_host_chain_id(cluster_tag),
            host: HostSettings::Solana(SolanaHostSettings {
                host_program_id: Pubkey::new_from_array([7; 32]),
                proof_servers: vec![ProofServer {
                    url: endpoint.parse().unwrap(),
                    signer_address: Address::repeat_byte(0xD0),
                }],
            }),
        }
    }

    /// An Ethereum node that answers nothing: these tests read no KMS context.
    fn ethereum() -> DynProvider {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new())
            .erased()
    }

    fn handle_on(chain_id: u64) -> B256 {
        let mut bytes = *rand_handle();
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        bytes.into()
    }

    fn public_request(handle: B256) -> SolanaPublicDecryptionRequest {
        SolanaPublicDecryptionRequest::new(U256::ONE, &[handle], &[B256::ZERO], vec![0x02; 65])
            .unwrap()
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

        let verifier = SolanaDecryptionVerifier::connect(&config, ethereum())
            .await
            .unwrap();

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
        let verifier = SolanaDecryptionVerifier::connect(&config, ethereum())
            .await
            .unwrap();
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
            timeout(Duration::from_secs(3), async {
                let batch = host.proofs.prepare(&queries).await?;
                host.proofs.read_proofs(0, &batch).await
            }),
        );

        assert!(rpc.expect("the RPC client must time out").is_err());
        assert!(proofs.expect("the proof client must time out").is_err());
        drop(listener);
    }

    #[tokio::test]
    async fn a_solana_host_chain_requires_the_wallet() {
        let config = Config {
            host_chains: vec![solana_chain(2, "http://coprocessor.test:8080")],
            private_key: None,
            ..Default::default()
        };
        let error = SolanaDecryptionVerifier::connect(&config, ethereum())
            .await
            .err()
            .expect("no wallet is configured");
        assert!(
            error.to_string().contains("Either AWS KMS or private key"),
            "{error}"
        );

        let evm_only = Config {
            private_key: None,
            ..Default::default()
        };
        assert!(
            SolanaDecryptionVerifier::connect(&evm_only, ethereum())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn a_request_for_a_chain_without_a_solana_host_is_retried() {
        let verifier = SolanaDecryptionVerifier::new(
            HashMap::new(),
            KmsContextMembers::new(Address::ZERO, ethereum()),
        );

        for result in check_both(&verifier, handle_on(solana_host_chain_id(12345))).await {
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
