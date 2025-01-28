use crate::{
    errors::{Error, Result},
    event::processor::EventProcessor,
};
use alloy::rpc::types::Log as RpcLog;
use alloy_sol_types::{sol, SolEvent};
use tracing::error;

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
}

pub struct TransferProcessor;

impl EventProcessor for TransferProcessor {
    fn process_event(&self, log: &RpcLog) -> Result<()> {
        println!("🔍 Received log: {:?}", log);
        println!("🔍 Topics count: {}", log.topics().len());
        println!("🔍 Data: {:?}", log.inner.data);
        // ✅ Expect RpcLog
        match Transfer::decode_log_data(log.data(), true) {
            // ✅ Directly decode from RPC log
            Ok(event) => {
                println!(
                    "Transfer from {:?} to {:?} of {:?} tokens",
                    event.from, event.to, event.value
                );
                Ok(())
            }
            Err(e) => {
                error!("Failed to decode Transfer event!");
                error!("Detailed error: {:?}", e);
                Err(Error::ProcessingError(format!(
                    "Failed to decode Transfer event: {:?}",
                    e
                )))
            }
        }
    }
}
