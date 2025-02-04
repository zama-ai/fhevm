use crate::{errors::EventProcessingError, event::types::ContractEvent};
#[cfg(test)]
use alloy::primitives::{Address, Bytes, U256};
use alloy::{primitives::B256, rpc::types::Log as RpcLog};
use alloy_sol_types::{sol, SolEvent};
use tracing::{debug, info, instrument, warn};

// Define the Transfer event structure using alloy_sol_types
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

#[derive(Debug)]
pub struct TransferEventHandler {
    #[cfg(test)]
    last_transfer: std::sync::Mutex<Option<TransferEvent>>,
}

#[derive(Debug, Clone)]
pub enum EventType {
    Transfer(Transfer),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

impl Default for TransferEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferEventHandler {
    pub fn new() -> Self {
        TransferEventHandler {
            #[cfg(test)]
            last_transfer: std::sync::Mutex::new(None),
        }
    }

    #[instrument(skip_all)]
    fn handle_event(&self, event: EventType) -> Result<(), EventProcessingError> {
        match event {
            EventType::Transfer(transfer) => {
                info!(
                    from = ?transfer.from,
                    to = ?transfer.to,
                    amount = ?transfer.value,
                    "Transfer processed"
                );

                #[cfg(test)]
                {
                    let mut last = self.last_transfer.lock().unwrap();
                    *last = Some(TransferEvent {
                        from: transfer.from,
                        to: transfer.to,
                        value: transfer.value,
                    });
                }

                Ok(())
            }
        }
    }

    #[cfg(test)]
    pub fn get_last_transfer(&self) -> Option<TransferEvent> {
        self.last_transfer.lock().unwrap().clone()
    }
}

impl ContractEvent for TransferEventHandler {
    fn topics(&self) -> Vec<B256> {
        vec![Transfer::SIGNATURE_HASH]
    }

    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
        debug!(?log.inner.address, "Processing Transfer event");

        // Decoded without checking event signature,
        // Assumed that it will match, else decode error.
        let event = match Transfer::decode_log_data(log.data(), true) {
            Ok(transfer) => {
                debug!(
                    from = ?transfer.from,
                    to = ?transfer.to,
                    value = ?transfer.value,
                    "Successfully decoded Transfer event"
                );
                EventType::Transfer(transfer)
            }
            Err(e) => {
                warn!(error = ?e, "Failed to decode Transfer event");
                return Err(EventProcessingError::DecodingError(e));
            }
        };

        self.handle_event(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Log as PrimitiveLog, LogData};
    use alloy_sol_types::SolValue;
    use std::str::FromStr;

    const TRANSFER_EVENT_SIGNATURE: &str =
        "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

    fn create_test_log() -> RpcLog {
        // Create value (100 tokens)
        let value = U256::from(100);
        let value_bytes: Bytes = value.abi_encode().into();

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
    fn test_transfer_processor_creation() {
        let processor = TransferEventHandler::new();
        assert!(processor.get_last_transfer().is_none());
    }

    #[test]
    fn test_process_valid_transfer_event() {
        let processor = TransferEventHandler::new();
        let log = create_test_log();

        processor.process_event(&log).unwrap();

        let transfer = processor.get_last_transfer().unwrap();
        assert_eq!(
            transfer.from,
            Address::from_str("0x1000000000000000000000000000000000000000").unwrap()
        );
        assert_eq!(
            transfer.to,
            Address::from_str("0x2000000000000000000000000000000000000000").unwrap()
        );
        assert_eq!(transfer.value, U256::from(100));
    }

    #[test]
    fn test_process_invalid_transfer_event() {
        let processor = TransferEventHandler::new();
        let mut log = create_test_log();
        // Corrupt the log data
        log.inner.data = LogData::new_unchecked(vec![], Bytes::default());

        let result = processor.process_event(&log);
        assert!(matches!(
            result,
            Err(EventProcessingError::DecodingError(_))
        ));
        assert!(processor.get_last_transfer().is_none());
    }
}
