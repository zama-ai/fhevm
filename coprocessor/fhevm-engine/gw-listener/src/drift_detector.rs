use std::collections::HashMap;

use alloy::primitives::{Address, FixedBytes};
use fhevm_engine_common::utils::to_hex;
use sqlx::{Pool, Postgres, Row};
use tracing::{debug, warn};

use crate::metrics::{
    CONSENSUS_CONFIRMED_COUNTER, CONSENSUS_HANDLE_NOT_FOUND_COUNTER, DRIFT_DETECTED_COUNTER,
    DRIFT_EARLY_WARNING_COUNTER,
};

use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits;

struct PerCoprocessorDigest {
    #[allow(dead_code)] // needed for M2 recovery
    coprocessor: Address,
    ct64_digest: FixedBytes<32>,
    ct128_digest: FixedBytes<32>,
}

pub(crate) struct DriftDetector {
    submissions: HashMap<FixedBytes<32>, Vec<PerCoprocessorDigest>>,
    /// Deferred metric increments, flushed after the block's DB update succeeds.
    drift_detected: u64,
    consensus_confirmed: u64,
    drift_early_warning: u64,
    consensus_handle_not_found: u64,
}

impl DriftDetector {
    pub(crate) fn new() -> Self {
        Self {
            submissions: HashMap::new(),
            drift_detected: 0,
            consensus_confirmed: 0,
            drift_early_warning: 0,
            consensus_handle_not_found: 0,
        }
    }

    pub(crate) fn handle_add_ciphertext_material(
        &mut self,
        event: CiphertextCommits::AddCiphertextMaterial,
    ) {
        let entry = self.submissions.entry(event.ctHandle).or_default();

        let new_digest = PerCoprocessorDigest {
            coprocessor: event.coprocessorTxSender,
            ct64_digest: event.ciphertextDigest,
            ct128_digest: event.snsCiphertextDigest,
        };

        // Early warning: check if any existing entry has a different digest.
        for existing in entry.iter() {
            if existing.ct64_digest != new_digest.ct64_digest
                || existing.ct128_digest != new_digest.ct128_digest
            {
                warn!(
                    handle = %event.ctHandle,
                    our_ct64_digest = %existing.ct64_digest,
                    our_ct128_digest = %existing.ct128_digest,
                    peer_ct64_digest = %new_digest.ct64_digest,
                    peer_ct128_digest = %new_digest.ct128_digest,
                    peer_address = %event.coprocessorTxSender,
                    "Drift early warning: peer submitted different digest"
                );
                self.drift_early_warning += 1;
                break;
            }
        }

        entry.push(new_digest);
    }

    pub(crate) async fn handle_consensus(
        &mut self,
        event: CiphertextCommits::AddCiphertextMaterialConsensus,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<()> {
        // Clean up the in-memory buffer — consensus is final for this handle.
        self.submissions.remove(&event.ctHandle);

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
                "Consensus event for handle not yet computed locally"
            );
            self.consensus_handle_not_found += 1;
            return Ok(());
        };

        let local_ct64: Option<Vec<u8>> = row.get("ciphertext");
        let local_ct128: Option<Vec<u8>> = row.get("ciphertext128");

        // If both digest columns are NULL the SNS worker hasn't computed yet — treat as not found.
        let (Some(local_ct64), Some(local_ct128)) = (local_ct64, local_ct128) else {
            debug!(
                handle = %event.ctHandle,
                "Consensus event for handle whose digests are not yet computed locally"
            );
            self.consensus_handle_not_found += 1;
            return Ok(());
        };

        let ct64_match = event.ciphertextDigest.as_slice() == local_ct64.as_slice();
        let ct128_match = event.snsCiphertextDigest.as_slice() == local_ct128.as_slice();

        if ct64_match && ct128_match {
            self.consensus_confirmed += 1;
        } else {
            let local_ct64_hex = to_hex(&local_ct64);
            let local_ct128_hex = to_hex(&local_ct128);
            warn!(
                handle = %event.ctHandle,
                consensus_ct64_digest = %event.ciphertextDigest,
                consensus_ct128_digest = %event.snsCiphertextDigest,
                local_ct64_digest = %local_ct64_hex,
                local_ct128_digest = %local_ct128_hex,
                key_id = %event.keyId,
                status = "detected",
                "Drift detected: local digest does not match consensus"
            );
            self.drift_detected += 1;
        }

        Ok(())
    }

    /// Flush deferred metric increments. Call after the block's DB update succeeds.
    pub(crate) fn flush_metrics(&mut self) {
        if self.drift_detected == 0
            && self.consensus_confirmed == 0
            && self.drift_early_warning == 0
            && self.consensus_handle_not_found == 0
        {
            return;
        }

        DRIFT_DETECTED_COUNTER.inc_by(self.drift_detected);
        CONSENSUS_CONFIRMED_COUNTER.inc_by(self.consensus_confirmed);
        DRIFT_EARLY_WARNING_COUNTER.inc_by(self.drift_early_warning);
        CONSENSUS_HANDLE_NOT_FOUND_COUNTER.inc_by(self.consensus_handle_not_found);

        self.drift_detected = 0;
        self.consensus_confirmed = 0;
        self.drift_early_warning = 0;
        self.consensus_handle_not_found = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, FixedBytes, U256};

    fn make_material_event(
        handle: FixedBytes<32>,
        ct64_digest: FixedBytes<32>,
        ct128_digest: FixedBytes<32>,
        sender: Address,
    ) -> CiphertextCommits::AddCiphertextMaterial {
        CiphertextCommits::AddCiphertextMaterial {
            ctHandle: handle,
            keyId: U256::from(1),
            ciphertextDigest: ct64_digest,
            snsCiphertextDigest: ct128_digest,
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

    // ── Unit tests (no DB) ──────────────────────────────────────────

    #[test]
    fn matching_submissions_no_early_warning() {
        let mut d = DriftDetector::new();
        let handle = FixedBytes::from([1u8; 32]);
        let digest64 = FixedBytes::from([2u8; 32]);
        let digest128 = FixedBytes::from([3u8; 32]);

        d.handle_add_ciphertext_material(make_material_event(
            handle,
            digest64,
            digest128,
            Address::left_padding_from(&[1]),
        ));
        d.handle_add_ciphertext_material(make_material_event(
            handle,
            digest64,
            digest128,
            Address::left_padding_from(&[2]),
        ));

        assert_eq!(d.drift_early_warning, 0);
        assert_eq!(d.submissions[&handle].len(), 2);
    }

    #[test]
    fn differing_submissions_triggers_early_warning() {
        let mut d = DriftDetector::new();
        let handle = FixedBytes::from([1u8; 32]);

        d.handle_add_ciphertext_material(make_material_event(
            handle,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            Address::left_padding_from(&[1]),
        ));
        d.handle_add_ciphertext_material(make_material_event(
            handle,
            FixedBytes::from([9u8; 32]), // different ct64
            FixedBytes::from([3u8; 32]),
            Address::left_padding_from(&[2]),
        ));

        assert_eq!(d.drift_early_warning, 1);
    }

    #[test]
    fn differing_ct128_only_triggers_early_warning() {
        let mut d = DriftDetector::new();
        let handle = FixedBytes::from([1u8; 32]);

        d.handle_add_ciphertext_material(make_material_event(
            handle,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            Address::left_padding_from(&[1]),
        ));
        d.handle_add_ciphertext_material(make_material_event(
            handle,
            FixedBytes::from([2u8; 32]), // same ct64
            FixedBytes::from([9u8; 32]), // different ct128
            Address::left_padding_from(&[2]),
        ));

        assert_eq!(d.drift_early_warning, 1);
    }

    #[test]
    fn flush_noop_when_all_zero() {
        let mut d = DriftDetector::new();
        // Should not panic or touch atomics.
        d.flush_metrics();
        assert_eq!(d.drift_detected, 0);
    }

    #[test]
    fn flush_resets_counters() {
        let mut d = DriftDetector::new();
        d.drift_detected = 3;
        d.consensus_confirmed = 5;
        d.flush_metrics();
        assert_eq!(d.drift_detected, 0);
        assert_eq!(d.consensus_confirmed, 0);
    }

    // ── DB-backed tests ─────────────────────────────────────────────

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
        let ct64 = [0xBB; 32];
        let ct128 = [0xCC; 32];

        insert_ciphertext_digest(&pool, 12345, [0u8; 32], &handle, &ct64, &ct128, 0)
            .await
            .unwrap();

        let mut d = DriftDetector::new();
        d.handle_consensus(
            make_consensus_event(
                FixedBytes::from(handle),
                FixedBytes::from(ct64),
                FixedBytes::from(ct128),
            ),
            &pool,
        )
        .await
        .unwrap();

        assert_eq!(d.consensus_confirmed, 1);
        assert_eq!(d.drift_detected, 0);
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

        assert_eq!(d.drift_detected, 1);
        assert_eq!(d.consensus_confirmed, 0);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_handle_not_in_db() {
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

        assert_eq!(d.consensus_handle_not_found, 1);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_null_digests_treated_as_not_found() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAA; 32];

        // Insert row with NULL digests (SNS worker hasn't computed yet).
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

        assert_eq!(d.consensus_handle_not_found, 1);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_cleans_up_submissions_buffer() {
        let (pool, _inst) = setup_db().await;
        let handle = FixedBytes::from([0xAA; 32]);

        let mut d = DriftDetector::new();
        // Buffer a submission.
        d.handle_add_ciphertext_material(make_material_event(
            handle,
            FixedBytes::from([0xBB; 32]),
            FixedBytes::from([0xCC; 32]),
            Address::ZERO,
        ));
        assert!(d.submissions.contains_key(&handle));

        // Consensus removes the buffer entry (handle not in DB is fine — we still clean up).
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

        assert!(!d.submissions.contains_key(&handle));
    }
}
