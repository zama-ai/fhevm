use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rocksdb::{IteratorMode, Options, DB};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, error};

use super::kvstore::KVStore;

/// RocksDB implementation of the KVStore trait.
/// This implementation is meant for production purpose.
pub struct RocksDBKVStore {
    db: Arc<DB>,
}

impl RocksDBKVStore {
    // Open a rocks db key value store using the database stored at the given path.
    // If the directory doesn't exist, a new one is created.
    pub fn open(path: PathBuf) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, &path)?;
        Ok(Self { db: Arc::new(db) })
    }
}

#[async_trait]
impl KVStore for RocksDBKVStore {
    async fn put(&self, key: &str, value: &str) -> Result<()> {
        self.db.put(key.as_bytes(), value.as_bytes())?;
        debug!("Stored value for key: {}", key);
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<String>> {
        match self.db.get(key.as_bytes())? {
            Some(v) => {
                let s =
                    String::from_utf8(v.to_vec()).map_err(|e| anyhow!("Invalid UTF-8: {}", e))?;
                debug!("Retrieved value for key: {}, found: true", key);
                Ok(Some(s))
            }
            None => {
                debug!("Retrieved value for key: {}, found: false", key);
                Ok(None)
            }
        }
    }

    async fn delete(&self, key: &str) -> Result<()> {
        self.db.delete(key.as_bytes())?;
        debug!("Deleted key: {}", key);
        Ok(())
    }

    async fn get_by_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        let mut pairs = Vec::new();
        let iter = self.db.iterator(IteratorMode::From(
            prefix.as_bytes(),
            rocksdb::Direction::Forward,
        ));
        for item in iter {
            let (k, v) = item?;
            let key_str =
                String::from_utf8(k.to_vec()).map_err(|e| anyhow!("Invalid UTF-8: {}", e))?;
            if !key_str.starts_with(prefix) {
                break; // Stop iteration once keys no longer match the prefix
            }
            let val_str =
                String::from_utf8(v.to_vec()).map_err(|e| anyhow!("Invalid UTF-8: {}", e))?;
            pairs.push((key_str, val_str));
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        debug!("Retrieved {} pairs with prefix: {}", pairs.len(), prefix);
        Ok(pairs)
    }

    async fn delete_by_prefix(&self, prefix: &str) -> Result<()> {
        let mut batch = rocksdb::WriteBatch::default();
        println!("Deleting all keys with prefix: {}", prefix);

        let mut read_opts = rocksdb::ReadOptions::default();
        read_opts.set_iterate_range(rocksdb::PrefixRange(prefix.as_bytes()));

        let iter = self.db.iterator_opt(
            IteratorMode::From(prefix.as_bytes(), rocksdb::Direction::Forward),
            read_opts,
        );
        for item in iter {
            match item {
                Ok((k, _)) => {
                    batch.delete(&k);
                    println!("Queued key for deletion: {}", String::from_utf8_lossy(&k));
                    debug!(
                        "Queued key for deletion by prefix: {}",
                        String::from_utf8_lossy(&k)
                    );
                }
                Err(e) => {
                    error!("Error iterating over prefix: {}", e);
                    return Err(anyhow!("Error iterating over prefix: {}", e));
                }
            }
        }
        self.db.write(batch)?;
        debug!("Deleted all keys with prefix: {}", prefix);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::key_value_db::kvstore_tests::suite::run_kvstore_interface_tests;

    use tempfile::tempdir;

    #[tokio::test]
    async fn test_rocksdb_kvstore_interface() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        let store = Arc::new(RocksDBKVStore::open(path).unwrap());
        run_kvstore_interface_tests(store).await.unwrap();
        // tempdir will be cleaned up when dropped
    }

    #[tokio::test]
    async fn test_multiple_instances_should_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();

        // Create new DB and do not close it.
        let _store = RocksDBKVStore::open(path.clone()).unwrap();

        // Create another DB instances with the same path..
        let result = RocksDBKVStore::open(path);
        assert!(
            result.is_err(),
            "Opening a second instance for same DB should error"
        );
    }

    #[tokio::test]
    async fn test_reopen_existing_db_with_no_data() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();

        // Create new DB and without writing any data, close it (by dropping).
        let store = RocksDBKVStore::open(path.clone()).unwrap();
        drop(store);

        // Open same DB (should succeed, no data written)
        let _opened = RocksDBKVStore::open(path).unwrap();
    }

    #[tokio::test]
    async fn test_reopen_existing_db_with_data() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();

        // Create new DB, write data and close it (by dropping).
        let store = RocksDBKVStore::open(path.clone()).unwrap();
        store.put("key", "value").await.unwrap();
        drop(store);

        // Now, opening should error (since DB is not empty)
        let result = RocksDBKVStore::open(path);
        assert!(result.is_ok(), "Re-opening an existing DB should not error");
    }
}
