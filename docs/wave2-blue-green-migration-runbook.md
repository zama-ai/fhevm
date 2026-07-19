# Wave2 Block-Scoped Execution — Blue-Green Migration Runbook

**Scope.** Operator-facing requirements for rolling out the wave2 block-scoped
coprocessor (branch `antoniu/block_context_rerand_wave2`) through the
blue-green upgrade process (`upgrade-controller` / `consensus-detector`,
see `coprocessor/fhevm-engine/upgrade-controller/README.md` on the upgrade
branch). This document does **not** describe the blue-green mechanics
themselves; it lists what must be true *in addition* for this specific
upgrade to be consensus-safe, because wave2 changes execution semantics:
post-cutover ciphertext bytes intentionally differ from legacy bytes for the
same logical values (RFC 019/020).

**References.** RFC 011 (fork tracking / settlement), RFC 019 (block-context
re-randomization), RFC 020 (compress/decompress consensus), RFC 023
(off-chain ciphertext commits, Step 1 shadow mode).

**Terminology.** BCS = live legacy stack. GCS = wave2 stack dry-running in
its schema namespace. `start_block` / `end_block` = the upgrade proposal's
dry-run window on the host chain. Window handles = handles produced in
`[start_block, end_block)`.

---

## R1 — Cutover coordinate: `FHEVM_BRANCH_CUTOVER_BLOCK = start_block`, everywhere

The wave2 semantic boundary and the upgrade proposal are two separate
coordination mechanisms and MUST agree.

- Every operator's GCS must run with
  `FHEVM_BRANCH_CUTOVER_BLOCK == proposal.start_block`.
- The value must be identical across **all three consuming services** of each
  deployment: tfhe-worker, host-listener, sns-worker (one env source of
  truth; do not set per-service values independently).
- `cutover = end_block` is a misconfiguration that silently voids the dry
  run: wave2 skips all below-cutover computations, every window block hashes
  as empty, and unanimity validates nothing.
- Until the upgrade-controller injects this value from the proposal
  automatically, treat it as a manual, checklisted step with cross-operator
  verification **before activation** (compare the literal value, not "it is
  set").

Verification: each operator confirms the GCS env; the activation checklist
records the value next to the proposal's `start_block`.

## R2 — GCS external side effects fenced during the dry run

The schema namespace isolates the database only. S3 and the gateway are
shared with the live BCS.

- The GCS must not send gateway transactions during the dry run
  (`addCiphertextMaterial`, `verifyProofResponse`, allow propagation).
- The GCS must not write to canonical S3 ciphertext paths during the dry
  run. Wave2 uploads to deterministic **handle-keyed** paths
  (`<hex(handle)>/<context_id>`, RFC 023 layout); a dry-run upload would
  overwrite the exact objects the live KMS reads for BCS-attested handles.
  State-hash blobs (`state_hash/chain=…/block=….bin`) are the only intended
  GCS S3 writes.
- Verify the fence empirically in the e2e upgrade simulation: after a
  dry-run window with traffic, `HEAD` a sample of window-handle canonical
  paths and confirm attestation signer/digests still match BCS.

## R3 — Window dual identity: containment rules for `[start_block, end_block)`

During the window, BCS is live: its legacy bytes are attested and reach
`CiphertextCommits` consensus, which cannot be recalled or amended
(`CoprocessorAlreadyAdded`). After the merge, coprocessor databases hold the
GCS (block-scoped) bytes for those same handles. This dual identity is
acceptable and contained if and only if:

1. **Digest-keyed S3 objects from BCS's window uploads are never deleted or
   overwritten.** Decryption of window handles continues to resolve through
   the on-chain materials plus digest-keyed objects. Cleanup jobs must
   exclude them.
2. **Local-vs-on-chain digest cross-checks must not treat window handles as
   drift.** The drift detector's divergence cross-check and any auto-revert
   trigger must be scoped to exclude the window (or be disarmed for the
   window and re-armed after review). An auto-revert firing on
   upgrade-manufactured "drift" is the worst-case failure of this rollout.
3. **RFC 023 shadow-mode discrepancies on window handles are expected.**
   Post-cutover, wave2's canonical repair queue republishes handle-keyed
   objects (GCS bytes + fresh attestations) for settled window blocks whose
   publication was fenced during the dry run. Attestation-based readiness
   will then disagree with on-chain materials for window handles. This noise
   must be annotated and excluded from any Step-2 (contract removal) parity
   assessment; do not advance RFC 023 to Step 2 on data that includes an
   upgrade window.

## R4 — BCS drain barrier before pause

Wave2 permanently skips computations below the cutover block. Any legacy
computation left incomplete when BCS is paused is stranded forever
(undecryptable handle, no automatic recovery).

- Cutover must not pause BCS until its legacy queues are drained for all
  blocks `< start_block`: `computations` incomplete count is zero (excluding
  terminal errors), pbs/sns backlog drained, unsent
  `ciphertext_digest` rows flushed, bridge associations for pre-window
  events completed.
- If the process's readiness check does not already include these, add
  explicit SQL checkpoints to the cutover gate.

## R5 — Identical GCS builds fleet-wide

Cross-operator unanimity during the window catches divergence only on the
code paths the window traffic exercises.

- All operators must run the same GCS build, pinned by **image digest**, not
  tag. Source-built deployments must build from the same commit with the
  same `Cargo.lock` (the tfhe-csprng 0.8.1→0.9.0 incident was caused by
  lockfile drift with byte-identical wrapper code).
- The wave2 branch bumps no FHE crypto dependencies relative to main; any
  future rebase that changes `tfhe`/`tfhe-csprng` pins re-opens this class
  and requires a fresh unanimity window.

---

## Sequencing summary

1. **Pre-activation:** R1 value agreed and verified on every operator/service;
   R5 build digests compared; window-drift exclusions (R3.2) configured;
   drain checkpoints (R4) wired into the cutover gate.
2. **Activation → dry run:** confirm GCS produces non-empty state hashes for
   non-empty window blocks (a stream of `EMPTY_BLOCK_STATE_HASH` under
   traffic means R1 is misconfigured). Spot-check R2 fencing.
3. **Cutover gate:** unanimity reached AND R4 drain checkpoints green.
   Non-unanimity or timeout = abort; do not retry without diagnosing which
   operator diverged and why.
4. **Post-cutover:** watch the canonical repair queue drain (fenced window
   publications re-published); expect and annotate R3.3 shadow-mode noise;
   confirm `settled_height` advances contiguously from `start_block`; confirm
   BCS workers report `StaleStackError` and no legacy writes.
5. **Never** drop the legacy `computations`/`ciphertexts` tables as part of
   this rollout: pre-cutover history and bridged destination copies resolve
   through the legacy fallback indefinitely (see open issues).

## Abort criteria

- Any operator's GCS state hashes diverge during the window (non-unanimity).
- R2 fence violation observed (canonical S3 object or gateway tx from GCS).
- BCS drain checkpoints cannot reach zero before `end_block` — reschedule
  with a later window rather than cutting over undrained.
- Post-cutover: drift-detector or auto-revert activity attributable to
  non-window handles (window-handle noise is expected per R3, anything else
  is not).
