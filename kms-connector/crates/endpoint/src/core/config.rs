use connector_utils::{
    config::{DeserializeConfig, default_database_pool_size},
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
    tasks::default_task_limit,
};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

/// Configuration of the `Endpoint` service.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(Serialize))]
pub struct Config {
    /// The URL of the Postgres database.
    pub database_url: String,
    /// The size of the database connection pool.
    #[serde(default = "default_database_pool_size")]
    pub database_pool_size: u32,

    /// The public bind address of the `v1` HTTP interface.
    #[serde(default = "default_http_endpoint")]
    pub http_endpoint: SocketAddr,
    /// The maximum number of decryption requests held in-flight by this endpoint.
    #[serde(default = "default_max_in_flight_decryptions")]
    pub max_in_flight_decryptions: usize,
    /// How long a decryption request waits for its response before answering `504 timeout`.
    #[serde(with = "humantime_serde", default = "default_decryption_timeout")]
    pub decryption_timeout: Duration,
    /// The maximum accepted size of a JSON request body.
    #[serde(default = "default_max_body_bytes")]
    pub max_body_bytes: usize,
    /// The maximum total bit size of the handles of a single decryption request. Mirrors
    /// `MAX_DECRYPTION_REQUEST_BITS` of the `Decryption` gateway contract.
    #[serde(default = "default_max_decryption_request_bits")]
    pub max_decryption_request_bits: u64,
    /// The maximum number of `allowedContracts` of a user decryption request. Mirrors
    /// `MAX_USER_DECRYPT_CONTRACT_ADDRESSES` of the `Decryption` gateway contract.
    #[serde(default = "default_max_allowed_contracts")]
    pub max_allowed_contracts: usize,
    /// The host chain ids accepted in ciphertext handles.
    pub supported_chain_ids: Vec<u64>,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The maximum number of tasks that can be executed concurrently.
    #[serde(default = "default_task_limit")]
    pub task_limit: usize,
    /// The monitoring server endpoint of the `Endpoint` service.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(with = "humantime_serde", default = "default_healthcheck_timeout")]
    pub healthcheck_timeout: Duration,
}

impl DeserializeConfig for Config {}

fn default_service_name() -> String {
    "kms-connector-endpoint".to_string()
}

pub fn default_http_endpoint() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080)
}

fn default_max_in_flight_decryptions() -> usize {
    1000
}

fn default_decryption_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_max_body_bytes() -> usize {
    1024 * 1024 // 1 MiB
}

fn default_max_decryption_request_bits() -> u64 {
    2048
}

fn default_max_allowed_contracts() -> usize {
    10
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost/kms-connector".to_string(),
            database_pool_size: default_database_pool_size(),
            http_endpoint: default_http_endpoint(),
            max_in_flight_decryptions: default_max_in_flight_decryptions(),
            decryption_timeout: default_decryption_timeout(),
            max_body_bytes: default_max_body_bytes(),
            max_decryption_request_bits: default_max_decryption_request_bits(),
            max_allowed_contracts: default_max_allowed_contracts(),
            supported_chain_ids: vec![11155111],
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
    use serial_test::serial;
    use std::{env, path::PathBuf};

    fn example_config_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/endpoint.toml")
    }

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_DATABASE_URL");
            env::remove_var("KMS_CONNECTOR_DATABASE_POOL_SIZE");
            env::remove_var("KMS_CONNECTOR_HTTP_ENDPOINT");
            env::remove_var("KMS_CONNECTOR_MAX_IN_FLIGHT_DECRYPTIONS");
            env::remove_var("KMS_CONNECTOR_DECRYPTION_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_MAX_BODY_BYTES");
            env::remove_var("KMS_CONNECTOR_MAX_DECRYPTION_REQUEST_BITS");
            env::remove_var("KMS_CONNECTOR_MAX_ALLOWED_CONTRACTS");
            env::remove_var("KMS_CONNECTOR_SUPPORTED_CHAIN_IDS");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_TASK_LIMIT");
            env::remove_var("KMS_CONNECTOR_MONITORING_ENDPOINT");
            env::remove_var("KMS_CONNECTOR_HEALTHCHECK_TIMEOUT");
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
        unsafe {
            env::set_var(
                "KMS_CONNECTOR_DATABASE_URL",
                "postgres://postgres:postgres@localhost",
            );
            env::set_var("KMS_CONNECTOR_DATABASE_POOL_SIZE", "4");
            env::set_var("KMS_CONNECTOR_HTTP_ENDPOINT", "127.0.0.1:9090");
            env::set_var("KMS_CONNECTOR_MAX_IN_FLIGHT_DECRYPTIONS", "12");
            env::set_var("KMS_CONNECTOR_DECRYPTION_TIMEOUT", "15s");
            env::set_var("KMS_CONNECTOR_MAX_BODY_BYTES", "2048");
            env::set_var("KMS_CONNECTOR_MAX_DECRYPTION_REQUEST_BITS", "512");
            env::set_var("KMS_CONNECTOR_MAX_ALLOWED_CONTRACTS", "3");
            env::set_var("KMS_CONNECTOR_SUPPORTED_CHAIN_IDS", "1,31337");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
            env::set_var("KMS_CONNECTOR_TASK_LIMIT", "42");
            env::set_var("KMS_CONNECTOR_MONITORING_ENDPOINT", "127.0.0.1:9101");
            env::set_var("KMS_CONNECTOR_HEALTHCHECK_TIMEOUT", "7s");
        }

        let config = Config::from_env_and_file::<&str>(None).unwrap();

        assert_eq!(
            config.database_url,
            "postgres://postgres:postgres@localhost"
        );
        assert_eq!(config.database_pool_size, 4);
        assert_eq!(config.http_endpoint, "127.0.0.1:9090".parse().unwrap());
        assert_eq!(config.max_in_flight_decryptions, 12);
        assert_eq!(config.decryption_timeout, Duration::from_secs(15));
        assert_eq!(config.max_body_bytes, 2048);
        assert_eq!(config.max_decryption_request_bits, 512);
        assert_eq!(config.max_allowed_contracts, 3);
        assert_eq!(config.supported_chain_ids, vec![1, 31337]);
        assert_eq!(config.service_name, "kms-connector-test");
        assert_eq!(config.task_limit, 42);
        assert_eq!(
            config.monitoring_endpoint,
            "127.0.0.1:9101".parse().unwrap()
        );
        assert_eq!(config.healthcheck_timeout, Duration::from_secs(7));

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_missing_required_field_fails() {
        cleanup_env_vars();
        let toml = r#"supported_chain_ids = [1]"#;
        let tmp = std::env::temp_dir().join(format!("endpoint-cfg-{}.toml", std::process::id()));
        std::fs::write(&tmp, toml).unwrap();
        assert!(Config::from_env_and_file(Some(&tmp)).is_err());
        std::fs::remove_file(&tmp).ok();
    }
}
