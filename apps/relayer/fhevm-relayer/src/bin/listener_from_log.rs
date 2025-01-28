use alloy::primitives::Address;
use alloy::primitives::{B256, U256};
use alloy::rpc::types::Log as RpcLog;
use std::collections::HashMap;

#[cfg(test)]
use alloy::primitives::{Log as PrimitiveLog, LogData};

#[derive(Debug, Clone)]
pub enum EventType {
    Transfer {
        from: Address,
        to: Address,
        value: U256,
    },
    Unknown,
}

const TRANSFER_EVENT_SIGNATURE: &str =
    "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

/// Registry mapping contract addresses to event signature mappings
pub type EventRegistry = HashMap<Address, HashMap<B256, fn(&RpcLog) -> Option<EventType>>>;

impl EventType {
    pub fn from_log(log: &RpcLog, registry: &EventRegistry) -> Self {
        if log.topics().is_empty() || log.data().data.is_empty() {
            return EventType::Unknown;
        }

        let contract_address = log.address();
        let event_signature: B256 = *log.topic0().unwrap(); // Ensure it remains a B256

        if let Some(contract_events) = registry.get(&contract_address) {
            if let Some(decode_fn) = contract_events.get(&event_signature) {
                return decode_fn(log).unwrap_or(EventType::Unknown);
            }
        }

        EventType::Unknown
    }
}

fn decode_transfer_event(log: &RpcLog) -> Option<EventType> {
    if log.topics().len() < 3 || log.data().data.len() < 32 {
        return None;
    }

    println!("Decoding event Transfer");
    let from = Address::from_slice(&log.topics()[1].to_owned()[12..]); // Take the last 20 bytes
    let to = Address::from_slice(&log.topics()[2].to_owned()[12..]); // Take the last 20 byte

    let value = U256::from_be_bytes::<32>(log.data().data[..32].try_into().unwrap());

    Some(EventType::Transfer { from, to, value })
}

pub trait EventProcessor {
    fn process_event(&self, event: EventType) -> Result<(), String>;
}

fn handle_log(log: RpcLog, registry: &EventRegistry, processor: &dyn EventProcessor) {
    let event = EventType::from_log(&log, registry);

    if let Err(e) = processor.process_event(event) {
        eprintln!("Error processing event: {}", e);
    }
}

fn main() {
    println!("Starting services...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex_literal::hex, U256};
    use std::str::FromStr;

    #[test]
    fn test_manual_transfer_event() {
        let fake_log = RpcLog {
            inner: PrimitiveLog {
                address: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(), //  Contract address
                data: LogData::new_unchecked(
                    vec![
                        B256::from_str(TRANSFER_EVENT_SIGNATURE).unwrap(), // Transfer event signature
                        B256::from_str(
                            "0000000000000000000000000000000000000000000000000000000000000001",
                        )
                        .unwrap(), //  from address
                        B256::from_str(
                            "0000000000000000000000000000000000000000000000000000000000000002",
                        )
                        .unwrap(), // to address
                    ],
                    vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
                    ]
                    .into(), // uint256 value (100 in hex)
                ),
            },
            block_hash: Some(B256::ZERO),
            block_number: Some(12345),
            block_timestamp: Some(1678901234),
            transaction_hash: Some(B256::ZERO),
            transaction_index: Some(1),
            log_index: Some(1),
            removed: false,
        };

        let mut registry: EventRegistry = HashMap::new();
        registry.insert(
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            HashMap::from([(
                B256::from_slice(&hex!(
                    "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                )),
                decode_transfer_event as fn(&RpcLog) -> Option<EventType>,
            )]),
        );

        let event = EventType::from_log(&fake_log, &registry);
        match event {
            EventType::Transfer { from, to, value } => {
                assert_eq!(
                    from,
                    Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
                );
                assert_eq!(
                    to,
                    Address::from_str("0x0000000000000000000000000000000000000002").unwrap()
                );
                assert_eq!(value, U256::from(100));
            }
            _ => panic!("Failed to decode Transfer event"),
        }
    }

    #[test]
    fn test_manual_transfer_event_handle_log() {
        let fake_log = RpcLog {
            inner: PrimitiveLog {
                address: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(), //  Contract address
                data: LogData::new_unchecked(
                    vec![
                        B256::from_str(TRANSFER_EVENT_SIGNATURE).unwrap(), // Transfer event signature
                        B256::from_str(
                            "0000000000000000000000000000000000000000000000000000000000000001",
                        )
                        .unwrap(), //  from address
                        B256::from_str(
                            "0000000000000000000000000000000000000000000000000000000000000002",
                        )
                        .unwrap(), // to address
                    ],
                    vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
                    ]
                    .into(), // uint256 value (100 in hex)
                ),
            },
            block_hash: Some(B256::ZERO),
            block_number: Some(12345),
            block_timestamp: Some(1678901234),
            transaction_hash: Some(B256::ZERO),
            transaction_index: Some(1),
            log_index: Some(1),
            removed: false,
        };

        let mut registry: EventRegistry = HashMap::new();
        registry.insert(
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            HashMap::from([(
                B256::from_slice(&hex!(
                    "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                )),
                decode_transfer_event as fn(&RpcLog) -> Option<EventType>,
            )]),
        );

        struct TestProcessor;

        impl EventProcessor for TestProcessor {
            fn process_event(&self, event: EventType) -> Result<(), String> {
                println!("✅ Processed event: {:?}", event);
                Ok(())
            }
        }

        // ✅ Call handle_log instead of from_log
        handle_log(fake_log, &registry, &TestProcessor);
    }
}
