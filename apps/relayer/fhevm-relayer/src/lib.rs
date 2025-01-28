// Re-export public modules
pub mod common;
pub mod config;
pub mod errors;
pub mod ethereum;
pub mod event;
pub mod service;

// Re-export commonly used types
pub use errors::Error;
pub use event::{processor::EventProcessor, registry::EventRegistry, types::EventType};
pub use service::handler::RealEventHandler;
