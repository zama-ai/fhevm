use alloy::{
    primitives::Address,
    transports::{RpcError, TransportErrorKind},
};
use anyhow::{anyhow, Result};
use fhevm_gateway_bindings::gateway_config_checks::GatewayConfigChecks::GatewayConfigChecksErrors;
use std::convert::TryInto;
use thiserror::Error;

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
