use crate::store::key_value_db::KVStore;
use anyhow::Result;
use std::sync::Arc;

/// Public test suite for KVStore trait implementations.
/// Can be used outside this crate to test custom KVStore implementations.
pub mod suite {
    use super::*;

    async fn assert_get(store: &Arc<dyn KVStore>, key: &str, expected: Option<&str>) {
        let v = store.get(key).await.unwrap();
        match expected {
            Some(val) => assert_eq!(v, Some(val.to_string()), "Key: {key}"),
            None => assert_eq!(v, None, "Key: {key}"),
        }
    }

    async fn put_many(store: &Arc<dyn KVStore>, pairs: &[(&str, &str)]) {
        for (k, v) in pairs {
            store.put(k, v).await.unwrap();
        }
    }

    async fn assert_prefix_group(
        store: &Arc<dyn KVStore>,
        prefix: &str,
        expected: Vec<(&str, &str)>,
    ) {
        let mut pairs = store.get_by_prefix(prefix).await.unwrap();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut expected_sorted: Vec<(String, String)> = expected
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        expected_sorted.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(pairs, expected_sorted, "Prefix: {prefix}");
    }

    pub async fn run_kvstore_interface_tests(store: Arc<dyn KVStore>) -> Result<()> {
        // Test put and get
        store.put("key1", "value1").await?;
        assert_get(&store, "key1", Some("value1")).await;

        // Test overwrite
        store.put("key1", "value2").await?;
        assert_get(&store, "key1", Some("value2")).await;

        // Test delete
        store.delete("key1").await?;
        assert_get(&store, "key1", None).await;

        // Test delete non-existent key (should not error)
        store.delete("nonexistent").await?;

        // Robust prefix tests: 3 groups, 4 elements each
        let group_a = [
            ("groupA-1", "A1"),
            ("groupA-2", "A2"),
            ("groupA-3", "A3"),
            ("groupA-4", "A4"),
        ];
        let group_b = [
            ("groupB-1", "B1"),
            ("groupB-2", "B2"),
            ("groupB-3", "B3"),
            ("groupB-4", "B4"),
        ];
        let group_c = [
            ("groupC-1", "C1"),
            ("groupC-2", "C2"),
            ("groupC-3", "C3"),
            ("groupC-4", "C4"),
        ];

        put_many(&store, &group_a).await;
        put_many(&store, &group_b).await;
        put_many(&store, &group_c).await;

        // Test get_by_prefix for each group
        assert_prefix_group(&store, "groupA-", group_a.to_vec()).await;
        assert_prefix_group(&store, "groupB-", group_b.to_vec()).await;
        assert_prefix_group(&store, "groupC-", group_c.to_vec()).await;

        // Test get_by_prefix for all (empty prefix returns all keys)
        let mut all_expected = group_a
            .iter()
            .chain(&group_b)
            .chain(&group_c)
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();
        all_expected.sort_by(|a, b| a.0.cmp(&b.0));
        let mut all_pairs = store.get_by_prefix("").await?;
        all_pairs.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(all_pairs, all_expected);

        // Test delete_by_prefix for each group, checking that only the current group is deleted
        for (i, (prefix, group)) in [
            ("groupA-", &group_a),
            ("groupB-", &group_b),
            ("groupC-", &group_c),
        ]
        .iter()
        .enumerate()
        {
            store.delete_by_prefix(prefix).await?;
            // Current group should be deleted
            for (k, _) in group.iter() {
                assert_get(&store, k, None).await;
            }
            // Later groups should still be present
            for later_group in [&group_a, &group_b, &group_c].iter().skip(i + 1) {
                for (k, v) in later_group.iter() {
                    assert_get(&store, k, Some(v)).await;
                }
            }
        }

        Ok(())
    }
}
