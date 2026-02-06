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

#[derive(Debug, Error)]
pub(crate) enum CoprocessorConfigError {
    #[error("NotCoprocessorSigner({0})")]
    NotCoprocessorSigner(Address),
    #[error("NotCoprocessorTxSender({0})")]
    NotCoprocessorTxSender(Address),
    #[error("CoprocessorSignerDoesNotMatchTxSender({signer},{tx_sender})")]
    CoprocessorSignerDoesNotMatchTxSender { signer: Address, tx_sender: Address },
}

pub(crate) fn try_extract_terminal_config_error(
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
