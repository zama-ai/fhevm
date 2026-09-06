//! Injection of the deterministic synthetic Gateway input, GCS (green) side only.
//!
//! The derivation and the proving itself live in
//! [`fhevm_engine_common::synthetic_input`]; this module is the database half: decide whether
//! the current block range covers the trigger block, build the blob, and insert the
//! `verify_proofs` row that the zkproof-worker then picks up on the production path.
//!
//! # Why here and not in a traffic generator
//!
//! The Gateway input-verification consensus track anchors on handles derived from a verified
//! proof. With no user traffic during the dry-run window that track never anchors and the
//! upgrade times out. Green synthesizing one input keeps the dry-run self-contained: no
//! external generator, no host-chain transaction.
//!
//! # What is deliberately *not* fenced off
//!
//! Everything after the insert is the production path - the same notification, the same
//! zkproof-worker verification, re-randomization, handle derivation and ciphertext insert.
//! That is the point: a dry-run that skipped those steps would not be testing them.
//!
//! The one place the synthetic row must diverge is the Gateway response: no contract requested
//! this input, so `transaction-sender` must never publish a `verifyProofResponse` for it. That
//! is enforced there by excluding ids at or above
//! [`fhevm_engine_common::synthetic_input::SYNTHETIC_ZK_PROOF_ID_BASE`], not by any flag on
//! this row.

use alloy::primitives::hex;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::synthetic_input::{
    build_synthetic_input, load_input_proving_material, synthetic_aux_data, synthetic_input_seed,
    synthetic_zk_proof_id, SyntheticInputContext, SYNTHETIC_GW_BLOCK_OFFSET,
};
use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy};
use sqlx::{Pool, Postgres, Row};
use tracing::{info, warn};

/// The active GCS upgrade's plan for the synthetic input, as read from `upgrade_state`.
struct SyntheticInputPlan {
    proposal_id: Vec<u8>,
    target_version: String,
    host_chain_id: ChainId,
    acl_contract_address: String,
    gw_start_block: i64,
}

/// Inject the synthetic input once the listener has reached its designated block.
///
/// Returns `true` when a row was inserted. A no-op on BCS (blue), outside the dry-run states,
/// before the trigger block, or when the row already exists.
///
/// The trigger block is a *floor*, not an exact match: the caller swallows errors so the
/// watermark keeps advancing, and a one-shot equality test would let a single transient failure
/// (a DB hiccup while loading the key, say) skip the injection for the whole window with no
/// second chance. Retrying on later ticks costs nothing, because `block_number` recorded on the
/// row stays the deterministic trigger block regardless of when the insert actually lands - so
/// every operator still anchors on the same Gateway block even if they inject at different
/// times. The window's own states bound how late that can be: once the proposal leaves
/// `UpgradeActivated`/`DryRunStarted` the plan query stops returning it.
///
/// Errors are the caller's to log and swallow: a failed injection costs this tick, not the
/// listener's block processing.
pub async fn maybe_inject_synthetic_input(
    db_pool: &Pool<Postgres>,
    gcs_mode: bool,
    to_block: u64,
    notify_channel: &str,
) -> anyhow::Result<bool> {
    if !gcs_mode {
        return Ok(false);
    }

    let Some(plan) = read_synthetic_input_plan(db_pool).await? else {
        return Ok(false);
    };

    let trigger_block = plan
        .gw_start_block
        .saturating_add(SYNTHETIC_GW_BLOCK_OFFSET);
    let Ok(to) = i64::try_from(to_block) else {
        return Ok(false);
    };
    if to < trigger_block {
        return Ok(false);
    }

    let ctx = SyntheticInputContext {
        proposal_id: &plan.proposal_id,
        target_version: &plan.target_version,
        host_chain_id: plan.host_chain_id,
        gw_block_number: trigger_block,
    };
    let zk_proof_id = synthetic_zk_proof_id(&ctx);

    // Proving costs seconds of CPU, so skip it outright when the row is already there - every
    // later tick re-checks this, and re-proving each time would burn a core for nothing.
    //
    // `EXISTS` rather than `SELECT 1`: the latter is `INT4` in Postgres, which does not decode
    // into `i64`.
    let already_present: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM verify_proofs WHERE zk_proof_id = $1)")
            .bind(zk_proof_id)
            .fetch_one(db_pool)
            .await?;
    if already_present {
        return Ok(false);
    }

    let aux_data = synthetic_aux_data(plan.host_chain_id, &plan.acl_contract_address)?;
    let seed = synthetic_input_seed(&ctx);

    // `aux_data` and `seed` are logged in full because they are the two inputs that decide the
    // proof bytes. When operators disagree on the Gateway state hash, comparing these two lines
    // localizes the divergence immediately: same seed but different aux_data means a config
    // mismatch (ACL address or chain selection), different seed means a proposal mismatch.
    // Neither is secret - the plaintext is a public constant, see `synthetic_input`'s docs.
    //
    // Deliberately `hex::encode`, not `to_hex`: the latter truncates under the `compact-hex`
    // feature, which would defeat a byte-for-byte comparison.
    info!(
        zk_proof_id,
        trigger_block,
        host_chain_id = plan.host_chain_id.as_u64(),
        target_version = %plan.target_version,
        acl_contract_address = %plan.acl_contract_address,
        aux_data = %hex::encode(aux_data),
        seed = %hex::encode(seed),
        "GCS: building synthetic Gateway input for consensus anchoring"
    );

    let material = load_input_proving_material(db_pool).await?;
    // Proving is CPU-bound and would otherwise stall the listener's runtime.
    let input =
        tokio::task::spawn_blocking(move || build_synthetic_input(&material, &aux_data, &seed))
            .await??;

    // Same write fence as every other listener write. A retired blue stack never reaches
    // here (`gcs_mode` is false for it), but a green stack rolled back mid-window can, and
    // `Continue` is what keeps raw ingestion going after a rollback.
    let Some(mut tx) = begin_write_guarded(db_pool, true, GcsRollbackPolicy::Continue)
        .await?
        .into_tx()
    else {
        info!("Cutover completed — gw-listener skipping synthetic input insert on retired stack");
        return Ok(false);
    };

    // `extra_data` is empty: it carries per-request calldata a real Gateway event supplies,
    // and nothing downstream requires content for verification.
    //
    // `transaction_id` is left NULL. Unlike the host-chain synthetic ops - where
    // `computations.transaction_id` is the cleanup marker - here the deterministic
    // `zk_proof_id` is the marker, and a fabricated Gateway transaction hash would only
    // invite confusion with a real one.
    let inserted = sqlx::query(
        "WITH ins AS (
            INSERT INTO verify_proofs (
                zk_proof_id, chain_id, contract_address, user_address,
                input, extra_data, block_number
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (zk_proof_id) DO NOTHING
            RETURNING zk_proof_id
        )
        SELECT pg_notify($8, ''), (SELECT COUNT(*) FROM ins) AS inserted",
    )
    .bind(zk_proof_id)
    .bind(plan.host_chain_id.as_i64())
    .bind(fhevm_engine_common::synthetic_input::SYNTHETIC_INPUT_CONTRACT_ADDRESS)
    .bind(fhevm_engine_common::synthetic_input::SYNTHETIC_INPUT_USER_ADDRESS)
    .bind(&input)
    .bind(Vec::<u8>::new())
    .bind(trigger_block)
    .bind(notify_channel)
    .fetch_one(tx.as_mut())
    .await?
    .try_get::<i64, _>("inserted")?;
    tx.commit().await?;

    if inserted == 0 {
        // Lost a race with another pass over the same range; the existing row is identical.
        return Ok(false);
    }

    info!(
        zk_proof_id,
        trigger_block,
        input_len = input.len(),
        "GCS: inserted synthetic Gateway input"
    );
    Ok(true)
}

/// Read the active GCS upgrade's synthetic-input plan.
///
/// The host chain is chosen as the lowest `host_chain_id` in the proposal that is also a
/// configured host chain - deterministic across operators, and the join is what guarantees the
/// zkproof-worker will accept the row (it filters on its host-chains cache) and that an ACL
/// address exists for the aux data.
async fn read_synthetic_input_plan(
    db_pool: &Pool<Postgres>,
) -> anyhow::Result<Option<SyntheticInputPlan>> {
    let row = sqlx::query(
        "SELECT u.proposal_id, u.version, u.host_chain_id, u.gw_start_block,
                h.acl_contract_address
           FROM upgrade_state u
           JOIN host_chains h ON h.chain_id = u.host_chain_id
          WHERE u.stack_role = 'GCS'
            AND u.status = 'in_progress'
            AND u.state IN ('UpgradeActivated', 'DryRunStarted')
            AND u.proposal_id IS NOT NULL
            AND u.version IS NOT NULL
            AND u.gw_start_block IS NOT NULL
          ORDER BY u.host_chain_id
          LIMIT 1",
    )
    .fetch_optional(db_pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let host_chain_id_raw: i64 = row.try_get("host_chain_id")?;
    let host_chain_id = match ChainId::try_from(host_chain_id_raw) {
        Ok(chain_id) => chain_id,
        Err(err) => {
            warn!(
                host_chain_id = host_chain_id_raw,
                error = %err,
                "Skipping synthetic input: upgrade_state carries an invalid host chain id"
            );
            return Ok(None);
        }
    };

    Ok(Some(SyntheticInputPlan {
        proposal_id: row.try_get("proposal_id")?,
        target_version: row.try_get("version")?,
        host_chain_id,
        acl_contract_address: row.try_get("acl_contract_address")?,
        gw_start_block: row.try_get("gw_start_block")?,
    }))
}
