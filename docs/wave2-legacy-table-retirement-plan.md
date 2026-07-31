# Wave2 Legacy Table Retirement — Plan (Draft)

**Status:** draft for review — no implementation in this branch.

**Problem.** After the wave2 cutover, the legacy tables (`computations`,
`ciphertexts`, `ciphertexts128`, `allowed_handles`, `pbs_computations`,
`ciphertext_digest` in its legacy role) remain load-bearing indefinitely.
RFC 011 allows retaining them "until operators have verified that
pre-cutover work has drained", but two dependents never drain on their own:

1. **Pre-cutover history.** Dependency resolution's pass-3 fallback serves
   every pre-cutover handle from legacy `ciphertexts` (wave1 dual-wrote
   both, but the branch rows are block-keyed while pass 1/2 exclude
   pre-cutover heights — resolution lands on the legacy read). Any
   post-cutover computation or decryption touching an old handle needs it.
2. **Bridged destination copies.** `associate_pair` (tfhe-worker bridge)
   writes the destination handle's ciphertext and digest to the legacy
   tables only. These are created *post*-cutover, forever, so legacy tables
   are not merely a shrinking archive — they keep growing.

A third, softer dependent: assorted tooling reads legacy tables
(`revert_coprocessor_db_state`, drift-revert fallback queries, metrics).

## Target state

All ciphertext state lives in the branch tables. Fork-independent state
(pre-cutover history, ZK inputs, bridged copies, fallback materializations)
lives in the **branchless namespace** (`producer_block_hash = ''`,
`block_number = NULL`) that RFC 011 already defines for exactly this
purpose. Legacy tables are dropped.

## Phase 0 — Observability (prerequisite for every later phase)

- Add counters for every legacy-table read and write path:
  `legacy_ciphertext_read_total` (pass-3 fallback in tfhe-worker,
  sns-worker legacy reads), `legacy_ciphertext_write_total` (bridge
  worker, any residual writer). Label by caller.
- These counters are the drain criterion: each later phase completes when
  its counter is structurally zero (not merely quiet).

## Phase 1 — Stop the bleeding: move the bridge to the branchless namespace

- `associate_pair` writes the destination copy to `ciphertexts_branch`
  (branchless keys) instead of legacy `ciphertexts`; the digest copy moves
  to the branch digest table's branchless shape.
- Dependency resolution already prefers a branchless branch row over the
  legacy fallback (`query_branchless_ciphertext_handles` unions both), so
  readers need no change; old bridged handles remain readable via legacy
  until Phase 2 backfills them.
- Settlement is unaffected: branchless rows are exempt from the write
  guard and outside height-based settlement by design (RFC 011).
- Sequencing note: this is a consensus-relevant write-path change for the
  bridge copy and must be deployed fleet-wide before Phase 2's backfill
  (otherwise the backfill chases a moving target).
- Interaction with RFC 023 Step 2 (open issue #9 from the branch review):
  moving the copy to branch tables does **not** by itself publish a
  handle-keyed S3 object for the destination handle. The canonical
  publication plan must either enqueue the destination handle into the
  canonical repair/publication queue at association time, or Step 2
  readiness for bridged handles breaks. Track jointly with
  `wave2-s3-canonical-publication-plan.md`.

## Phase 2 — Backfill pre-cutover and historical bridged state

- One-shot, idempotent, chunked backfill job:
  `ciphertexts` → `ciphertexts_branch` (branchless), `ciphertexts128` →
  `ciphertexts128_branch` (branchless), legacy `allowed_handles` →
  `allowed_handles_branch` (branchless), skipping handles that already
  have any branch row for the same version.
- Run under the existing branch-cleanup-jobs machinery (quarantine table
  gives retry/poison handling for free).
- Verification query: zero legacy rows without a corresponding branch row.
- This is per-operator local state; no cross-operator coordination is
  required (bytes are copied verbatim, digests unchanged, no re-execution).

## Phase 3 — Retire the read paths

- Flag-gate the pass-3 legacy arm (`has_legacy_ciphertext` fallback and the
  legacy union arms in dependency-metadata and sns queries) behind
  `FHEVM_LEGACY_FALLBACK=off`; default stays `on`.
- Flip the flag per-operator once Phase 2 verification passes and
  `legacy_ciphertext_read_total` has been zero for an agreed soak period.
  Reads are operator-local; this flag does not need fleet-wide
  synchronization for consensus (bytes served are identical either way) —
  but a mixed fleet complicates debugging, so coordinate anyway.
- Migrate tooling: `revert_coprocessor_db_state` and drift-revert queries
  switch to branch tables; `wait_for_error`-style test helpers are already
  migrated.

## Phase 4 — Drop

- After a full release cycle with the fallback off fleet-wide: drop the
  legacy tables in a migration; remove the flag and dead query arms.
- Keep a documented restore path (the Phase 2 backfill is reversible from
  branch → legacy shape if a rollback to a pre-Phase-3 binary is ever
  needed during the soak).

## Risks

| Risk | Mitigation |
|---|---|
| Backfill volume (ciphertexts are large) | Chunked, rate-limited job under cleanup-jobs machinery; per-operator scheduling; no fleet coordination needed. |
| A missed legacy reader discovered after Phase 4 | Phase 0 counters + one full release soak with fallback off before dropping. |
| Bridge write-path change (Phase 1) deployed unevenly | It changes only local storage placement, not bytes or digests — consensus-neutral; still deploy fleet-wide before backfill for operational simplicity. |
| Branchless namespace growth (no settlement pruning) | Same lifecycle as today's legacy tables; revisit retention with the S3 canonical publication plan. |
