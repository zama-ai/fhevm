use alloy::{
    primitives::{keccak256, Address, B256},
    rpc::types::Log as RpcLog,
};
use alloy_sol_types::{sol, SolEvent};
use fhevm_relayer::{
    errors::EventProcessingError,
    event::{registry::EventRegistry, types::ContractEvent},
};
use std::{str::FromStr, sync::Arc};
use tracing::{debug, info};

// Define the event structure
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

#[derive(Debug)]
pub struct CustomProcessor;

#[derive(Debug, Clone)]
pub enum EventType {
    Transfer(Transfer),
}

impl Default for CustomProcessor {
    fn default() -> Self {
        Self::new()
    }
}
impl CustomProcessor {
    pub fn new() -> Self {
        CustomProcessor
    }

    fn handle_event(&self, event: EventType) -> Result<(), EventProcessingError> {
        match event {
            EventType::Transfer(transfer) => {
                info!(
                    from = ?transfer.from,
                    to = ?transfer.to,
                    value = ?transfer.value,
                    "Custom transfer processed"
                );
                Ok(())
            }
        }
    }
}

impl ContractEvent for CustomProcessor {
    fn topics(&self) -> Vec<B256> {
        vec![keccak256("Transfer(address,address,uint256)")]
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        debug!(?log.inner.address, "Processing custom event");

        let event = Transfer::decode_log_data(log.data(), true)
            .map(EventType::Transfer)
            .map_err(EventProcessingError::DecodingError)?;

        self.handle_event(event)
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let registry = Arc::new(EventRegistry::new());

    let contract_address = Address::from_str("0x1234567890123456789012345678901234567890")?;
    registry.register_contract(contract_address);

    let processor = CustomProcessor::new();
    registry.register_event(contract_address, processor);

    println!(
        "Registered custom processor for contract: {}",
        contract_address
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Bytes, Log as PrimitiveLog, LogData, U256};
    use std::str::FromStr;

    const TRANSFER_EVENT_SIGNATURE: &str =
        "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

    fn create_test_log() -> RpcLog {
        let value = U256::from(100);
        let value_bytes: Bytes = value.encode().into();

        RpcLog {
            inner: PrimitiveLog {
                address: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
                data: LogData::new_unchecked(
                    vec![
                        B256::from_str(TRANSFER_EVENT_SIGNATURE).unwrap(),
                        B256::from_str(
                            "0000000000000000000000001000000000000000000000000000000000000000",
                        )
                        .unwrap(),
                        B256::from_str(
                            "0000000000000000000000002000000000000000000000000000000000000000",
                        )
                        .unwrap(),
                    ],
                    value_bytes,
                ),
            },
            block_hash: Some(B256::ZERO),
            block_number: Some(12345),
            block_timestamp: Some(1678901234),
            transaction_hash: Some(B256::ZERO),
            transaction_index: Some(1),
            log_index: Some(1),
            removed: false,
        }
    }

    #[test]
    fn test_process_valid_custom_event() {
        let processor = CustomProcessor::new();
        let log = create_test_log();

        let result = processor.process_event(&log);
        assert!(result.is_ok());
    }
}
