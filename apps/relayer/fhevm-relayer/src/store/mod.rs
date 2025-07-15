//! Store module provides functionality to persist different types of data.
//!
//! It operates in two layers.
//! 1. A generic key value store layer. Eg: in-memory, rocks db etc.
//! 2. A data translation layer for storing different kinds of data . Eg: EventStore.

// Export the store components and traits
mod block_number;
mod event;
pub mod key_value_db;
mod pub_decrypt_cache;
mod user_decrypt_cache;

// Re-export for easier access
pub use block_number::BlockNumberStore;
pub use event::EventStore;
pub use pub_decrypt_cache::PublicDecryptCacheStore;
pub use user_decrypt_cache::{UserDecryptRequestCacheStore, UserDecryptResponseCacheStore};
