use alloy::{primitives::Address, transports::http::reqwest::Url};
use ciphertext_attestation::MAX_SNS_CIPHERTEXT_SERIALIZED_SIZE;
use connector_utils::{
    config::{
        ContractConfig, DeserializeConfig,
        contract::{
            default_decryption_contract_config, default_gateway_config_contract_config,
            default_kms_generation_contract_config, default_protocol_config_contract_config,
            deserialize_decryption_contract_config, deserialize_gateway_config_contract_config,
            deserialize_kms_generation_contract_config,
            deserialize_protocol_config_contract_config,
        },
        default_database_pool_size, deserialize_non_zero_duration,
    },
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::{Deserialize, Deserializer};
use solana_pubkey::Pubkey;
use std::{net::SocketAddr, num::NonZeroUsize, str::FromStr, time::Duration};
use zama_solana_request::host_chain::{EVM_CHAIN_TYPE, SOLANA_CHAIN_TYPE, chain_type_byte};

/// Configuration of the `KmsWorker`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,
    /// The timeout for polling the database for fast events (decryption for ex).
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_db_fast_event_polling"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub db_fast_event_polling: Duration,
    /// The timeout for polling the database for long events (prep keygen for ex).
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_db_long_event_polling"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub db_long_event_polling: Duration,
    /// The limit number of events to fetch from the database.
    #[serde(default = "default_events_batch_size")]
    pub events_batch_size: u8,

    /// The Gateway RPC endpoint.
    pub gateway_url: Url,
    /// The Chain ID of the Gateway.
    pub gateway_chain_id: u64,
    /// The `Decryption` contract configuration (on Gateway).
    #[serde(deserialize_with = "deserialize_decryption_contract_config")]
    pub decryption_contract: ContractConfig,
    /// The `GatewayConfig` contract configuration (on Gateway).
    #[serde(deserialize_with = "deserialize_gateway_config_contract_config")]
    pub gateway_config_contract: ContractConfig,

    /// The Ethereum RPC endpoint.
    pub ethereum_url: Url,
    /// The Chain ID of the Ethereum chain.
    pub ethereum_chain_id: u64,
    /// The `KMSGeneration` contract configuration (on Ethereum).
    #[serde(deserialize_with = "deserialize_kms_generation_contract_config")]
    pub kms_generation_contract: ContractConfig,
    /// The `ProtocolConfig` contract configuration (on Ethereum).
    #[serde(deserialize_with = "deserialize_protocol_config_contract_config")]
    pub protocol_config_contract: ContractConfig,

    /// The Host Chains configuration.
    #[serde(deserialize_with = "deserialize_host_chains")]
    #[cfg_attr(test, serde(skip_serializing))]
    pub host_chains: Vec<HostChainConfig>,

    /// The KMS Core endpoints.
    #[serde(deserialize_with = "deserialize_non_empty")]
    pub kms_core_endpoints: Vec<String>,
    /// Number of retries for GRPC requests sent to the KMS Core.
    #[serde(default = "default_grpc_request_retries")]
    pub grpc_request_retries: u8,
    /// The maximum number of decryption attempts.
    #[serde(default = "default_max_decryption_attempts")]
    pub max_decryption_attempts: u16,

    /// Number of attempts for S3 ciphertext retrieval.
    #[serde(default = "default_s3_ciphertext_retrieval_attempts")]
    pub s3_ciphertext_retrieval_attempts: u8,
    /// Timeout to connect to a S3 bucket.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_s3_connect_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub s3_connect_timeout: Duration,
    /// Timeout of a single attestation `HEAD` on a Coprocessor bucket.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_s3_head_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub s3_head_timeout: Duration,
    /// Timeout of a single ciphertext `GET`.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_s3_get_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub s3_get_timeout: Duration,
    /// Ceiling on the attestation `HEAD`s in flight on a single Coprocessor bucket, across all
    /// requests.
    #[serde(default = "default_s3_max_concurrent_heads_per_bucket")]
    pub s3_max_concurrent_heads_per_bucket: NonZeroUsize,
    /// Ceiling on the ciphertext `GET`s in flight, across all requests and buckets.
    #[serde(default = "default_s3_max_concurrent_gets")]
    pub s3_max_concurrent_gets: NonZeroUsize,
    /// Ceiling on the size of a single ciphertext body, in bytes.
    #[serde(default = "default_s3_max_ciphertext_size")]
    pub s3_max_ciphertext_size: NonZeroUsize,
    /// Refresh interval of the Coprocessor registry, read from the `GatewayConfig` contract.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_copro_registry_refresh"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub copro_registry_refresh: Duration,

    /// Ceiling on the calls a single host chain endpoint may have in flight, across all requests.
    #[serde(default = "default_host_rpc_max_concurrent_calls")]
    pub host_rpc_max_concurrent_calls: NonZeroUsize,
    /// Deadline for one host RPC call or Solana coprocessor proof request, including its body.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_host_rpc_call_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub host_rpc_call_timeout: Duration,

    /// Gas cap for the host-chain `IERC1271.isValidSignature` static call (RFC-012).
    /// Bounded to prevent resource exhaustion from malicious smart-account contracts.
    #[serde(default = "default_erc1271_gas_limit")]
    pub erc1271_gas_limit: u64,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    /// The monitoring server endpoint of the `KmsWorker`.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_healthcheck_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub healthcheck_timeout: Duration,
}

/// Configuration of a single host chain. Its kind is its chain id's type byte, and it carries
/// only that kind's settings.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "HostChainEntry")]
pub struct HostChainConfig {
    /// The host chain RPC endpoint.
    pub url: Url,
    /// The chain id of the host chain.
    pub chain_id: u64,
    /// What the connector verifies decryptions against on this chain.
    pub host: HostSettings,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HostSettings {
    Evm {
        /// The `ACL` contract that gates decryptions.
        acl_address: Address,
    },
    Solana(SolanaHostSettings),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SolanaHostSettings {
    /// The zama-host program id.
    pub host_program_id: Pubkey,
    /// The coprocessors' leaf-proof routes, one per coprocessor, at least one. They are asked in
    /// this order, each only for the leaves the ones before it could not prove. A coprocessor
    /// that does not answer delays the next by up to `host_rpc_call_timeout`.
    pub proof_routes: Vec<ProofRoute>,
}

/// One coprocessor's leaf-proof endpoint and the bearer key that endpoint expects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProofRoute {
    pub url: Url,
    #[serde(alias = "apiKey")]
    pub api_key: ApiKey,
}

/// A bearer key. Its [`Debug`] output is redacted, so it does not reach logs.
#[derive(Clone, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl From<String> for ApiKey {
    fn from(key: String) -> Self {
        Self(key)
    }
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiKey(<redacted>)")
    }
}

/// A host chain as written in the configuration, before its settings are checked against the
/// kind its chain id names.
#[derive(Deserialize)]
struct HostChainEntry {
    url: Url,
    #[serde(alias = "chainId")]
    chain_id: u64,
    #[serde(default, alias = "aclAddress")]
    acl_address: Option<Address>,
    #[serde(
        default,
        deserialize_with = "deserialize_optional_solana_pubkey",
        alias = "solanaHostProgramId"
    )]
    solana_host_program_id: Option<Pubkey>,
    #[serde(default, alias = "solanaProofRoutes")]
    solana_proof_routes: Vec<ProofRoute>,
}

impl TryFrom<HostChainEntry> for HostChainConfig {
    type Error = String;

    fn try_from(entry: HostChainEntry) -> Result<Self, Self::Error> {
        let chain_id = entry.chain_id;
        let host = match chain_type_byte(chain_id) {
            EVM_CHAIN_TYPE => {
                if entry.solana_host_program_id.is_some() || !entry.solana_proof_routes.is_empty() {
                    return Err(format!(
                        "EVM host chain {chain_id} must not set solana_host_program_id or solana_proof_routes"
                    ));
                }
                let acl_address = entry.acl_address.ok_or_else(|| {
                    format!(
                        "EVM host chain {chain_id} requires acl_address (the ACL contract to gate decryptions)"
                    )
                })?;
                HostSettings::Evm { acl_address }
            }
            SOLANA_CHAIN_TYPE => {
                if entry.acl_address.is_some() {
                    return Err(format!(
                        "Solana host chain {chain_id} must not set acl_address"
                    ));
                }
                let host_program_id = entry.solana_host_program_id.ok_or_else(|| {
                    format!("Solana host chain {chain_id} requires solana_host_program_id")
                })?;
                if entry.solana_proof_routes.is_empty() {
                    return Err(format!(
                        "Solana host chain {chain_id} requires at least one solana_proof_routes entry"
                    ));
                }
                if let Some(route) = entry
                    .solana_proof_routes
                    .iter()
                    .find(|route| route.api_key.expose().is_empty())
                {
                    return Err(format!(
                        "Solana host chain {chain_id} proof route {} has an empty api_key",
                        route.url
                    ));
                }
                HostSettings::Solana(SolanaHostSettings {
                    host_program_id,
                    proof_routes: entry.solana_proof_routes,
                })
            }
            other => {
                return Err(format!(
                    "host chain {chain_id} has type byte {other:#04x}, which names no host kind"
                ));
            }
        };
        Ok(Self {
            url: entry.url,
            chain_id,
            host,
        })
    }
}

fn deserialize_host_chains<'de, D>(d: D) -> Result<Vec<HostChainConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let host_chains: Vec<HostChainConfig> =
        if let Ok(host_chains_json_str) = std::env::var("KMS_CONNECTOR_HOST_CHAINS") {
            serde_json::from_str(&host_chains_json_str).map_err(serde::de::Error::custom)?
        } else {
            <Vec<HostChainConfig>>::deserialize(d)?
        };
    if host_chains.is_empty() {
        return Err(serde::de::Error::custom(
            "Field should not be an empty array",
        ));
    }
    let mut chain_ids = std::collections::HashSet::with_capacity(host_chains.len());
    for host_chain in &host_chains {
        if !chain_ids.insert(host_chain.chain_id) {
            return Err(serde::de::Error::custom(format!(
                "Duplicate host chain in config for chain ID {}",
                host_chain.chain_id
            )));
        }
    }
    Ok(host_chains)
}

fn deserialize_optional_solana_pubkey<'de, D>(d: D) -> Result<Option<Pubkey>, D::Error>
where
    D: Deserializer<'de>,
{
    let Some(pubkey) = Option::<String>::deserialize(d)? else {
        return Ok(None);
    };
    let pubkey = Pubkey::from_str(&pubkey).map_err(serde::de::Error::custom)?;
    Ok(Some(pubkey))
}

fn deserialize_non_empty<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec = <Vec<T>>::deserialize(d)?;
    if vec.is_empty() {
        Err(serde::de::Error::custom(
            "Field should not be an empty array",
        ))
    } else {
        Ok(vec)
    }
}

fn default_service_name() -> String {
    "kms-connector-kms-worker".to_string()
}

fn default_db_fast_event_polling() -> Duration {
    Duration::from_secs(3)
}

fn default_db_long_event_polling() -> Duration {
    Duration::from_secs(60)
}

fn default_events_batch_size() -> u8 {
    50
}

fn default_grpc_request_retries() -> u8 {
    3
}

fn default_max_decryption_attempts() -> u16 {
    20
}

fn default_s3_ciphertext_retrieval_attempts() -> u8 {
    3
}

fn default_s3_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

fn default_s3_head_timeout() -> Duration {
    Duration::from_secs(5)
}

fn default_s3_get_timeout() -> Duration {
    Duration::from_secs(20)
}

fn default_copro_registry_refresh() -> Duration {
    Duration::from_secs(60)
}

fn default_host_rpc_max_concurrent_calls() -> NonZeroUsize {
    NonZeroUsize::new(128).unwrap()
}

fn default_host_rpc_call_timeout() -> Duration {
    Duration::from_secs(10)
}

fn default_s3_max_concurrent_heads_per_bucket() -> NonZeroUsize {
    NonZeroUsize::new(64).unwrap()
}

fn default_s3_max_concurrent_gets() -> NonZeroUsize {
    // Kept low: SNS ciphertexts are big, so too many buffered at once would OOM the worker.
    NonZeroUsize::new(16).unwrap()
}

fn default_s3_max_ciphertext_size() -> NonZeroUsize {
    NonZeroUsize::new(MAX_SNS_CIPHERTEXT_SERIALIZED_SIZE as usize).unwrap()
}

fn default_erc1271_gas_limit() -> u64 {
    250_000
}

impl DeserializeConfig for Config {}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost/kms-connector".to_string(),
            database_pool_size: default_database_pool_size(),
            db_fast_event_polling: default_db_fast_event_polling(),
            db_long_event_polling: default_db_long_event_polling(),
            events_batch_size: default_events_batch_size(),
            gateway_url: Url::from_str("http://localhost:8545").unwrap(),
            gateway_chain_id: 54321,
            decryption_contract: default_decryption_contract_config(),
            gateway_config_contract: default_gateway_config_contract_config(),
            ethereum_url: Url::from_str("http://localhost:8545").unwrap(),
            ethereum_chain_id: 11155111, // Sepolia
            kms_generation_contract: default_kms_generation_contract_config(),
            protocol_config_contract: default_protocol_config_contract_config(),
            host_chains: vec![HostChainConfig {
                url: Url::from_str("http://localhost:8545").unwrap(),
                chain_id: 12345,
                host: HostSettings::Evm {
                    acl_address: Address::default(),
                },
            }],
            kms_core_endpoints: vec!["http://localhost:50051".to_string()],
            grpc_request_retries: default_grpc_request_retries(),
            max_decryption_attempts: default_max_decryption_attempts(),
            s3_ciphertext_retrieval_attempts: default_s3_ciphertext_retrieval_attempts(),
            s3_connect_timeout: default_s3_connect_timeout(),
            s3_head_timeout: default_s3_head_timeout(),
            s3_get_timeout: default_s3_get_timeout(),
            s3_max_concurrent_heads_per_bucket: default_s3_max_concurrent_heads_per_bucket(),
            s3_max_concurrent_gets: default_s3_max_concurrent_gets(),
            s3_max_ciphertext_size: default_s3_max_ciphertext_size(),
            copro_registry_refresh: default_copro_registry_refresh(),
            host_rpc_max_concurrent_calls: default_host_rpc_max_concurrent_calls(),
            host_rpc_call_timeout: default_host_rpc_call_timeout(),
            erc1271_gas_limit: default_erc1271_gas_limit(),
            service_name: default_service_name(),
            task_limit: default_task_limit(),
            monitoring_endpoint: default_monitoring_endpoint(),
            healthcheck_timeout: default_healthcheck_timeout(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use serial_test::serial;
    use std::{env, str::FromStr};
    use zama_solana_request::host_chain::solana_host_chain_id;

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_DATABASE_URL");
            env::remove_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE");
            env::remove_var("KMS_CONNECTOR_GATEWAY_URL");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_ETHEREUM_URL");
            env::remove_var("KMS_CONNECTOR_ETHEREUM_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_HOST_CHAINS");
            env::remove_var("KMS_CONNECTOR_KMS_CORE_ENDPOINTS");
            env::remove_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES");
            env::remove_var("KMS_CONNECTOR_MAX_DECRYPTION_ATTEMPTS");
            env::remove_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_ATTEMPTS");
            env::remove_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_S3_HEAD_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_S3_GET_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_S3_MAX_CONCURRENT_HEADS_PER_BUCKET");
            env::remove_var("KMS_CONNECTOR_S3_MAX_CONCURRENT_GETS");
            env::remove_var("KMS_CONNECTOR_S3_MAX_CIPHERTEXT_SIZE");
            env::remove_var("KMS_CONNECTOR_COPRO_REGISTRY_REFRESH");
            env::remove_var("KMS_CONNECTOR_HOST_RPC_MAX_CONCURRENT_CALLS");
            env::remove_var("KMS_CONNECTOR_HOST_RPC_CALL_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
        }
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let default_config = Config::default();
        let example_config = Config::from_env_and_file(Some(example_config_path())).unwrap();
        assert_eq!(default_config, example_config);
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_from_env() {
        cleanup_env_vars();

        // Set environment variables
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_DATABASE_URL",
                "postgres://postgres:postgres@localhost",
            );
            env::set_var("KMS_CONNECTOR_EVENTS_BATCH_SIZE", "15");
            env::set_var("KMS_CONNECTOR_GATEWAY_URL", "http://localhost:9545");
            env::set_var("KMS_CONNECTOR_GATEWAY_CHAIN_ID", "31888");
            env::set_var("KMS_CONNECTOR_ETHEREUM_URL", "http://localhost:9546");
            env::set_var("KMS_CONNECTOR_ETHEREUM_CHAIN_ID", "31444");
            env::set_var(
                "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS",
                "0x5fbdb2315678afecb367f032d93f642f64180aa3",
            );
            env::set_var(
                "KMS_CONNECTOR_HOST_CHAINS",
                r#"
                    [
                        {
                            "url": "http://localhost:9545",
                            "chain_id": 31888,
                            "acl_address": "0x5fbdb2315678afecb367f032d93f642f64180aa3"
                        }
                    ]
                "#,
            );
            env::set_var(
                "KMS_CONNECTOR_KMS_CORE_ENDPOINTS",
                "http://localhost:50053,http://localhost:50054",
            );
            env::set_var("KMS_CONNECTOR_GRPC_REQUEST_RETRIES", "5");
            env::set_var("KMS_CONNECTOR_MAX_DECRYPTION_ATTEMPTS", "300");
            env::set_var("KMS_CONNECTOR_S3_CIPHERTEXT_RETRIEVAL_ATTEMPTS", "5");
            env::set_var("KMS_CONNECTOR_S3_CONNECT_TIMEOUT", "4s");
            env::set_var("KMS_CONNECTOR_S3_HEAD_TIMEOUT", "6s");
            env::set_var("KMS_CONNECTOR_S3_GET_TIMEOUT", "30s");
            env::set_var("KMS_CONNECTOR_S3_MAX_CONCURRENT_HEADS_PER_BUCKET", "32");
            env::set_var("KMS_CONNECTOR_S3_MAX_CONCURRENT_GETS", "8");
            env::set_var("KMS_CONNECTOR_S3_MAX_CIPHERTEXT_SIZE", "1048576");
            env::set_var("KMS_CONNECTOR_COPRO_REGISTRY_REFRESH", "90s");
            env::set_var("KMS_CONNECTOR_HOST_RPC_MAX_CONCURRENT_CALLS", "64");
            env::set_var("KMS_CONNECTOR_HOST_RPC_CALL_TIMEOUT", "15s");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
        }

        // Load config from environment
        let config = Config::from_env_and_file::<&str>(None).unwrap();

        // Verify values
        assert_eq!(config.events_batch_size, 15);
        assert_eq!(
            config.gateway_url,
            Url::from_str("http://localhost:9545").unwrap()
        );
        assert_eq!(config.gateway_chain_id, 31888);
        assert_eq!(
            config.ethereum_url,
            Url::from_str("http://localhost:9546").unwrap()
        );
        assert_eq!(config.ethereum_chain_id, 31444);
        assert_eq!(
            config.decryption_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.gateway_config_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.kms_generation_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.protocol_config_contract.address,
            Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap()
        );
        assert_eq!(
            config.host_chains,
            vec![HostChainConfig {
                url: Url::from_str("http://localhost:9545").unwrap(),
                chain_id: 31888,
                host: HostSettings::Evm {
                    acl_address: Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3")
                        .unwrap(),
                },
            }]
        );
        assert_eq!(
            config.kms_core_endpoints,
            vec!["http://localhost:50053", "http://localhost:50054"]
        );
        assert_eq!(config.grpc_request_retries, 5);
        assert_eq!(config.max_decryption_attempts, 300);
        assert_eq!(config.s3_ciphertext_retrieval_attempts, 5);
        assert_eq!(config.s3_connect_timeout.as_secs(), 4);
        assert_eq!(config.s3_head_timeout.as_secs(), 6);
        assert_eq!(config.s3_get_timeout.as_secs(), 30);
        assert_eq!(config.s3_max_concurrent_heads_per_bucket.get(), 32);
        assert_eq!(config.s3_max_concurrent_gets.get(), 8);
        assert_eq!(config.s3_max_ciphertext_size.get(), 1048576);
        assert_eq!(config.copro_registry_refresh.as_secs(), 90);
        assert_eq!(config.host_rpc_max_concurrent_calls.get(), 64);
        assert_eq!(config.host_rpc_call_timeout.as_secs(), 15);
        assert_eq!(config.service_name, "kms-connector-test");

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_zero_ceiling_is_rejected() {
        for ceiling in [
            "KMS_CONNECTOR_S3_MAX_CONCURRENT_HEADS_PER_BUCKET",
            "KMS_CONNECTOR_S3_MAX_CONCURRENT_GETS",
            "KMS_CONNECTOR_S3_MAX_CIPHERTEXT_SIZE",
            "KMS_CONNECTOR_HOST_RPC_MAX_CONCURRENT_CALLS",
        ] {
            cleanup_env_vars();
            unsafe { env::set_var(ceiling, "0") }

            let config = Config::from_env_and_file(Some(example_config_path()));
            assert!(config.is_err(), "{ceiling} = 0 should not have loaded");
        }

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_env_overrides_file() {
        cleanup_env_vars();
        let example_config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        // Set an environment variable to override the file
        let gateway_chain_id = 77737;
        let service_name = "kms-connector-override";
        let mut expected_config = example_config.clone();
        expected_config.gateway_chain_id = gateway_chain_id;
        expected_config.service_name = service_name.to_string();
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_GATEWAY_CHAIN_ID",
                gateway_chain_id.to_string(),
            );
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", service_name);
        }

        // Load config from both sources
        let config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        // Verify that environment variables take precedence
        assert_ne!(config.gateway_chain_id, example_config.gateway_chain_id);
        assert_ne!(config.service_name, example_config.service_name);
        assert_eq!(config, expected_config);

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_host_chains_from_env_camel_case() {
        cleanup_env_vars();

        // Set environment variables
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_HOST_CHAINS",
                r#"
                    [
                        {
                            "url": "http://localhost:9545",
                            "chainId": 31888,
                            "aclAddress": "0x5fbdb2315678afecb367f032d93f642f64180aa3"
                        }
                    ]
                "#,
            );
        }

        // Load config from environment
        let config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        // Verify values
        assert_eq!(
            config.host_chains,
            vec![HostChainConfig {
                url: Url::from_str("http://localhost:9545").unwrap(),
                chain_id: 31888,
                host: HostSettings::Evm {
                    acl_address: Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3")
                        .unwrap()
                },
            }]
        );
        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn a_solana_host_chain_from_env_camel_case() {
        cleanup_env_vars();
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_HOST_CHAINS",
                r#"
                    [
                        {
                            "url": "http://localhost:8899",
                            "chainId": 72057594037959824,
                            "solanaHostProgramId": "11111111111111111111111111111111",
                            "solanaProofRoutes": [
                                {"url": "http://coprocessor-1:8080", "apiKey": "first-key"}
                            ]
                        }
                    ]
                "#,
            );
        }

        let config = Config::from_env_and_file(Some(example_config_path())).unwrap();

        assert_eq!(
            config.host_chains,
            vec![HostChainConfig {
                url: Url::from_str("http://localhost:8899").unwrap(),
                // Type byte 0x01, cluster tag 31888.
                chain_id: solana_host_chain_id(31888),
                host: HostSettings::Solana(SolanaHostSettings {
                    host_program_id: Pubkey::new_from_array([0; 32]),
                    proof_routes: vec![ProofRoute {
                        url: Url::from_str("http://coprocessor-1:8080").unwrap(),
                        api_key: ApiKey::from("first-key".to_owned()),
                    }],
                }),
            }]
        );
        cleanup_env_vars();
    }

    fn parse_host_chains(entries: serde_json::Value) -> Result<Vec<HostChainConfig>, String> {
        deserialize_host_chains(entries).map_err(|error| error.to_string())
    }

    fn solana_entry(cluster_tag: u64) -> serde_json::Value {
        serde_json::json!({
            "url": "http://localhost:8899",
            "chain_id": solana_host_chain_id(cluster_tag),
            "solana_host_program_id": "11111111111111111111111111111111",
            "solana_proof_routes": [{"url": "http://coprocessor-1:8080", "api_key": "first-key"}]
        })
    }

    /// A host chain entry as main writes it: no kind, only the EVM fields.
    #[test]
    #[serial(config_tests)]
    fn an_evm_entry_from_main_parses_unchanged() {
        cleanup_env_vars();
        let chains = parse_host_chains(serde_json::json!([{
            "url": "http://localhost:8545",
            "chain_id": 12345,
            "acl_address": "0x5fbdb2315678afecb367f032d93f642f64180aa3"
        }]))
        .unwrap();
        assert!(matches!(chains[0].host, HostSettings::Evm { .. }));
    }

    /// The chain id's type byte names the kind, so an entry carrying the other kind's settings,
    /// missing its own, or naming no kind does not load.
    #[test]
    #[serial(config_tests)]
    fn an_entry_whose_settings_do_not_match_its_chain_id_is_refused() {
        cleanup_env_vars();
        let evm = serde_json::json!({
            "url": "http://localhost:8545",
            "chain_id": 12345,
            "acl_address": "0x5fbdb2315678afecb367f032d93f642f64180aa3"
        });
        let with = |mut entry: serde_json::Value, field: &str, value: serde_json::Value| {
            entry[field] = value;
            entry
        };
        let without = |mut entry: serde_json::Value, field: &str| {
            entry.as_object_mut().unwrap().remove(field);
            entry
        };
        let cases = [
            (without(evm.clone(), "acl_address"), "requires acl_address"),
            (
                with(
                    evm.clone(),
                    "solana_host_program_id",
                    "11111111111111111111111111111111".into(),
                ),
                "must not set solana_host_program_id or solana_proof_routes",
            ),
            (
                with(
                    evm.clone(),
                    "solana_proof_routes",
                    solana_entry(1)["solana_proof_routes"].clone(),
                ),
                "must not set solana_host_program_id or solana_proof_routes",
            ),
            (
                with(solana_entry(1), "acl_address", evm["acl_address"].clone()),
                "must not set acl_address",
            ),
            (
                without(solana_entry(1), "solana_host_program_id"),
                "requires solana_host_program_id",
            ),
            (
                without(solana_entry(1), "solana_proof_routes"),
                "requires at least one solana_proof_routes entry",
            ),
            (
                with(
                    solana_entry(1),
                    "solana_proof_routes",
                    serde_json::json!([{"url": "http://coprocessor-1:8080", "api_key": ""}]),
                ),
                "proof route http://coprocessor-1:8080/ has an empty api_key",
            ),
            (
                with(evm.clone(), "chain_id", 0x0200_0000_0000_0009_u64.into()),
                "has type byte 0x02, which names no host kind",
            ),
        ];
        for (entry, expected) in cases {
            let error = parse_host_chains(serde_json::json!([entry])).unwrap_err();
            assert!(
                error.contains(expected),
                "expected {expected:?}, got {error}"
            );
        }
    }

    #[test]
    #[serial(config_tests)]
    fn a_chain_configured_twice_is_refused() {
        cleanup_env_vars();
        let error =
            parse_host_chains(serde_json::json!([solana_entry(7), solana_entry(7)])).unwrap_err();
        assert!(
            error.contains(&format!(
                "Duplicate host chain in config for chain ID {}",
                solana_host_chain_id(7)
            )),
            "{error}"
        );
    }

    #[test]
    #[serial(config_tests)]
    fn a_proof_key_is_redacted_from_debug_output() {
        cleanup_env_vars();
        let chains = parse_host_chains(serde_json::json!([solana_entry(1)])).unwrap();
        let debug = format!("{chains:?}");
        assert!(!debug.contains("first-key"), "{debug}");
    }

    /// The sample's commented Solana fields as a real entry: routes are TOML inline tables.
    #[test]
    #[serial(config_tests)]
    fn a_solana_entry_loads_from_toml() {
        cleanup_env_vars();
        let sample = std::fs::read_to_string(example_config_path()).unwrap();
        let path = env::temp_dir().join(format!("kms-worker-solana-{}.toml", std::process::id()));
        let solana_chain_id = solana_host_chain_id(1);
        std::fs::write(
            &path,
            format!(
                r#"{sample}
[[host_chains]]
url = "http://localhost:8899"
chain_id = {solana_chain_id}
solana_host_program_id = "11111111111111111111111111111111"
solana_proof_routes = [
    {{ url = "http://coprocessor-1:8080", api_key = "first-key" }},
    {{ url = "http://coprocessor-2:8080", api_key = "second-key" }},
]
"#
            ),
        )
        .unwrap();
        let config = Config::from_env_and_file(Some(&path));
        std::fs::remove_file(&path).unwrap();

        let HostSettings::Solana(solana) = &config.unwrap().host_chains[1].host else {
            panic!("the second entry is a Solana chain");
        };
        let routes: Vec<_> = solana
            .proof_routes
            .iter()
            .map(|route| (route.url.as_str(), route.api_key.expose()))
            .collect();
        assert_eq!(
            routes,
            [
                ("http://coprocessor-1:8080/", "first-key"),
                ("http://coprocessor-2:8080/", "second-key"),
            ]
        );
    }

    fn example_config_path() -> String {
        format!(
            "{}/../../config/kms-worker.toml",
            env!("CARGO_MANIFEST_DIR")
        )
    }
}
