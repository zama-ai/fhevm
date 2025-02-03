// Re-export public modules
pub mod config;
pub mod constants;
pub mod errors;
pub mod ethereum;
pub mod event;
pub mod service;
pub mod orchestrator;

// Re-export commonly used types
pub use errors::Error;
pub use event::registry::EventRegistry;
pub use service::handler::RealEventHandler;
