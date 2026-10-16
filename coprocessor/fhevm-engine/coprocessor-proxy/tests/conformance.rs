//! The `fhevm-sig-proxy` conformance suite, run against the `coprocessor-proxy` binary started
//! with the example configuration.

use fhevm_sig_proxy::conformance::{
    Launch, MetricsPort, Profile, SignedRoute, AUDIENCE, CHAIN_ID, ENDPOINT_HEALTHCHECK_FREQUENCY,
    ENDPOINT_RESPONSE_TIMEOUT, PROTOCOL_CONFIG, SIGNER_REFRESH_INTERVAL,
};
use http::Method;
use std::process::Command;

const PROFILE: Profile = Profile {
    binary: env!("CARGO_BIN_EXE_coprocessor-proxy"),
    configure,
    route: SignedRoute {
        method: Method::GET,
        path:
            "/v1/prf/1/labels/abababababababababababababababababababababababababababababababab/32",
        body: b"",
        template: "/v1/prf/{prf:u16}/labels/{label:hex32}/{padLength:u16}",
        other_method: Method::POST,
        allow: "GET, HEAD",
        non_canonical_path: Some(
            "/v1/prf/01/labels/abababababababababababababababababababababababababababababababab/32",
        ),
    },
    metrics: MetricsPort::Own,
    forwards_authorization: true,
    error_body: coprocessor_proxy::error::error_body,
};

/// The example configuration with the stub endpoint and a short response timeout, and the rest
/// of the launch in the environment.
fn configure(launch: &Launch, command: &mut Command) {
    let example = include_str!("../config/proxy.toml");
    let timeout = format!("\"{}ms\"", ENDPOINT_RESPONSE_TIMEOUT.as_millis());
    let config = [
        (
            "[\"prf-service:8080\"]",
            format!("[\"{}\"]", launch.endpoint),
        ),
        ("\"120s\"", timeout),
    ]
    .into_iter()
    .fold(example.to_owned(), |config, (from, to)| {
        assert!(config.contains(from), "{from}");
        config.replace(from, &to)
    });
    let path = launch.tls.dir().join("proxy.toml");
    std::fs::write(&path, config).unwrap();

    let ms = |duration: std::time::Duration| format!("{}ms", duration.as_millis());
    command.arg("--config").arg(path).envs([
        ("COPROCESSOR_PROXY_BIND_ADDRESS", launch.bind.to_string()),
        (
            "COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH",
            launch.tls.cert_path.display().to_string(),
        ),
        (
            "COPROCESSOR_PROXY_TLS_CONFIG__KEY_PATH",
            launch.tls.key_path.display().to_string(),
        ),
        ("COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID", CHAIN_ID.to_string()),
        (
            "COPROCESSOR_PROXY_ETHEREUM_RPC_URL",
            launch.rpc_url.to_string(),
        ),
        (
            "COPROCESSOR_PROXY_PROTOCOL_CONFIG_CONTRACT__ADDRESS",
            PROTOCOL_CONFIG.to_string(),
        ),
        (
            "COPROCESSOR_PROXY_SIGNER_REFRESH_INTERVAL",
            ms(SIGNER_REFRESH_INTERVAL),
        ),
        (
            "COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS",
            AUDIENCE.to_string(),
        ),
        (
            "COPROCESSOR_PROXY_ENDPOINT_HEALTHCHECK_FREQUENCY",
            ms(ENDPOINT_HEALTHCHECK_FREQUENCY),
        ),
        (
            "COPROCESSOR_PROXY_HEALTHCHECK_PORT",
            launch.health.port().to_string(),
        ),
        (
            "COPROCESSOR_PROXY_METRICS_ENDPOINT",
            launch.metrics.to_string(),
        ),
    ]);
}

fhevm_sig_proxy::conformance_tests!(PROFILE);
fhevm_sig_proxy::protocol_config_signer_tests!(PROFILE);
