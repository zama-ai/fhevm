//! Safe diagnostics for Gateway failures. Error values remain intact for retry
//! classification; only their representation in logs, health output and DB
//! columns is restricted. Upstream bodies/messages can echo credentials even
//! without a URL, so redacting URL-shaped substrings alone is insufficient.
use alloy::transports::{TransportError, TransportErrorKind};
use std::error::Error;

pub fn safe_error(error: &(dyn Error + 'static)) -> String {
    let mut cause = Some(error);
    while let Some(inner) = cause {
        if let Some(rpc) = inner.downcast_ref::<TransportError>() {
            return safe_rpc_error(rpc);
        }
        if inner.is::<alloy::transports::http::reqwest::Error>() {
            return "Gateway HTTP request failed".into();
        }
        cause = inner.source();
    }
    // Non-Gateway errors (e.g. database availability) retain their diagnostics.
    error.to_string()
}

pub fn safe_rpc_error(error: &TransportError) -> String {
    match error {
        TransportError::ErrorResp(payload) => {
            // Preserve only known messages, never arbitrary response data.
            let message = match payload.message.trim().to_ascii_lowercase().as_str() {
                "context deadline exceeded" => "context deadline exceeded",
                "service unavailable" => "service unavailable",
                "temporarily unavailable" => "temporarily unavailable",
                "too many requests" => "too many requests",
                "nonce too low" => "nonce too low",
                "nonce too high" => "nonce too high",
                "replacement transaction underpriced" => "replacement transaction underpriced",
                "insufficient funds" => "insufficient funds",
                "execution reverted" => "execution reverted",
                _ => "response details omitted",
            };
            format!("Gateway JSON-RPC error {}: {message}", payload.code)
        }
        TransportError::Transport(TransportErrorKind::HttpError(error)) => {
            format!(
                "Gateway HTTP error {} (response body omitted)",
                error.status
            )
        }
        TransportError::Transport(TransportErrorKind::BackendGone) => "Gateway backend gone".into(),
        TransportError::Transport(TransportErrorKind::MissingBatchResponse(_)) => {
            "Gateway missing batch response".into()
        }
        TransportError::Transport(TransportErrorKind::Custom(error)) => {
            if error.to_string() == "eth_sendRawTransactionSync timeout" {
                return "eth_sendRawTransactionSync timeout".into();
            }
            let mut cause: Option<&(dyn Error + 'static)> = Some(error.as_ref());
            while let Some(inner) = cause {
                if let Some(http) = inner.downcast_ref::<alloy::transports::http::reqwest::Error>()
                {
                    return if http.is_timeout() {
                        "Gateway HTTP request timed out"
                    } else if http.is_connect() {
                        "Gateway HTTP connection failed"
                    } else {
                        "Gateway HTTP request failed"
                    }
                    .into();
                }
                cause = inner.source();
            }
            "Gateway transport error (details omitted)".into()
        }
        TransportError::DeserError { .. } => "Gateway invalid JSON response (body omitted)".into(),
        TransportError::SerError(_) => "Gateway request serialization failed".into(),
        TransportError::NullResp => "Gateway returned an unexpected null response".into(),
        TransportError::LocalUsageError(_) => {
            "Gateway local signing or request preparation failed".into()
        }
        _ => "Gateway RPC failure (details omitted)".into(),
    }
}

/// These dependencies emit full RPC URLs, response bodies, or request details
/// independently of our error formatting. Disable their events AND spans in
/// both JSON logs and OTLP, including when the sender runs at DEBUG/TRACE.
pub fn gateway_tracing_filter(metadata: &tracing::Metadata<'_>) -> bool {
    let target = metadata.target();
    ![
        "alloy",
        "reqwest",
        "hyper",
        "rustls",
        "h2",
        "tower_http",
        "tungstenite",
        "tokio_tungstenite",
    ]
    .iter()
    .any(|prefix| target.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_omit_urls_bare_secrets_and_anyhow_contexts() {
        let secret = "SECRET_PATH_QUERY_USER_PASSWORD";
        let url = format!("https://user:{secret}@gateway.invalid/{secret}?key={secret}");
        let errors = [
            TransportErrorKind::http_error(400, format!("{url} bare secret: {secret}")),
            TransportErrorKind::custom_str(&url),
            TransportError::ErrorResp(
                serde_json::from_value(serde_json::json!({
                    "code": -32000, "message": url, "data": secret
                }))
                .unwrap(),
            ),
            TransportError::DeserError {
                err: serde_json::from_str::<serde_json::Value>("<html>").unwrap_err(),
                text: secret.into(),
            },
            TransportError::local_usage_str(secret),
        ];
        for error in errors {
            let wrapped = anyhow::Error::new(error).context(format!("{url} {secret}"));
            let diagnostic = safe_error(wrapped.as_ref());
            assert!(!diagnostic.contains(secret));
            assert!(!diagnostic.contains("gateway.invalid"));
            assert!(!diagnostic.contains("https://"));
            // Formatting must not replace the original typed error used by
            // retry and contract-error classification.
            assert!(wrapped.downcast_ref::<TransportError>().is_some());
        }
    }

    #[tokio::test]
    async fn actual_http_error_does_not_expose_credential_bearing_url() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!(
            "http://USER_SECRET:PASS_SECRET@{}/PATH_SECRET?token=QUERY_SECRET",
            listener.local_addr().unwrap()
        );
        drop(listener);
        let client = alloy::transports::http::reqwest::Client::builder()
            .no_proxy()
            .build()
            .unwrap();
        let error = client.get(url).send().await.unwrap_err();
        assert!(error.is_connect());
        let error = TransportErrorKind::custom(error);
        assert_eq!(safe_rpc_error(&error), "Gateway HTTP connection failed");
        assert!(error
            .as_transport_err()
            .unwrap()
            .as_custom()
            .unwrap()
            .downcast_ref::<alloy::transports::http::reqwest::Error>()
            .unwrap()
            .is_connect());
    }
}
