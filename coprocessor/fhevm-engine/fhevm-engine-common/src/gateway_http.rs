use std::time::Duration;

use alloy::rpc::client::RpcClient;
use alloy::transports::http::reqwest::Url;
use alloy::transports::layers::{RateLimitRetryPolicy, RetryBackoffLayer};
use alloy::transports::{TransportError, TransportErrorKind};

pub const DEFAULT_GATEWAY_HTTP_MAX_RETRIES: u32 = 10;
pub const DEFAULT_GATEWAY_HTTP_RETRY_INTERVAL: Duration = Duration::from_secs(4);
pub const DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT_SECS: u16 = 70;
pub const DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT: Duration =
    Duration::from_secs(DEFAULT_GATEWAY_HTTP_REQUEST_TIMEOUT_SECS as u64);
const ALLOY_RETRY_EXHAUSTED_ERROR_PREFIX: &str = "Max retries exceeded ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayTransportErrorClass {
    Retryable,
    ProviderExhausted,
    Fatal,
}

pub fn gateway_http_client(url: &Url, max_retries: u32, retry_interval: Duration) -> RpcClient {
    let retry_interval_ms = retry_interval.as_millis();
    let retry_interval_ms = u64::try_from(retry_interval_ms).unwrap_or(u64::MAX);
    let retry_policy = RateLimitRetryPolicy::default().or(|err| {
        matches!(err, TransportError::Transport(inner) if is_gateway_retryable_transport_error(inner))
    });
    let retry_layer =
        RetryBackoffLayer::new_with_policy(max_retries, retry_interval_ms, 100, retry_policy);
    RpcClient::builder().layer(retry_layer).http(url.clone())
}

pub fn gateway_http_retry_budget(
    max_retries: u32,
    retry_interval: Duration,
) -> anyhow::Result<Duration> {
    retry_interval
        .checked_mul(max_retries)
        .ok_or_else(|| anyhow::anyhow!("gateway HTTP retry budget overflows Duration"))
}

pub fn validate_gateway_http_timeout(
    timeout_name: &str,
    timeout: Duration,
    max_retries: u32,
    retry_interval: Duration,
) -> anyhow::Result<()> {
    let retry_budget = gateway_http_retry_budget(max_retries, retry_interval)?;
    if timeout <= retry_budget {
        anyhow::bail!(
            "{timeout_name} ({timeout:?}) must be greater than the Gateway HTTP retry budget ({retry_budget:?}: provider_max_retries={max_retries}, provider_retry_interval={retry_interval:?})"
        );
    }
    Ok(())
}

pub fn classify_gateway_transport_error(inner: &TransportErrorKind) -> GatewayTransportErrorClass {
    if is_gateway_provider_exhausted_transport_error_kind(inner) {
        return GatewayTransportErrorClass::ProviderExhausted;
    }
    if is_gateway_retryable_transport_error_kind(inner) {
        return GatewayTransportErrorClass::Retryable;
    }
    GatewayTransportErrorClass::Fatal
}

pub fn is_gateway_provider_exhausted_transport_error(inner: &TransportErrorKind) -> bool {
    classify_gateway_transport_error(inner) == GatewayTransportErrorClass::ProviderExhausted
}

pub fn is_gateway_retryable_transport_error(inner: &TransportErrorKind) -> bool {
    classify_gateway_transport_error(inner) == GatewayTransportErrorClass::Retryable
}

pub fn is_gateway_transient_transport_error(inner: &TransportErrorKind) -> bool {
    matches!(
        classify_gateway_transport_error(inner),
        GatewayTransportErrorClass::Retryable | GatewayTransportErrorClass::ProviderExhausted
    )
}

fn is_gateway_provider_exhausted_transport_error_kind(inner: &TransportErrorKind) -> bool {
    matches!(inner, TransportErrorKind::BackendGone)
        || matches!(
            inner,
            TransportErrorKind::Custom(err)
                // Alloy's RetryBackoffLayer returns a Custom error once its
                // retry budget is exhausted: `Max retries exceeded {err}`.
                if err.to_string().starts_with(ALLOY_RETRY_EXHAUSTED_ERROR_PREFIX)
        )
}

fn is_gateway_retryable_transport_error_kind(inner: &TransportErrorKind) -> bool {
    // Keep Alloy's explicit retry hints, such as HTTP 429 and its custom
    // `429 Too Many Requests` form, but do not retry arbitrary Custom errors.
    inner.is_retry_err()
        || matches!(
            inner,
            TransportErrorKind::HttpError(http_error)
                // Retry gateway/load-balancer transient failures:
                // - 502 Bad Gateway: proxy could not reach a healthy upstream
                // - 503 Service Unavailable: upstream/proxy is temporarily unavailable
                // - 504 Gateway Timeout: proxy timed out waiting for upstream
                //
                // Do not retry client/configuration errors:
                // - 400 Bad Request: malformed request
                // - 401 Unauthorized: invalid/missing auth
                // - 403 Forbidden: forbidden by the provider/proxy/auth layer
                // - 404 Not Found: wrong endpoint/path
                //
                // Keep 500 Internal Server Error out of the gateway policy for now:
                // unlike 502/503/504, it can be an application-level JSON-RPC
                // failure from the gateway service itself and should not be
                // hidden as an infrastructure outage without a stronger signal.
                if matches!(http_error.status, 502..=504)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::transports::TransportFut;
    use alloy_json_rpc::{Id, Request, RequestPacket, ResponsePacket, SerializedRequest};
    use std::task::{Context, Poll};
    use tower::{Layer, Service};

    #[derive(Clone)]
    struct TransientGatewayErrorTransport;

    impl Service<RequestPacket> for TransientGatewayErrorTransport {
        type Response = ResponsePacket;
        type Error = TransportError;
        type Future = TransportFut<'static>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: RequestPacket) -> Self::Future {
            Box::pin(async { Err(TransportErrorKind::http_error(502, String::new())) })
        }
    }

    #[test]
    fn gateway_retry_policy_includes_lb_transient_http_errors() {
        for status in [502, 503, 504] {
            let err = TransportErrorKind::HttpError(alloy::transports::HttpError {
                status,
                body: String::new(),
            });
            assert_eq!(
                classify_gateway_transport_error(&err),
                GatewayTransportErrorClass::Retryable
            );
            assert!(is_gateway_retryable_transport_error(&err));
            assert!(is_gateway_transient_transport_error(&err));
        }
    }

    #[test]
    fn gateway_retry_policy_excludes_non_transient_http_errors() {
        for status in [400, 401, 403, 404, 500] {
            let err = TransportErrorKind::HttpError(alloy::transports::HttpError {
                status,
                body: String::new(),
            });
            assert_eq!(
                classify_gateway_transport_error(&err),
                GatewayTransportErrorClass::Fatal
            );
            assert!(!is_gateway_retryable_transport_error(&err));
            assert!(!is_gateway_transient_transport_error(&err));
        }
    }

    #[test]
    fn exhausted_gateway_retries_are_provider_exhausted() {
        let err = TransportErrorKind::BackendGone;
        assert_eq!(
            classify_gateway_transport_error(&err),
            GatewayTransportErrorClass::ProviderExhausted
        );
        assert!(is_gateway_provider_exhausted_transport_error(&err));
        assert!(is_gateway_transient_transport_error(&err));

        let err = TransportErrorKind::Custom(
            format!("{ALLOY_RETRY_EXHAUSTED_ERROR_PREFIX}HTTP error 502").into(),
        );
        assert_eq!(
            classify_gateway_transport_error(&err),
            GatewayTransportErrorClass::ProviderExhausted
        );
        assert!(is_gateway_provider_exhausted_transport_error(&err));
        assert!(is_gateway_transient_transport_error(&err));
    }

    #[test]
    fn gateway_retry_policy_excludes_arbitrary_custom_errors() {
        let err = TransportErrorKind::Custom("gateway authentication failed".into());
        assert_eq!(
            classify_gateway_transport_error(&err),
            GatewayTransportErrorClass::Fatal
        );
        assert!(!is_gateway_retryable_transport_error(&err));
        assert!(!is_gateway_provider_exhausted_transport_error(&err));
        assert!(!is_gateway_transient_transport_error(&err));
    }

    #[test]
    fn gateway_retry_policy_keeps_alloy_custom_rate_limit_errors() {
        let err = TransportErrorKind::Custom("429 Too Many Requests".into());
        assert_eq!(
            classify_gateway_transport_error(&err),
            GatewayTransportErrorClass::Retryable
        );
        assert!(is_gateway_retryable_transport_error(&err));
        assert!(!is_gateway_provider_exhausted_transport_error(&err));
        assert!(is_gateway_transient_transport_error(&err));
    }

    #[test]
    fn timeout_validation_requires_timeout_above_retry_budget() {
        assert!(validate_gateway_http_timeout(
            "send_txn_sync_timeout",
            Duration::from_secs(61),
            15,
            Duration::from_secs(4),
        )
        .is_ok());
        assert!(validate_gateway_http_timeout(
            "send_txn_sync_timeout",
            Duration::from_secs(60),
            15,
            Duration::from_secs(4),
        )
        .is_err());
    }

    #[tokio::test]
    async fn alloy_retry_exhaustion_error_matches_provider_exhaustion_detection() {
        let retry_policy = RateLimitRetryPolicy::default().or(|err| {
            matches!(err, TransportError::Transport(inner) if is_gateway_retryable_transport_error(inner))
        });
        let mut service = RetryBackoffLayer::new_with_policy(0, 0, 100, retry_policy)
            .layer(TransientGatewayErrorTransport);
        let request = Request::new("eth_blockNumber", Id::Number(1), ());
        let request: SerializedRequest = request.try_into().unwrap();
        let err = service
            .call(RequestPacket::from(request))
            .await
            .unwrap_err();

        let TransportError::Transport(inner) = err else {
            panic!("expected transport error from exhausted Alloy retry layer");
        };
        assert!(is_gateway_provider_exhausted_transport_error(&inner));
    }
}
