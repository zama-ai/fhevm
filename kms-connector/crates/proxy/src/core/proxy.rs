use crate::{core::Config, monitoring::health};
use fhevm_sig_auth::AuthError;
use fhevm_sig_proxy::{
    ErrorBody, ProxyConfig, ProxyError, Route, SignerSource, TlsConfig, Upstream,
};
use http::{Method, StatusCode};
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
};

/// The `Proxy` service: sender authentication and load-balancing in front of the endpoints.
pub struct Proxy(fhevm_sig_proxy::Proxy);

impl Proxy {
    /// Creates the `Proxy` from its configuration, and the `health::State` used to monitor it.
    pub fn from_config(config: Config) -> anyhow::Result<(Self, health::State)> {
        let (proxy, state) = fhevm_sig_proxy::Proxy::from_config(proxy_config(config))?;
        Ok((Self(proxy), health::State::new(state)))
    }

    /// Runs the proxy until it receives `SIGTERM`, `SIGINT` or `SIGQUIT`.
    pub fn run(self) -> anyhow::Result<()> {
        self.0.run()
    }
}

/// The upstream of the connector's endpoints.
const ENDPOINT_UPSTREAM: &str = "endpoint";

/// The connector's interface: its three routes, all served by the endpoints.
fn proxy_config(config: Config) -> ProxyConfig {
    let routes = [
        (Method::POST, PUBLIC_DECRYPTION_ROUTE),
        (Method::POST, USER_DECRYPTION_ROUTE),
        (Method::GET, VERSION_ROUTE),
    ]
    .map(|(method, path)| Route {
        methods: vec![method],
        path: path.to_owned(),
        upstream: ENDPOINT_UPSTREAM.to_owned(),
        max_body_bytes: config.max_body_bytes,
        endpoint_response_timeout: config.endpoint_response_timeout,
    })
    .to_vec();
    ProxyConfig {
        bind_address: config.bind_address,
        tls_config: TlsConfig {
            cert_path: config.tls_config.cert_path,
            key_path: config.tls_config.key_path,
        },
        ethereum_chain_id: config.ethereum_chain_id,
        protocol_config_address: config.protocol_config_contract.address,
        signers: SignerSource::Static(config.allowed_senders),
        audience: config.tx_sender_address,
        error_body,
        forward_authorization: false,
        routes,
        upstreams: vec![Upstream {
            name: ENDPOINT_UPSTREAM.to_owned(),
            endpoint_addresses: config.endpoint_addresses,
            healthcheck_path: VERSION_ROUTE.to_owned(),
        }],
        endpoint_connect_timeout: config.endpoint_connect_timeout,
        endpoint_healthcheck_frequency: config.endpoint_healthcheck_frequency,
        endpoint_idle_timeout: config.endpoint_idle_timeout,
        request_read_timeout: config.request_read_timeout,
        shutdown_grace_period: config.shutdown_grace_period,
        service_name: config.service_name,
        healthcheck_timeout: config.healthcheck_timeout,
    }
}

/// Renders the errors the proxy answers itself as a kms-connector-api `ErrorResponse`.
fn error_body(error: &ProxyError) -> ErrorBody {
    let response = match error {
        ProxyError::NotOriginForm | ProxyError::NonCanonicalPath | ProxyError::BodyTooLarge => {
            ErrorResponse::new(ErrorCode::Malformed, error.to_string(), None)
        }
        ProxyError::Auth(error) => auth_error_response(error),
        ProxyError::EndpointUnavailable => {
            ErrorResponse::new(ErrorCode::UpstreamTransient, error.to_string(), None)
        }
        ProxyError::EndpointTimeout => {
            ErrorResponse::new(ErrorCode::Timeout, error.to_string(), None)
        }
    };
    ErrorBody {
        status: StatusCode::from_u16(response.code.http_status())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        json: serde_json::to_vec(&response).unwrap_or_default(),
    }
}

/// Converts an `AuthError` into an `ErrorResponse`.
fn auth_error_response(error: &AuthError) -> ErrorResponse {
    let code = if matches!(error, AuthError::Expired) {
        ErrorCode::AuthExpired
    } else {
        ErrorCode::SenderAuthenticationFailed
    };
    ErrorResponse::new(code, format!("{}: {error}", error.code()), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_expired_auth_rejection_is_retryable() {
        let expired = auth_error_response(&AuthError::Expired);
        assert_eq!(expired.code, ErrorCode::AuthExpired);
        assert!(expired.retryable);

        let missing = auth_error_response(&AuthError::Missing);
        assert_eq!(missing.code, ErrorCode::SenderAuthenticationFailed);
        assert!(!missing.retryable);
        assert!(missing.message.starts_with("auth_missing: "));
    }
}
