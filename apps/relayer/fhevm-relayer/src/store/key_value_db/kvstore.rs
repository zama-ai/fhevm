use anyhow::Result;
use async_trait::async_trait;

/// Trait for a pluggable key-value store backend.
/// This defines the core operations that any KV store implementation must provide.
#[async_trait]
pub trait KVStore: Send + Sync + 'static {
    /// Store a value with the given key.
    async fn put(&self, key: &str, value: &str) -> Result<()>;

    /// Get a value by key.
    async fn get(&self, key: &str) -> Result<Option<String>>;

    /// Delete a value by key.
    async fn delete(&self, key: &str) -> Result<()>;

    /// Delete all key-value pairs with keys having a given prefix.
    async fn delete_by_prefix(&self, prefix: &str) -> Result<()>;

    /// Get all key-value pairs with keys having a given prefix.
    async fn get_by_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>>;
}
