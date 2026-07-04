# Runbook — Wave 1 block-scope materialization rollout

This runbook covers applying the **wave 1** (expand-phase) coprocessor migration to a
**large, live** production database via a rolling `helm upgrade`.

Wave 1 is additive and consensus-neutral: it creates the `*_branch` sibling tables and
installs dual-write mirror triggers, while every reader stays on the legacy tables. It
does **not** change any executed ciphertext bytes. The only apply-time hazards are
(a) a blocking index build on the large `host_chain_blocks_valid` table, and (b) the
migration Job racing the service Deployments. This runbook closes both.

## TL;DR

1. **Before** `helm upgrade`, run the init image once with
   `RUN_BLOCK_SCOPE_WAVE1_PREREQUISITES=true` to build the ancestry index
   `CONCURRENTLY` against the live DB.
2. **Mandatory:** run the full db-migration image/job to completion before
   rolling any wave 1 service Deployment. The in-migration index build no-ops
   because step 1 already built it.
3. Run `helm upgrade` only after the migration has completed.
4. Verify the index is `valid`, the migration completed, and watch DB
   write load.

> The host-listener also applies an **in-code schema guard**
> (`Database::wait_for_branch_schema`): if a binary starts before the migration
> has applied, it waits (bounded) for the branch schema instead of crash-looping
> on `*_branch` / `parent_hash` writes. This is a backstop, not the rollout
> plan: operators must still complete the migration before rolling wave 1 pods.

## Why the pre-step is required

`migrations/20260610130000_branch_context_parent_hash.sql` builds
`idx_host_chain_blocks_valid_parent_hash` on `host_chain_blocks_valid`, which grows ~1
row per observed block per chain (millions of rows on a long-running chain). A plain
`CREATE INDEX` takes a `SHARE` lock that blocks block ingestion for the entire build.

`initialize_db.sh` therefore exposes `run_block_scope_materialization_wave1_prerequisites()`
(gated by `RUN_BLOCK_SCOPE_WAVE1_PREREQUISITES=true`), which:

- adds the metadata-only `parent_hash` column (idempotent), then
- builds the index with `CREATE INDEX CONCURRENTLY` (non-blocking), using the existing
  `precreate_index` state guard so it is safe to re-run.

The in-migration `CREATE INDEX IF NOT EXISTS` then finds the index present and no-ops.

> Check the table size first: `SELECT chain_id, count(*) FROM host_chain_blocks_valid GROUP BY 1;`
> If it is small (≲ a few hundred K rows) the pre-step is optional — the inline build is a blip.

## Step-by-step

### 0. Pre-flight
- Confirm `DATABASE_URL` points at the target DB and the operator has DDL rights.
- Capture the row count above to decide whether the pre-step is strictly needed.

### 1. Concurrent pre-create (live DB, existing services still running)
Run the db-migration image (or `initialize_db.sh` directly) with:

```
RUN_BLOCK_SCOPE_WAVE1_PREREQUISITES=true
```

This builds `idx_host_chain_blocks_valid_parent_hash` CONCURRENTLY. It does **not** run
the remaining migrations and does **not** seed `host_chains`. It is idempotent: re-running
skips an already-`valid` index and fails loudly on a left-over `invalid` index (drop it and
re-run, per the `precreate_index` message).

### 2. Mandatory migration gate before workload rollout
Before any wave 1 service Deployment rolls, run the full db-migration image/job to completion
against the target DB. This is the required migration-vs-binary ordering gate: a wave 1 binary
can issue `*_branch` / `parent_hash` queries, so the branch schema must exist before those pods
start.

Do not rely on the chart's normal db-migration Job object for this ordering: with Helm hooks
disabled by default, Helm may create the Job and service Deployments in the same upgrade. The
normal chart Job is still useful and idempotent, but it is not a rollout gate by itself.

Infra may opt into a `pre-upgrade` hook by uncommenting `dbMigration.annotations` in
`charts/coprocessor/values.yaml`. If a hook is enabled, hook ordering is by
**`helm.sh/hook-weight` only** (Helm has no `hook-needs`). Every mounted or `valueFrom`
dependency of the migration Job (DB URL, IAM auth inputs, Secrets, ConfigMaps, including the
chart-managed RDS CA ConfigMap) must either already exist before the upgrade or be rendered as
an earlier hook with a **lower** weight. Pre-install hooks have the same restriction and are not
safe with chart-created dependencies on fresh installs.

The trigger-attach migrations (`20260610130300`, `20260610145100`) set
`SET LOCAL lock_timeout = '3s'`, so a contended `CREATE TRIGGER` on the hot `allowed_handles`
/ `ciphertext_digest` tables fails fast and is retried (Job `backoffLimit: 3`) instead of
convoying every query behind it. If the migration fails on lock contention, re-running the Job
is safe — all migrations are idempotent.

### 3. helm upgrade
Run the normal `helm upgrade` only after step 2 has completed successfully. At that point the
chart-rendered db-migration Job may run again during the upgrade; this is expected and should
no-op or apply only already-idempotent migrations.

### 4. Verify
- Index is valid:
  ```sql
  SELECT indisvalid FROM pg_index i JOIN pg_class c ON c.oid = i.indexrelid
  WHERE c.relname = 'idx_host_chain_blocks_valid_parent_hash';   -- expect t
  ```
- Migration Job completed (`kubectl get jobs -l app=coprocessor-db-migration`).
- Branch tables exist and are being dual-written (`computations_branch`, `pbs_computations_branch`,
  `allowed_handles_branch`, `ciphertext_digest_branch`, `ciphertexts_branch`, `ciphertexts128_branch`).
- DB load: the digest mirror trigger fans a legacy `ciphertext_digest` write across branch
  contexts. The `ciphertext_digest` writers (sns-worker, transaction-sender) now run with a
  10s `statement_timeout`, so a pathological reorg fan-out is bounded (cancelled statement →
  retry) rather than holding locks unbounded. Watch for elevated statement cancellations /
  deadlock retries (`40P01`) during reorg-heavy periods; sustained churn means the fan-out
  needs further attention, not a correctness failure.

## Reorg / cleanup behavior (wave 1)

Orphan cleanup on a reorg removes the dropped fork's **branch** rows, and — only for
handles that existed *solely* on the orphaned fork (no surviving branch context, enforced
by a `NOT EXISTS` guard) — the corresponding legacy `computations` / ACL / PBS / digest
rows. Those are genuinely dead fork state, safe to remove.

Legacy ciphertext **bytes** (`ciphertexts` / `ciphertexts128`) are intentionally **not**
deleted by orphan cleanup. As a result an orphan-only handle leaves its (now unreferenced)
ciphertext bytes behind — a small, bounded storage residue, accepted in wave 1 to keep the
authoritative pre-cutover byte store intact for the wave-2 legacy fallback.

## Branch activation height (`FHEVM_BRANCH_ACTIVATION_BLOCK`)

Without it, every node starts dual-writing branch rows the moment its binary upgrades.
During a rolling upgrade that start time is node-local: handles produced in the window
get branch rows on some operators and not others (and with different producer keying if
a reorg lands mid-window), and the reorg cleanup's `NOT EXISTS` guards then make
different legacy-deletion decisions per operator — a fleet-divergence hazard on live
read paths.

Set `FHEVM_BRANCH_ACTIVATION_BLOCK` (host-listener env) to a **fleet-common host-chain
height comfortably above the expected completion of the rolling upgrade** (all
operators must use the same value). Below it, ingestion writes legacy state only and
producers resolve as branchless, so branch-row keying is identical on every node by
construction, regardless of upgrade timing. The default `0` (active from genesis) is
for fresh chains and single-operator test stacks.

Wave-2 interaction: the wave-2 cutover height (`FHEVM_BRANCH_CUTOVER_BLOCK`) must be
`>=` the activation height — blocks below activation have no branch rows to execute
from, and wave-2's legacy fallback covers them.

New-feature state (confidential-bridge event tables, fallback-grant observations) is
not gated: those tables are keyed by observation block hash and are deterministic
across operators regardless of upgrade timing.

## Rollback

- Reverting the binaries to the prior release is safe: the dual-write mirror triggers keep
  firing into branch tables that nobody reads, with no harm.
- Migrations are forward-only (no `.down.sql`). To tear down wave-1 schema, drop in order:
  legacy-table triggers → branch-table triggers → mirror functions → `*_branch` tables →
  `host_chain_blocks_valid.parent_hash` (column + index) → `coprocessor_settlement`.
- The data-state revert (`db-scripts/revert_coprocessor_db_state.sql`) is safe to run while a
  prior-release binary runs.
