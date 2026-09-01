#![allow(clippy::too_many_arguments)]

use crate::core::event::GatewayChainEventData;
use alloy::primitives::{FixedBytes, TxHash};
use alloy::rpc::types::Log;
use alloy::sol;
use alloy::sol_types::SolEvent;

pub use fhevm_gateway_bindings::{
    ciphertext_commits::CiphertextCommits,
    decryption::{Decryption, IDecryption},
    input_verification::InputVerification,
};

// Define the Transfer event structure using alloy_sol_types
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

type IntoEvent = fn(Log, TxHash) -> GatewayChainEventData;

/// Every gateway event the relayer routes. Both listeners filter `topic0` against this table
/// and both decode through it, so a subscribed signature is always decodable.
fn gateway_chain_events() -> [(FixedBytes<32>, IntoEvent); 5] {
    [
        (
            Decryption::UserDecryptionResponse::SIGNATURE_HASH,
            |log, tx_hash| GatewayChainEventData::UserDecryptionResponse { log, tx_hash },
        ),
        (
            Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH,
            |log, tx_hash| GatewayChainEventData::UserDecryptionResponseThresholdReached {
                log,
                tx_hash,
            },
        ),
        (
            Decryption::PublicDecryptionResponse::SIGNATURE_HASH,
            |log, tx_hash| GatewayChainEventData::PublicDecryptionResponse { log, tx_hash },
        ),
        (
            InputVerification::VerifyProofResponse::SIGNATURE_HASH,
            |log, tx_hash| GatewayChainEventData::VerifyProofResponse { log, tx_hash },
        ),
        (
            InputVerification::RejectProofResponse::SIGNATURE_HASH,
            |log, tx_hash| GatewayChainEventData::RejectProofResponse { log, tx_hash },
        ),
    ]
}

pub fn gateway_chain_event_signatures() -> Vec<FixedBytes<32>> {
    gateway_chain_events()
        .iter()
        .map(|(topic0, _)| *topic0)
        .collect()
}

/// Resolves a gateway log's `topic0` to the `GatewayChainEventData` variant it represents.
pub fn gateway_chain_event_for_log(log: Log, tx_hash: TxHash) -> Option<GatewayChainEventData> {
    let topic0 = FixedBytes::<32>::from_slice(log.topic0()?.as_slice());
    let (_, into_event) = gateway_chain_events()
        .into_iter()
        .find(|(signature, _)| *signature == topic0)?;

    Some(into_event(log, tx_hash))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::sol_types::SolEvent;

    #[test]
    fn test_decryption() {
        println!(
            "DecryptionManager UserDecryptionRequest (legacy / v2):\n{}\n{}\n",
            Decryption::UserDecryptionRequest_0::SIGNATURE,
            Decryption::UserDecryptionRequest_0::SIGNATURE_HASH
        );
        println!(
            "DecryptionManager UserDecryptionRequest (unified / v3):\n{}\n{}\n",
            Decryption::UserDecryptionRequest_1::SIGNATURE,
            Decryption::UserDecryptionRequest_1::SIGNATURE_HASH
        );
        println!(
            "DecryptionManager UserDecryptionResponse:\n{}\n{}\n",
            Decryption::UserDecryptionResponse::SIGNATURE,
            Decryption::UserDecryptionResponse::SIGNATURE_HASH
        );
    }

    #[test]
    fn test_input_verification() {
        println!(
            "InputVerification VerifyProofRequest:\n{}\n{}\n",
            InputVerification::VerifyProofRequest::SIGNATURE,
            InputVerification::VerifyProofRequest::SIGNATURE_HASH
        );
        println!(
            "InputVerification VerifyProofResponse:\n{}\n{}\n",
            InputVerification::VerifyProofResponse::SIGNATURE,
            InputVerification::VerifyProofResponse::SIGNATURE_HASH
        );
    }
}
