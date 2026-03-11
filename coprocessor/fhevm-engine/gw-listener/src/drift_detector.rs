use std::collections::HashMap;

use alloy::primitives::FixedBytes;
use fhevm_engine_common::utils::to_hex;
use sqlx::{Pool, Postgres, Row};
use tracing::{debug, warn};

use crate::metrics::DRIFT_DETECTED_COUNTER;

use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits;

#[derive(Clone, Copy)]
struct ObservedDigests {
    ciphertext_digest: FixedBytes<32>,
    ciphertext128_digest: FixedBytes<32>,
}

pub(crate) struct DriftDetector {
    first_seen_submissions: HashMap<FixedBytes<32>, ObservedDigests>,
    /// Deferred metric increments, flushed after the block's DB update succeeds.
    deferred_drift_count: u64,
}

impl DriftDetector {
    pub(crate) fn new() -> Self {
        Self {
            first_seen_submissions: HashMap::new(),
            deferred_drift_count: 0,
        }
    }

    pub(crate) fn observe_submission(&mut self, event: CiphertextCommits::AddCiphertextMaterial) {
        let observed = ObservedDigests {
            ciphertext_digest: event.ciphertextDigest,
            ciphertext128_digest: event.snsCiphertextDigest,
        };

        match self.first_seen_submissions.entry(event.ctHandle) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(observed);
            }
            std::collections::hash_map::Entry::Occupied(entry) => {
                let first_seen = entry.get();
                if first_seen.ciphertext_digest != observed.ciphertext_digest
                    || first_seen.ciphertext128_digest != observed.ciphertext128_digest
                {
                    warn!(
                        handle = %event.ctHandle,
                        first_seen_ciphertext_digest = %first_seen.ciphertext_digest,
                        first_seen_ciphertext128_digest = %first_seen.ciphertext128_digest,
                        new_ciphertext_digest = %observed.ciphertext_digest,
                        new_ciphertext128_digest = %observed.ciphertext128_digest,
                        coprocessor_tx_sender = %event.coprocessorTxSender,
                        source = "peer_submission",
                        "Drift detected: peer submitted a different digest"
                    );
                    self.deferred_drift_count += 1;
                }
            }
        }
    }

    pub(crate) async fn handle_consensus(
        &mut self,
        event: CiphertextCommits::AddCiphertextMaterialConsensus,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<()> {
        self.first_seen_submissions.remove(&event.ctHandle);
        let handle_bytes = event.ctHandle.as_slice();

        let row = sqlx::query(
            "SELECT ciphertext, ciphertext128 FROM ciphertext_digest WHERE handle = $1",
        )
        .bind(handle_bytes)
        .fetch_optional(db_pool)
        .await?;

        let Some(row) = row else {
            debug!(
                handle = %event.ctHandle,
                "Consensus arrived before local digest was available; skipping drift check"
            );
            return Ok(());
        };

        let local_ciphertext_digest: Option<Vec<u8>> = row.get("ciphertext");
        let local_ciphertext128_digest: Option<Vec<u8>> = row.get("ciphertext128");

        let (Some(local_ciphertext_digest), Some(local_ciphertext128_digest)) =
            (local_ciphertext_digest, local_ciphertext128_digest)
        else {
            debug!(
                handle = %event.ctHandle,
                "Consensus arrived before local digests were ready; skipping drift check"
            );
            return Ok(());
        };

        if event.ciphertextDigest.as_slice() != local_ciphertext_digest.as_slice()
            || event.snsCiphertextDigest.as_slice() != local_ciphertext128_digest.as_slice()
        {
            warn!(
                handle = %event.ctHandle,
                consensus_ciphertext_digest = %event.ciphertextDigest,
                consensus_ciphertext128_digest = %event.snsCiphertextDigest,
                local_ciphertext_digest = %to_hex(&local_ciphertext_digest),
                local_ciphertext128_digest = %to_hex(&local_ciphertext128_digest),
                key_id = %event.keyId,
                source = "consensus",
                "Drift detected: local digest does not match consensus"
            );
            self.deferred_drift_count += 1;
        }

        Ok(())
    }

    /// Flush deferred metric increments. Call after the block's DB update succeeds.
    pub(crate) fn flush_metrics(&mut self) {
        if self.deferred_drift_count == 0 {
            return;
        }

        DRIFT_DETECTED_COUNTER.inc_by(self.deferred_drift_count);
        self.deferred_drift_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, FixedBytes, U256};

    fn make_submission_event(
        handle: FixedBytes<32>,
        ciphertext_digest: FixedBytes<32>,
        ciphertext128_digest: FixedBytes<32>,
        sender: Address,
    ) -> CiphertextCommits::AddCiphertextMaterial {
        CiphertextCommits::AddCiphertextMaterial {
            ctHandle: handle,
            keyId: U256::from(1),
            ciphertextDigest: ciphertext_digest,
            snsCiphertextDigest: ciphertext128_digest,
            coprocessorTxSender: sender,
        }
    }

    fn make_consensus_event(
        handle: FixedBytes<32>,
        ct64_digest: FixedBytes<32>,
        ct128_digest: FixedBytes<32>,
    ) -> CiphertextCommits::AddCiphertextMaterialConsensus {
        CiphertextCommits::AddCiphertextMaterialConsensus {
            ctHandle: handle,
            keyId: U256::from(1),
            ciphertextDigest: ct64_digest,
            snsCiphertextDigest: ct128_digest,
            coprocessorTxSenders: vec![Address::ZERO],
        }
    }

    #[test]
    fn matching_submissions_keep_single_first_seen_digest() {
        let mut d = DriftDetector::new();
        let handle = FixedBytes::from([1u8; 32]);
        let ciphertext_digest = FixedBytes::from([2u8; 32]);
        let ciphertext128_digest = FixedBytes::from([3u8; 32]);

        d.observe_submission(make_submission_event(
            handle,
            ciphertext_digest,
            ciphertext128_digest,
            Address::left_padding_from(&[1]),
        ));
        d.observe_submission(make_submission_event(
            handle,
            ciphertext_digest,
            ciphertext128_digest,
            Address::left_padding_from(&[2]),
        ));

        assert_eq!(d.deferred_drift_count, 0);
        assert_eq!(d.first_seen_submissions.len(), 1);
        let first_seen = d.first_seen_submissions.get(&handle).unwrap();
        assert_eq!(first_seen.ciphertext_digest, ciphertext_digest);
        assert_eq!(first_seen.ciphertext128_digest, ciphertext128_digest);
    }

    #[test]
    fn differing_submissions_increment_drift_counter() {
        let mut d = DriftDetector::new();
        let handle = FixedBytes::from([1u8; 32]);

        d.observe_submission(make_submission_event(
            handle,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            Address::left_padding_from(&[1]),
        ));
        d.observe_submission(make_submission_event(
            handle,
            FixedBytes::from([9u8; 32]), // different ct64
            FixedBytes::from([3u8; 32]),
            Address::left_padding_from(&[2]),
        ));

        assert_eq!(d.deferred_drift_count, 1);
        assert_eq!(d.first_seen_submissions.len(), 1);
    }

    #[test]
    fn differing_ciphertext128_only_increments_drift_counter() {
        let mut d = DriftDetector::new();
        let handle = FixedBytes::from([1u8; 32]);

        d.observe_submission(make_submission_event(
            handle,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            Address::left_padding_from(&[1]),
        ));
        d.observe_submission(make_submission_event(
            handle,
            FixedBytes::from([2u8; 32]), // same ct64
            FixedBytes::from([9u8; 32]), // different ct128
            Address::left_padding_from(&[2]),
        ));

        assert_eq!(d.deferred_drift_count, 1);
    }

    #[test]
    fn flush_noop_when_all_zero() {
        let mut d = DriftDetector::new();
        d.flush_metrics();
        assert_eq!(d.deferred_drift_count, 0);
    }

    #[test]
    fn flush_resets_counters() {
        let mut d = DriftDetector::new();
        d.deferred_drift_count = 3;
        d.flush_metrics();
        assert_eq!(d.deferred_drift_count, 0);
    }

    use serial_test::serial;
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use test_harness::db_utils::insert_ciphertext_digest;
    use test_harness::instance::ImportMode;

    async fn setup_db() -> (Pool<Postgres>, Option<test_harness::instance::DBInstance>) {
        let instance = test_harness::instance::setup_test_db(ImportMode::None)
            .await
            .expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .connect(instance.db_url.as_str())
            .await
            .expect("pool");
        sqlx::query("TRUNCATE ciphertext_digest")
            .execute(&pool)
            .await
            .expect("truncate");
        (pool, Some(instance))
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_match() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAA; 32];
        let ciphertext_digest = [0xBB; 32];
        let ciphertext128_digest = [0xCC; 32];

        insert_ciphertext_digest(
            &pool,
            12345,
            [0u8; 32],
            &handle,
            &ciphertext_digest,
            &ciphertext128_digest,
            0,
        )
        .await
        .unwrap();

        let mut d = DriftDetector::new();
        d.handle_consensus(
            make_consensus_event(
                FixedBytes::from(handle),
                FixedBytes::from(ciphertext_digest),
                FixedBytes::from(ciphertext128_digest),
            ),
            &pool,
        )
        .await
        .unwrap();

        assert_eq!(d.deferred_drift_count, 0);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_mismatch() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAA; 32];

        insert_ciphertext_digest(
            &pool,
            12345,
            [0u8; 32],
            &handle,
            &[0xBB; 32], // local ct64
            &[0xCC; 32], // local ct128
            0,
        )
        .await
        .unwrap();

        let mut d = DriftDetector::new();
        d.handle_consensus(
            make_consensus_event(
                FixedBytes::from(handle),
                FixedBytes::from([0xFF; 32]), // different consensus ct64
                FixedBytes::from([0xCC; 32]),
            ),
            &pool,
        )
        .await
        .unwrap();

        assert_eq!(d.deferred_drift_count, 1);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_handle_not_in_db_is_ignored() {
        let (pool, _inst) = setup_db().await;

        let mut d = DriftDetector::new();
        d.handle_consensus(
            make_consensus_event(
                FixedBytes::from([0xDD; 32]),
                FixedBytes::from([0xEE; 32]),
                FixedBytes::from([0xFF; 32]),
            ),
            &pool,
        )
        .await
        .unwrap();

        assert_eq!(d.deferred_drift_count, 0);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_null_digests_are_ignored() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAA; 32];

        sqlx::query(
            "INSERT INTO ciphertext_digest (host_chain_id, key_id_gw, handle) VALUES ($1, $2, $3)",
        )
        .bind(12345_i64)
        .bind(&[0u8; 32][..])
        .bind(&handle[..])
        .execute(&pool)
        .await
        .unwrap();

        let mut d = DriftDetector::new();
        d.handle_consensus(
            make_consensus_event(
                FixedBytes::from(handle),
                FixedBytes::from([0xBB; 32]),
                FixedBytes::from([0xCC; 32]),
            ),
            &pool,
        )
        .await
        .unwrap();

        assert_eq!(d.deferred_drift_count, 0);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_cleans_up_submissions_buffer() {
        let (pool, _inst) = setup_db().await;
        let handle = FixedBytes::from([0xAA; 32]);

        let mut d = DriftDetector::new();
        d.observe_submission(make_submission_event(
            handle,
            FixedBytes::from([0xBB; 32]),
            FixedBytes::from([0xCC; 32]),
            Address::ZERO,
        ));
        assert!(d.first_seen_submissions.contains_key(&handle));

        d.handle_consensus(
            make_consensus_event(
                handle,
                FixedBytes::from([0xBB; 32]),
                FixedBytes::from([0xCC; 32]),
            ),
            &pool,
        )
        .await
        .unwrap();

        assert!(!d.first_seen_submissions.contains_key(&handle));
    }
}
