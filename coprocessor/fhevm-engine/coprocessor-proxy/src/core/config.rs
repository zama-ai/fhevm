use alloy::{primitives::Address, transports::http::reqwest::Url};
use clap::{
    Arg, Command, Parser,
    builder::TypedValueParser,
    error::{Error, ErrorKind},
};
use http::uri::Authority;
use std::{
    ffi::OsStr,
    net::SocketAddr,
    num::{NonZeroU32, NonZeroUsize},
    path::PathBuf,
    time::Duration,
};
use tracing::Level;

/// Configuration of the `Proxy` service, from command-line flags or environment variables.
#[derive(Parser)]
#[command(version, about = "Public HTTPS entry point of a coprocessor")]
pub struct Config {
    /// The public bind address of the proxy.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_BIND_ADDRESS",
        default_value = "0.0.0.0:8443"
    )]
    pub bind_address: SocketAddr,
    /// The PEM certificate chain the proxy serves.
    #[arg(long, env = "COPROCESSOR_PROXY_TLS_CERT_PATH")]
    pub tls_cert_path: PathBuf,
    /// The PEM private key of the certificate.
    #[arg(long, env = "COPROCESSOR_PROXY_TLS_KEY_PATH")]
    pub tls_key_path: PathBuf,

    /// The Ethereum chain id, part of the `FhevmSig` (RFC-038) EIP-712 domain.
    #[arg(long, env = "COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID")]
    pub ethereum_chain_id: u64,
    /// The `ProtocolConfig` contract on Ethereum: part of the `FhevmSig` EIP-712 domain, and the
    /// source of the accepted signers, the KMS tx-senders.
    #[arg(long, env = "COPROCESSOR_PROXY_PROTOCOL_CONFIG_ADDRESS")]
    pub protocol_config_address: Address,
    /// The Ethereum RPC the KMS tx-senders are read through. Never logged: it can carry an API key.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_ETHEREUM_RPC_URL",
        hide_env_values = true,
        value_parser = SecretUrlParser
    )]
    pub ethereum_rpc_url: Url,
    /// How often the latest finalized block is checked; the KMS tx-senders are read again when it
    /// moved forward. Unknown signers wake a poll; woken polls start at least 1 s apart. Each poll
    /// is bounded at 30 s.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_KMS_REGISTRY_REFRESH_INTERVAL",
        default_value = "5s",
        value_parser = parse_non_zero_duration
    )]
    pub kms_registry_refresh_interval: Duration,
    /// This coprocessor's registered signer address: the `audience` the accepted headers are
    /// signed for.
    #[arg(long, env = "COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS")]
    pub coprocessor_signer_address: Address,
    /// How many requests a signer can send to this proxy replica per second; more are answered
    /// `429`. A connector spreads its connections over the replicas, so its limit across them is
    /// about this times the replica count.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_MAX_REQUESTS_PER_SIGNER",
        default_value = "200"
    )]
    pub max_requests_per_signer: NonZeroU32,

    /// The `host:port` address of prf-service, resolved once at startup: its Kubernetes Service,
    /// which routes to the ready pods.
    #[arg(long, env = "COPROCESSOR_PROXY_PRF_SERVICE_ADDRESS")]
    pub prf_service_address: Authority,
    /// The timeout to establish a TCP connection to prf-service.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_PRF_SERVICE_CONNECT_TIMEOUT",
        default_value = "3s",
        value_parser = parse_non_zero_duration
    )]
    pub prf_service_connect_timeout: Duration,
    /// How long the proxy waits for a prf-service response before answering `504`. prf-service can
    /// hold a request until its result is ready, so this must exceed how long the callers wait. It
    /// is also the shutdown grace period, which in-flight requests get to complete.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_PRF_SERVICE_RESPONSE_TIMEOUT",
        default_value = "120s",
        value_parser = parse_non_zero_duration
    )]
    pub prf_service_response_timeout: Duration,
    /// How long an idle pooled prf-service connection is kept before being closed. Behind a
    /// Service, a new connection can reach newly scaled prf-service pods.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_PRF_SERVICE_IDLE_TIMEOUT",
        default_value = "10s",
        value_parser = parse_non_zero_duration
    )]
    pub prf_service_idle_timeout: Duration,

    /// How long the proxy keeps accepting connections after `SIGTERM`, closing each one after its
    /// next response, before the shutdown grace period starts. Behind a Kubernetes Service, it must
    /// exceed the time the pod's removal takes to reach every node.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_SHUTDOWN_DELAY",
        default_value = "0s",
        value_parser = humantime::parse_duration
    )]
    pub shutdown_delay: Duration,
    /// The worker threads of the proxy service. TLS handshakes, signature checks and forwarding
    /// all run on them, as does the readiness self-check's request.
    #[arg(long, env = "COPROCESSOR_PROXY_THREADS", default_value = "1")]
    pub threads: NonZeroUsize,

    /// The service name used for tracing.
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "coprocessor-proxy")]
    pub service_name: String,
    #[arg(long, env = "COPROCESSOR_PROXY_LOG_LEVEL", default_value_t = Level::INFO)]
    pub log_level: Level,
    /// The port of `/healthz`, `/liveness` and `/version`.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_HEALTHCHECK_PORT",
        default_value_t = 8080
    )]
    pub healthcheck_port: u16,
    /// The address of the Prometheus `/metrics` endpoint.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_METRICS_ENDPOINT",
        default_value = "0.0.0.0:9100"
    )]
    pub metrics_endpoint: SocketAddr,
    /// The timeout of the TLS listener's self-check.
    #[arg(
        long,
        env = "COPROCESSOR_PROXY_HEALTHCHECK_TIMEOUT",
        default_value = "3s",
        value_parser = parse_non_zero_duration
    )]
    pub healthcheck_timeout: Duration,
}

fn parse_non_zero_duration(value: &str) -> Result<Duration, String> {
    let duration = humantime::parse_duration(value).map_err(|e| e.to_string())?;
    if duration.is_zero() {
        return Err("duration must be greater than 0".to_string());
    }
    Ok(duration)
}

/// Parses a URL that can carry an API key: unlike clap's own errors, the error leaves the value
/// out.
#[derive(Clone)]
struct SecretUrlParser;

impl TypedValueParser for SecretUrlParser {
    type Value = Url;

    fn parse_ref(&self, cmd: &Command, arg: Option<&Arg>, value: &OsStr) -> Result<Url, Error> {
        let parsed = value
            .to_str()
            .ok_or_else(|| "not UTF-8".to_string())
            .and_then(|value| value.parse::<Url>().map_err(|e| e.to_string()));
        parsed.map_err(|reason| {
            let arg = arg.map(ToString::to_string).unwrap_or_default();
            Error::raw(
                ErrorKind::ValueValidation,
                format!("invalid URL for {arg}: {reason}\n"),
            )
            .with_cmd(cmd)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    /// The program name and the flags without a default, the prf-service address last.
    const REQUIRED: [&str; 8] = [
        "coprocessor-proxy",
        "--tls-cert-path=/etc/proxy/tls.crt",
        "--tls-key-path=/etc/proxy/tls.key",
        "--ethereum-chain-id=31444",
        "--protocol-config-address=0xc0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0",
        "--ethereum-rpc-url=https://rpc.example/v1/key",
        "--coprocessor-signer-address=0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1",
        "--prf-service-address=prf-service:8090",
    ];

    /// Parses [`REQUIRED`] and `extra`, whose flags replace those of [`REQUIRED`].
    fn parse(extra: &[&str]) -> Result<Config, Error> {
        let flag = |arg: &&str| arg.split('=').next().unwrap().to_string();
        let replaced: Vec<String> = extra.iter().map(flag).collect();
        let required = REQUIRED.iter().filter(|arg| !replaced.contains(&flag(arg)));
        Config::try_parse_from(required.chain(extra))
    }

    // Every test reads the environment, which `test_load_from_env` sets: they run one at a time.

    #[test]
    #[serial(config_tests)]
    fn test_defaults() {
        let config = parse(&[]).unwrap();
        assert_eq!(config.kms_registry_refresh_interval, Duration::from_secs(5));
        assert_eq!(config.prf_service_connect_timeout, Duration::from_secs(3));
        assert_eq!(
            config.prf_service_response_timeout,
            Duration::from_secs(120)
        );
        assert_eq!(config.prf_service_idle_timeout, Duration::from_secs(10));
        assert_eq!(config.shutdown_delay, Duration::ZERO);
        assert_eq!(config.threads.get(), 1);
        assert_eq!(config.max_requests_per_signer.get(), 200);
        assert_eq!(config.bind_address, "0.0.0.0:8443".parse().unwrap());
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_from_env() {
        let vars = [
            ("COPROCESSOR_PROXY_BIND_ADDRESS", "127.0.0.1:9443"),
            ("COPROCESSOR_PROXY_TLS_CERT_PATH", "/etc/proxy/tls.crt"),
            ("COPROCESSOR_PROXY_TLS_KEY_PATH", "/etc/proxy/tls.key"),
            ("COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID", "31444"),
            (
                "COPROCESSOR_PROXY_PROTOCOL_CONFIG_ADDRESS",
                "0xc0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0",
            ),
            (
                "COPROCESSOR_PROXY_ETHEREUM_RPC_URL",
                "https://rpc.example/v1/key",
            ),
            ("COPROCESSOR_PROXY_KMS_REGISTRY_REFRESH_INTERVAL", "2m"),
            (
                "COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS",
                "0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1",
            ),
            ("COPROCESSOR_PROXY_MAX_REQUESTS_PER_SIGNER", "50"),
            (
                "COPROCESSOR_PROXY_PRF_SERVICE_ADDRESS",
                "prf-service-1:8090",
            ),
            ("COPROCESSOR_PROXY_PRF_SERVICE_CONNECT_TIMEOUT", "1s"),
            ("COPROCESSOR_PROXY_PRF_SERVICE_RESPONSE_TIMEOUT", "45s"),
            ("COPROCESSOR_PROXY_PRF_SERVICE_IDLE_TIMEOUT", "7s"),
            ("COPROCESSOR_PROXY_SHUTDOWN_DELAY", "9s"),
            ("COPROCESSOR_PROXY_THREADS", "4"),
            ("OTEL_SERVICE_NAME", "coprocessor-proxy-test"),
            ("COPROCESSOR_PROXY_LOG_LEVEL", "debug"),
            ("COPROCESSOR_PROXY_HEALTHCHECK_PORT", "8081"),
            ("COPROCESSOR_PROXY_METRICS_ENDPOINT", "127.0.0.1:9101"),
            ("COPROCESSOR_PROXY_HEALTHCHECK_TIMEOUT", "7s"),
        ];
        for (name, value) in vars {
            unsafe { env::set_var(name, value) };
        }
        let config = Config::try_parse_from(["coprocessor-proxy"]);
        for (name, _) in vars {
            unsafe { env::remove_var(name) };
        }
        let config = config.unwrap();

        assert_eq!(config.bind_address, "127.0.0.1:9443".parse().unwrap());
        assert_eq!(config.tls_cert_path, PathBuf::from("/etc/proxy/tls.crt"));
        assert_eq!(config.tls_key_path, PathBuf::from("/etc/proxy/tls.key"));
        assert_eq!(config.ethereum_chain_id, 31444);
        assert_eq!(config.protocol_config_address, Address::repeat_byte(0xc0));
        assert_eq!(
            config.ethereum_rpc_url,
            "https://rpc.example/v1/key".parse().unwrap()
        );
        assert_eq!(
            config.kms_registry_refresh_interval,
            Duration::from_secs(120)
        );
        assert_eq!(
            config.coprocessor_signer_address,
            Address::repeat_byte(0xa1)
        );
        assert_eq!(config.max_requests_per_signer.get(), 50);
        assert_eq!(config.prf_service_address, "prf-service-1:8090");
        assert_eq!(config.prf_service_connect_timeout, Duration::from_secs(1));
        assert_eq!(config.prf_service_response_timeout, Duration::from_secs(45));
        assert_eq!(config.prf_service_idle_timeout, Duration::from_secs(7));
        assert_eq!(config.shutdown_delay, Duration::from_secs(9));
        assert_eq!(config.threads.get(), 4);
        assert_eq!(config.service_name, "coprocessor-proxy-test");
        assert_eq!(config.log_level, Level::DEBUG);
        assert_eq!(config.healthcheck_port, 8081);
        assert_eq!(config.metrics_endpoint, "127.0.0.1:9101".parse().unwrap());
        assert_eq!(config.healthcheck_timeout, Duration::from_secs(7));
    }

    #[test]
    #[serial(config_tests)]
    fn test_missing_required_field_fails() {
        let Err(err) = Config::try_parse_from(&REQUIRED[..7]) else {
            panic!("a missing prf-service address must be refused");
        };
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    #[serial(config_tests)]
    fn test_invalid_rpc_url_error_leaves_the_url_out() {
        // No scheme: a relative URL, refused.
        let Err(err) = parse(&["--ethereum-rpc-url=eth.example/v2/SECRET-KEY"]) else {
            panic!("a relative RPC URL must be refused");
        };
        let message = err.to_string();
        assert!(message.contains("invalid URL"), "{message}");
        assert!(!message.contains("SECRET-KEY"), "{message}");
    }

    #[test]
    #[serial(config_tests)]
    fn test_zero_refresh_interval_is_refused() {
        // The reader would poll the RPC without pause.
        let Err(err) = parse(&["--kms-registry-refresh-interval=0s"]) else {
            panic!("a zero refresh interval must be refused");
        };
        let message = err.to_string();
        assert!(message.contains("greater than 0"), "{message}");
    }

    #[test]
    #[serial(config_tests)]
    fn test_zero_max_requests_per_signer_is_refused() {
        // Every authenticated request would be refused.
        let Err(err) = parse(&["--max-requests-per-signer=0"]) else {
            panic!("a zero request limit must be refused");
        };
        assert_eq!(err.kind(), ErrorKind::ValueValidation);
    }
}
