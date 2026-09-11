use alloy::{
    primitives::Address,
    transports::{RpcError, TransportErrorKind},
};
use anyhow::{anyhow, Result};
use fhevm_gateway_bindings::gateway_config_checks::GatewayConfigChecks::GatewayConfigChecksErrors;
use std::convert::TryInto;
use thiserror::Error;

/// Infrastructure failures do not say whether a proof is invalid. Keep them
/// outside its terminal retry budget, but return the error to the operation
/// loop so its normal backoff (or BackendGone shutdown) still applies.
pub(crate) fn is_transient_gateway_error(err: &RpcError<TransportErrorKind>) -> bool {
    match err {
        // HTML maintenance pages, empty bodies and malformed JSON provide no
        // evidence that the proof is invalid. Preserve work with scheduling
        // delay even when an intermediary returned HTTP 200.
        RpcError::DeserError { .. } => true,
        RpcError::Transport(TransportErrorKind::BackendGone)
        | RpcError::Transport(TransportErrorKind::MissingBatchResponse(_)) => true,
        RpcError::Transport(TransportErrorKind::HttpError(err)) => {
            // HTTP 500 deliberately preserves work too: the status alone does
            // not establish a terminal proof failure. A persistent application
            // defect returning 500 can therefore keep work retrying indefinitely.
            matches!(err.status, 408 | 429 | 500 | 502 | 503 | 504)
        }
        RpcError::Transport(TransportErrorKind::Custom(err)) => {
            // This exact error is emitted by NonceManagedProvider's local send
            // deadline. Do not classify arbitrary custom errors by substring.
            if err.to_string() == "eth_sendRawTransactionSync timeout" {
                return true;
            }
            let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(err.as_ref());
            while let Some(inner) = cause {
                if let Some(http) = inner.downcast_ref::<alloy::transports::http::reqwest::Error>()
                {
                    return http.is_connect()
                        || http.is_timeout()
                        || http.is_request()
                        || http.is_body();
                }
                cause = inner.source();
            }
            false
        }
        RpcError::ErrorResp(err) => {
            // Do not interpret contract reverts, nonce errors, or generic
            // internal errors as infrastructure failures. Match explicit
            // unavailability messages on the Gateway's server-error codes.
            // This is narrower than Alloy's provider-specific rate-limit policy:
            // e.g. -32005 is not covered and retains limited-retry accounting.
            err.code == 429
                || (matches!(err.code, -32000 | -32603)
                    && matches!(
                        err.message.trim().to_ascii_lowercase().as_str(),
                        "context deadline exceeded"
                            | "service unavailable"
                            | "temporarily unavailable"
                            | "too many requests"
                    ))
        }
        _ => false,
    }
}

pub(crate) fn try_into_array<const SIZE: usize>(vec: Vec<u8>) -> Result<[u8; SIZE]> {
    if vec.len() != SIZE {
        return Err(anyhow!(
            "invalid len, expected {} but got {}",
            SIZE,
            vec.len()
        ));
    }

    vec.try_into()
        .map_err(|_| anyhow!("Failed to convert Vec to array"))
}

/// Errors that the gateway's [`GatewayConfigChecks`] base contract can emit
/// when the coprocessor is misconfigured.
///
/// These are **non-retryable**: they indicate a permanent mismatch between the
/// coprocessor's on-chain identity (tx-sender / signer addresses) and what is
/// registered in `GatewayConfig`, so retrying the same transaction will always
/// fail.
///
/// # Production reachability
///
/// - `NotCoprocessorTxSender` — `MultichainACL`, `CiphertextCommits`, `InputVerification`
/// - `NotCoprocessorSigner` — `InputVerification` only
/// - `CoprocessorSignerDoesNotMatchTxSender` — `InputVerification` only
#[derive(Debug, Error)]
pub(crate) enum CoprocessorConfigError {
    #[error("NotCoprocessorSigner({0})")]
    NotCoprocessorSigner(Address),
    #[error("NotCoprocessorTxSender({0})")]
    NotCoprocessorTxSender(Address),
    #[error("CoprocessorSignerDoesNotMatchTxSender({signer},{tx_sender})")]
    CoprocessorSignerDoesNotMatchTxSender { signer: Address, tx_sender: Address },
}

/// Tries to decode a non-retryable coprocessor configuration error from an RPC
/// failure.
///
/// The gateway's `GatewayConfigChecks` contract can revert with three distinct
/// config errors (see [`CoprocessorConfigError`]).  When the coprocessor's
/// on-chain identity does not match what is registered in `GatewayConfig`,
/// these reverts fire *before* any business logic runs, making the transaction
/// permanently un-sendable.
///
/// Returns `Some(error)` when the RPC payload matches one of the known config
/// errors, `None` otherwise.
pub(crate) fn try_extract_non_retryable_config_error(
    err: &RpcError<TransportErrorKind>,
) -> Option<CoprocessorConfigError> {
    err.as_error_resp()
        .and_then(|payload| payload.as_decoded_interface_error::<GatewayConfigChecksErrors>())
        .and_then(|decoded| match decoded {
            GatewayConfigChecksErrors::NotCoprocessorSigner(inner) => Some(
                CoprocessorConfigError::NotCoprocessorSigner(inner.signerAddress),
            ),
            GatewayConfigChecksErrors::NotCoprocessorTxSender(inner) => Some(
                CoprocessorConfigError::NotCoprocessorTxSender(inner.txSenderAddress),
            ),
            GatewayConfigChecksErrors::CoprocessorSignerDoesNotMatchTxSender(inner) => Some(
                CoprocessorConfigError::CoprocessorSignerDoesNotMatchTxSender {
                    signer: inner.signerAddress,
                    tx_sender: inner.txSenderAddress,
                },
            ),
            _ => None,
        })
}

#[cfg(test)]
mod transient_gateway_tests {
    use super::*;

    #[test]
    fn http_status_policy_excludes_permanent_errors() {
        for status in [408, 429, 500, 502, 503, 504] {
            assert!(
                is_transient_gateway_error(&TransportErrorKind::http_error(status, String::new())),
                "HTTP {status}"
            );
        }
        for status in [400, 401, 403, 404, 405, 501, 505] {
            assert!(
                !is_transient_gateway_error(&TransportErrorKind::http_error(
                    status,
                    "context deadline exceeded".into()
                )),
                "HTTP {status}"
            );
        }
    }

    #[test]
    fn rpc_policy_does_not_hide_transaction_or_contract_errors() {
        for (code, message, expected) in [
            (-32000, "context deadline exceeded", true),
            (-32603, "context deadline exceeded", true),
            (-32000, "service unavailable", true),
            (-32603, "temporarily unavailable", true),
            (-32000, "too many requests", true),
            (429, "rate limited", true),
            (-32603, "internal error", false),
            (-32000, "nonce too low", false),
            (-32000, "nonce too high", false),
            (-32000, "replacement transaction underpriced", false),
            (-32000, "insufficient funds", false),
            (
                -32000,
                "execution reverted: context deadline exceeded",
                false,
            ),
            (3, "context deadline exceeded", false),
            (-32601, "method not found", false),
            (-32602, "invalid params", false),
        ] {
            let error = RpcError::ErrorResp(
                serde_json::from_value(serde_json::json!({
                    "code": code, "message": message,
                }))
                .unwrap(),
            );
            assert_eq!(
                is_transient_gateway_error(&error),
                expected,
                "{code}: {message}"
            );
        }
    }

    #[test]
    fn custom_errors_are_not_all_transient() {
        assert!(is_transient_gateway_error(&TransportErrorKind::custom_str(
            "eth_sendRawTransactionSync timeout"
        )));
        assert!(is_transient_gateway_error(
            &TransportErrorKind::backend_gone()
        ));
        assert!(!is_transient_gateway_error(
            &TransportErrorKind::custom_str("signer unavailable")
        ));
        assert!(!is_transient_gateway_error(
            &TransportErrorKind::custom_str("context deadline exceeded")
        ));
    }

    #[tokio::test]
    async fn actual_reqwest_connection_failure_and_timeout_are_transient() {
        use alloy::transports::http::reqwest;
        use std::time::Duration;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        // TCP connects, but no HTTP response arrives.
        let client = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();
        let error = client.get(&url).send().await.unwrap_err();
        assert!(error.is_timeout());
        assert!(is_transient_gateway_error(&TransportErrorKind::custom(
            error
        )));

        drop(listener);
        let error = client.get(&url).send().await.unwrap_err();
        assert!(error.is_connect());
        assert!(is_transient_gateway_error(&TransportErrorKind::custom(
            error
        )));

        let error = client.get("not a URL").send().await.unwrap_err();
        assert!(error.is_builder());
        assert!(!is_transient_gateway_error(&TransportErrorKind::custom(
            error
        )));
    }
}
