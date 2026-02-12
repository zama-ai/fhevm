use anyhow::{anyhow, Result};
use std::{convert::TryInto, fmt::Display, time::Duration};

use crate::nonce_managed_provider::NonceManagedProvider;
use alloy::{
    network::Ethereum,
    primitives::Address,
    providers::{ext::DebugApi, Provider},
    rpc::types::{
        trace::geth::{CallConfig, CallFrame, GethDebugTracingOptions},
        TransactionReceipt,
    },
    sol_types::SolInterface,
    transports::{RpcError, TransportErrorKind},
};
use fhevm_gateway_bindings::gateway_config_checks::GatewayConfigChecks::GatewayConfigChecksErrors;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RevertReason {
    ConfigError(CoprocessorConfigError),
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TerminalConfigError {
    pub(crate) config_error: CoprocessorConfigError,
    pub(crate) db_error: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CoprocessorConfigError {
    NotCoprocessorSigner(Address),
    NotCoprocessorTxSender(Address),
    CoprocessorSignerDoesNotMatchTxSender { signer: Address, tx_sender: Address },
}

impl CoprocessorConfigError {
    pub(crate) fn to_db_error_string(&self) -> String {
        format!("gw: {self}")
    }
}

impl From<CoprocessorConfigError> for TerminalConfigError {
    fn from(config_error: CoprocessorConfigError) -> Self {
        Self {
            db_error: config_error.to_db_error_string(),
            config_error,
        }
    }
}

impl Display for CoprocessorConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCoprocessorSigner(address) => {
                write!(f, "NotCoprocessorSigner({address})")
            }
            Self::NotCoprocessorTxSender(address) => {
                write!(f, "NotCoprocessorTxSender({address})")
            }
            Self::CoprocessorSignerDoesNotMatchTxSender { signer, tx_sender } => write!(
                f,
                "CoprocessorSignerDoesNotMatchTxSender({signer},{tx_sender})"
            ),
        }
    }
}

pub(crate) fn try_decode_coprocessor_config_error_from_revert_data(
    revert_data: &[u8],
) -> Option<CoprocessorConfigError> {
    let decoded = GatewayConfigChecksErrors::abi_decode(revert_data).ok()?;
    map_gateway_config_error(decoded)
}

fn map_gateway_config_error(decoded: GatewayConfigChecksErrors) -> Option<CoprocessorConfigError> {
    match decoded {
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
    }
}

pub(crate) fn try_decode_coprocessor_config_error(
    err: &RpcError<TransportErrorKind>,
) -> Option<CoprocessorConfigError> {
    err.as_error_resp()
        .and_then(|payload| payload.as_decoded_interface_error::<GatewayConfigChecksErrors>())
        .and_then(map_gateway_config_error)
}

pub(crate) fn try_extract_terminal_config_error(
    err: &RpcError<TransportErrorKind>,
) -> Option<TerminalConfigError> {
    try_decode_coprocessor_config_error(err).map(Into::into)
}

pub(crate) async fn classify_failed_receipt<P>(
    provider: &NonceManagedProvider<P>,
    receipt: &TransactionReceipt,
    trace_timeout: Duration,
) -> std::result::Result<TerminalConfigError, String>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    match get_revert_reason(provider, receipt, trace_timeout).await {
        RevertReason::ConfigError(config_error) => Ok(config_error.into()),
        RevertReason::Other(reason) => Err(reason),
    }
}

pub(crate) async fn get_revert_reason<P>(
    provider: &NonceManagedProvider<P>,
    receipt: &TransactionReceipt,
    trace_timeout: Duration,
) -> RevertReason
where
    P: Provider<Ethereum> + Clone + 'static,
{
    let trace = match tokio::time::timeout(
        trace_timeout,
        provider.inner().debug_trace_transaction(
            receipt.transaction_hash,
            GethDebugTracingOptions::call_tracer(CallConfig::default()),
        ),
    )
    .await
    {
        Ok(Ok(trace)) => trace,
        Ok(Err(e)) => {
            return RevertReason::Other(format!("Unable to use `debug_trace_transaction`: {e}"));
        }
        Err(_) => {
            return RevertReason::Other(format!(
                "`debug_trace_transaction` timed out after {trace_timeout:?}"
            ));
        }
    };

    let call_frame = match trace.try_into_call_frame() {
        Ok(call_frame) => call_frame,
        Err(e) => {
            return RevertReason::Other(format!(
                "Unable to retrieve call frame from debug trace: {e}"
            ));
        }
    };

    if let Some(config_error) = find_config_error_in_trace(&call_frame) {
        return RevertReason::ConfigError(config_error);
    }

    RevertReason::Other(
        call_frame
            .revert_reason
            .clone()
            .or_else(|| call_frame.error.clone())
            .unwrap_or_else(|| {
                "Unable to find revert reason in debug trace root frame (missing root `revert_reason` and `error`)"
                    .to_owned()
            }),
    )
}

fn find_config_error_in_trace(call_frame: &CallFrame) -> Option<CoprocessorConfigError> {
    if call_frame.error.is_none() && call_frame.revert_reason.is_none() {
        return None;
    }

    call_frame
        .output
        .as_ref()
        .and_then(|revert_data| try_decode_coprocessor_config_error_from_revert_data(revert_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{address, Bytes};
    use alloy::sol_types::SolError;
    use fhevm_gateway_bindings::gateway_config_checks::GatewayConfigChecks::{
        CoprocessorSignerDoesNotMatchTxSender, HostChainNotRegistered, NotCoprocessorSigner,
        NotCoprocessorTxSender,
    };

    #[test]
    fn decode_not_coprocessor_signer() {
        let signer = address!("1234567890123456789012345678901234567890");
        let revert_data = NotCoprocessorSigner {
            signerAddress: signer,
        }
        .abi_encode();

        assert_eq!(
            try_decode_coprocessor_config_error_from_revert_data(&revert_data),
            Some(CoprocessorConfigError::NotCoprocessorSigner(signer))
        );
    }

    #[test]
    fn decode_not_coprocessor_tx_sender() {
        let tx_sender = address!("1111111111111111111111111111111111111111");
        let revert_data = NotCoprocessorTxSender {
            txSenderAddress: tx_sender,
        }
        .abi_encode();

        assert_eq!(
            try_decode_coprocessor_config_error_from_revert_data(&revert_data),
            Some(CoprocessorConfigError::NotCoprocessorTxSender(tx_sender))
        );
    }

    #[test]
    fn decode_signer_tx_sender_mismatch() {
        let signer = address!("2222222222222222222222222222222222222222");
        let tx_sender = address!("3333333333333333333333333333333333333333");
        let revert_data = CoprocessorSignerDoesNotMatchTxSender {
            signerAddress: signer,
            txSenderAddress: tx_sender,
        }
        .abi_encode();

        assert_eq!(
            try_decode_coprocessor_config_error_from_revert_data(&revert_data),
            Some(
                CoprocessorConfigError::CoprocessorSignerDoesNotMatchTxSender { signer, tx_sender }
            )
        );
    }

    #[test]
    fn decode_ignores_non_target_error() {
        let revert_data = HostChainNotRegistered {
            chainId: alloy::primitives::U256::from(42),
        }
        .abi_encode();

        assert_eq!(
            try_decode_coprocessor_config_error_from_revert_data(&revert_data),
            None
        );
    }

    #[test]
    fn decode_ignores_invalid_payload() {
        assert_eq!(
            try_decode_coprocessor_config_error_from_revert_data(&[1, 2, 3, 4]),
            None
        );
    }

    #[test]
    fn finds_config_error_in_failing_root_trace_output() {
        let signer = address!("1234567890123456789012345678901234567890");
        let revert_data = NotCoprocessorSigner {
            signerAddress: signer,
        }
        .abi_encode();

        let root = CallFrame {
            error: Some("execution reverted".to_owned()),
            output: Some(Bytes::from(revert_data)),
            ..Default::default()
        };

        assert_eq!(
            find_config_error_in_trace(&root),
            Some(CoprocessorConfigError::NotCoprocessorSigner(signer))
        );
    }

    #[test]
    fn ignores_config_error_in_nested_trace_output() {
        let signer = address!("1234567890123456789012345678901234567890");
        let revert_data = NotCoprocessorSigner {
            signerAddress: signer,
        }
        .abi_encode();

        let nested = CallFrame {
            error: Some("execution reverted".to_owned()),
            output: Some(Bytes::from(revert_data)),
            ..Default::default()
        };
        let root = CallFrame {
            error: Some("execution reverted".to_owned()),
            calls: vec![nested],
            ..Default::default()
        };

        assert_eq!(find_config_error_in_trace(&root), None);
    }

    #[test]
    fn ignores_config_error_in_non_failing_trace_output() {
        let signer = address!("1234567890123456789012345678901234567890");
        let revert_data = NotCoprocessorSigner {
            signerAddress: signer,
        }
        .abi_encode();

        let nested = CallFrame {
            output: Some(Bytes::from(revert_data)),
            ..Default::default()
        };
        let root = CallFrame {
            error: Some("execution reverted".to_owned()),
            calls: vec![nested],
            ..Default::default()
        };

        assert_eq!(find_config_error_in_trace(&root), None);
    }
}
