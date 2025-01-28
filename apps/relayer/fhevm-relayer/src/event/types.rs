use alloy::{
    primitives::{keccak256, B256},
    rpc::types::Log as RpcLog,
};
use alloy_sol_types::SolEvent;

use crate::common::provider::{
    DecryptionOracle, GatewayContract, TFHEExecutor, DECRYPTION_EVENT_SIGNATURE,
    DECRYPTION_ORACLE_EVENT_SIGNATURE, TFHE_EXECUTOR_FHE_ADD_EVENT_SIGNATURE,
};

#[derive(Debug, Clone)]
pub enum EventType {
    EventDecryption(GatewayContract::EventDecryption),
    DecryptionRequest(DecryptionOracle::DecryptionRequest),
    FheAdd(TFHEExecutor::FheAdd),
    Unknown,
}

impl EventType {
    pub fn from_log(log: &RpcLog) -> Self {
        //  Extract the event signature (first topic in the log)
        let event_signature = log.inner.data.topics().first().unwrap_or(&B256::ZERO);

        match event_signature {
            sig if sig == &keccak256(DECRYPTION_EVENT_SIGNATURE) => {
                match GatewayContract::EventDecryption::decode_log_data(log.data(), true) {
                    Ok(decoded) => EventType::EventDecryption(decoded),
                    Err(_) => EventType::Unknown,
                }
            }
            sig if sig == &keccak256(DECRYPTION_ORACLE_EVENT_SIGNATURE) => {
                match DecryptionOracle::DecryptionRequest::decode_log_data(log.data(), true) {
                    Ok(decoded) => EventType::DecryptionRequest(decoded),
                    Err(_) => EventType::Unknown,
                }
            }
            sig if sig == &keccak256(TFHE_EXECUTOR_FHE_ADD_EVENT_SIGNATURE) => {
                match TFHEExecutor::FheAdd::decode_log_data(log.data(), true) {
                    Ok(decoded) => EventType::FheAdd(decoded),
                    Err(_) => EventType::Unknown,
                }
            }
            _ => EventType::Unknown,
        } // ✅ No extra return statement needed
    }
}
