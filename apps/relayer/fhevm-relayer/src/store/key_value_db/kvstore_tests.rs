use crate::store::key_value_db::KVStore;
use anyhow::Result;
use std::sync::Arc;

/// Public test suite for KVStore trait implementations.
/// Can be used outside this crate to test custom KVStore implementations.
pub mod suite {
    use super::*;
    /// Run a set of tests to verify if the implementation meets all the
    /// expectations of KVStore trait.
    pub async fn run_kvstore_interface_tests(store: Arc<dyn KVStore>) -> Result<()> {
        // Test put and get
        store.put("key1", "value1").await?;
        let v = store.get("key1").await?;
        assert_eq!(v, Some("value1".to_string()));

        // Test overwrite
        store.put("key1", "value2").await?;
        let v = store.get("key1").await?;
        assert_eq!(v, Some("value2".to_string()));

        // Test delete
        store.delete("key1").await?;
        let v = store.get("key1").await?;
        assert_eq!(v, None);

        // Test delete non-existent key (should not error)
        store.delete("nonexistent").await?;

        // Test get_by_prefix and lexical ordering for various key categories
        // Cat 1: Numeric keys
        let numeric_keys = vec!["1", "2", "10", "11", "3"];
        for k in &numeric_keys {
            store.put(k, &format!("val_{}", k)).await?;
        }
        let pairs = store.get_by_prefix("").await?;
        let mut keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        keys.sort();
        let store_keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        assert_eq!(store_keys, keys);

        // Cat 2: Alphabetic keys
        let alpha_keys = vec!["a", "b", "aa", "ab", "c"];
        for k in &alpha_keys {
            store.put(k, &format!("val_{}", k)).await?;
        }
        let pairs = store.get_by_prefix("").await?;
        let mut keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        keys.sort();
        let store_keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        assert_eq!(store_keys, keys);

        // Cat 3: Alphanumeric keys
        let alphanum_keys = vec!["a1", "a2", "b1", "b2", "c3"];
        for k in &alphanum_keys {
            store.put(k, &format!("val_{}", k)).await?;
        }
        let pairs = store.get_by_prefix("").await?;
        let mut keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        keys.sort();
        let store_keys: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        assert_eq!(store_keys, keys);

        // Test get_by_prefix for a specific prefix
        store.put("prefixA-key1", "value1").await?;
        store.put("prefixA-key2", "value2").await?;
        store.put("prefixB-key1", "value3").await?;
        let pairs = store.get_by_prefix("prefixA-").await?;
        let expected = vec![
            ("prefixA-key1".to_string(), "value1".to_string()),
            ("prefixA-key2".to_string(), "value2".to_string()),
        ];
        assert_eq!(pairs, expected);

        // Test delete_by_prefix
        store.delete_by_prefix("prefixA-").await?;
        let v1 = store.get("prefixA-key1").await?;
        let v2 = store.get("prefixA-key2").await?;
        let v3 = store.get("prefixB-key1").await?;
        assert_eq!(v1, None);
        assert_eq!(v2, None);
        assert_eq!(v3, Some("value3".to_string()));

        Ok(())
    }
}
