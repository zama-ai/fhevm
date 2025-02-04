use crate::errors::EventProcessingError;
use crate::event::types::ContractEvent;
use alloy::primitives::Address;
use alloy::rpc::types::Log as RpcLog;
use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, error, info, instrument};

/// A thread-safe registry for managing contract event processors.
///
/// The `EventRegistry` maintains mappings between contract addresses, event topics,
/// and their corresponding processors. It uses concurrent hashmaps to allow
/// safe access from multiple threads.
///
/// # Examples
///
/// ```rust
/// use fhevm_relayer::event::registry::EventRegistry;
/// use fhevm_relayer::event::processors::tfhe_executor::TfheExecutorEventHandler;
/// use alloy::primitives::Address;
/// use std::str::FromStr;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a new registry
/// let registry = EventRegistry::new();
///
/// // Create a contract address
/// let contract_address = Address::from_str("0x0000000000000000000000000000000000000000")?;
///
/// // Register a contract
/// registry.register_contract(contract_address);
///
/// // Register an event processor
/// let processor = TfheExecutorEventHandler::new();
/// registry.register_event(contract_address, processor);
/// # Ok(())
/// # }
/// ```
pub struct EventRegistry {
    contracts: DashMap<Address, DashMap<String, Arc<dyn ContractEvent>>>,
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EventRegistry {
    /// Creates a new, empty event registry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fhevm_relayer::event::registry::EventRegistry;
    /// let registry = EventRegistry::new();
    /// ```
    #[instrument(skip_all)]
    pub fn new() -> Self {
        Self {
            contracts: DashMap::new(),
        }
    }

    /// Registers a contract address for event processing.
    ///
    /// This must be called before registering any events for the contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The Ethereum address of the contract
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fhevm_relayer::event::registry::EventRegistry;
    /// use std::str::FromStr;
    /// use alloy::primitives::Address;
    /// let registry = EventRegistry::new();
    /// let contract_address = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    /// registry.register_contract(contract_address);
    /// ```
    pub fn register_contract(&self, contract: Address) {
        self.contracts.entry(contract).or_default();
    }

    #[instrument(skip_all)]
    pub fn register_event<T: ContractEvent + 'static>(&self, contract: Address, processor: T) {
        if let Some(event_map) = self.contracts.get(&contract) {
            let processor: Arc<dyn ContractEvent> = Arc::new(processor);
            for topic in processor.topics() {
                info!(?contract, ?topic, "Registering event handler");
                event_map.insert(topic.to_string(), Arc::clone(&processor));
            }
        }
    }

    #[instrument(skip_all)]
    pub fn process_event(
        &self,
        contract: Address,
        topic: &str,
        log: &RpcLog,
    ) -> Result<(), EventProcessingError> {
        let start = std::time::Instant::now();

        let event_map = self
            .contracts
            .get(&contract)
            .ok_or(EventProcessingError::UnregisteredContract { contract })?;

        let result = if let Some(handler) = event_map.get(topic) {
            handler.process_event(log)
        } else {
            debug!(?contract, ?topic, "No handler found for event");
            Ok(()) // Skip unknown events
        };

        let duration = start.elapsed();

        match &result {
            Ok(_) => debug!(?contract, ?topic, ?duration, "Successfully processed event"),
            Err(e) => error!(
                ?contract,
                ?topic,
                ?duration,
                error = ?e,
                "Failed to process event"
            ),
        }

        result
    }

    pub fn get_contracts(&self) -> Vec<Address> {
        self.contracts.iter().map(|entry| *entry.key()).collect()
    }
}
