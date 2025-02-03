use crate::errors::EventProcessingError;
use alloy::{primitives::B256, rpc::types::Log as RpcLog};
use std::fmt::Debug;

/// Represents a processor for Ethereum contract events.
///
/// This trait should be implemented by any type that wants to process specific
/// Ethereum contract events. Implementations should handle event decoding,
/// validation, and business logic.
///
/// # Safety
///
/// All implementations must be both `Send` and `Sync` as they will be used
/// across multiple threads.
///
/// # Examples
///
/// ```rust
/// use fhevm_relayer::errors::EventProcessingError;
/// use alloy::{primitives::{B256,keccak256}, rpc::types::Log as RpcLog};
/// use fhevm_relayer::event::types::ContractEvent;
///
/// #[derive(Debug)]
/// struct MyEventProcessor;
///
/// impl ContractEvent for MyEventProcessor {
///     fn topics(&self) -> Vec<B256> {
///         vec![keccak256("MyEvent(address,uint256)")]
///     }
///
///     fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError> {
///         // Process the event
///         Ok(())
///     }
/// }
/// ```
pub trait ContractEvent: Send + Sync + Debug {
    /// Returns the list of event topics this processor handles.
    ///
    /// The topics returned should be keccak256 hashes of the event signatures.
    fn topics(&self) -> Vec<B256>;

    /// Processes an Ethereum event log.
    ///
    /// # Arguments
    ///
    /// * `log` - The Ethereum event log to process
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if processing succeeds, otherwise returns an `EventProcessingError`.
    ///
    /// # Errors
    ///
    /// Common error cases include:
    /// - Invalid event data format
    /// - Missing required fields
    /// - Business logic violations
    fn process_event(&self, log: &RpcLog) -> Result<(), EventProcessingError>;
}
