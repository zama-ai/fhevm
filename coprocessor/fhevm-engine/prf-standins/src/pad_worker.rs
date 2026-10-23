//! Stand-in for the RFC-038 pad-worker: claims pending `pads` rows, writes a deterministic keccak
//! pad in place of the FHE PRF output, and signs the attestation with the coprocessor key. Nobody
//! decrypts these pads.

use crate::faults::{Fault, Faults};
use alloy::{
    primitives::{Address, B256},
    signers::local::PrivateKeySigner,
};
use anyhow::Context;
use ciphertext_attestation::{
    keccak_b256, prf::Version, PrfOutputAttestation, PrfOutputAttestationPayload,
};
use prf_service::{path::block_bits, PrfPath};
use sqlx::{postgres::PgListener, PgPool, Row};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Notify, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

/// Sent by a trigger on `pads` when a row enters the queue.
const PAD_WORK_AVAILABLE: &str = "pad_work_available";

pub struct PadWorkerConfig {
    pub faults: Faults,
    /// Rows evaluated at once: the role of RFC-038's `PRF_MAX_CONCURRENT`.
    pub concurrency: usize,
    /// A claimed row whose lease is older than this goes back to the queue. The worker renews the
    /// lease while it evaluates, so only a dead worker loses its rows.
    pub claim_timeout: Duration,
    /// After this many claims, an expired row is marked failed.
    pub max_attempts: i32,
    pub poll_interval: Duration,
    /// Bytes of each PRF block in the stand-in ciphertexts, to give bodies a realistic size. Every
    /// party must use the same value, or their digests differ.
    pub block_bytes: usize,
}

pub struct Pad {
    pub ct64: Vec<u8>,
    pub ct128: Vec<u8>,
}

/// `block_bytes` of chained keccak for each PRF block. `salt` makes a pad only one party computes.
pub fn keccak_pad(
    path: &PrfPath,
    block_bits: u32,
    block_bytes: usize,
    salt: Option<Address>,
) -> Pad {
    let blocks = u32::from(path.pad_length).div_ceil(block_bits);
    let salt = salt.map(|s| s.to_vec()).unwrap_or_default();
    let entries = |tag: &[u8]| -> Vec<u8> {
        (0..blocks)
            .flat_map(|i| {
                let preimage = [
                    tag,
                    &salt,
                    &path.prf.to_be_bytes(),
                    path.label.as_slice(),
                    &path.pad_length.to_be_bytes(),
                    &i.to_be_bytes(),
                ]
                .concat();
                std::iter::successors(Some(keccak_b256(&preimage)), |prev| {
                    Some(keccak_b256(prev.as_slice()))
                })
                .flat_map(|digest| digest.0)
                .take(block_bytes)
                .collect::<Vec<u8>>()
            })
            .collect()
    };
    Pad {
        ct64: entries(b"STANDIN_CT64"),
        ct128: entries(b"STANDIN_CT128"),
    }
}

struct Claimed {
    path: PrfPath,
    retry_count: i32,
}

pub struct PadWorker {
    pool: PgPool,
    signer: PrivateKeySigner,
    config: PadWorkerConfig,
    work_available: Notify,
}

impl PadWorker {
    pub fn new(pool: PgPool, signer: PrivateKeySigner, config: PadWorkerConfig) -> Arc<Self> {
        Arc::new(Self {
            pool,
            signer,
            config,
            work_available: Notify::new(),
        })
    }

    /// Runs until `cancel`, or until a task fails.
    pub async fn run(self: Arc<Self>, cancel: CancellationToken) -> anyhow::Result<()> {
        info!(signer = %self.signer.address(), faults = ?self.config.faults, "stand-in pad-worker started");
        let mut tasks = JoinSet::new();
        tasks.spawn(self.clone().listen(cancel.clone()));
        tasks.spawn(self.clone().reap(cancel.clone()));
        for _ in 0..self.config.concurrency {
            tasks.spawn(self.clone().work(cancel.clone()));
        }
        let result = match tasks.join_next().await {
            Some(res) => res.context("pad-worker task panicked")?,
            None => Ok(()),
        };
        cancel.cancel();
        tasks.shutdown().await;
        result
    }

    async fn listen(self: Arc<Self>, cancel: CancellationToken) -> anyhow::Result<()> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(PAD_WORK_AVAILABLE).await?;
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(()),
                notification = listener.try_recv() => {
                    notification?;
                    self.work_available.notify_waiters();
                }
            }
        }
    }

    async fn reap(self: Arc<Self>, cancel: CancellationToken) -> anyhow::Result<()> {
        let timeout = self.config.claim_timeout.as_secs_f64();
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(()),
                _ = tokio::time::sleep(self.config.claim_timeout / 2) => {}
            }
            let released = sqlx::query(
                "UPDATE pads SET claimed_at = NULL, retry_count = retry_count + 1
                  WHERE claimed_at < NOW() - $1 * INTERVAL '1 second'
                    AND signature IS NULL AND NOT is_error AND retry_count + 1 < $2",
            )
            .bind(timeout)
            .bind(self.config.max_attempts)
            .execute(&self.pool)
            .await?
            .rows_affected();
            let failed = sqlx::query(
                "UPDATE pads SET is_error = TRUE
                  WHERE claimed_at < NOW() - $1 * INTERVAL '1 second'
                    AND signature IS NULL AND NOT is_error",
            )
            .bind(timeout)
            .execute(&self.pool)
            .await?
            .rows_affected();
            if released + failed > 0 {
                warn!(released, failed, "reaped rows whose lease expired");
            }
        }
    }

    async fn work(self: Arc<Self>, cancel: CancellationToken) -> anyhow::Result<()> {
        loop {
            let woken = self.work_available.notified();
            tokio::pin!(woken);
            woken.as_mut().enable();
            match self.claim().await {
                Ok(Some(claimed)) => {
                    if let Err(err) = self.evaluate(&claimed).await {
                        error!(label = %claimed.path.label, error = %err, "evaluation failed");
                    }
                    continue;
                }
                Ok(None) => {}
                Err(err) => warn!(error = %err, "claim failed"),
            }
            tokio::select! {
                _ = cancel.cancelled() => return Ok(()),
                _ = &mut woken => {}
                _ = tokio::time::sleep(self.config.poll_interval) => {}
            }
        }
    }

    async fn claim(&self) -> anyhow::Result<Option<Claimed>> {
        let row = sqlx::query(
            "UPDATE pads SET claimed_at = NOW()
              WHERE (prf_instance, label, pad_length) = (
                    SELECT prf_instance, label, pad_length FROM pads
                     WHERE claimed_at IS NULL AND signature IS NULL AND NOT is_error
                     ORDER BY created_at
                     LIMIT 1
                       FOR UPDATE SKIP LOCKED)
              RETURNING prf_instance, label, pad_length, retry_count",
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else { return Ok(None) };
        let label: Vec<u8> = row.get("label");
        let claimed = Claimed {
            path: PrfPath {
                prf: u16::try_from(row.get::<i32, _>("prf_instance"))?,
                label: B256::try_from(label.as_slice())?,
                pad_length: u16::try_from(row.get::<i32, _>("pad_length"))?,
            },
            retry_count: row.get("retry_count"),
        };
        info!(
            label = %claimed.path.label,
            pad_length = claimed.path.pad_length,
            attempt = claimed.retry_count + 1,
            "claimed pad"
        );
        Ok(Some(claimed))
    }

    async fn evaluate(&self, claimed: &Claimed) -> anyhow::Result<()> {
        let path = &claimed.path;
        let fault = self.config.faults.for_label(&path.label);
        match fault {
            Some(Fault::CrashOnce) if claimed.retry_count == 0 => {
                error!(label = %path.label, "crash_once fault: exiting with the row claimed");
                std::process::exit(1);
            }
            Some(Fault::Fail) => {
                sqlx::query(
                    "UPDATE pads SET is_error = TRUE
                      WHERE prf_instance = $1 AND label = $2 AND pad_length = $3 AND signature IS NULL",
                )
                .bind(i32::from(path.prf))
                .bind(path.label.as_slice())
                .bind(i32::from(path.pad_length))
                .execute(&self.pool)
                .await?;
                info!(label = %path.label, pad_length = path.pad_length, fault = "fail", "marked pad failed");
                return Ok(());
            }
            Some(Fault::Delay(delay)) => self.hold_lease(path, *delay).await?,
            _ => {}
        }

        let block_bits = block_bits(path.prf).context("unknown prf in a pads row")?;
        let salt = matches!(fault, Some(Fault::WrongDigest)).then(|| self.signer.address());
        let pad = keccak_pad(path, block_bits, self.config.block_bytes, salt);
        let payload = PrfOutputAttestationPayload::new(
            Version::V1,
            path.prf,
            path.label,
            path.pad_length,
            keccak_b256(&pad.ct64),
            keccak_b256(&pad.ct128),
        );
        let attestation = match fault {
            Some(Fault::Unsigned) => PrfOutputAttestation {
                version: payload.version,
                ciphertext_digest: payload.ciphertext_digest,
                sns_ciphertext_digest: payload.sns_ciphertext_digest,
                signer: self.signer.address(),
                signature: vec![0; 65],
            },
            Some(Fault::ForeignSigner) => payload.sign(&PrivateKeySigner::random()).await?,
            _ => payload.sign(&self.signer).await?,
        };
        let written = sqlx::query(
            "UPDATE pads
                SET ciphertext128 = $4, ciphertext_digest = $5, sns_ciphertext_digest = $6,
                    signer = $7, signature = $8
              WHERE prf_instance = $1 AND label = $2 AND pad_length = $3
                AND signature IS NULL AND NOT is_error",
        )
        .bind(i32::from(path.prf))
        .bind(path.label.as_slice())
        .bind(i32::from(path.pad_length))
        .bind(&pad.ct128)
        .bind(attestation.ciphertext_digest.as_slice())
        .bind(attestation.sns_ciphertext_digest.as_slice())
        .bind(attestation.signer.as_slice())
        .bind(&attestation.signature)
        .execute(&self.pool)
        .await?
        .rows_affected();
        info!(
            label = %path.label,
            pad_length = path.pad_length,
            attempt = claimed.retry_count + 1,
            fault = fault.map(ToString::to_string).unwrap_or_default(),
            sns_digest = %attestation.sns_ciphertext_digest,
            written = written == 1,
            "evaluated pad"
        );
        Ok(())
    }

    /// Sleeps `delay`, renewing the claim so the reaper leaves the row alone.
    async fn hold_lease(&self, path: &PrfPath, delay: Duration) -> anyhow::Result<()> {
        let renew_every = self.config.claim_timeout / 3;
        let deadline = tokio::time::Instant::now() + delay;
        while tokio::time::Instant::now() < deadline {
            tokio::time::sleep_until(deadline.min(tokio::time::Instant::now() + renew_every)).await;
            sqlx::query(
                "UPDATE pads SET claimed_at = NOW()
                  WHERE prf_instance = $1 AND label = $2 AND pad_length = $3 AND signature IS NULL",
            )
            .bind(i32::from(path.prf))
            .bind(path.label.as_slice())
            .bind(i32::from(path.pad_length))
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
}
