use crate::common::utils::keccak256_hex;
use crate::errors::Result;
use crate::event::processor::{EventProcessor, EventProcessorBox};
use alloy::primitives::Address;
use alloy::rpc::types::Log as RpcLog;
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};

pub struct EventRegistry {
    contracts: DashMap<Address, DashMap<String, EventProcessorBox>>,
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for EventRegistry {}
unsafe impl Sync for EventRegistry {}

impl EventRegistry {
    #[instrument(skip_all)]
    pub fn new() -> Self {
        Self {
            contracts: DashMap::new(),
        }
    }

    pub fn register_contract(&self, contract: Address) {
        self.contracts.entry(contract).or_default();
    }

    pub fn register_event<T: EventProcessor + 'static>(
        &self,
        contract: Address,
        event_name: &str,
        processor: T,
    ) {
        if let Some(event_map) = self.contracts.get(&contract) {
            let topic = keccak256_hex(event_name);
            info!("*** Registering event signature {event_name} for contract {contract}.");
            info!("Topic -- keccack256(event_signature) = {} ", topic);

            event_map.insert(topic, Arc::new(processor));
        }
    }

    pub fn process_event(&self, contract: Address, event_name: &str, log: &RpcLog) -> Result<()> {
        if let Some(event_map) = self.contracts.get(&contract) {
            debug!("Registered events for contract: {:?}", contract);
            for key in event_map.iter() {
                debug!("  - Event Name: {}", key.key());
            }
            if let Some(handler) = event_map.get(event_name) {
                return handler.process_event(log);
            } else {
                debug!("No event found associated to contract {:?}", contract);
                Ok(())
            }
        } else {
            Err(crate::errors::Error::ProcessingError(format!(
                "No handler registered for contract {:?}, event: {}",
                contract, event_name
            )))
        }
    }

    pub fn get_contracts(&self) -> Vec<Address> {
        self.contracts.iter().map(|entry| *entry.key()).collect()
    }
}
