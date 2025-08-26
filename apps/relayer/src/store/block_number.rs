use crate::store::key_value_db::KVStore;
use anyhow::{anyhow, Result};
use std::sync::Arc;

const BLOCK_PREFIX: &str = "BLOCK";

pub struct BlockNumberStore {
    kv_store: Arc<dyn KVStore>,
    chain_name: String,
}

impl BlockNumberStore {
    pub fn new(kv_store: Arc<dyn KVStore>, chain_name: String) -> Self {
        Self {
            kv_store,
            chain_name,
        }
    }

    pub async fn persist_last_block_number(&self, block_number: u64) -> Result<()> {
        let key = format!("{}:{}", BLOCK_PREFIX, self.chain_name);
        self.kv_store.put(&key, &block_number.to_string()).await?;
        Ok(())
    }

    pub async fn get_last_block_number(&self) -> Result<Option<u64>> {
        let key = format!("{}:{}", BLOCK_PREFIX, self.chain_name);
        if let Some(value) = self.kv_store.get(&key).await? {
            let block_number = value
                .parse::<u64>()
                .map_err(|e| anyhow!("Invalid block number format: {}", e))?;
            Ok(Some(block_number))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::key_value_db::InMemoryKVStore;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_block_number_store() {
        let kv_store = Arc::new(InMemoryKVStore::default());
        let chain_name = "test_chain".to_string();

        let block_store = BlockNumberStore::new(kv_store, chain_name.clone());

        // Test persist_last_block_number
        block_store.persist_last_block_number(12345).await.unwrap();

        // Test get_last_block_number
        let last_block = block_store.get_last_block_number().await.unwrap();
        assert!(last_block.is_some());
        assert_eq!(last_block.unwrap(), 12345);
    }
}
