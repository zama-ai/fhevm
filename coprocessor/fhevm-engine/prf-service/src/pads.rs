//! Adds requests to the `pads` table, within the budget of each connector.

use alloy::primitives::{Address, B256};
use anyhow::Context;
use ciphertext_attestation::PrfOutputRef;
use ciphertext_attestation::{prf::Version, PrfOutputAttestation};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Locks the budget of the connector in `$1` until the end of the transaction.
pub const CONNECTOR_LOCK: &str =
    "SELECT pg_advisory_xact_lock(hashtextextended('pads:' || encode($1, 'hex'), 0))";

type Adds = Arc<Mutex<HashMap<Address, Arc<Semaphore>>>>;

/// The attestation and the body length of a complete row.
#[derive(Debug, Clone, PartialEq)]
pub struct PadOutput {
    pub body_length: u64,
    pub attestation: PrfOutputAttestation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PadState {
    Pending,
    Complete(PadOutput),
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Admission {
    /// This request added the row. Its connector pays for the evaluation.
    Created,
    /// The row exists. The request pays nothing.
    Existing(PadState),
    /// The connector has no budget for this row.
    OverBudget,
    /// The row is pending and expired: no lane computes it.
    Expired,
    /// The lane of the instance has no heartbeat, or an old one: no row is added.
    NoLane,
}

/// Admits requests in one pod.
pub struct Admissions {
    pool: PgPool,
    max_pending_bits: i64,
    max_adds_per_connector: NonZeroUsize,
    /// Pending rows older than this do not count in the budget.
    pending_max_age: Duration,
    /// A lane with an older heartbeat is down.
    lane_heartbeat_max_age: Duration,
    /// One semaphore for each connector with an add in progress in this pod.
    adds: Adds,
}

impl Admissions {
    pub fn new(
        pool: PgPool,
        max_pending_bits: i64,
        max_adds_per_connector: NonZeroUsize,
        pending_max_age: Duration,
        lane_heartbeat_max_age: Duration,
    ) -> Self {
        Self {
            pool,
            max_pending_bits,
            max_adds_per_connector,
            pending_max_age,
            lane_heartbeat_max_age,
            adds: Adds::default(),
        }
    }

    /// Joins the row of the request, or adds the row if `connector` has budget for it.
    /// `connector` must be the signer that coprocessor-proxy verified.
    pub async fn admit(
        &self,
        path: &PrfOutputRef,
        connector: Address,
    ) -> anyhow::Result<Admission> {
        let row = read_for_admission(
            &self.pool,
            path,
            self.pending_max_age,
            self.lane_heartbeat_max_age,
        )
        .await?;
        match row.state {
            // A complete or failed row is served also when the lane is down.
            Some(PadState::Pending) if row.expired => return Ok(Admission::Expired),
            Some(PadState::Pending) if !row.lane_live => return Ok(Admission::NoLane),
            Some(state) => return Ok(Admission::Existing(state)),
            None if !row.lane_live => return Ok(Admission::NoLane),
            None => {}
        }
        // Wait in memory: a request that waits for the lock in Postgres holds a connection.
        let slot = self.add_slot(connector).await;
        let (pool, max_pending_bits, pending_max_age, path) = (
            self.pool.clone(),
            self.max_pending_bits,
            self.pending_max_age,
            *path,
        );
        // In a task, so a cancelled request keeps its slot until its connection is free.
        tokio::spawn(async move {
            let _slot = slot;
            add(&pool, max_pending_bits, pending_max_age, &path, connector).await
        })
        .await?
    }

    async fn add_slot(&self, connector: Address) -> AddSlot {
        let semaphore = self
            .adds
            .lock()
            .expect("adds lock")
            .entry(connector)
            .or_insert_with(|| Arc::new(Semaphore::new(self.max_adds_per_connector.get())))
            .clone();
        let permit = semaphore
            .acquire_owned()
            .await
            .expect("the semaphore is never closed");
        AddSlot {
            adds: self.adds.clone(),
            connector,
            permit: Some(permit),
        }
    }
}

/// An add of one connector. On drop, it removes the semaphore of the connector if no other add uses it.
struct AddSlot {
    adds: Adds,
    connector: Address,
    permit: Option<OwnedSemaphorePermit>,
}

impl Drop for AddSlot {
    fn drop(&mut self) {
        let mut adds = self.adds.lock().expect("adds lock");
        // Release the permit under the lock, so the last add to drop always sees that it is the last.
        drop(self.permit.take());
        if adds
            .get(&self.connector)
            .is_some_and(|semaphore| Arc::strong_count(semaphore) == 1)
        {
            adds.remove(&self.connector);
        }
    }
}

async fn add(
    pool: &PgPool,
    max_pending_bits: i64,
    pending_max_age: Duration,
    path: &PrfOutputRef,
    connector: Address,
) -> anyhow::Result<Admission> {
    let mut tx = pool.begin().await?;
    // Lock the connector, so two requests of one connector cannot use the same budget.
    sqlx::query(CONNECTOR_LOCK)
        .bind(connector.as_slice())
        .execute(tx.as_mut())
        .await?;
    let pending = sqlx::query_scalar!(
        r#"SELECT COALESCE(SUM(pad_length), 0)::BIGINT AS "pending!"
                 FROM pads
                WHERE triggered_by = $1 AND signature IS NULL AND NOT is_error
                  AND created_at > NOW() - make_interval(secs => $2)"#,
        connector.as_slice(),
        pending_max_age.as_secs_f64(),
    )
    .fetch_one(tx.as_mut())
    .await?;
    if pending + i64::from(path.pad_length) > max_pending_bits {
        drop(tx);
        // Another connector can add the row after the first read. Then this request joins it.
        return Ok(match read(pool, path).await? {
            Some(state) => Admission::Existing(state),
            None => Admission::OverBudget,
        });
    }
    let inserted = sqlx::query!(
        "INSERT INTO pads (prf_instance, label, pad_length, triggered_by)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT DO NOTHING",
        i32::from(path.prf_id),
        path.label.as_slice(),
        i32::from(path.pad_length),
        connector.as_slice(),
    )
    .execute(tx.as_mut())
    .await?
    .rows_affected();
    tx.commit().await?;

    // No row inserted: another request added it at the same time.
    Ok(match inserted {
        0 => Admission::Existing(PadState::Pending),
        _ => Admission::Created,
    })
}

/// The row of a request, and the lane of its instance.
struct AdmissionRow {
    state: Option<PadState>,
    /// The row is older than `pending_max_age`.
    expired: bool,
    /// The lane has a heartbeat newer than `lane_heartbeat_max_age`.
    lane_live: bool,
}

/// Reads the row of `path`, its age and the lane of its instance, in one query.
async fn read_for_admission(
    pool: &PgPool,
    path: &PrfOutputRef,
    pending_max_age: Duration,
    lane_heartbeat_max_age: Duration,
) -> anyhow::Result<AdmissionRow> {
    let row = sqlx::query!(
        r#"SELECT p.is_error AS "is_error?", OCTET_LENGTH(p.ciphertext128) AS body_length,
                  p.attestation_version AS "attestation_version?",
                  p.ciphertext_digest AS "ciphertext_digest?",
                  p.ciphertext128_digest AS "ciphertext128_digest?", p.signer AS "signer?",
                  p.signature AS "signature?",
                  COALESCE(p.created_at <= NOW() - make_interval(secs => $4), FALSE) AS "expired!",
                  EXISTS (
                      SELECT 1 FROM prf_lanes
                       WHERE prf_instance = $1 AND heartbeat_at > NOW() - make_interval(secs => $5)
                  ) AS "lane_live!"
             FROM (VALUES (1)) AS one
             LEFT JOIN pads p
               ON p.prf_instance = $1 AND p.label = $2 AND p.pad_length = $3"#,
        i32::from(path.prf_id),
        path.label.as_slice(),
        i32::from(path.pad_length),
        pending_max_age.as_secs_f64(),
        lane_heartbeat_max_age.as_secs_f64(),
    )
    .fetch_one(pool)
    .await?;
    let state = match row.is_error {
        None => None,
        Some(is_error) => Some(pad_state(
            is_error,
            row.body_length,
            attestation(
                row.attestation_version,
                row.ciphertext_digest,
                row.ciphertext128_digest,
                row.signer,
                row.signature,
            )?,
        )?),
    };
    Ok(AdmissionRow {
        state,
        expired: row.expired,
        lane_live: row.lane_live,
    })
}

/// Reads the row of `path`, if it exists, without its body.
pub async fn read(pool: &PgPool, path: &PrfOutputRef) -> anyhow::Result<Option<PadState>> {
    let Some(row) = sqlx::query!(
        "SELECT is_error, OCTET_LENGTH(ciphertext128) AS body_length, attestation_version,
                ciphertext_digest, ciphertext128_digest, signer, signature
           FROM pads
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
        i32::from(path.prf_id),
        path.label.as_slice(),
        i32::from(path.pad_length),
    )
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };

    let attestation = attestation(
        row.attestation_version,
        row.ciphertext_digest,
        row.ciphertext128_digest,
        row.signer,
        row.signature,
    )?;
    pad_state(row.is_error, row.body_length, attestation).map(Some)
}

fn pad_state(
    is_error: bool,
    body_length: Option<i32>,
    attestation: Option<PrfOutputAttestation>,
) -> anyhow::Result<PadState> {
    if is_error {
        return Ok(PadState::Failed);
    }
    let (Some(body_length), Some(attestation)) = (body_length, attestation) else {
        return Ok(PadState::Pending);
    };
    Ok(PadState::Complete(PadOutput {
        body_length: u64::try_from(body_length).context("body_length")?,
        attestation,
    }))
}

/// Reads the attestation and the body of the row of `path` in one SELECT, so they match.
/// `None` if the row has no output.
pub async fn read_output(
    pool: &PgPool,
    path: &PrfOutputRef,
) -> anyhow::Result<Option<(PrfOutputAttestation, Vec<u8>)>> {
    let Some(row) = sqlx::query!(
        "SELECT ciphertext128, attestation_version, ciphertext_digest, ciphertext128_digest,
                signer, signature
           FROM pads
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
        i32::from(path.prf_id),
        path.label.as_slice(),
        i32::from(path.pad_length),
    )
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };

    let attestation = attestation(
        row.attestation_version,
        row.ciphertext_digest,
        row.ciphertext128_digest,
        row.signer,
        row.signature,
    )?;
    Ok(attestation.zip(row.ciphertext128))
}

/// The attestation of a row. `None` if the row has no output: a CHECK sets these columns all or none.
fn attestation(
    version: Option<i16>,
    digest: Option<Vec<u8>>,
    ct128_digest: Option<Vec<u8>>,
    signer: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
) -> anyhow::Result<Option<PrfOutputAttestation>> {
    let (Some(version), Some(digest), Some(ct128_digest), Some(signer), Some(signature)) =
        (version, digest, ct128_digest, signer, signature)
    else {
        return Ok(None);
    };
    Ok(Some(PrfOutputAttestation {
        version: u8::try_from(version)
            .ok()
            .and_then(|version| Version::try_from(version).ok())
            .with_context(|| format!("unknown attestation_version {version}"))?,
        ciphertext_digest: B256::try_from(digest.as_slice()).context("ciphertext_digest")?,
        sns_ciphertext_digest: B256::try_from(ct128_digest.as_slice())
            .context("ciphertext128_digest")?,
        signer: Address::try_from(signer.as_slice()).context("signer")?,
        signature,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use tokio::time::timeout;

    const CONNECTOR: Address = Address::repeat_byte(0x11);

    fn admissions() -> Admissions {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://localhost/unused")
            .unwrap();
        Admissions::new(
            pool,
            64,
            NonZeroUsize::new(2).unwrap(),
            Duration::from_secs(600),
            Duration::from_secs(30),
        )
    }

    #[tokio::test]
    async fn adds_removes_a_connector_after_its_last_add() {
        let admissions = admissions();
        let first = admissions.add_slot(CONNECTOR).await;
        let second = admissions.add_slot(CONNECTOR).await;
        // A third add waits until one of the two ends.
        let third = admissions.add_slot(CONNECTOR);
        tokio::pin!(third);
        assert!(timeout(Duration::from_millis(50), &mut third)
            .await
            .is_err());
        drop(first);
        let third = third.await;
        assert!(
            timeout(Duration::from_millis(50), admissions.add_slot(CONNECTOR))
                .await
                .is_err()
        );

        drop(second);
        assert_eq!(admissions.adds.lock().unwrap().len(), 1);
        drop(third);
        assert!(admissions.adds.lock().unwrap().is_empty());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn adds_is_empty_after_concurrent_adds() {
        let admissions = Arc::new(admissions());
        for _ in 0..1000 {
            // All adds take a slot, then all drop at the same time.
            let barrier = Arc::new(tokio::sync::Barrier::new(2));
            let tasks: Vec<_> = (0..2)
                .map(|_| {
                    let (admissions, barrier) = (admissions.clone(), barrier.clone());
                    tokio::spawn(async move {
                        let slot = admissions.add_slot(CONNECTOR).await;
                        barrier.wait().await;
                        drop(slot);
                    })
                })
                .collect();
            for task in tasks {
                task.await.unwrap();
            }
            assert!(admissions.adds.lock().unwrap().is_empty());
        }
    }
}
