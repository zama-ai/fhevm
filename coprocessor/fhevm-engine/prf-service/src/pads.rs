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
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// The response body and the attestation of a complete row.
#[derive(Debug, Clone, PartialEq)]
pub struct PadOutput {
    pub ciphertext128: Vec<u8>,
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
}

/// Admits requests in one pod.
pub struct Admissions {
    pool: PgPool,
    max_pending_bits: i64,
    max_adds_per_connector: NonZeroUsize,
    /// One semaphore for each connector that added a row in this pod.
    adds: Mutex<HashMap<Address, Arc<Semaphore>>>,
}

impl Admissions {
    pub fn new(pool: PgPool, max_pending_bits: i64, max_adds_per_connector: NonZeroUsize) -> Self {
        Self {
            pool,
            max_pending_bits,
            max_adds_per_connector,
            adds: Mutex::default(),
        }
    }

    /// Joins the row of the request, or adds the row if `connector` has budget for it.
    // TODO: take a signer that the FhevmSig check gives, not an Address. Do it after fhevm#4558.
    pub async fn admit(
        &self,
        path: &PrfOutputRef,
        connector: Address,
    ) -> anyhow::Result<Admission> {
        if let Some(state) = read(&self.pool, path).await? {
            return Ok(Admission::Existing(state));
        }
        // Wait in memory: a request that waits for the lock in Postgres holds a connection.
        let slot = self.add_slot(connector).await;
        let (pool, max_pending_bits, path) = (self.pool.clone(), self.max_pending_bits, *path);
        // In a task, so a cancelled request keeps its slot until its connection is free.
        tokio::spawn(async move {
            let _slot = slot;
            add(&pool, max_pending_bits, &path, connector).await
        })
        .await?
    }

    async fn add_slot(&self, connector: Address) -> OwnedSemaphorePermit {
        let semaphore = self
            .adds
            .lock()
            .expect("adds lock")
            .entry(connector)
            .or_insert_with(|| Arc::new(Semaphore::new(self.max_adds_per_connector.get())))
            .clone();
        semaphore
            .acquire_owned()
            .await
            .expect("the semaphore is never closed")
    }
}

async fn add(
    pool: &PgPool,
    max_pending_bits: i64,
    path: &PrfOutputRef,
    connector: Address,
) -> anyhow::Result<Admission> {
    let mut tx = pool.begin().await?;
    // Lock the connector, so two requests of one connector cannot use the same budget.
    sqlx::query!(
        "SELECT pg_advisory_xact_lock(hashtextextended('pads:' || encode($1, 'hex'), 0))",
        connector.as_slice(),
    )
    .execute(tx.as_mut())
    .await?;
    let pending = sqlx::query_scalar!(
        r#"SELECT COALESCE(SUM(pad_length), 0)::BIGINT AS "pending!"
                 FROM pads
                WHERE triggered_by = $1 AND signature IS NULL AND NOT is_error"#,
        connector.as_slice(),
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

/// Reads the row of `path`, if it exists.
pub async fn read(pool: &PgPool, path: &PrfOutputRef) -> anyhow::Result<Option<PadState>> {
    let Some(row) = sqlx::query!(
        "SELECT is_error, ciphertext128, ciphertext_digest, sns_ciphertext_digest, signer, signature
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

    if row.is_error {
        return Ok(Some(PadState::Failed));
    }
    // A CHECK makes the output columns all set or all NULL.
    let (Some(ciphertext128), Some(digest), Some(sns_digest), Some(signer), Some(signature)) = (
        row.ciphertext128,
        row.ciphertext_digest,
        row.sns_ciphertext_digest,
        row.signer,
        row.signature,
    ) else {
        return Ok(Some(PadState::Pending));
    };
    let attestation = PrfOutputAttestation {
        version: Version::V1,
        ciphertext_digest: B256::try_from(digest.as_slice()).context("ciphertext_digest")?,
        sns_ciphertext_digest: B256::try_from(sns_digest.as_slice())
            .context("sns_ciphertext_digest")?,
        signer: Address::try_from(signer.as_slice()).context("signer")?,
        signature,
    };
    Ok(Some(PadState::Complete(PadOutput {
        ciphertext128,
        attestation,
    })))
}
