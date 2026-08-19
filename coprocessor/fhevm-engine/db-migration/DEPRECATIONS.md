# Schema deprecations

## wave1 branch-context state — deprecated in v0.15, removed in v0.16

The wave1 branch-context (block-scoped materialization) staging schema is
retired in two releases so that neither release requires ordering between the
db-migration Job and the service Deployment rollout (the migration Job is not
hook-gated and may run concurrently with the rollout):

- **v0.15 (deprecation).** No v0.15 binary references the branch schema
  directly: the listener/worker dual-writes, the `wait_for_branch_schema`
  startup gate and the branch-scoped reorg cleanup are all removed (the
  event-table orphan retraction is retained; it touches legacy tables only).
  The schema itself is left in place, so pre-v0.15 binaries keep working
  against the same database throughout a rolling upgrade, and rolling a
  binary back to v0.14 needs no schema restore. The legacy-table mirror
  triggers also stay in place during v0.15 and keep writing the branch
  mirrors as a DB-side effect of legacy digest/allow writes, so the
  branchless mirror rows a v0.14 binary consults stay consistent across a
  rollback.
- **v0.16 (removal).** A v0.16 migration drops the objects below with
  `IF EXISTS` guards. By then no runnable binary references them in either
  direction, so the migration can run in any order relative to the rollout.

Objects to be dropped in v0.16:

- Tables: `computations_branch`, `pbs_computations_branch`,
  `allowed_handles_branch`, `ciphertext_digest_branch`, `ciphertexts_branch`,
  `ciphertexts128_branch`, `coprocessor_settlement`.
- Triggers (legacy-table mirrors): `mirror_allowed_handles_branchless_trigger`
  on `allowed_handles`; `mirror_ciphertext_digest_branchless_ins`,
  `mirror_ciphertext_digest_branchless_upd`,
  `mirror_ciphertext_digest_branchless_del` on `ciphertext_digest`;
  `mirror_ciphertext_digest_pbs_context_trigger` on
  `pbs_computations_branch`.
- Functions: `mirror_allowed_handles_branchless()`,
  `mirror_ciphertext_digest_branchless()`,
  `mirror_ciphertext_digest_for_pbs_context()`,
  `upsert_ciphertext_digest_branch_from_legacy(ciphertext_digest, BYTEA,
  BIGINT, BYTEA)`.

Known limitations during the v0.15 window:

- The mirror triggers keep copying legacy digest writes into the branch
  tables, so the deprecated tables continue to grow slightly until v0.16.
- `revert_coprocessor_db_state.sql` and the reorg event retraction are
  legacy-only as of v0.15; a drift revert or a reorg during the window
  leaves stale context-scoped mirror rows in the branch tables (wave1's
  branch cleanup used to delete these). They have no readers — v0.14 branch
  reads were never activated — and are erased by the v0.16 drop.
- v0.15 blue-green (GCS) clones omit the branch tables from the clone list.
  A v0.14 binary cannot be pointed at such a clone (its
  `wait_for_branch_schema` startup gate would stall); binary rollback on the
  blue-green path means switching back to the source database.
