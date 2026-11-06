//! Notify-driven state_hash computation + S3 upload of GCS hashes.

use std::sync::Arc;
use std::time::Duration;

use aws_sdk_s3::primitives::ByteStream;
use fhevm_engine_common::database::{GCS_SCHEMA, GCS_SCHEMA_QUOTED};
use fhevm_engine_common::gcs_activation::EVENT_GW_NEW_BLOCK;
use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy, WriteGuard};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, Pool, Postgres, Row, Transaction};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

pub const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

/// Notified by the sns-worker when a batch of ct128 squashes commits (its
/// `--pg-notify-channel`; every deployment wires it to this name). A block's
/// hash now also waits for the block's PBS work, which can finish after the
/// last tfhe-worker notify, so listen to the squash commits too instead of
/// relying on the next host block to retrigger the pass.
pub const EVENT_CIPHERTEXT128_COMPUTED: &str = "event_ciphertext128_computed";

/// Sentinel for blocks with `fhe_event_count = 0`. Generated via
/// `printf '' | shasum -a 256` (equivalently `encode(sha256(''::bytea), 'hex')`).
pub const EMPTY_BLOCK_STATE_HASH: &str =
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

// A stalled object store must not hold the containment/cutover fence forever.
const STATE_HASH_UPLOAD_TIMEOUT: Duration = Duration::from_secs(15);

/// One upgrade attempt, as the decision records already name it: the proposal
/// and the block it was (re-)activated at. The stack version the proposal
/// targets is deliberately NOT part of the identity: it is redundant with the
/// proposal, and it belongs in the signed decision payload, not in S3 keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeAttempt {
    pub proposal_id: Vec<u8>,
    pub proposal_block: i64,
}

// RFC 021 defers the state_hash S3 layout to RFC 023, but RFC 023 only
// specifies the layout for ciphertext objects, not state_hash blobs.
//
// Keys are scoped by the upgrade attempt, mirroring the decision-record layout
// (`decision/proposal=<hex>/attempt=<proposal_block>/outcome.json`). Nothing
// deletes these objects, and a rollback resets only the DB: a retry whose
// `gw_start_block` did not move would otherwise re-use the previous attempt's
// keys, so peers could read attempt-N blobs early in attempt N+1 (stale-read
// "divergence"), or a deterministic retry could reach unanimity on leftovers
// before anyone recomputed (fhevm-internal#2273). `attempt=<proposal_block>`
// is what names the attempt: re-activating the same proposal reuses its
// `proposal_id` but never its `proposal_block`. The `state_hash/` family
// prefix stays first so prefix-based listing, lifecycle rules and the reader's
// `{s3_url}/{key}` GET keep working. The target stack version is NOT in the
// key: it is redundant with the proposal and lives in the signed decision
// payload instead, exactly like the decision-record keys.
//
//   <s3BucketUrl>/state_hash/proposal=<proposal_id_hex>
//     /attempt=<proposal_block>/chain=<chain_id>/block=<block_number>.bin
//   body (host-chain rows)    = compact-JSON [`StateHashRecord`], byte-for-byte as
//                               stored in `state_hash.state_hash`.
//   body (gateway-input rows) = raw 32-byte SHA-256 (decoded from the stored hex).
//                               Gateway blocks have no PBS work, so there is
//                               nothing to attribute and the format stays flat.
pub fn state_hash_key(attempt: &UpgradeAttempt, chain_id: i64, block_number: i64) -> String {
    scoped_key(
        &hex::encode(&attempt.proposal_id),
        attempt.proposal_block,
        chain_id,
        block_number,
    )
}

/// The key as shown in logs: same shape as [`state_hash_key`], but the
/// proposal id abbreviated by [`display_proposal_id`] so a 32-byte L1 id does
/// not drown the line. Display only - never a real object key.
pub fn state_hash_log_key(attempt: &UpgradeAttempt, chain_id: i64, block_number: i64) -> String {
    scoped_key(
        &display_proposal_id(&attempt.proposal_id),
        attempt.proposal_block,
        chain_id,
        block_number,
    )
}

fn scoped_key(proposal_hex: &str, proposal_block: i64, chain_id: i64, block_number: i64) -> String {
    format!(
        "state_hash/proposal={proposal_hex}/attempt={proposal_block}/chain={chain_id}/block={block_number}.bin"
    )
}

/// Abbreviates a proposal id for logs: hex of the first 2 and last 4 bytes,
/// e.g. `0000...00111122`. Ids of 6 bytes or fewer print in full. The L1
/// proposal id is a caller-chosen uint256, so only the full 32 bytes identify
/// an attempt; this form is for human eyes, never for keys or comparisons.
pub fn display_proposal_id(proposal_id: &[u8]) -> String {
    if proposal_id.len() <= 6 {
        return hex::encode(proposal_id);
    }
    format!(
        "{}...{}",
        hex::encode(&proposal_id[..2]),
        hex::encode(&proposal_id[proposal_id.len() - 4..]),
    )
}

/// Version tag of [`StateHashRecord`]; bump on any change to the record layout.
pub const STATE_HASH_RECORD_VERSION: u8 = 1;

/// The host-chain state-hash commitment, stored in `state_hash.state_hash` and
/// uploaded verbatim as the S3 object body.
///
/// The two pipelines that can diverge are committed separately so a divergence
/// is attributable from the S3 records alone, which survive GCS rollback and
/// table cleanup: `fhe` covers the tfhe-worker's ct64 output, `pbs` the
/// sns-worker's squash (ct128 digests). The previous combined
/// `sha256(ct64 || ct128)` destroyed that attribution - and, with no length
/// separator between the two aggregates, did not even commit to their boundary.
///
/// Consensus parses this defensively ([`evaluate_host_slots`]): operators agree
/// exactly when every record validates and all `fhe` hashes match and all `pbs`
/// hashes match. Both components are always part of the commitment - a
/// divergence in either alone blocks the anchor and is reported naming the
/// failing pipeline - and a malformed or invalid peer object can only fail to
/// match, never force an anchor. Serialization must stay deterministic: compact
/// `serde_json` over this struct (fields serialize in declaration order),
/// serialized once at compute time and never re-serialized on read.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateHashRecord {
    /// Record-format version ([`STATE_HASH_RECORD_VERSION`]). Lets a reader
    /// report "format skew" instead of a bare unexplained divergence when
    /// operators run different detector versions.
    pub v: u8,
    /// Hex SHA-256 of the block's ct64 ciphertext aggregate (FHE compute).
    pub fhe: String,
    /// Hex SHA-256 of the block's ct128 digest aggregate (PBS squash); the
    /// hash of the empty aggregate when the block schedules no PBS work.
    pub pbs: String,
}

impl StateHashRecord {
    fn to_json(&self) -> String {
        // Infallible: no map keys, no non-string keys, no non-finite floats.
        serde_json::to_string(self).expect("state-hash record serialization cannot fail")
    }

    /// A record fit for consensus: both components are 64-char hex digests, and
    /// `pbs` actually commits to ct128 digests - the hash of the EMPTY aggregate
    /// (`sha256('')` = [`EMPTY_BLOCK_STATE_HASH`]) is rejected. A block whose
    /// record carries an empty-aggregate `pbs` exercised no squash work, so it
    /// proves nothing about PBS determinism and must not anchor consensus.
    /// Enforced on both sides: locally before publishing (the upload sweep) and
    /// remotely when counting votes ([`evaluate_host_slots`]). Error strings
    /// truncate the offending value - peers control it.
    pub(crate) fn validate(&self) -> Result<(), String> {
        fn hex64(s: &str) -> bool {
            s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
        }
        fn clip(s: &str) -> String {
            s.chars().take(16).collect()
        }
        if !hex64(&self.fhe) {
            return Err(format!(
                "fhe is not a 64-char hex digest: '{}…'",
                clip(&self.fhe)
            ));
        }
        if !hex64(&self.pbs) {
            return Err(format!(
                "pbs is not a 64-char hex digest: '{}…'",
                clip(&self.pbs)
            ));
        }
        if self.pbs == EMPTY_BLOCK_STATE_HASH {
            return Err(
                "empty pbs state-hash (sha256 of the empty ct128 aggregate) is not allowed".into(),
            );
        }
        Ok(())
    }
}

/// Verdict over one block's operator slots on a HOST track, where every vote is
/// a [`StateHashRecord`]. Votes are counted on the PARSED component hashes, not
/// on raw bytes: operators agree exactly when every record validates and all
/// `fhe` hashes match and all `pbs` hashes match. A divergence in either
/// component alone is still a divergence - there is no partial agreement - but
/// it is reported as such so the failing pipeline is named.
pub(crate) enum HostSlotsVerdict {
    /// Every operator's record validates and both components agree everywhere.
    /// `encoding_skew` is true when the raw bytes still differ (`v` or encoding
    /// differences across detector versions): the anchor stands, since the
    /// commitment is the hashes, but it is worth a warning.
    Anchor { encoding_skew: bool },
    /// Some operator's payload is not a valid record (unparsable, non-hex
    /// digest, or empty pbs). The block cannot anchor.
    Invalid { why: String },
    /// All records valid, but the hashes disagree across operators.
    Diverged { fhe: bool, pbs: bool },
}

/// Evaluates a host-track block's slots; `None` while any operator has not
/// responded yet. Parses peer-controlled bytes, so every failure mode is a
/// verdict, never a panic, and can only prevent an anchor, never force one.
pub(crate) fn evaluate_host_slots(slots: &[Option<Vec<u8>>]) -> Option<HostSlotsVerdict> {
    if slots.is_empty() || slots.iter().any(Option::is_none) {
        return None;
    }
    let mut records = Vec::with_capacity(slots.len());
    for (idx, slot) in slots.iter().enumerate() {
        let bytes = slot.as_deref().unwrap_or_default();
        let record = match serde_json::from_slice::<StateHashRecord>(bytes) {
            Ok(record) => record,
            Err(e) => {
                return Some(HostSlotsVerdict::Invalid {
                    why: format!("op{idx}: unparsable state-hash record: {e}"),
                })
            }
        };
        if let Err(why) = record.validate() {
            return Some(HostSlotsVerdict::Invalid {
                why: format!("op{idx}: {why}"),
            });
        }
        records.push(record);
    }
    let first = &records[0];
    let fhe = records.iter().any(|r| r.fhe != first.fhe);
    let pbs = records.iter().any(|r| r.pbs != first.pbs);
    if fhe || pbs {
        return Some(HostSlotsVerdict::Diverged { fhe, pbs });
    }
    let first_bytes = slots[0].as_deref();
    let encoding_skew = slots.iter().any(|s| s.as_deref() != first_bytes);
    Some(HostSlotsVerdict::Anchor { encoding_skew })
}

/// Compact per-operator view for the divergence error: enough hex to see who
/// disagrees with whom without dumping full records into one log line.
pub(crate) fn divergence_summary(slots: &[Option<Vec<u8>>]) -> String {
    // chars().take over byte slicing: the payload is peer-controlled, so a
    // non-ASCII component value must not panic a mid-char slice.
    let prefix = |s: &str| s.chars().take(12).collect::<String>();
    slots
        .iter()
        .enumerate()
        .map(|(idx, slot)| match slot {
            None => format!("op{idx}: <missing>"),
            Some(bytes) => match serde_json::from_slice::<StateHashRecord>(bytes) {
                Ok(r) => format!("op{idx}: fhe={} pbs={}", prefix(&r.fhe), prefix(&r.pbs)),
                Err(_) => format!("op{idx}: <non-record, {} bytes>", bytes.len()),
            },
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// Outcome of a guarded state-hash insert.
enum HashInsert {
    /// Inserted a new row.
    Wrote,
    /// The row already exists.
    Noop,
}

/// Inserts a state hash in the guarded transaction.
async fn insert_state_hash(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    block_number: i64,
    hash: &str,
    schema: &str,
) -> Result<HashInsert, sqlx::Error> {
    let affected = sqlx::query(&format!(
        "INSERT INTO {schema}.state_hash (chain_id, block_number, state_hash)
         VALUES ($1, $2, $3) ON CONFLICT (chain_id, block_number) DO NOTHING"
    ))
    .bind(chain_id)
    .bind(block_number)
    .bind(hash)
    .execute(&mut **tx)
    .await?
    .rows_affected();
    Ok(if affected > 0 {
        HashInsert::Wrote
    } else {
        HashInsert::Noop
    })
}

/// GCS path: stamp the GCS `state_hash` for one chain's blocks in `[start, end]`
/// that don't already have an entry. Empty blocks (`fhe_event_count = 0`) get
/// [`EMPTY_BLOCK_STATE_HASH`]; non-empty blocks get a [`StateHashRecord`]
/// committing separately to their GCS `ciphertexts` (`fhe`) and to the ct128
/// digests of the block's PBS work (`pbs`), and are held back until that PBS
/// work is done (see [`compute_gcs_hash`]).
async fn compute_and_insert_gcs(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    start: i64,
    end: i64,
    batch_limit: i64,
    schema: &str,
) -> anyhow::Result<()> {
    // A replay must observe this commitment before changing its inventory.
    // The caller holds the cutover fence; acquire history before reading work,
    // including when no hash row exists yet (row locks cannot cover that race).
    fhevm_engine_common::chain_recovery::lock_history_tx(
        tx,
        fhevm_engine_common::chain_recovery::ListenerSource::Host,
        schema != "public",
        chain_id,
    )
    .await?;
    // This is the write path, so both the pending-scan read and the INSERT
    // explicitly qualify the versioned GCS schema (`GCS_SCHEMA_QUOTED`, e.g.
    // `"gcs-0.14.0"`) instead of relying on the connection search_path. An
    // accidental fallback to `public.state_hash` would corrupt the live (BCS)
    // table, so these must never resolve anywhere but the GCS schema — and a
    // missing GCS schema errors loudly rather than silently writing to public.
    // Runtime `sqlx::query` (not the `query!` macro) because the schema name is
    // only known at runtime.
    //
    // `host_chain_blocks_valid` is keyed by (chain_id, block_hash), so a fork
    // leaves several rows at the same (chain_id, block_number). We restrict to
    // `block_status = 'finalized'`: finalization orphans every sibling fork and
    // leaves exactly one canonical row per height, so the emptiness verdict
    // (`bool_and(fhe_event_count = 0)`) reflects only the canonical block and is
    // never contaminated by orphaned branches. Blocks not yet finalized are
    // simply skipped and picked up on a later notify-driven pass — the same
    // "retry until ready" model as `compute_gcs_hash` returning `None`.
    let pending_sql = format!(
        r#"
        SELECT b.chain_id, b.block_number,
               bool_and(b.fhe_event_count = 0) AS is_empty
          FROM {schema}.host_chain_blocks_valid b
         WHERE b.chain_id = $1
           AND b.block_number BETWEEN $2 AND $3
           AND b.block_status = 'finalized'
           AND NOT EXISTS (
               SELECT 1 FROM {schema}.state_hash sh
                WHERE sh.chain_id = b.chain_id AND sh.block_number = b.block_number)
         GROUP BY b.chain_id, b.block_number
         ORDER BY b.block_number
         LIMIT $4
        "#
    );
    let pending = sqlx::query(&pending_sql)
        .bind(chain_id)
        .bind(start)
        .bind(end)
        .bind(batch_limit)
        .fetch_all(&mut **tx)
        .await?;

    for row in pending {
        let chain_id: i64 = row.try_get("chain_id")?;
        let block_number: i64 = row.try_get("block_number")?;
        let is_empty: bool = row.try_get("is_empty")?;
        let hash = if is_empty {
            Some(EMPTY_BLOCK_STATE_HASH.to_string())
        } else {
            compute_gcs_hash(tx, chain_id, block_number, schema).await?
        };
        let Some(hash) = hash else { continue };
        match insert_state_hash(tx, chain_id, block_number, &hash, schema).await? {
            HashInsert::Wrote => info!(chain_id, block_number, "gcs.state_hash inserted"),
            HashInsert::Noop => {}
        }
    }
    Ok(())
}

/// The block's [`StateHashRecord`] as compact JSON (`fhe` = SHA-256 over the
/// block's GCS `ciphertexts`, `pbs` = SHA-256 over the ct128 digests of the
/// block's PBS (squash) work); `None` until every non-errored computation
/// completes AND every PBS computation the block itself scheduled for its own
/// output handles is squashed with its `ciphertext_digest.ciphertext128`
/// recorded. Errored computations are excluded so operators agree.
///
/// The PBS anchor (fhevm-internal#1960): the plain ct64 hash is blind to the
/// sns-worker, so a divergent squash stack (e.g. a CPU worker in a GPU fleet)
/// would still reach consensus and break decryption after cutover. Committing
/// to the ct128 digests makes operators agree on the squash output too, and the
/// completeness gate keeps the record (and therefore its upload - the insert is
/// what the upload sweep reads) back until the block's PBS work is done. The
/// two components are separate hashes, not one combined digest, so a
/// divergence names the pipeline that produced it (see [`StateHashRecord`]).
///
/// Determinism of the PBS set: a row joins only when BOTH its handle is one
/// of the block's own non-errored output handles AND the row was inserted by
/// this block's own events (`pbs_computations.host_chain_id`/`block_number`,
/// stamped by the inserting ingest). Both conditions are frozen the moment
/// the block's ingestion commits - and a block is only hashed after that - so
/// every operator selects the same set. Without the stamp, a row inserted by
/// a LATER block's event for a handle computed here (a
/// `ConfidentialBridge.send` squashing its source, a late ACL allow) would
/// join or not depending on how far ingestion ran before this pass, and the
/// first inserted record freezes: two correct operators would permanently
/// commit different `pbs` aggregates. Such foreign-scheduled squashes are
/// instead excluded on EVERY operator - they are simply never anchored, which
/// is fine: consensus needs the squash pipeline exercised and compared at
/// least once per attempt, not every squash covered. The sns-worker commits
/// `is_completed` and the `ciphertext_digest.ciphertext128` digest in one
/// transaction, so the gate never admits a half-written squash; a handle
/// whose computation errored can never be squashed, and the `bc` join's
/// `is_error = false` filter keeps it from holding the hash forever. Blocks
/// with no PBS rows carry the
/// empty-aggregate hash in `pbs` (the digest aggregate degrades to `''`), a
/// recognizable constant that reads as "no PBS work scheduled" - such records
/// exist locally for completeness but are barred from upload, candidacy and
/// vote counting (see [`StateHashRecord::validate`]): with no squash work they
/// prove nothing about PBS determinism, so they must not anchor consensus.
///
/// Like [`compute_and_insert_gcs`], the source tables are explicitly qualified
/// with the versioned GCS schema (`GCS_SCHEMA_QUOTED`) and never fall back to
/// `public`: hashing the live (BCS) `computations`/`ciphertexts` would silently
/// produce the wrong digest, so a missing GCS schema must error rather than
/// read public. Runtime `sqlx::query` because the schema name is only known at
/// runtime.
async fn compute_gcs_hash(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    block_number: i64,
    schema: &str,
) -> anyhow::Result<Option<String>> {
    let sql = format!(
        r#"
        WITH bc AS (
            SELECT output_handle, tenant_id, is_completed
              FROM {schema}.computations
             WHERE host_chain_id = $1 AND block_number = $2 AND is_error = false
        ),
        pbs AS (
            SELECT DISTINCT p.handle, p.is_completed, cd.ciphertext128
              FROM {schema}.pbs_computations p
              JOIN bc ON bc.output_handle = p.handle
              LEFT JOIN {schema}.ciphertext_digest cd ON cd.handle = p.handle
             WHERE p.host_chain_id = $1 AND p.block_number = $2
        )
        SELECT
            (SELECT bool_and(is_completed) FROM bc) AS fhe_done,
            (SELECT count(*) FROM pbs) AS pbs_total,
            (SELECT count(*) FROM pbs
              WHERE NOT pbs.is_completed OR pbs.ciphertext128 IS NULL) AS pbs_pending,
            CASE WHEN (SELECT bool_and(is_completed) FROM bc)
                  AND NOT EXISTS (
                      SELECT 1 FROM pbs
                       WHERE NOT pbs.is_completed OR pbs.ciphertext128 IS NULL)
                THEN encode(
                    sha256(
                        (SELECT string_agg(ct.ciphertext, ''::bytea
                                           ORDER BY ct.handle, ct.ciphertext_version)
                           FROM bc
                           JOIN {schema}.ciphertexts ct
                             ON ct.tenant_id = bc.tenant_id AND ct.handle = bc.output_handle)
                    ),
                    'hex'
                )
            END AS fhe_hash,
            CASE WHEN (SELECT bool_and(is_completed) FROM bc)
                  AND NOT EXISTS (
                      SELECT 1 FROM pbs
                       WHERE NOT pbs.is_completed OR pbs.ciphertext128 IS NULL)
                THEN encode(
                    sha256(
                        COALESCE(
                            (SELECT string_agg(pbs.ciphertext128, ''::bytea
                                               ORDER BY pbs.handle)
                               FROM pbs),
                            ''::bytea)
                    ),
                    'hex'
                )
            END AS pbs_hash
        "#
    );
    let row = sqlx::query(&sql)
        .bind(chain_id)
        .bind(block_number)
        .fetch_one(&mut **tx)
        .await?;
    let fhe_done: Option<bool> = row.try_get("fhe_done")?;
    let pbs_total: i64 = row.try_get("pbs_total")?;
    let pbs_pending: i64 = row.try_get("pbs_pending")?;
    // fhe_hash stays NULL when the gate is closed OR the ct64 aggregate is
    // empty (e.g. every completed row lacks a ciphertext): both held back,
    // exactly as the combined recipe behaved (sha256(NULL) was NULL).
    let fhe_hash: Option<String> = row.try_get("fhe_hash")?;
    let pbs_hash: Option<String> = row.try_get("pbs_hash")?;
    let state_hash: Option<String> = fhe_hash.zip(pbs_hash).map(|(fhe, pbs)| {
        StateHashRecord {
            v: STATE_HASH_RECORD_VERSION,
            fhe,
            pbs,
        }
        .to_json()
    });
    if state_hash.is_some() {
        // Fires once per block: the insert that follows removes it from the
        // pending scan.
        if pbs_total > 0 {
            info!(
                chain_id,
                block_number,
                ct128_digests = pbs_total,
                "all ct128 digests ready; binding them into the block's state_hash"
            );
        }
    } else if fhe_done == Some(true) && pbs_pending > 0 {
        debug!(
            chain_id,
            block_number, pbs_pending, pbs_total, "state_hash held back: ct128 digests not ready"
        );
    }
    Ok(state_hash)
}

/// GCS Gateway-inputs path: stamp the GCS `state_hash` (keyed by `gw_chain_id`)
/// for **sealed, complete** Gateway blocks in `[gw_start_block, gw_tip)` that
/// carry input ciphertexts and don't already have an entry.
///
/// A Gateway block is a candidate when:
///   * it has ≥1 `input_handles` row (blocks with no inputs are skipped entirely
///     — proofs are sparse, so unlike the L1 path we emit no sentinel);
///   * it is **sealed**, established one of two ways:
///       - a non-anchor block by `block_number < gw_tip` — the Gateway tip has
///         advanced past it, so no new `VerifyProofRequest` can still land in it;
///       - the **synthetic anchor** block (`= gw_start_block`) by the presence of its
///         synthetic `verify_proofs` row (`zk_proof_id >= SYNTHETIC_ZK_PROOF_ID_BASE`).
///         `< gw_tip` can never seal it: on an idle Gateway `gw_start_block == gw_tip`,
///         and the synthetic is back-dated into it after the tip already passed, so the
///         tip never advances beyond it. The seal is completeness instead: the synthetic
///         is the only input that can arrive after the watermark (real requests are
///         frozen once the listener reached the block), so once its row exists the
///         `verified IS NULL` gate below waits for it to verify, and the zkproof-worker
///         commits `verified=true` atomically with the ciphertext. This guarantees the
///         anchor's hash always includes the synthetic and never a real-only subset
///         (which the one-shot `state_hash` insert would otherwise make permanent).
///   * every `verify_proofs` row for it is resolved (`verified IS NOT NULL`) — the
///     zkproof-worker has caught up, mirroring the L1 `bool_and(is_completed)`
///     gate. Otherwise the block is left for a later pass.
///
/// Like [`compute_and_insert_gcs`], every table is explicitly qualified with the
/// versioned GCS schema and never falls back to `public`.
async fn compute_and_insert_gw_input_hashes(
    tx: &mut Transaction<'_, Postgres>,
    gw_chain_id: i64,
    gw_start_block: i64,
    gw_tip: i64,
    gw_window_rewound: bool,
    batch_limit: i64,
) -> anyhow::Result<()> {
    // Compute nothing until BOTH hold:
    //
    //  * `gw_window_rewound` - the gw-listener's per-proposal alignment latch. The listener
    //    sets it only in (or after) the transaction that aligned its scan cursor to
    //    gw_start_block (rewinding when a gap existed). Without it, a proposal that
    //    activates mid-tick - after the listener's top-of-tick alignment check but before
    //    its synthetic injection and watermark write - leaves `gw_tip >= gw_start_block`
    //    and the synthetic row with the window start's real inputs never scanned; hashing
    //    then stamps a one-shot, permanent synthetic-only gw_start_block hash that the next
    //    tick's rewind cannot repair.
    //
    //  * `gw_tip >= gw_start_block` - the watermark has reached the window start, so (the
    //    scan being contiguous once aligned) every block in [gw_start_block, gw_tip) has
    //    complete inputs. The latch alone is not enough either: right after a rewind the
    //    cursor sits at gw_start_block - 1 with the window not yet re-scanned.
    if !gw_window_rewound || gw_tip < gw_start_block {
        return Ok(());
    }
    let pending_sql = format!(
        r#"
        SELECT DISTINCT ih.block_number
          FROM {GCS_SCHEMA_QUOTED}.input_handles ih
         WHERE ih.block_number >= $1
           AND (
                 -- Non-anchor blocks: sealed once the tip has advanced strictly past them.
                 (ih.block_number > $1 AND ih.block_number < $2)
                 -- Synthetic anchor block (gw_start_block, = $1): admitted only once its
                 -- synthetic verify_proofs row exists. This is the one block whose seal
                 -- cannot come from `< gw_tip` (on an idle Gateway it equals the tip), so
                 -- it comes from completeness instead — see the doc comment. EXISTS of the
                 -- synthetic row also implies the listener has reached $1, so every real
                 -- request in the block is already frozen.
              OR (ih.block_number = $1
                    AND EXISTS (
                        SELECT 1 FROM {GCS_SCHEMA_QUOTED}.verify_proofs vp
                         WHERE vp.block_number = $1 AND vp.zk_proof_id >= $5))
               )
           AND NOT EXISTS (
               SELECT 1 FROM {GCS_SCHEMA_QUOTED}.state_hash sh
                WHERE sh.chain_id = $3 AND sh.block_number = ih.block_number)
           AND NOT EXISTS (
               SELECT 1 FROM {GCS_SCHEMA_QUOTED}.verify_proofs vp
                WHERE vp.block_number = ih.block_number AND vp.verified IS NULL)
         ORDER BY ih.block_number
         LIMIT $4
        "#
    );
    let pending = sqlx::query(&pending_sql)
        .bind(gw_start_block)
        .bind(gw_tip)
        .bind(gw_chain_id)
        .bind(batch_limit)
        .bind(fhevm_engine_common::synthetic_input::SYNTHETIC_ZK_PROOF_ID_BASE)
        .fetch_all(&mut **tx)
        .await?;

    for row in pending {
        let block_number: i64 = row.try_get("block_number")?;
        // `None` when the input ciphertexts aren't all present yet (handle exists
        // in input_handles but its ciphertext row hasn't landed) — retry later.
        let Some(hash) = compute_gw_input_hash(tx, block_number).await? else {
            continue;
        };
        match insert_state_hash(tx, gw_chain_id, block_number, &hash, GCS_SCHEMA_QUOTED).await? {
            HashInsert::Wrote => {
                info!(
                    gw_chain_id,
                    block_number, "gcs.state_hash (gw inputs) inserted"
                )
            }
            HashInsert::Noop => {}
        }
    }
    Ok(())
}

/// SHA-256 over a Gateway block's input ciphertexts; `None` until every
/// `input_handles` handle for the block has its `ciphertexts` row present (the
/// join yields an empty set → `string_agg` NULL → retry). The digest recipe
/// matches [`compute_gcs_hash`] (order by `handle, ciphertext_version`) so the
/// two tracks are byte-compatible, but the source is `input_handles` joined to
/// `is_input` ciphertexts rather than `computations` outputs.
async fn compute_gw_input_hash(
    tx: &mut Transaction<'_, Postgres>,
    block_number: i64,
) -> anyhow::Result<Option<String>> {
    let sql = format!(
        r#"
        SELECT encode(
            sha256(string_agg(ct.ciphertext, ''::bytea
                              ORDER BY ct.handle, ct.ciphertext_version)),
            'hex'
        ) AS state_hash
          FROM {GCS_SCHEMA_QUOTED}.input_handles ih
          JOIN {GCS_SCHEMA_QUOTED}.ciphertexts ct
            ON ct.handle = ih.handle AND ct.is_input = TRUE
         WHERE ih.block_number = $1
        "#
    );
    let Some(row) = sqlx::query(&sql)
        .bind(block_number)
        .fetch_optional(&mut **tx)
        .await?
    else {
        return Ok(None);
    };
    let state_hash: Option<String> = row.try_get("state_hash")?;
    Ok(state_hash)
}

/// The Gateway tip = the GCS gw-listener watermark. `None` when the watermark
/// row is absent/NULL (listener hasn't ingested any Gateway block yet), which
/// disables Gateway hashing until it does.
pub(crate) async fn gw_listener_tip<'e, E>(executor: E) -> Result<Option<i64>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = Postgres>,
{
    let sql = format!(
        "SELECT last_block_num FROM {GCS_SCHEMA_QUOTED}.gw_listener_last_block
          WHERE dummy_id = true"
    );
    let row: Option<(Option<i64>,)> = sqlx::query_as(&sql).fetch_optional(executor).await?;
    Ok(row.and_then(|(v,)| v))
}

/// `gw_start_block` of the active GCS upgrade row, or `None` when unset.
pub(crate) async fn gw_start_block<'e, E>(executor: E) -> Result<Option<i64>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = Postgres>,
{
    let row: Option<(Option<i64>,)> = sqlx::query_as(
        "SELECT gw_start_block FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'
          ORDER BY host_chain_id
          LIMIT 1",
    )
    .fetch_optional(executor)
    .await?;
    Ok(row.and_then(|(v,)| v))
}

/// Whether the active GCS upgrade has already reached Gateway consensus. Once it has,
/// the Gateway anchor is agreed and any further GW `state_hash` compute/upload is wasted
/// work: the Gateway chain keeps producing blocks for the whole dry-run window, so left
/// unchecked the worker firehoses thousands of GW hashes that no longer feed consensus and
/// only steal throughput from the host tracks, delaying host unanimity. Stop GW work here
/// as soon as the latch is set.
pub(crate) async fn gw_consensus_reached<'e, E>(executor: E) -> Result<bool, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = Postgres>,
{
    let v: Option<bool> = sqlx::query_scalar(
        "SELECT BOOL_OR(gw_consensus_reached) FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'",
    )
    .fetch_one(executor)
    .await?;
    Ok(v.unwrap_or(false))
}

/// Whether the GCS gw-listener has aligned its scan cursor to the active window's
/// `gw_start_block` - the per-proposal `gw_window_rewound` latch, `BOOL_AND`ed over the
/// in-progress rows so one unaligned row keeps the gate closed. `false` with no rows.
///
/// This closes the activation race on the watermark gate alone: a proposal that activates
/// mid-tick - after the listener's top-of-tick alignment check but before its synthetic
/// injection and watermark write - can leave `last_block_num >= gw_start_block` (and the
/// synthetic `verify_proofs` row) WITHOUT the window start's real inputs having been
/// scanned. Hashing then would stamp a one-shot, permanent synthetic-only `gw_start_block`
/// hash that the next tick's rewind cannot repair. The latch is only written by the
/// listener in (or after) its alignment transaction, so requiring it here holds the GW
/// track until the listener has provably aligned to this window.
///
/// Returns `(gw_start_block, gw_tip, gw_window_rewound)` read in ONE statement, so all three
/// come from one snapshot even under READ COMMITTED. The listener lowers the watermark and
/// sets the latch in the same transaction; separate reads could pair a stale pre-rewind
/// watermark with the post-rewind latch and seal a block still being re-ingested. `None`
/// when `gw_start_block` or the watermark is unset.
pub(crate) async fn gw_window_rewound<'e, E>(
    executor: E,
) -> Result<Option<(i64, i64, bool)>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = Postgres>,
{
    let (gw_start, gw_tip, rewound): (Option<i64>, Option<i64>, Option<bool>) =
        sqlx::query_as(&format!(
            "SELECT
               (SELECT gw_start_block FROM upgrade_state
                 WHERE stack_role = 'GCS' AND status = 'in_progress'
                 ORDER BY host_chain_id
                 LIMIT 1),
               (SELECT last_block_num FROM {GCS_SCHEMA_QUOTED}.gw_listener_last_block
                 WHERE dummy_id = true),
               (SELECT BOOL_AND(gw_window_rewound) FROM upgrade_state
                 WHERE stack_role = 'GCS' AND status = 'in_progress')"
        ))
        .fetch_one(executor)
        .await?;
    Ok(gw_start
        .zip(gw_tip)
        .map(|(start, tip)| (start, tip, rewound.unwrap_or(false))))
}

/// Uploads GCS `state_hash` rows with `s3_uploaded_at IS NULL`, attaching the
/// host-chain block_hash as S3 object metadata. Stamps `NOW()` on success;
/// failures stay NULL and retry next sweep.
///
/// The block_hash join is constrained to `block_status = 'finalized'`: since
/// `host_chain_blocks_valid` is keyed by (chain_id, block_hash), an unqualified
/// join would match every fork row at that height and PUT the same object under
/// the same S3 key repeatedly with whichever branch's `block-hash` metadata the
/// planner happened to pick. Finalization leaves exactly one canonical row per
/// height, so the join now yields a single deterministic block_hash per upload.
/// The in-progress attempt `(proposal_id, proposal_block)`, if any.
type Attempt = Option<(Vec<u8>, i64)>;

/// How an upload pass keys its objects.
enum Keying {
    /// An active attempt - the only production shape. Every object is scoped
    /// to the attempt, so retries never collide.
    Attempt(UpgradeAttempt),
    /// Test-box builds keep uploading after cutover, when no attempt exists,
    /// under the legacy unscoped layout the box consensus loop reads.
    #[cfg(feature = "post-cutover-state-hash")]
    PostCutover,
}

impl Keying {
    /// `None` means there is nothing to publish this pass: the consensus
    /// readers only ever fetch attempt-scoped keys, so with no attempt an
    /// upload could serve no one.
    fn of(started: &Attempt) -> Option<Keying> {
        let keying = started.as_ref().map(|(proposal_id, proposal_block)| {
            Keying::Attempt(UpgradeAttempt {
                proposal_id: proposal_id.clone(),
                proposal_block: *proposal_block,
            })
        });
        #[cfg(feature = "post-cutover-state-hash")]
        let keying = keying.or(Some(Keying::PostCutover));
        keying
    }

    fn key(&self, chain_id: i64, block_number: i64) -> String {
        match self {
            Keying::Attempt(attempt) => state_hash_key(attempt, chain_id, block_number),
            #[cfg(feature = "post-cutover-state-hash")]
            Keying::PostCutover => format!("state_hash/chain={chain_id}/block={block_number}.bin"),
        }
    }

    /// The key as logged: the proposal id abbreviated, everything else exact.
    fn log_key(&self, chain_id: i64, block_number: i64) -> String {
        match self {
            Keying::Attempt(attempt) => state_hash_log_key(attempt, chain_id, block_number),
            #[cfg(feature = "post-cutover-state-hash")]
            Keying::PostCutover => self.key(chain_id, block_number),
        }
    }
}

/// Blue ingested the `end_block` of this chain at least `cutoff` ago.
pub(crate) async fn chain_upload_closed(
    pool: &Pool<Postgres>,
    chain_id: i64,
    end_block: i64,
    cutoff: Duration,
) -> Result<bool, sqlx::Error> {
    let (closed,): (bool,) = sqlx::query_as(
        "SELECT EXISTS(
            SELECT 1 FROM public.host_chain_blocks_valid b
             WHERE b.chain_id = $1 AND b.block_number >= $2
               AND b.created_at <= NOW() - make_interval(secs => $3))",
    )
    .bind(chain_id)
    .bind(end_block)
    .bind(cutoff.as_secs_f64())
    .fetch_one(pool)
    .await?;
    Ok(closed)
}

/// `(in-progress attempt, every chain's end_block was ingested by Blue at least cutoff
/// ago)`. Past the cutoff peers have decided, so a hash published now could only
/// mislead a returning operator.
pub(crate) async fn upload_window<'e>(
    executor: impl sqlx::Executor<'e, Database = Postgres>,
    cutoff: Duration,
) -> Result<(Attempt, bool), sqlx::Error> {
    type WindowRow = (Option<Vec<u8>>, Option<i64>, bool);
    let (proposal_id, proposal_block, closed): WindowRow = sqlx::query_as(
        "SELECT (SELECT proposal_id FROM upgrade_state
                  WHERE stack_role = 'GCS' AND status = 'in_progress' LIMIT 1),
                (SELECT COALESCE(proposal_block, -1) FROM upgrade_state
                  WHERE stack_role = 'GCS' AND status = 'in_progress' LIMIT 1),
                COALESCE(BOOL_AND(EXISTS(
            SELECT 1 FROM public.host_chain_blocks_valid b
             WHERE b.chain_id = w.host_chain_id AND b.block_number >= w.end_block
               AND b.created_at <= NOW() - make_interval(secs => $1))), FALSE)
           FROM upgrade_state w WHERE w.stack_role = 'GCS' AND w.status = 'in_progress'",
    )
    .bind(cutoff.as_secs_f64())
    .fetch_one(executor)
    .await?;
    Ok((proposal_id.zip(proposal_block), closed))
}

/// Fence each upload and its bookkeeping against containment and cutover.
/// Also stops a batch whose attempt was rolled back or replaced after it started.
/// All checks use the held connection, including with a single-connection pool.
/// No history admission lock is held during the network operation.
async fn begin_hash_upload(
    pool: &Pool<Postgres>,
    cutoff: Duration,
    started: &Attempt,
) -> Result<Option<Transaction<'static, Postgres>>, sqlx::Error> {
    let Some(mut tx) = begin_write_guarded(pool, true, GcsRollbackPolicy::Skip)
        .await?
        .into_tx()
    else {
        return Ok(None);
    };
    let (current, closed) = upload_window(&mut *tx, cutoff).await?;
    if closed || (started.is_some() && current != *started) {
        tx.rollback().await?;
        return Ok(None);
    }
    Ok(Some(tx))
}

/// One conditional state-hash PUT. `If-None-Match: "*"` because within one
/// attempt the blob for a (chain, block) is immutable, an overwrite is never
/// legitimate - so an existing object (`PreconditionFailed`) means a previous
/// PUT of this same attempt succeeded without its bookkeeping: adopt it.
async fn put_state_hash(
    s3: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    block_hash_hex: Option<&str>,
    bytes: Vec<u8>,
    timeout_context: &'static str,
) -> anyhow::Result<()> {
    use aws_sdk_s3::error::ProvideErrorMetadata;
    let mut request = s3
        .put_object()
        .bucket(bucket)
        .key(key)
        .if_none_match("*")
        .body(ByteStream::from(bytes));
    if let Some(block_hash_hex) = block_hash_hex {
        request = request.metadata("block-hash", block_hash_hex);
    }
    match tokio::time::timeout(STATE_HASH_UPLOAD_TIMEOUT, request.send()).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(err))
            if err.as_service_error().and_then(ProvideErrorMetadata::code)
                == Some("PreconditionFailed") =>
        {
            Ok(())
        }
        Ok(Err(err)) => Err(anyhow::Error::from(err)),
        Err(elapsed) => Err(anyhow::Error::from(elapsed).context(timeout_context)),
    }
}

async fn upload_pending_state_hashes(
    pool: &Pool<Postgres>,
    s3: &aws_sdk_s3::Client,
    my_bucket: &str,
    batch_limit: i64,
    cutoff: Duration,
) -> anyhow::Result<()> {
    // Only upload state_hashes for ELIGIBLE blocks: a block carrying at least one
    // non-trivial, successful FHE op (fhe_operation <> trivialEncrypt). Empty and
    // trivial-only blocks are vacuous — their ciphertexts are deterministic across
    // operators regardless of GCS compute correctness — so publishing their
    // state_hash for consensus lets a chain anchor on a block that never exercised
    // GCS compute. Withholding them forces consensus onto real work (e.g. the
    // synthetic op). query_as (runtime) so the opcode constant can be interpolated
    // and no .sqlx cache entry is needed.
    let sql = format!(
        r#"
        SELECT sh.chain_id, sh.block_number, sh.state_hash, b.block_hash
          FROM state_hash sh
          JOIN host_chain_blocks_valid b
            ON b.chain_id = sh.chain_id AND b.block_number = sh.block_number
           AND b.block_status = 'finalized'
         WHERE sh.s3_uploaded_at IS NULL
           -- Never publish the empty-block hash. This is the decisive gate:
           -- empty blocks AND the synthetic block (whose hash is empty until its
           -- FheAdd is computed, yet carries a non-trivial computation) both hash
           -- to EMPTY_BLOCK_STATE_HASH, so consensus must not anchor on them.
           AND sh.state_hash <> '{empty}'
           -- Likewise never publish a record whose pbs is the empty-aggregate
           -- hash: no squash work means nothing proven about PBS determinism,
           -- and consensus rejects such votes anyway (StateHashRecord::validate).
           -- Filtered here so the row is not re-selected and re-refused forever.
           AND sh.state_hash NOT LIKE '%"pbs":"{empty}"%'
           -- Also exclude trivial-only blocks (non-empty but vacuous hash).
           AND EXISTS (
               SELECT 1 FROM computations c
                WHERE c.host_chain_id = sh.chain_id
                  AND c.block_number = sh.block_number
                  AND c.is_error = false
                  AND c.fhe_operation <> {opcode})
         ORDER BY sh.chain_id, sh.block_number
         LIMIT $1
        "#,
        empty = EMPTY_BLOCK_STATE_HASH,
        opcode = crate::FHE_TRIVIAL_ENCRYPT_OPCODE,
    );
    let started = upload_window(pool, cutoff).await?.0;
    let Some(keying) = Keying::of(&started) else {
        return Ok(());
    };
    let pending: Vec<(i64, i64, String, Vec<u8>)> = sqlx::query_as(&sql)
        .bind(batch_limit)
        .fetch_all(pool)
        .await?;

    for row in pending {
        let chain_id = row.0;
        let block_number = row.1;
        let key = keying.key(chain_id, block_number);
        let log_key = keying.log_key(chain_id, block_number);
        #[cfg(feature = "test-failpoints")]
        let report_fault = crate::test_host_report::fault(pool, chain_id, block_number).await?;
        #[cfg(feature = "test-failpoints")]
        if matches!(report_fault, Some(crate::test_host_report::Fault::Withhold)) {
            continue;
        }
        // The stored record IS the payload: uploaded byte-for-byte, never
        // re-serialized, so the object is the exact bytes the compute pass
        // committed. Parsed here only to refuse publishing a corrupt row.
        match serde_json::from_str::<StateHashRecord>(&row.2) {
            Ok(record) => {
                if let Err(why) = record.validate() {
                    error!(
                        chain_id,
                        block_number, why, "invalid state-hash record in DB; refusing to publish"
                    );
                    continue;
                }
            }
            Err(e) => {
                error!(chain_id, block_number, error = %e, "malformed state-hash record in DB; skipping row");
                continue;
            }
        }
        let bytes = row.2.clone().into_bytes();
        #[cfg(feature = "test-failpoints")]
        let bytes = {
            let mut bytes = bytes;
            if matches!(report_fault, Some(crate::test_host_report::Fault::Diverge)) {
                crate::test_host_report::journal(
                    pool,
                    chain_id,
                    block_number,
                    &row.2,
                    &format!("0x{}", hex::encode(&row.3)),
                    my_bucket,
                    &key,
                )
                .await?;
                crate::test_host_report::divergent_hash(&mut bytes);
            }
            bytes
        };
        let block_hash_hex = format!("0x{}", hex::encode(&row.3));
        // The hex of the exact bytes PUT as the object body (the state-hash
        // itself), not the DB column: test failpoints may diverge the body.
        let payload_hex = hex::encode(&bytes);
        let Some(mut tx) = begin_hash_upload(pool, cutoff, &started).await? else {
            return Ok(());
        };
        let uploaded = put_state_hash(
            s3,
            my_bucket,
            &key,
            Some(&block_hash_hex),
            bytes,
            "state_hash S3 upload timed out",
        )
        .await;
        match uploaded {
            Ok(_) => {
                sqlx::query!(
                    "UPDATE state_hash SET s3_uploaded_at = NOW()
                      WHERE chain_id = $1 AND block_number = $2",
                    chain_id,
                    block_number,
                )
                .execute(&mut *tx)
                .await?;
                tx.commit().await?;
                info!(chain_id, block_number, block_hash = %block_hash_hex, bucket = my_bucket, key = %log_key, state_hash = %payload_hex, "state_hash uploaded");
            }
            Err(e) => {
                tx.rollback().await?;
                warn!(chain_id, block_number, bucket = my_bucket, key = %log_key, state_hash = %payload_hex, error = %e, "state_hash upload failed; will retry");
            }
        }
    }
    Ok(())
}

/// Uploads Gateway-inputs `state_hash` rows (`chain_id = gw_chain_id`) with
/// `s3_uploaded_at IS NULL`. Unlike [`upload_pending_state_hashes`] there is no
/// `host_chain_blocks_valid` join — Gateway blocks have no host block, so no
/// `block-hash` metadata is attached (none is stored anywhere for the Gateway
/// chain, and the consensus consumer reads only the blob body).
async fn upload_pending_gw_state_hashes(
    pool: &Pool<Postgres>,
    s3: &aws_sdk_s3::Client,
    my_bucket: &str,
    gw_chain_id: i64,
    batch_limit: i64,
    cutoff: Duration,
) -> anyhow::Result<()> {
    let select_sql = format!(
        r#"
        SELECT block_number, state_hash
          FROM {GCS_SCHEMA_QUOTED}.state_hash
         WHERE chain_id = $1 AND s3_uploaded_at IS NULL
         ORDER BY block_number
         LIMIT $2
        "#
    );
    let started = upload_window(pool, cutoff).await?.0;
    let Some(keying) = Keying::of(&started) else {
        return Ok(());
    };
    let pending = sqlx::query(&select_sql)
        .bind(gw_chain_id)
        .bind(batch_limit)
        .fetch_all(pool)
        .await?;

    let update_sql = format!(
        "UPDATE {GCS_SCHEMA_QUOTED}.state_hash SET s3_uploaded_at = NOW()
          WHERE chain_id = $1 AND block_number = $2"
    );
    for row in pending {
        let block_number: i64 = row.try_get("block_number")?;
        let state_hash: String = row.try_get("state_hash")?;
        let bytes = match hex::decode(&state_hash) {
            Ok(b) => b,
            Err(e) => {
                warn!(gw_chain_id, block_number, error = %e, "malformed gw state_hash hex in DB; skipping row");
                continue;
            }
        };
        let key = keying.key(gw_chain_id, block_number);
        let log_key = keying.log_key(gw_chain_id, block_number);
        let Some(mut tx) = begin_hash_upload(pool, cutoff, &started).await? else {
            return Ok(());
        };
        let uploaded = put_state_hash(
            s3,
            my_bucket,
            &key,
            None,
            bytes,
            "gw state_hash S3 upload timed out",
        )
        .await;
        match uploaded {
            Ok(_) => {
                sqlx::query(&update_sql)
                    .bind(gw_chain_id)
                    .bind(block_number)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
                info!(
                    gw_chain_id,
                    block_number,
                    bucket = my_bucket,
                    key = %log_key,
                    state_hash = %state_hash,
                    "gw state_hash uploaded"
                );
            }
            Err(e) => {
                tx.rollback().await?;
                warn!(gw_chain_id, block_number, bucket = my_bucket, key = %log_key, state_hash = %state_hash, error = %e, "gw state_hash upload failed; will retry");
            }
        }
    }
    Ok(())
}

/// True iff the versioned GCS schema (e.g. `gcs-0.15.0`) exists in the catalog.
/// Checked against `information_schema.schemata` by exact (unquoted) name, so the
/// hyphen/dots in the schema name need no identifier quoting. See
/// [`compute_and_upload_state_hashes`] for why the pass is gated on this.
async fn gcs_schema_exists(tx: &mut Transaction<'_, Postgres>) -> anyhow::Result<bool> {
    let (exists,): (bool,) = sqlx::query_as(
        "SELECT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
    )
    .bind(GCS_SCHEMA)
    .fetch_one(&mut **tx)
    .await?;
    Ok(exists)
}

/// One reconciliation pass: compute + insert any pending GCS state hashes for
/// the active upgrade window (both the host-chain track and the Gateway-inputs
/// track), then upload any rows not yet in S3. Both steps are idempotent and
/// drain-on-each-call, so a failed insert or PUT simply retries on the next pass.
async fn compute_and_upload_state_hashes(
    pool: &Pool<Postgres>,
    s3: Option<&aws_sdk_s3::Client>,
    my_bucket: &str,
    batch_limit: i64,
    gw_chain_id: i64,
    cutoff: Duration,
) -> anyhow::Result<()> {
    let mut tx = match begin_write_guarded(pool, true, GcsRollbackPolicy::Skip).await? {
        WriteGuard::Proceed(tx) => tx,
        WriteGuard::Stop | WriteGuard::Skip => return Ok(()),
    };

    // Every branch below touches the versioned GCS schema — the compute paths and
    // the GW-inputs upload explicitly via GCS_SCHEMA_QUOTED, the host-chain upload
    // implicitly through the gcs-first search_path. That schema is created by the
    // GCS-side upgrade-controller at startup, and only for the stack version it is
    // dry-running. Before an upgrade of THIS binary's version — or on a plain BCS
    // deployment whose live version differs from the compiled STACK_VERSION — the
    // schema is absent, and the hard-qualified GW upload would fail every pass with
    // `relation "gcs-<ver>.state_hash" does not exist`. There is nothing to compute
    // or upload without the schema, so skip the whole pass cleanly.
    if !gcs_schema_exists(&mut tx).await? {
        debug!(
            schema = GCS_SCHEMA,
            "GCS schema absent — skipping state_hash pass"
        );
        tx.rollback().await?;
        return Ok(());
    }

    // GCS hashes are only produced during an active upgrade window; BCS hashes
    // are not produced because they would never be uploaded or consumed.
    //
    // Two independent reasons to skip the Gateway track this pass:
    //  - skip_gw: the GW track has already anchored (gw_consensus_reached). Further GW hashes
    //    feed nothing and would firehose thousands of hashes on the fast Gateway chain, starving
    //    the host tracks' uploads and delaying host unanimity.
    //  - watermark not yet at gw_start_block: the GCS gw-listener tails the Gateway from startup,
    //    before the proposal activates (see upgrade-controller `run`), and gw_start_block is
    //    pinned to the tip at proposal time, so a listener that was already tailing has processed
    //    gw_start_block contiguously; `gw_tip >= gw_start` then means every hashed block in
    //    [gw_start, gw_tip) has complete inputs. Uploading before that would let an operator that
    //    never saw the window start contribute hashes, so the fleet could anchor on a later block
    //    while that operator silently diverged at gw_start. (A listener first started after
    //    activation, past gw_start, is out of scope - intended order is Green tailing before the
    //    proposal.)
    //
    // gw_ready captures "GW track eligible this pass" (not anchored AND watermark past gw_start);
    // it gates compute AND upload.
    let mut gw_ready = false;
    if let Some((_, _, _, mut windows)) = crate::active_upgrade_windows(&mut *tx).await? {
        let skip_gw = gw_consensus_reached(&mut *tx).await?;
        windows.sort_unstable_by_key(|(chain_id, _, _)| *chain_id);
        for (chain_id, start, end) in windows {
            compute_and_insert_gcs(
                &mut tx,
                chain_id,
                start,
                end,
                batch_limit,
                GCS_SCHEMA_QUOTED,
            )
            .await?;
        }

        // Gateway-inputs track: only while not yet anchored (skip_gw), once the GCS
        // gw-listener has latched its window alignment (gw_window_rewound - closes the
        // mid-tick activation race), and once its watermark has reached gw_start_block.
        // The gate itself lives in compute_and_insert_gw_input_hashes (it self-skips);
        // here it is mirrored into gw_ready to gate the upload.
        if !skip_gw {
            if let Some((gw_start, gw_tip, gw_aligned)) = gw_window_rewound(&mut *tx).await? {
                gw_ready = gw_aligned && gw_tip >= gw_start;
                compute_and_insert_gw_input_hashes(
                    &mut tx,
                    gw_chain_id,
                    gw_start,
                    gw_tip,
                    gw_aligned,
                    batch_limit,
                )
                .await?;
            }
        } else {
            debug!("gw consensus reached — skipping GW state_hash compute/upload");
        }
    } else {
        compute_post_cutover_state_hashes(&mut tx, batch_limit).await?;
    }
    tx.commit().await?;

    // S3 upload: drains pending rows; runs every pass so failed PUTs retry. Gateway uploads run
    // only when the track was eligible this pass (gw_ready: not anchored and watermark past
    // gw_start).
    if let Some(s3) = s3 {
        upload_pending_state_hashes(pool, s3, my_bucket, batch_limit, cutoff).await?;
        if gw_ready {
            upload_pending_gw_state_hashes(pool, s3, my_bucket, gw_chain_id, batch_limit, cutoff)
                .await?;
        }
    }
    Ok(())
}

/// Test-box-only build: keep computing `state_hash` rows AFTER cutover, once
/// there is no active upgrade window, against the retained gcs schema now living
/// in `public`. Hash every finalized, not-yet-hashed block for each host chain.
/// `compute_and_insert_gcs` skips already-hashed blocks (`NOT EXISTS`) and is
/// bounded by `batch_limit`, so a full `[0, i64::MAX]` range is safe and
/// idempotent.
///
/// Used by the blue/green consensus test loop to keep verifying operator
/// agreement past cutover; not needed by the upgrade procedure itself, which is
/// why it only exists behind the `post-cutover-state-hash` Cargo feature — a
/// production build never compiles this in. Enable with `cargo build --features
/// post-cutover-state-hash` (or add it under `[features]` in the consuming
/// Cargo.toml) on the test box only.
#[cfg(feature = "post-cutover-state-hash")]
async fn compute_post_cutover_state_hashes(
    tx: &mut Transaction<'_, Postgres>,
    batch_limit: i64,
) -> anyhow::Result<()> {
    let chain_rows = sqlx::query("SELECT chain_id FROM public.host_chains ORDER BY chain_id")
        .fetch_all(&mut **tx)
        .await?;
    for row in chain_rows {
        let chain_id: i64 = row.try_get("chain_id")?;
        compute_and_insert_gcs(tx, chain_id, 0, i64::MAX, batch_limit, "public").await?;
    }
    Ok(())
}

/// Production build: no active upgrade window means nothing to hash — see
/// [`compute_and_upload_state_hashes`]. The `post-cutover-state-hash` feature
/// swaps in the test-box variant above.
#[cfg(not(feature = "post-cutover-state-hash"))]
async fn compute_post_cutover_state_hashes(
    _tx: &mut Transaction<'_, Postgres>,
    _batch_limit: i64,
) -> anyhow::Result<()> {
    Ok(())
}

/// Pure pg-notify driven: one pass at startup, then one on every
/// `event_ciphertext_computed`, `event_ciphertext128_computed`,
/// `event_new_block`, or `event_gw_new_block`.
/// Listening on `event_new_block` covers host windows with no FHE activity —
/// `event_ciphertext_computed` only fires when work happens, so empty dry-run
/// windows would otherwise never produce sentinel rows.
/// `event_ciphertext128_computed` fires when the sns-worker commits a squash
/// batch, which is what unblocks a block whose hash waits on PBS work.
/// `event_gw_new_block` advances the Gateway tip so newly-sealed Gateway
/// blocks get hashed.
pub async fn run(
    pool: Pool<Postgres>,
    s3: Option<Arc<aws_sdk_s3::Client>>,
    my_bucket: String,
    batch_limit: i64,
    gw_chain_id: i64,
    cutoff: Duration,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    info!(batch_limit, bucket = %my_bucket, gw_chain_id, "starting state_hash worker");

    let mut listener = PgListener::connect_with(&pool).await?;
    listener
        .listen_all([
            EVENT_CIPHERTEXT_COMPUTED,
            EVENT_CIPHERTEXT128_COMPUTED,
            crate::NEW_BLOCK_CHANNEL,
            EVENT_GW_NEW_BLOCK,
        ])
        .await?;

    if let Err(e) = compute_and_upload_state_hashes(
        &pool,
        s3.as_deref(),
        &my_bucket,
        batch_limit,
        gw_chain_id,
        cutoff,
    )
    .await
    {
        warn!(error = %e, "initial state_hash pass failed");
    }
    loop {
        select! {
            _ = cancel.cancelled() => return Ok(()),
            recv = listener.recv() => match recv {
                Ok(n) => {
                    debug!(channel = n.channel(), "state_hash pass triggered");
                    if let Err(e) =
                        compute_and_upload_state_hashes(&pool, s3.as_deref(), &my_bucket, batch_limit, gw_chain_id, cutoff).await
                    {
                        warn!(error = %e, "state_hash pass failed");
                    }
                }
                Err(e) => {
                    error!(error = %e, "listener recv error; sleeping");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn uploaded_upgrade_hash_contains_replay_that_would_add_work() -> anyhow::Result<()> {
        use crate::ingestion_recovery_tests::{
            object_store, partial_block, repair_is_contained, seed_attempt,
        };
        let (mut db, pool, _instance) = partial_block().await?;
        seed_attempt(&pool).await?;
        let mut tx = pool.begin().await?;
        compute_and_insert_gcs(&mut tx, 42, 100, 100, 10, "public").await?;
        tx.commit().await?;
        let hash: String = sqlx::query_scalar(
            "SELECT state_hash FROM state_hash WHERE chain_id=42 AND block_number=100",
        )
        .fetch_one(&pool)
        .await?;
        assert_ne!(hash, EMPTY_BLOCK_STATE_HASH);
        let (s3, uploaded) = object_store().await?;
        upload_pending_state_hashes(&pool, &s3, "test", 10, Duration::from_secs(3600)).await?;
        let body = tokio::time::timeout(Duration::from_secs(10), uploaded).await??;
        assert_eq!(body, hash.as_bytes(), "record uploads byte-for-byte");
        repair_is_contained(&mut db, &pool).await?;
        let retained: (String, bool) = sqlx::query_as("SELECT state_hash, s3_uploaded_at IS NOT NULL FROM state_hash WHERE chain_id=42 AND block_number=100").fetch_one(&pool).await?;
        assert_eq!(retained, (hash, true));
        Ok(())
    }

    #[tokio::test]
    async fn host_hash_waits_for_ingestion_before_reading_completion() -> anyhow::Result<()> {
        use fhevm_engine_common::chain_recovery::{lock_history_tx, ListenerSource};
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await?;
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, block_number, block_status, fhe_event_count)
            VALUES (42, '\\x10', 100, 'finalized', 2)",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO computations
            (output_handle, dependencies, fhe_operation, is_scalar, transaction_id,
             host_chain_id, block_number, is_allowed, is_completed)
            VALUES ('\\x01', '{}', 0, FALSE, '\\x01', 42, 100, TRUE, TRUE),
                   ('\\x02', '{}', 0, FALSE, '\\x02', 42, 100, FALSE, TRUE)",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO ciphertexts
            (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
            VALUES (0, '\\x01', '\\xaa', 0, 5)",
        )
        .execute(&pool)
        .await?;

        let mut repair = pool.begin().await?;
        lock_history_tx(&mut repair, ListenerSource::Host, false, 42).await?;
        sqlx::query(
            "UPDATE computations SET is_allowed = TRUE, is_completed = FALSE
            WHERE output_handle = '\\x02'",
        )
        .execute(&mut *repair)
        .await?;
        let hash_pool = pool.clone();
        let hash_task = tokio::spawn(async move {
            let mut tx = hash_pool.begin().await?;
            compute_and_insert_gcs(&mut tx, 42, 100, 100, 10, "public").await?;
            tx.commit().await?;
            anyhow::Ok(())
        });
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let waiting: bool = sqlx::query_scalar(
                    "SELECT EXISTS(SELECT 1 FROM pg_locks
                    WHERE locktype = 'advisory' AND NOT granted AND database =
                        (SELECT oid FROM pg_database WHERE datname = current_database()))",
                )
                .fetch_one(&pool)
                .await
                .unwrap();
                if waiting {
                    break;
                }
                assert!(
                    !hash_task.is_finished(),
                    "hashing bypassed ingestion's fence"
                );
                tokio::task::yield_now().await;
            }
        })
        .await?;
        repair.commit().await?;
        tokio::time::timeout(Duration::from_secs(5), hash_task).await???;
        let hashes: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM state_hash")
            .fetch_one(&pool)
            .await?;
        assert_eq!(hashes, 0, "repaired B is pending; do not freeze A alone");

        sqlx::query("UPDATE computations SET is_completed = TRUE WHERE output_handle = '\\x02'")
            .execute(&pool)
            .await?;
        sqlx::query(
            "INSERT INTO ciphertexts
            (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
            VALUES (0, '\\x02', '\\xbb', 0, 5)",
        )
        .execute(&pool)
        .await?;
        let mut tx = pool.begin().await?;
        compute_and_insert_gcs(&mut tx, 42, 100, 100, 10, "public").await?;
        tx.commit().await?;
        let includes_b: bool = sqlx::query_scalar(
            r#"SELECT state_hash = '{"v":1,"fhe":"' || encode(sha256('\xaabb'::bytea), 'hex')
                   || '","pbs":"' || encode(sha256(''::bytea), 'hex') || '"}'
            FROM state_hash WHERE chain_id = 42 AND block_number = 100"#,
        )
        .fetch_one(&pool)
        .await?;
        assert!(includes_b);
        Ok(())
    }

    /// fhevm-internal#1960: the hash waits for the block's PBS (squash) work
    /// and binds the ct128 digests, so a divergent sns-worker breaks consensus
    /// instead of being invisible to it.
    #[tokio::test]
    async fn host_hash_waits_for_pbs_and_binds_ct128_digest() -> anyhow::Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await?;
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, block_number, block_status, fhe_event_count)
            VALUES (42, '\\x10', 100, 'finalized', 1)",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO computations
            (output_handle, dependencies, fhe_operation, is_scalar, transaction_id,
             host_chain_id, block_number, is_allowed, is_completed)
            VALUES ('\\x01', '{}', 0, FALSE, '\\x01', 42, 100, TRUE, TRUE)",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO ciphertexts
            (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
            VALUES (0, '\\x01', '\\xaa', 0, 5)",
        )
        .execute(&pool)
        .await?;
        // The ACL Allowed ingest scheduled PBS work for the output handle;
        // the squash has not run yet.
        sqlx::query(
            "INSERT INTO pbs_computations (handle, host_chain_id, block_number, is_completed)
             VALUES ('\\x01', 42, 100, FALSE)",
        )
        .execute(&pool)
        .await?;

        let hashes = |pool: Pool<Postgres>| async move {
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM state_hash WHERE chain_id = 42 AND block_number = 100",
            )
            .fetch_one(&pool)
            .await
            .unwrap()
        };
        let pass = |pool: Pool<Postgres>| async move {
            let mut tx = pool.begin().await?;
            compute_and_insert_gcs(&mut tx, 42, 100, 100, 10, "public").await?;
            tx.commit().await?;
            anyhow::Ok(())
        };

        pass(pool.clone()).await?;
        assert_eq!(
            hashes(pool.clone()).await,
            0,
            "no hash while the block's PBS work is pending"
        );

        // Completed without a recorded ct128 digest (the sns-worker commits
        // both atomically, so this state only exists mid-crash or on a skipped
        // enqueue): still held back.
        sqlx::query("UPDATE pbs_computations SET is_completed = TRUE WHERE handle = '\\x01'")
            .execute(&pool)
            .await?;
        pass(pool.clone()).await?;
        assert_eq!(
            hashes(pool.clone()).await,
            0,
            "no hash while the ct128 digest is unrecorded"
        );

        sqlx::query(
            "INSERT INTO ciphertext_digest (handle, ciphertext128) VALUES ('\\x01', '\\xdd')",
        )
        .execute(&pool)
        .await?;
        pass(pool.clone()).await?;
        let binds_ct128: bool = sqlx::query_scalar(
            r#"SELECT state_hash = '{"v":1,"fhe":"' || encode(sha256('\xaa'::bytea), 'hex')
                   || '","pbs":"' || encode(sha256('\xdd'::bytea), 'hex') || '"}'
            FROM state_hash WHERE chain_id = 42 AND block_number = 100"#,
        )
        .fetch_one(&pool)
        .await?;
        assert!(
            binds_ct128,
            "record commits fhe=sha256(ct64 bytes) and pbs=sha256(ct128 digest) once PBS is done"
        );
        Ok(())
    }

    /// The PBS set is the squash work the block itself scheduled for its own
    /// output handles. A row a LATER block inserted for a handle computed
    /// here (e.g. a bridge send squashing its source) must not hold or bind
    /// this block's hash - whether that insert is visible depends on
    /// ingestion timing, so a stamp-blind set would commit differently per
    /// operator; it is excluded on every operator instead. A row for a handle
    /// whose computation errored can never be squashed and must not hold the
    /// hash either (the `bc` join's `is_error` filter drops it).
    #[tokio::test]
    async fn pbs_set_is_the_blocks_own_scheduling_not_its_output_handles() -> anyhow::Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await?;
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, block_number, block_status, fhe_event_count)
            VALUES (42, '\\x10', 100, 'finalized', 2)",
        )
        .execute(&pool)
        .await?;
        // Two completed computations with ciphertexts; one errored one without.
        for (handle, completed, error) in [
            ("\\x01", true, false),
            ("\\x03", true, false),
            ("\\x05", false, true),
        ] {
            sqlx::query(&format!(
                "INSERT INTO computations
                (output_handle, dependencies, fhe_operation, is_scalar, transaction_id,
                 host_chain_id, block_number, is_allowed, is_completed, is_error)
                VALUES ('{handle}', '{{}}', 0, FALSE, '\\x01', 42, 100, TRUE, {completed}, {error})"
            ))
            .execute(&pool)
            .await?;
        }
        for (handle, ct) in [("\\x01", "\\xaa"), ("\\x03", "\\xbb")] {
            sqlx::query(&format!(
                "INSERT INTO ciphertexts
                (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                VALUES (0, '{handle}', '{ct}', 0, 5)"
            ))
            .execute(&pool)
            .await?;
        }
        sqlx::query(
            "INSERT INTO pbs_computations (handle, host_chain_id, block_number, is_completed)
             VALUES ('\\x01', 42, 100, TRUE),  -- this block's work, squashed
                    ('\\x03', 42, 101, FALSE), -- a LATER block scheduled this handle
                    ('\\x05', 42, 100, FALSE)  -- this block's work, handle errored",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO ciphertext_digest (handle, ciphertext128) VALUES ('\\x01', '\\xdd')",
        )
        .execute(&pool)
        .await?;

        let mut tx = pool.begin().await?;
        compute_and_insert_gcs(&mut tx, 42, 100, 100, 10, "public").await?;
        tx.commit().await?;
        let frozen: bool = sqlx::query_scalar(
            r#"SELECT state_hash = '{"v":1,"fhe":"' || encode(sha256('\xaabb'::bytea), 'hex')
                   || '","pbs":"' || encode(sha256('\xdd'::bytea), 'hex') || '"}'
            FROM state_hash WHERE chain_id = 42 AND block_number = 100"#,
        )
        .fetch_one(&pool)
        .await?;
        assert!(
            frozen,
            "hash must commit immediately, binding only the block's own squash"
        );
        Ok(())
    }

    #[tokio::test]
    async fn containment_stops_state_hash_uploads() -> anyhow::Result<()> {
        use fhevm_engine_common::chain_recovery::{record_halt, HistoryMismatch, ListenerSource};
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await?;
        record_halt(
            &pool,
            HistoryMismatch {
                source: ListenerSource::Host,
                gcs_mode: false,
                chain_id: 42,
                block_number: 100,
                expected_hash: None,
                observed_hash: None,
                reason: "committed inventory changed",
            },
        )
        .await?;
        assert!(begin_hash_upload(&pool, Duration::from_secs(60), &None)
            .await?
            .is_none());
        Ok(())
    }

    #[tokio::test]
    async fn gw_hashes_skipped_until_watermark_reaches_gw_start() {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .unwrap();

        // Fresh gcs schema mirroring the tables the Gateway hash computation touches.
        sqlx::query(&format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"))
            .execute(&pool)
            .await
            .unwrap();
        for table in [
            "state_hash",
            "input_handles",
            "ciphertexts",
            "verify_proofs",
        ] {
            sqlx::query(&format!(
                "CREATE TABLE {GCS_SCHEMA_QUOTED}.{table} \
                 (LIKE public.{table} INCLUDING ALL)"
            ))
            .execute(&pool)
            .await
            .unwrap_or_else(|e| panic!("create gcs.{table}: {e}"));
        }

        let gw_chain_id: i64 = 54321;
        let gw_start: i64 = 100;

        // A fully computable anchor block: one input handle with its is_input ciphertext, and a
        // resolved (verified) synthetic verify_proofs row. Only gw_tip (the watermark) varies.
        sqlx::query(&format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.input_handles (handle, block_number) \
             VALUES ('\\xaa', $1)"
        ))
        .bind(gw_start)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(&format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.ciphertexts \
                (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type, is_input) \
             VALUES (1, '\\xaa', '\\xbb', 0, 0, TRUE)"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(&format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.verify_proofs \
                (zk_proof_id, chain_id, contract_address, user_address, block_number, verified) \
             VALUES ($1, $2, '', '', $3, TRUE)"
        ))
        .bind(fhevm_engine_common::synthetic_input::SYNTHETIC_ZK_PROOF_ID_BASE)
        .bind(gw_chain_id)
        .bind(gw_start)
        .execute(&pool)
        .await
        .unwrap();

        let hashes = |pool: Pool<Postgres>| async move {
            sqlx::query_scalar::<_, i64>(&format!(
                "SELECT COUNT(*) FROM {GCS_SCHEMA_QUOTED}.state_hash \
                  WHERE chain_id = $1 AND block_number = $2"
            ))
            .bind(gw_chain_id)
            .bind(gw_start)
            .fetch_one(&pool)
            .await
            .unwrap()
        };

        // Watermark has NOT reached gw_start (gw_tip < gw_start): no Gateway hash is produced,
        // even though the synthetic row and inputs are present.
        let mut tx = pool.begin().await.unwrap();
        compute_and_insert_gw_input_hashes(&mut tx, gw_chain_id, gw_start, gw_start - 1, true, 10)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        assert_eq!(
            hashes(pool.clone()).await,
            0,
            "no Gateway hash before the watermark reaches gw_start"
        );

        // Alignment latch NOT set (gw_window_rewound = false): no Gateway hash either, even
        // with the watermark at gw_start - the mid-tick activation race leaves exactly this
        // state with the window start unscanned.
        let mut tx = pool.begin().await.unwrap();
        compute_and_insert_gw_input_hashes(&mut tx, gw_chain_id, gw_start, gw_start, false, 10)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        assert_eq!(
            hashes(pool.clone()).await,
            0,
            "no Gateway hash before the listener has latched its window alignment"
        );

        // Latched AND watermark at gw_start: the anchor hash is produced.
        let mut tx = pool.begin().await.unwrap();
        compute_and_insert_gw_input_hashes(&mut tx, gw_chain_id, gw_start, gw_start, true, 10)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        assert_eq!(
            hashes(pool.clone()).await,
            1,
            "Gateway hash is produced once the watermark reaches gw_start"
        );
    }

    #[tokio::test]
    async fn host_upload_stops_once_the_window_closes() {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .unwrap();
        let localstack = test_harness::localstack::start_localstack().await.unwrap();
        let s3 = test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
        s3.create_bucket().bucket("hashes").send().await.unwrap();
        let cutoff = Duration::from_secs(60);
        sqlx::query(
            "INSERT INTO upgrade_state
                (stack_role, host_chain_id, state, status, proposal_id, proposal_block, version, end_block)
             VALUES ('GCS', 1, 'DryRunStarted', 'in_progress', '\\x02', 10, '1.2.3', 200)",
        )
        .execute(&pool)
        .await
        .unwrap();
        let attempt = UpgradeAttempt {
            proposal_id: vec![2],
            proposal_block: 10,
        };
        // A finalized block with a non-trivial computation and a pending hash.
        let pending_block = |block: i64| {
            let pool = pool.clone();
            async move {
                sqlx::query(
                    "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
                     VALUES (1, $1, $2, 'finalized')",
                )
                .bind(vec![block as u8])
                .bind(block)
                .execute(&pool)
                .await
                .unwrap();
                sqlx::query(
                    "INSERT INTO computations (
                         output_handle, dependencies, fhe_operation, is_scalar,
                         transaction_id, host_chain_id, block_number, is_error, is_completed
                     ) VALUES ($1, $2, 0, FALSE, $1, 1, $3, FALSE, TRUE)",
                )
                .bind(vec![block as u8])
                .bind(Vec::<Vec<u8>>::new())
                .bind(block)
                .execute(&pool)
                .await
                .unwrap();
                sqlx::query(
                    r#"INSERT INTO state_hash (chain_id, block_number, state_hash)
                       VALUES (1, $1, '{"v":1,"fhe":"' || repeat('a', 64)
                                  || '","pbs":"' || repeat('b', 64) || '"}')"#,
                )
                .bind(block)
                .execute(&pool)
                .await
                .unwrap();
            }
        };
        let uploaded = |block: i64| {
            sqlx::query_scalar::<_, bool>(
                "SELECT s3_uploaded_at IS NOT NULL FROM state_hash WHERE chain_id = 1 AND block_number = $1",
            )
            .bind(block)
        };

        pending_block(150).await;
        upload_pending_state_hashes(&pool, &s3, "hashes", 10, cutoff)
            .await
            .unwrap();
        assert!(
            uploaded(150).fetch_one(&pool).await.unwrap(),
            "open window uploads"
        );

        sqlx::query(
            "INSERT INTO host_chain_blocks_valid
                (chain_id, block_hash, block_number, block_status, created_at)
             VALUES (1, '\\xc8', 200, 'finalized', NOW() - INTERVAL '2 minutes')",
        )
        .execute(&pool)
        .await
        .unwrap();
        pending_block(151).await;
        upload_pending_state_hashes(&pool, &s3, "hashes", 10, cutoff)
            .await
            .unwrap();
        assert!(
            !uploaded(151).fetch_one(&pool).await.unwrap(),
            "closed window uploads nothing"
        );
        assert!(s3
            .head_object()
            .bucket("hashes")
            .key(state_hash_key(&attempt, 1, 151))
            .send()
            .await
            .is_err());
        // The uploaded object sits at the attempt-scoped key.
        assert!(s3
            .head_object()
            .bucket("hashes")
            .key(state_hash_key(&attempt, 1, 150))
            .send()
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn gw_upload_stops_once_the_window_closes_or_the_attempt_ends() {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(instance.db_url())
            .await
            .unwrap();
        let localstack = test_harness::localstack::start_localstack().await.unwrap();
        let s3 = test_harness::localstack::create_localstack_s3_client(localstack.host_port).await;
        s3.create_bucket().bucket("hashes").send().await.unwrap();
        let cutoff = Duration::from_secs(60);
        for sql in [
            format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"),
            format!("CREATE TABLE {GCS_SCHEMA_QUOTED}.state_hash (LIKE public.state_hash INCLUDING ALL)"),
            "INSERT INTO public.upgrade_state
                (stack_role, host_chain_id, state, status, proposal_id, proposal_block, version, end_block)
             VALUES ('GCS', 1, 'DryRunStarted', 'in_progress', '\\x02', 10, '1.2.3', 200)"
                .into(),
        ] {
            sqlx::query(&sql).execute(&pool).await.unwrap();
        }
        let insert_sql = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.state_hash (chain_id, block_number, state_hash)
             VALUES (54321, $1, 'aa')"
        );
        let uploaded_sql = format!(
            "SELECT s3_uploaded_at IS NOT NULL FROM {GCS_SCHEMA_QUOTED}.state_hash
              WHERE block_number = $1"
        );
        let pending_hash = |block: i64| sqlx::query(&insert_sql).bind(block);
        let uploaded = |block: i64| sqlx::query_scalar::<_, bool>(&uploaded_sql).bind(block);

        pending_hash(1).execute(&pool).await.unwrap();
        upload_pending_gw_state_hashes(&pool, &s3, "hashes", 54321, 10, cutoff)
            .await
            .unwrap();
        assert!(
            uploaded(1).fetch_one(&pool).await.unwrap(),
            "open window uploads"
        );

        sqlx::query(
            "INSERT INTO public.host_chain_blocks_valid
                (chain_id, block_hash, block_number, block_status, created_at)
             VALUES (1, '\\x01', 200, 'finalized', NOW() - INTERVAL '2 minutes')",
        )
        .execute(&pool)
        .await
        .unwrap();
        pending_hash(2).execute(&pool).await.unwrap();
        upload_pending_gw_state_hashes(&pool, &s3, "hashes", 54321, 10, cutoff)
            .await
            .unwrap();
        let attempt = UpgradeAttempt {
            proposal_id: vec![2],
            proposal_block: 10,
        };
        assert!(
            !uploaded(2).fetch_one(&pool).await.unwrap(),
            "closed window uploads nothing"
        );
        assert!(s3
            .head_object()
            .bucket("hashes")
            .key(state_hash_key(&attempt, 54321, 2))
            .send()
            .await
            .is_err());
        // The uploaded object sits at the attempt-scoped key.
        assert!(s3
            .head_object()
            .bucket("hashes")
            .key(state_hash_key(&attempt, 54321, 1))
            .send()
            .await
            .is_ok());

        let started = Some((vec![2u8], 10));
        sqlx::query("UPDATE public.upgrade_state SET status = 'failed'")
            .execute(&pool)
            .await
            .unwrap();
        assert!(
            begin_hash_upload(&pool, cutoff, &started)
                .await
                .unwrap()
                .is_none(),
            "attempt ended mid-batch"
        );
        assert!(
            begin_hash_upload(&pool, cutoff, &None)
                .await
                .unwrap()
                .is_some(),
            "no attempt at batch start"
        );
        sqlx::query(
            "UPDATE public.upgrade_state
                SET status = 'in_progress', proposal_id = '\\x03', end_block = 300",
        )
        .execute(&pool)
        .await
        .unwrap();
        assert!(
            begin_hash_upload(&pool, cutoff, &started)
                .await
                .unwrap()
                .is_none(),
            "attempt replaced mid-batch"
        );
        sqlx::query("UPDATE public.upgrade_state SET proposal_id = '\\x02', proposal_block = 11")
            .execute(&pool)
            .await
            .unwrap();
        assert!(
            begin_hash_upload(&pool, cutoff, &started)
                .await
                .unwrap()
                .is_none(),
            "same proposal re-proposed at another block"
        );
    }

    fn attempt(proposal_id: Vec<u8>, proposal_block: i64) -> UpgradeAttempt {
        UpgradeAttempt {
            proposal_id,
            proposal_block,
        }
    }

    #[test]
    fn key_format_is_stable() {
        assert_eq!(
            state_hash_key(&attempt(vec![0xab, 0xcd], 7), 1, 42),
            "state_hash/proposal=abcd/attempt=7/chain=1/block=42.bin"
        );
    }

    #[test]
    fn log_key_abbreviates_the_proposal_id() {
        // Production ids are 32 bytes (a BE uint256 from the L1 event): the
        // log form keeps the first 2 and last 4 bytes.
        let mut id = vec![0u8; 32];
        id[28..].copy_from_slice(&[0x00, 0x11, 0x11, 0x22]);
        assert_eq!(
            state_hash_log_key(&attempt(id, 7), 1, 42),
            "state_hash/proposal=0000...00111122/attempt=7/chain=1/block=42.bin"
        );
        // Short ids print in full - nothing to elide.
        let short = attempt(vec![0xab, 0xcd], 7);
        assert_eq!(
            state_hash_log_key(&short, 1, 42),
            state_hash_key(&short, 1, 42)
        );
    }

    #[test]
    fn key_is_chain_scoped_for_gateway() {
        // The Gateway track reuses the same key scheme under the gw chain id, so
        // gateway and host objects never collide (distinct chain ids).
        let a = attempt(vec![0xab, 0xcd], 7);
        assert_eq!(
            state_hash_key(&a, 54321, 7),
            "state_hash/proposal=abcd/attempt=7/chain=54321/block=7.bin"
        );
        assert_ne!(state_hash_key(&a, 1, 7), state_hash_key(&a, 54321, 7));
    }

    #[test]
    fn key_is_attempt_scoped() {
        // The motivating failure (fhevm-internal#2273): a retry whose
        // gw_start_block did not move reuses (chain, block) pairs, so only the
        // attempt identity keeps its objects apart - including a re-activation
        // of the SAME proposal, which reuses proposal_id but not proposal_block.
        let first = attempt(vec![0xab], 10);
        let retried = attempt(vec![0xab], 20);
        let other_proposal = attempt(vec![0xcd], 10);
        let keys: Vec<String> = [&first, &retried, &other_proposal]
            .iter()
            .map(|a| state_hash_key(a, 1, 42))
            .collect();
        for (i, key) in keys.iter().enumerate() {
            assert!(key.starts_with("state_hash/"), "family prefix first: {key}");
            assert!(!key.contains("//") && !key.starts_with('/'), "{key}");
            for later in &keys[i + 1..] {
                assert_ne!(key, later);
            }
        }
    }

    #[test]
    fn empty_block_state_hash_is_well_formed() {
        assert_eq!(EMPTY_BLOCK_STATE_HASH.len(), 64);
        assert!(EMPTY_BLOCK_STATE_HASH
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
        assert_eq!(
            EMPTY_BLOCK_STATE_HASH,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    /// The exact bytes are the consensus commitment: any serialization change
    /// (field order, spacing, version) diverges the whole fleet against
    /// operators on the previous build. This pins the canonical form.
    #[test]
    fn record_serialization_is_canonical_and_stable() {
        let record = StateHashRecord {
            v: STATE_HASH_RECORD_VERSION,
            fhe: "aa".into(),
            pbs: "bb".into(),
        };
        assert_eq!(record.to_json(), r#"{"v":1,"fhe":"aa","pbs":"bb"}"#);
        let parsed: StateHashRecord = serde_json::from_str(&record.to_json()).unwrap();
        assert_eq!(parsed, record);
    }

    #[test]
    fn evaluate_host_slots_counts_votes_on_parsed_components() {
        let hex = |c: char| c.to_string().repeat(64);
        let rec = |fhe: char, pbs: char| {
            Some(
                StateHashRecord {
                    v: STATE_HASH_RECORD_VERSION,
                    fhe: hex(fhe),
                    pbs: hex(pbs),
                }
                .to_json()
                .into_bytes(),
            )
        };
        // Any missing operator: no verdict yet.
        assert!(evaluate_host_slots(&[]).is_none());
        assert!(evaluate_host_slots(&[rec('a', 'c'), None]).is_none());
        // Full agreement anchors, without encoding skew for identical bytes.
        assert!(matches!(
            evaluate_host_slots(&[rec('a', 'c'), rec('a', 'c'), rec('a', 'c')]),
            Some(HostSlotsVerdict::Anchor {
                encoding_skew: false
            })
        ));
        // Same components under a different version: anchors, flags the skew.
        let skewed =
            Some(format!(r#"{{"v":2,"fhe":"{}","pbs":"{}"}}"#, hex('a'), hex('c')).into_bytes());
        assert!(matches!(
            evaluate_host_slots(&[rec('a', 'c'), skewed]),
            Some(HostSlotsVerdict::Anchor {
                encoding_skew: true
            })
        ));
        // Partial divergence names the component; it never anchors.
        assert!(matches!(
            evaluate_host_slots(&[rec('a', 'c'), rec('b', 'c')]),
            Some(HostSlotsVerdict::Diverged {
                fhe: true,
                pbs: false
            })
        ));
        assert!(matches!(
            evaluate_host_slots(&[rec('a', 'c'), rec('a', 'd')]),
            Some(HostSlotsVerdict::Diverged {
                fhe: false,
                pbs: true
            })
        ));
        assert!(matches!(
            evaluate_host_slots(&[rec('a', 'c'), rec('b', 'd')]),
            Some(HostSlotsVerdict::Diverged {
                fhe: true,
                pbs: true
            })
        ));
        // A gateway blob / old flat payload / corrupt object: invalid, no anchor.
        assert!(matches!(
            evaluate_host_slots(&[rec('a', 'c'), Some(vec![0x42; 32])]),
            Some(HostSlotsVerdict::Invalid { .. })
        ));
    }

    #[test]
    fn record_validation_rejects_empty_or_non_hex_components() {
        let rec = |fhe: &str, pbs: &str| StateHashRecord {
            v: STATE_HASH_RECORD_VERSION,
            fhe: fhe.into(),
            pbs: pbs.into(),
        };
        let a64 = "a".repeat(64);
        assert!(rec(&a64, &"b".repeat(64)).validate().is_ok());
        // Empty, short, or non-hex components are refused.
        assert!(rec("", &a64).validate().is_err());
        assert!(rec(&a64, "").validate().is_err());
        assert!(rec(&a64, "zz").validate().is_err());
        assert!(rec(&"g".repeat(64), &a64).validate().is_err());
        // The empty-aggregate pbs hash (sha256('')) is not allowed: a record
        // that committed to no squash work must not vote.
        let err = rec(&a64, EMPTY_BLOCK_STATE_HASH).validate().unwrap_err();
        assert!(err.contains("empty pbs"), "{err}");
        // ...and such a record never anchors through the vote count either.
        let slot = Some(rec(&a64, EMPTY_BLOCK_STATE_HASH).to_json().into_bytes());
        assert!(matches!(
            evaluate_host_slots(&[slot.clone(), slot]),
            Some(HostSlotsVerdict::Invalid { .. })
        ));
    }

    /// The slots hold peer-controlled bytes: every malformed shape must come
    /// back as a verdict (never a panic), and none may anchor.
    #[test]
    fn evaluate_host_slots_never_panics_on_hostile_payloads() {
        let a64 = "a".repeat(64);
        let b64 = "b".repeat(64);
        let good = Some(format!(r#"{{"v":1,"fhe":"{a64}","pbs":"{b64}"}}"#).into_bytes());
        let hostiles: Vec<Vec<u8>> = vec![
            vec![],                                                              // empty body
            vec![0xff, 0xfe, 0x00],                                              // invalid UTF-8
            b"{".to_vec(),                                                       // truncated JSON
            "[".repeat(100_000).into_bytes(),                                    // nesting bomb
            format!(r#"{{"v":4096,"fhe":"{a64}","pbs":"{b64}"}}"#).into_bytes(), // v > u8
            format!(r#"{{"fhe":"{a64}","pbs":"{b64}"}}"#).into_bytes(),          // missing v
            br#"{"v":1,"fhe":123,"pbs":[]}"#.to_vec(), // wrong field types
            format!(r#"{{"v":1,"fhe":"{a64}","pbs":"é{b64}"}}"#).into_bytes(), // non-ASCII
        ];
        for hostile in hostiles {
            let verdict = evaluate_host_slots(&[good.clone(), Some(hostile)]);
            assert!(
                matches!(verdict, Some(HostSlotsVerdict::Invalid { .. })),
                "hostile payload must be Invalid, never anchor"
            );
        }
        // divergence_summary must also stay panic-free on the same inputs.
        let _ = divergence_summary(&[good, Some(vec![0xff; 3]), None]);
    }

    #[test]
    fn divergence_summary_is_safe_on_hostile_payloads() {
        // Multibyte chars in a component must not panic the prefix slice.
        let hostile = r#"{"v":1,"fhe":"ééé","pbs":"bb"}"#.as_bytes().to_vec();
        let summary = divergence_summary(&[Some(hostile), None, Some(vec![0xff, 0xfe])]);
        assert!(summary.contains("op0: fhe="));
        assert!(summary.contains("op1: <missing>"));
        assert!(summary.contains("op2: <non-record, 2 bytes>"));
    }
}

#[cfg(test)]
#[path = "state_hash_upload_tests.rs"]
mod upload_tests;
