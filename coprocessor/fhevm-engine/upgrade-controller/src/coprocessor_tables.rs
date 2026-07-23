//! The single source of truth for how every coprocessor `public` table is
//! treated during a blue-green (BCS/GCS) upgrade.
//!
//! **Every** table in the coprocessor database must appear in
//! [`COPROCESSOR_TABLES`] exactly once. The
//! [`tests::coprocessor_tables_cover_public_schema`] test enforces this against
//! a freshly-migrated database, so a migration that adds (or removes) a table
//! fails the build until an explicit decision is recorded here. That is the
//! whole point of this file: nobody should be able to add a table without
//! answering whether, during the GCS dry-run, it must be
//!
//!   1. **ignored** — no `gcs.*` copy; if a GCS-mode service writes it, the
//!      write lands in `public` (correct for shared config / control-plane
//!      state that both stacks must agree on);
//!   2. **duplicated-for-isolation** — a `gcs.*` copy keeps the dry-run stack's
//!      writes out of `public`, and is dropped at cutover because `public`
//!      already holds the canonical rows (deterministically re-derived by the
//!      always-live blue stack); or
//!   3. **duplicated-and-merged** — the `gcs.*` copy is merged back into
//!      `public` at cutover, GCS winning on the `ON CONFLICT` key.
//!
//! Encoded with two fields:
//!
//! | `duplicated` | `conflict_cols` | treatment                  |
//! |--------------|-----------------|----------------------------|
//! | `false`      | `&[]`           | ignored                    |
//! | `true`       | `&[]`           | duplicate-for-isolation    |
//! | `true`       | `&[cols…]`      | duplicate-and-merge        |
//!
//! `conflict_cols` is only meaningful when `duplicated` is `true`; it must name
//! an existing unique/PK constraint on `public.<name>` (the merge INSERT is
//! planned against that constraint — see `merge_gcs_table`). A non-empty
//! `conflict_cols` with `duplicated = false` is a contradiction rejected by
//! [`tests::coprocessor_tables_are_consistent`].
//!
//! Why `search_path` makes this matter: GCS-mode services connect with
//! `search_path = "gcs-<ver>",public`, so an *unqualified* write resolves to
//! `gcs.<table>` only if that table exists — otherwise it silently falls
//! through to `public.<table>`. So "ignored" is a real, load-bearing choice for
//! any table a GCS-mode service writes, not a no-op.

/// One coprocessor `public` table and its blue-green upgrade treatment.
///
/// To add a table to the dry-run, set `duplicated: true` and either leave
/// `conflict_cols` empty (isolation-only) or set it to the table's PK / unique
/// key (merge at cutover).
#[derive(Debug, Clone, Copy)]
pub struct CoprocessorTable {
    /// The `public.<name>` table.
    pub name: &'static str,
    /// Whether a `gcs.<name>` duplicate is created at upgrade-controller
    /// startup (`CREATE TABLE gcs.<name> (LIKE public.<name> INCLUDING ALL)`).
    pub duplicated: bool,
    /// `ON CONFLICT` target used to merge `gcs.<name>` back into `public.<name>`
    /// at cutover. Empty means the duplicate is dropped, not merged. Must name a
    /// unique/PK constraint on `public.<name>`.
    pub conflict_cols: &'static [&'static str],
}

impl CoprocessorTable {
    /// `true` if the `gcs.<name>` duplicate is merged back into `public` at
    /// cutover (as opposed to duplicated purely for write-isolation).
    pub const fn is_merged(&self) -> bool {
        self.duplicated && !self.conflict_cols.is_empty()
    }
}

/// Every coprocessor `public` table, with its blue-green upgrade treatment.
///
/// Kept in rough migration/topical order. Each `*_branch` table is the
/// block-context-aware form that becomes the canonical table after v0.15 (see
/// `20260610130100_branch_context_tables`); it is treated exactly like its
/// legacy sibling, except its `conflict_cols` are the branch table's own PK,
/// which extends the legacy key with `producer_block_hash` (and `block_hash`
/// for some) so competing fork branches coexist.
pub const COPROCESSOR_TABLES: &[CoprocessorTable] = &[
    // ---------------------------------------------------------------------
    // Ciphertext / computation data — the GCS stack writes these during the
    // dry-run and they merge back at cutover (GCS wins on collisions).
    // ---------------------------------------------------------------------
    CoprocessorTable {
        name: "ciphertexts",
        duplicated: true,
        conflict_cols: &["handle", "ciphertext_version"],
    },
    CoprocessorTable {
        name: "ciphertexts_branch",
        duplicated: true,
        conflict_cols: &["handle", "ciphertext_version", "producer_block_hash"],
    },
    CoprocessorTable {
        name: "ciphertexts128",
        duplicated: true,
        conflict_cols: &["tenant_id", "handle"],
    },
    CoprocessorTable {
        name: "ciphertexts128_branch",
        duplicated: true,
        conflict_cols: &["handle", "producer_block_hash"],
    },
    // Green-wins overwrite propagates GCS's NULL digests into public, re-arming
    // the sns-worker resubmit loop to backfill S3 after cutover.
    CoprocessorTable {
        name: "ciphertext_digest",
        duplicated: true,
        conflict_cols: &["handle"],
    },
    CoprocessorTable {
        name: "ciphertext_digest_branch",
        duplicated: true,
        conflict_cols: &["handle", "producer_block_hash", "block_hash"],
    },
    CoprocessorTable {
        name: "computations",
        duplicated: true,
        conflict_cols: &["output_handle", "transaction_id"],
    },
    CoprocessorTable {
        name: "computations_branch",
        duplicated: true,
        conflict_cols: &["output_handle", "transaction_id", "producer_block_hash"],
    },
    CoprocessorTable {
        name: "pbs_computations",
        duplicated: true,
        conflict_cols: &["tenant_id", "handle"],
    },
    CoprocessorTable {
        name: "pbs_computations_branch",
        duplicated: true,
        conflict_cols: &["handle", "producer_block_hash", "block_hash"],
    },
    CoprocessorTable {
        name: "verify_proofs",
        duplicated: true,
        conflict_cols: &["zk_proof_id"],
    },
    CoprocessorTable {
        name: "dependence_chain",
        duplicated: true,
        conflict_cols: &["dependence_chain_id"],
    },
    // ---------------------------------------------------------------------
    // Bookkeeping tables merged at cutover.
    // ---------------------------------------------------------------------
    // Merged so the GCS stack's consensus state-hashes survive cutover.
    CoprocessorTable {
        name: "state_hash",
        duplicated: true,
        conflict_cols: &["chain_id", "block_number"],
    },
    // Merged so input-ciphertext handles produced during the dry-run survive.
    CoprocessorTable {
        name: "input_handles",
        duplicated: true,
        conflict_cols: &["handle"],
    },
    // Merged so blocks the GCS stack validated during the dry-run survive.
    CoprocessorTable {
        name: "host_chain_blocks_valid",
        duplicated: true,
        conflict_cols: &["chain_id", "block_hash"],
    },
    // Singleton row (dummy_id PK); merged so the GCS gw-listener's last-seen
    // block advances public forward at cutover (GCS wins the collision).
    CoprocessorTable {
        name: "gw_listener_last_block",
        duplicated: true,
        conflict_cols: &["dummy_id"],
    },
    // ---------------------------------------------------------------------
    // Duplicated purely for write-isolation, NOT merged: the always-live blue
    // stack re-derives the canonical rows into public deterministically, but
    // without a gcs.* copy the GCS search_path would let the dry-run stack's
    // writes fall through into public before cutover.
    // ---------------------------------------------------------------------
    CoprocessorTable {
        name: "transactions",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "allowed_handles",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "allowed_handles_branch",
        duplicated: true,
        conflict_cols: &[],
    },
    // Consensus construction, downloaded peer evidence, verification queues,
    // and the mutable remediation inventory must be isolated from the live
    // blue stack during a green dry-run. The live stack deterministically
    // rebuilds its own state, so these copies are not merged at cutover.
    CoprocessorTable {
        name: "block_consensus",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_range",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_manifest",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_verification_target",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_peer_download",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_verification_attempt",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_verification_scope",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_verification_scope_member",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "block_consensus_drift_handle",
        duplicated: true,
        conflict_cols: &[],
    },
    // Written by the GCS host-listener kms_generation module during the dry-run.
    CoprocessorTable {
        name: "kms_key_activation_events",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "kms_crs_activation_events",
        duplicated: true,
        conflict_cols: &[],
    },
    // Durable observations of on-chain events, written by the GCS host-listener
    // (tfhe_event_propagate); handle_bridged_events also by the GCS tfhe-worker.
    CoprocessorTable {
        name: "fallback_granted_events",
        duplicated: true,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "handle_bridged_events",
        duplicated: true,
        conflict_cols: &[],
    },
    // ---------------------------------------------------------------------
    // Ignored: no gcs.* duplicate. Shared configuration, key material, and
    // control-plane / listener-progress state that both stacks must agree on
    // (or that only the always-live blue stack owns). This preserves the
    // pre-refactor behaviour — none of these were ever duplicated.
    //
    // NOTE: several of these ARE written by GCS-mode services (host-listener /
    // tfhe-worker), so "ignored" means those writes land in public during the
    // dry-run. That is intended for shared config (tenants/host_chains/keys/crs)
    // and control-plane rows (upgrade_state/versioning), but the event-derived
    // and poller-progress tables below deserve an isolation review — see the
    // per-table notes.
    // ---------------------------------------------------------------------
    // Control plane — the blue-green FSM's own coordination state; MUST stay
    // shared and single-copy across both stacks.
    CoprocessorTable {
        name: "upgrade_state",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "versioning",
        duplicated: false,
        conflict_cols: &[],
    },
    // Shared configuration / key material — both stacks must read the same
    // rows, so isolating a green copy would be wrong.
    CoprocessorTable {
        name: "tenants",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "host_chains",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "keys",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "crs",
        duplicated: false,
        conflict_cols: &[],
    },
    // Current signer and bucket registry snapshot. Both stacks must authorize
    // manifests against the same observed GatewayConfig state.
    CoprocessorTable {
        name: "gateway_config_coprocessors",
        duplicated: false,
        conflict_cols: &[],
    },
    // Listener-progress bookkeeping. Analogous to gw_listener_last_block (which
    // IS duplicated); left un-duplicated to preserve current behaviour, but a
    // candidate for isolation review (a green write could rewind blue's cursor).
    CoprocessorTable {
        name: "host_listener_poller_state",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "host_chain_consumer_blocks",
        duplicated: false,
        conflict_cols: &[],
    },
    // Event-derived tables written by GCS-mode services. bridge_handle_events is
    // the sibling of handle_bridged_events (which IS duplicated-for-isolation);
    // these are un-duplicated only to preserve current behaviour and are prime
    // candidates for the same isolation treatment.
    CoprocessorTable {
        name: "bridge_handle_events",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "delegate_user_decrypt",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "drift_revert_signal",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "input_blobs",
        duplicated: false,
        conflict_cols: &[],
    },
    CoprocessorTable {
        name: "coprocessor_settlement",
        duplicated: false,
        conflict_cols: &[],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The two fields must not contradict each other: `conflict_cols` is only
    /// meaningful for a duplicated table. A non-empty value with
    /// `duplicated = false` would silently never merge — reject it here.
    #[test]
    fn coprocessor_tables_are_consistent() {
        use std::collections::HashSet;

        let mut seen = HashSet::new();
        for t in COPROCESSOR_TABLES {
            assert!(
                seen.insert(t.name),
                "duplicate entry for `{}` in COPROCESSOR_TABLES",
                t.name
            );
            if !t.duplicated {
                assert!(
                    t.conflict_cols.is_empty(),
                    "`{}` is not duplicated but has conflict_cols {:?} — a \
                     non-duplicated table can never be merged",
                    t.name,
                    t.conflict_cols
                );
            }
        }
    }

    /// Every `public` base table in a freshly-migrated coprocessor database
    /// must have exactly one entry in [`COPROCESSOR_TABLES`]. This is the guard
    /// that forces a deliberate ignore / duplicate-for-isolation /
    /// duplicate-and-merge decision whenever a migration adds a table.
    ///
    /// Failure prints the exact drift so the fix is obvious:
    ///   - "missing" = in the DB but not classified here → add an entry.
    ///   - "stale"   = classified here but not in the DB → remove/rename entry.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn coprocessor_tables_cover_public_schema() {
        use sqlx::postgres::PgPoolOptions;
        use std::collections::BTreeSet;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::WithKeysNoSns)
            .await
            .expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(instance.db_url())
            .await
            .expect("pool");

        // Base tables in `public`, excluding sqlx's own migration ledger.
        let db_tables: BTreeSet<String> = sqlx::query_scalar::<_, String>(
            "SELECT table_name
               FROM information_schema.tables
              WHERE table_schema = 'public'
                AND table_type = 'BASE TABLE'
                AND table_name <> '_sqlx_migrations'",
        )
        .fetch_all(&pool)
        .await
        .expect("list public tables")
        .into_iter()
        .collect();

        let classified: BTreeSet<String> = COPROCESSOR_TABLES
            .iter()
            .map(|t| t.name.to_string())
            .collect();

        let missing: Vec<&String> = db_tables.difference(&classified).collect();
        let stale: Vec<&String> = classified.difference(&db_tables).collect();

        assert!(
            missing.is_empty() && stale.is_empty(),
            "COPROCESSOR_TABLES is out of sync with the coprocessor schema.\n\
             Tables in the DB but NOT classified (add an entry — ignore, \
             duplicate-for-isolation, or duplicate-and-merge): {missing:?}\n\
             Tables classified but NOT in the DB (remove or rename the entry): \
             {stale:?}"
        );
    }
}
