//! The `fhevm-sig-proxy` conformance suite, run against the connector's `proxy` binary
//! configured through its environment variables.

use fhevm_sig_proxy::conformance::{
    AUDIENCE, CHAIN_ID, ENDPOINT_HEALTHCHECK_FREQUENCY, ENDPOINT_RESPONSE_TIMEOUT, Launch,
    MetricsPort, PROTOCOL_CONFIG, Profile, SignedRoute, sender,
};
use http::Method;
use kms_connector_api::PUBLIC_DECRYPTION_ROUTE;
use std::process::Command;

const PROFILE: Profile = Profile {
    binary: env!("CARGO_BIN_EXE_proxy"),
    configure,
    route: SignedRoute {
        method: Method::POST,
        path: PUBLIC_DECRYPTION_ROUTE,
        body: br#"{"ciphertext_handles":["0x01"]}"#,
        template: PUBLIC_DECRYPTION_ROUTE,
        other_method: Method::GET,
        methods: &[Method::POST],
        non_canonical_path: None,
    },
    metrics: MetricsPort::Health,
    forwards_authorization: false,
    error_body: proxy::core::error_body,
};

fn configure(launch: &Launch, command: &mut Command) {
    let ms = |duration: std::time::Duration| format!("{}ms", duration.as_millis());
    command.envs([
        ("KMS_CONNECTOR_PROXY_BIND_ADDRESS", launch.bind.to_string()),
        (
            "KMS_CONNECTOR_TLS_CONFIG__CERT_PATH",
            launch.tls.cert_path.display().to_string(),
        ),
        (
            "KMS_CONNECTOR_TLS_CONFIG__KEY_PATH",
            launch.tls.key_path.display().to_string(),
        ),
        ("KMS_CONNECTOR_ETHEREUM_CHAIN_ID", CHAIN_ID.to_string()),
        (
            "KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS",
            PROTOCOL_CONFIG.to_string(),
        ),
        (
            "KMS_CONNECTOR_ALLOWED_SENDERS",
            sender().address().to_string(),
        ),
        ("KMS_CONNECTOR_TX_SENDER_ADDRESS", AUDIENCE.to_string()),
        (
            "KMS_CONNECTOR_ENDPOINT_ADDRESSES",
            launch.endpoint.to_string(),
        ),
        (
            "KMS_CONNECTOR_ENDPOINT_RESPONSE_TIMEOUT",
            ms(ENDPOINT_RESPONSE_TIMEOUT),
        ),
        (
            "KMS_CONNECTOR_ENDPOINT_HEALTHCHECK_FREQUENCY",
            ms(ENDPOINT_HEALTHCHECK_FREQUENCY),
        ),
        (
            "KMS_CONNECTOR_MONITORING_ENDPOINT",
            launch.health.to_string(),
        ),
    ]);
}

fhevm_sig_proxy::conformance_tests!(PROFILE);
