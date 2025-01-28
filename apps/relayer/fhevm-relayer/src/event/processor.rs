use crate::errors::Result;
use alloy::rpc::types::Log as RpcLog;
use std::sync::Arc;

pub trait EventProcessor: Send + Sync {
    fn process_event(&self, log: &RpcLog) -> Result<()>;
}

/// Type alias for dynamic event handlers.
pub type EventProcessorBox = Arc<dyn EventProcessor>;
