// Re-export public modules
pub mod config;
pub mod errors;
pub mod ethereum;
pub mod event;
pub mod orchestrator;
pub mod service;

// Re-export commonly used types
pub use errors::Error;
pub use event::registry::EventRegistry;
pub use service::handler::EthereumHostL1;
