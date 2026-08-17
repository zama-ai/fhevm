use std::{collections::HashSet, time::Duration};

use alloy_primitives::{Address, B256, U256};
use sqlx::{PgPool, Postgres, Transaction};

use crate::manifest_consensus::ExecutionError;

use super::verification_scope::VerificationScope;
use super::verification_utils::{address, b256, duration_micros, internal, manifest_version, u256};

/// One pinned registry peer participating in verification, including after recovery.
///
/// `known_revisions` is an attempt-local snapshot of authenticated archive
/// rows. The downloader only fetches strictly newer listed keys, and skips
/// `rejected_object_keys` that cannot become valid.
#[derive(Clone, Debug)]
pub(super) struct ClaimedPeer {
    pub(super) publisher: Address,
    pub(super) s3_bucket_url: String,
    /// Only skips current-publication download work, never historical verification.
    pub(super) current_download_complete: bool,
    pub(super) known_revisions: HashSet<u64>,
    pub(super) rejected_object_keys: HashSet<String>,
}

/// Immutable, attempt-local ownership of one verification task.
///
/// The scope and quorum are pinned when the task is claimed so an attempt cannot
/// mix generations, publication blocks, or registry snapshots. Every processing
/// write must first prove that `worker_id` still owns the unexpired DB claim.
#[derive(Clone, Debug)]
pub(crate) struct VerificationClaim {
    pub(super) task_id: i64,
    pub(super) worker_id: String,
    pub(super) attempt: i32,
    pub(super) lease_duration: Duration,
    pub(super) required_quorum: usize,
    pub(super) scope: VerificationScope,
    pub(super) peers: Vec<ClaimedPeer>,
}

pub(super) async fn bind_one_unbound_pending_task(
    pool: &PgPool,
    generation: &str,
) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    let task_id = sqlx::query_scalar!(
        r#"
        SELECT id
          FROM block_manifest_verification_task
         WHERE state = 'pending'
           AND generation = $1
           AND required_quorum IS NULL
           AND next_attempt_at <= NOW()
         ORDER BY next_attempt_at, id
         FOR UPDATE SKIP LOCKED
         LIMIT 1
        "#,
        generation,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    if let Some(task_id) = task_id {
        bind_task_to_current_registry(&mut trx, task_id).await?;
    }
    trx.commit().await?;
    Ok(())
}

pub(super) async fn bind_task_to_current_registry(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
) -> Result<bool, ExecutionError> {
    let target = sqlx::query!(
        r#"
        SELECT task.generation,
               manifest.publisher AS local_publisher,
               manifest.version,
               manifest.coprocessor_context_id,
               manifest.host_chain_id,
               manifest.publication_block_number,
               manifest.publication_block_hash,
               task.state
          FROM block_manifest_verification_task task
          JOIN block_manifest manifest
            ON manifest.id = task.local_manifest_id
           AND manifest.generation = task.generation
         WHERE task.id = $1
         FOR UPDATE
        "#,
        task_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if target.state != "pending" {
        return Ok(false);
    }

    let registry = sqlx::query!(
        r#"
        SELECT signer_address,
               s3_bucket_url,
               coprocessor_threshold,
               gateway_chain_id,
               gateway_config_address,
               snapshot_block_number,
               snapshot_block_hash
          FROM gateway_config_coprocessors
         ORDER BY signer_address
        "#,
    )
    .fetch_all(trx.as_mut())
    .await?;
    if registry.is_empty() {
        postpone_registry_binding(trx, task_id, "GatewayConfig registry snapshot is empty").await?;
        return Ok(false);
    }

    let local_publisher = address("verification task local publisher", &target.local_publisher)?;
    let first = &registry[0];
    let threshold = first.coprocessor_threshold;
    let coprocessor_count = i32::try_from(registry.len())
        .map_err(|_| internal("registered coprocessor count exceeds INTEGER"))?;
    let required_quorum = i32::try_from(threshold)
        .map_err(|_| internal("GatewayConfig threshold exceeds INTEGER"))?;
    if required_quorum <= 0 || required_quorum > coprocessor_count {
        postpone_registry_binding(
            trx,
            task_id,
            &format!(
                "invalid GatewayConfig threshold {required_quorum} for {coprocessor_count} coprocessors"
            ),
        )
        .await?;
        return Ok(false);
    }

    let gateway_chain_id = first.gateway_chain_id;
    let gateway_config_address = first.gateway_config_address.clone();
    let snapshot_block_number = first.snapshot_block_number;
    let snapshot_block_hash = first.snapshot_block_hash.clone();
    let mut local_registered = false;
    let mut peers = Vec::with_capacity(registry.len().saturating_sub(1));
    for row in registry {
        if row.coprocessor_threshold != threshold
            || row.gateway_chain_id != gateway_chain_id
            || row.gateway_config_address != gateway_config_address
            || row.snapshot_block_number != snapshot_block_number
            || row.snapshot_block_hash != snapshot_block_hash
        {
            return Err(internal(
                "GatewayConfig registry rows do not form one consistent snapshot",
            ));
        }
        let publisher = address("GatewayConfig signer address", &row.signer_address)?;
        if publisher == local_publisher {
            local_registered = true;
        } else {
            peers.push((publisher, row.s3_bucket_url));
        }
    }
    if !local_registered {
        postpone_registry_binding(
            trx,
            task_id,
            "local manifest publisher is absent from GatewayConfig registry",
        )
        .await?;
        return Ok(false);
    }

    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET state = 'pending',
               gateway_chain_id = $2,
               gateway_config_address = $3,
               registry_block_number = $4,
               registry_block_hash = $5,
               registered_coprocessor_count = $6,
               required_quorum = $7,
               last_error = NULL,
               updated_at = NOW()
         WHERE id = $1
        "#,
        task_id,
        gateway_chain_id,
        &gateway_config_address,
        snapshot_block_number,
        &snapshot_block_hash,
        coprocessor_count,
        required_quorum,
    )
    .execute(trx.as_mut())
    .await?;
    for (publisher, s3_bucket_url) in peers {
        sqlx::query!(
            r#"
            INSERT INTO block_manifest_peer_download (
                generation,
                task_id,
                publisher,
                s3_bucket_url
            )
            VALUES ($4, $1, $2, $3)
            ON CONFLICT (generation, task_id, publisher) DO NOTHING
            "#,
            task_id,
            publisher.as_slice(),
            s3_bucket_url,
            target.generation,
        )
        .execute(trx.as_mut())
        .await?;
    }
    Ok(true)
}

async fn postpone_registry_binding(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
    error: &str,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET next_attempt_at = GREATEST(
                   eligible_at,
                   NOW() + retry_delay_micros * INTERVAL '1 microsecond'
               ),
               last_error = $2,
               updated_at = NOW()
         WHERE id = $1
        "#,
        task_id,
        error,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

pub(crate) async fn claim_due_task(
    pool: &PgPool,
    worker_id: &str,
    claim_duration: Duration,
    generation: &str,
) -> Result<Option<VerificationClaim>, ExecutionError> {
    let claim_micros = duration_micros("verification claim", claim_duration)?;
    let mut trx = pool.begin().await?;
    let row = sqlx::query!(
        r#"
        WITH selected AS (
            SELECT id
              FROM block_manifest_verification_task
             WHERE required_quorum IS NOT NULL
               AND generation = $3
               AND next_attempt_at <= NOW()
               AND (
                    state = 'pending'
                    OR (state = 'claimed' AND claim_expires_at <= NOW())
               )
             ORDER BY next_attempt_at, id
             FOR UPDATE SKIP LOCKED
             LIMIT 1
        )
        UPDATE block_manifest_verification_task target
           SET state = 'claimed',
               claim_owner = $1,
               claim_expires_at = NOW() + $2::BIGINT * INTERVAL '1 microsecond',
               updated_at = NOW()
          FROM selected, block_manifest manifest
         WHERE target.id = selected.id
           AND manifest.id = target.local_manifest_id
           AND manifest.generation = target.generation
        RETURNING target.id,
                  target.generation,
                  target.attempt_count + 1 AS "attempt!",
                  target.required_quorum AS "required_quorum!",
                  manifest.publisher AS "local_publisher!",
                  manifest.version AS "version!",
                  manifest.coprocessor_context_id AS "coprocessor_context_id!",
                  manifest.host_chain_id AS "host_chain_id!",
                  manifest.publication_block_number AS "publication_block_number!",
                  manifest.publication_block_hash AS "publication_block_hash!",
                  manifest.revision AS "revision!",
                  manifest.manifest_digest AS "local_manifest_digest!"
        "#,
        worker_id,
        claim_micros,
        generation,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    let Some(row) = row else {
        trx.commit().await?;
        return Ok(None);
    };
    let task_id = row.id;
    let attempt = row.attempt;
    let context = u256("verification task context", &row.coprocessor_context_id)?;
    let version = manifest_version(row.version)?;
    let scope = VerificationScope {
        generation: row.generation,
        local_publisher: address("verification task local publisher", &row.local_publisher)?,
        version,
        coprocessor_context_id: context,
        host_chain_id: row.host_chain_id,
        publication_block_number: row.publication_block_number,
        publication_block_hash: b256(
            "verification task publication block hash",
            &row.publication_block_hash,
        )?,
        revision: u64::try_from(row.revision)
            .map_err(|_| internal("verification task revision is negative"))?,
        local_manifest_digest: b256(
            "verification task local manifest digest",
            &row.local_manifest_digest,
        )?,
    };

    let peer_rows = sqlx::query!(
        r#"
        SELECT publisher,
               s3_bucket_url,
               completed_attempt >= $2 AS "current_download_complete!",
               rejected_object_keys AS "rejected_object_keys!"
          FROM block_manifest_peer_download
         WHERE task_id = $1
         ORDER BY publisher
        "#,
        task_id,
        attempt,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let mut peers = Vec::with_capacity(peer_rows.len());
    for peer in peer_rows {
        let publisher = address("peer download publisher", &peer.publisher)?;
        let known_revisions = load_known_peer_revisions(&mut trx, &scope, publisher).await?;
        let rejected_object_keys = peer.rejected_object_keys.into_iter().collect();
        peers.push(ClaimedPeer {
            publisher,
            s3_bucket_url: peer.s3_bucket_url,
            current_download_complete: peer.current_download_complete,
            known_revisions,
            rejected_object_keys,
        });
    }
    trx.commit().await?;
    Ok(Some(VerificationClaim {
        task_id,
        worker_id: worker_id.to_owned(),
        attempt,
        lease_duration: claim_duration,
        required_quorum: usize::try_from(row.required_quorum)
            .map_err(|_| internal("required quorum is negative"))?,
        scope,
        peers,
    }))
}

async fn load_known_peer_revisions(
    trx: &mut Transaction<'_, Postgres>,
    scope: &VerificationScope,
    publisher: Address,
) -> Result<HashSet<u64>, ExecutionError> {
    load_known_peer_revisions_at(
        trx,
        publisher,
        scope.version,
        scope.coprocessor_context_id,
        scope.host_chain_id,
        &scope.generation,
        scope.publication_block_number,
        scope.publication_block_hash,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn load_known_peer_revisions_at(
    trx: &mut Transaction<'_, Postgres>,
    publisher: Address,
    version: block_manifest::ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    generation: &str,
    publication_block_number: i64,
    publication_block_hash: B256,
) -> Result<HashSet<u64>, ExecutionError> {
    let context = coprocessor_context_id.to_be_bytes::<32>();
    let revisions = sqlx::query_scalar!(
        r#"
            SELECT revision
              FROM block_manifest
             WHERE publisher = $1
               AND generation = $7
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND publication_block_number = $5
               AND publication_block_hash = $6
            "#,
        publisher.as_slice(),
        i16::from(u8::from(version)),
        context.as_slice(),
        host_chain_id,
        publication_block_number,
        publication_block_hash.as_slice(),
        generation,
    )
    .fetch_all(trx.as_mut())
    .await?;
    revisions
        .into_iter()
        .map(|revision| {
            u64::try_from(revision)
                .map_err(|_| internal("archived peer manifest revision is negative"))
        })
        .collect()
}
