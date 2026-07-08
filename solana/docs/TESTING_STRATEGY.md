# Testing Strategy — Solana EncryptedValue + MMR ACL

How the MMR-ACL rewrite and the confidential-token flows are tested, layer by layer, and what is
deliberately deferred. Companion to [`MMR_ACL_MVP.md`](./MMR_ACL_MVP.md) (the model) and
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (the rationale).

## The three authorization paths (the core invariant surface)

Every decrypt authorizes through exactly one of these, and each has dedicated coverage:

| Path | On-chain check | Where tested |
|---|---|---|
| **current** | `handle == current_handle && subject ∈ subjects` (no proof) | `zama-solana-acl` unit (`authorize_current`); `host_mollusk` current-decrypt cases |
| **historical** | MMR proof of `HistoricalAccessLeaf(handle, subject)` vs live peaks | `zama-solana-acl` (`authorize_historical`, `mmr_verify`); `host_mollusk` supersede-then-prove; relayer `solana_proof` reconstruction |
| **public** | MMR proof of `PublicDecryptLeaf(handle)` vs live peaks | `zama-solana-acl` (`authorize_public`); `token_mollusk` burn→redeem and disclose after-supersession |

Negative coverage for each: wrong subject, wrong handle, foreign-lineage proof, invalid/forged proof —
all fail closed (see the `*_rejects_*` mollusk tests).

## Test layers

1. **Shared-crate unit** (`solana/crates/zama-solana-acl`): MMR append/verify (incl. peak-cap
   `append_at_peak_cap_fails_without_mutating`), domain-separated leaf commitments, the three
   `authorize_*` functions, lineage reconstruction, and the `resource_bounds_match_liveness_doc`
   doc-sync guard (keeps `MMR_ACL_MVP.md`'s liveness numbers honest).
2. **On-chain integration — Mollusk** (`solana/runtime-tests/tests/{host,token}_mollusk.rs`): runs the
   **real compiled `.so`** (built `--features poc`) against Mollusk. Covers all three auth paths, the
   full token flows (wrap / transfer / burn→redeem / disclose), and the guards: born-public frame
   guard, consume-once replay markers, expired-request rejection, handle supersession. Because Mollusk
   enforces the 1.4M CU budget, every passing test is also an implicit CU-fits assertion.
3. **Handle-derivation / event-transport** (`zama-host` lib unit): `event_budget` born-public frame
   guard, `should_emit_eval_events_as_cpi` threshold, handle-derivation determinism.
4. **Off-chain reconstruction — host-listener** (`coprocessor/fhevm-engine/host-listener`, features
   `solana-grpc,solana-reconstruct`): reconstructs MMR leaves from instruction data + sysvar-streamed
   block entropy (Yellowstone gRPC), with no dependence on emitted events. Derives supersede/born-public
   handles directly; fails closed on incomplete plans.
5. **Off-chain proof service — relayer** (`relayer/src/solana_proof`): ingest (atomic, gap-free,
   fail-closed), decode (incl. `emit_cpi!` op-event resolution for born-public handles), replay, and
   `build_verified_proof` cross-check against finalized peaks (a wrong record surfaces as
   `PeaksDiverged`/`CorruptCache`, never a bad proof).
6. **ABI / IDL golden** (`scripts/check-zama-host-idl.sh`, `plan_contracts.rs`): vendored IDLs and the
   Borsh golden manifest must match the freshly-built Anchor IDLs; EVENT_VERSION consistency across
   zama-host / confidential-token / host-listener is asserted (a mismatch would silently drop events).
7. **End-to-end** (`.github/workflows/solana-e2e.yml`): matrix `reconstruct: [false, true]`.
   `false` = zama-host emits events, host-listener decodes them. `true` = zama-host EMITLESS, coprocessor
   fed purely by Yellowstone gRPC reconstruction. Both arms exercise the token flows against a local
   validator + coprocessor, proving the reconstruct path is behavior-equivalent to the emit path.

## Reconstruction parity strategy

The rewrite's central correctness bet is that off-chain consumers reproduce on-chain MMR state exactly.
Two independent guards enforce it: the e2e `reconstruct=true` arm (host-listener reconstruction must
produce the same allow-signals as the emit path), and `build_verified_proof`'s peak cross-check (any
reconstruction divergence fails closed rather than yielding a wrong proof, which the KMS would then
re-verify against finalized peaks anyway — DD-035).

## Deliberately deferred (filed as follow-ups, not gaps in the merge)

- **Explicit Mollusk CU-trace assertions.** CU fit is currently implicit (Mollusk enforces the budget,
  so passing = fits) and bounded by the liveness audit's op-count analysis (≤80 SHA-256/supersession,
  leaf-count-independent). An explicit `compute_units_consumed` assertion per hot instruction would turn
  the estimate into a measured, regression-guarded number.
- **Cross-language Rust↔TS vectors.** No TS harness exists yet; a shared fixture set (leaf commitments,
  handle derivation, MMR proofs) checked against a TS reimplementation would guard the SDK/relayer
  language boundary. Tracked as a follow-up.
- **litesvm gate** (zama-ai/fhevm#3045): a lighter-weight in-process runtime alongside Mollusk.
