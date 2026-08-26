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

/// Resolves a gateway log's `topic0` (event signature hash) to the specific
/// `GatewayChainEventData` variant it represents. Both the polling and
/// WebSocket listeners route every log through this function so the
/// topic-to-event mapping cannot drift between the two paths.
pub fn gateway_chain_event_for_log(log: Log, tx_hash: TxHash) -> Option<GatewayChainEventData> {
    let topic0 = FixedBytes::<32>::from_slice(log.topic0()?.as_slice());

    Some(match topic0 {
        t if t == Decryption::UserDecryptionResponse::SIGNATURE_HASH => {
            GatewayChainEventData::UserDecryptionResponse { log, tx_hash }
        }
        t if t == Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH => {
            GatewayChainEventData::UserDecryptionResponseThresholdReached { log, tx_hash }
        }
        t if t == Decryption::PublicDecryptionResponse::SIGNATURE_HASH => {
            GatewayChainEventData::PublicDecryptionResponse { log, tx_hash }
        }
        t if t == InputVerification::VerifyProofResponse::SIGNATURE_HASH => {
            GatewayChainEventData::VerifyProofResponse { log, tx_hash }
        }
        t if t == InputVerification::RejectProofResponse::SIGNATURE_HASH => {
            GatewayChainEventData::RejectProofResponse { log, tx_hash }
        }
        _ => return None,
    })
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
