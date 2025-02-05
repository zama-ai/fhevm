use tracing::info;

use crate::ethereum::{
    bindings::{DecryptionOracle, GatewayContract, TFHEExecutor, Transfer},
    extract_event_signature,
};

use alloy_sol_types::SolEvent;

pub fn handle_event(event: alloy::rpc::types::Log) -> Result<(), eyre::Error> {
    match extract_event_signature(&event)? {
        &GatewayContract::EventDecryption::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                GatewayContract::EventDecryption::SIGNATURE,
                event.block_number
            )
        }
        &DecryptionOracle::DecryptionRequest::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                DecryptionOracle::DecryptionRequest::SIGNATURE,
                event.block_number
            )
        }
        &TFHEExecutor::FheAdd::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                TFHEExecutor::FheAdd::SIGNATURE,
                event.block_number
            )
        }
        &TFHEExecutor::FheSub::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                TFHEExecutor::FheSub::SIGNATURE,
                event.block_number
            )
        }
        &Transfer::SIGNATURE_HASH => {
            info!("{:?} {:?}", Transfer::SIGNATURE, event.block_number)
        }
        _ => {
            // Ignore the event
        }
    }
    Ok(())
}
