# Testing Strategy — Solana EncryptedValue + MMR ACL

How the MMR-ACL rewrite and the confidential-token flows are tested, layer by layer, and what is
deliberately deferred. Companion to [`MMR_ACL_MVP.md`](./MMR_ACL_MVP.md) (the model) and
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (the rationale).

## The three authorization paths (the core invariant surface)

Every decrypt authorizes through exactly one of these, and each has dedicated coverage:

| Path | On-chain check | Where tested |
|---|---|---|
| **current** | `handle == current_handle && subject ∈ subjects` (no proof) | `zama-solana-acl` unit (`authorize_current`); `host_mollusk` current-decrypt cases |
| **historical** | MMR proof of `HistoricalAccessLeaf(handle, subject)` vs live peaks | `zama-solana-acl` (`authorize_historical`, `mmr_verify`); `host_mollusk` supersede-then-prove; `solana-proof-service` reconstruction |
| **public** | MMR proof of `PublicDecryptLeaf(handle)` vs live peaks | `zama-solana-acl` (`authorize_public`); `token_mollusk` burn→redeem and `disclose_secp` after-supersession; `host_mollusk` `verify_public_decrypt` negatives (DD-040) |

Negative coverage for each: wrong subject, wrong handle, foreign-lineage proof, invalid/forged proof —
all fail closed (see the `*_rejects_*` mollusk tests).

## Test layers

1. **Shared-crate unit** (`solana/crates/zama-solana-acl`): MMR append/verify (incl. peak-cap
   `append_at_peak_cap_fails_without_mutating`), domain-separated leaf commitments, the three
   `authorize_*` functions, lineage reconstruction, and the `resource_bounds_match_liveness_doc`
   doc-sync guard (keeps `MMR_ACL_MVP.md`'s liveness numbers honest).
2. **On-chain integration — Mollusk** (`solana/runtime-tests/tests/{host,token}_mollusk.rs`): runs the
   **real compiled `.so`** (built `--features poc`) against Mollusk. Covers all three auth paths, the
   full token flows (wrap / transfer / burn→redeem / disclose), the produced-public lifecycle batch
   (zero/one/multiple/max-size/fail-closed), the burn-redemption consume-once replay marker and
   expired-request rejection, and handle supersession. Token disclosure is now the thin `disclose_secp`
   consumer of the host `verify_public_decrypt` verifier (DD-040); `token_mollusk` covers its happy path
   (amount + balance), after-supersession consume, idempotency / no-replay-marker, foreign-proof
   rejection surfaced from the host, and mint-domain binding. The verifier's own negatives (destroyed
   context, sub-threshold cert, handle/proof mismatch, non-canonical context, survives-supersede) live
   in `host_mollusk` (#3220). There are no more `request_disclose_*` / `disclose_*_secp` witness-bound
   disclosure tests. The token and batcher Mollusk suites explicitly set `compute_unit_limit = 1_400_000`;
   the host suite uses Mollusk's default per-instruction budget (stricter than 1.4M). In all cases,
   every passing test is an implicit CU-fits assertion at the configured budget.
3. **Handle-derivation / lifecycle transport** (`zama-host` lib unit): the maximum 16-record batch's
    exact 1,077-byte CPI envelope and signer/readonly event-authority metadata, plus handle-derivation
    determinism.
4. **Off-chain reconstruction — host-listener** (`coprocessor/fhevm-engine/host-listener`, feature
    `solana-grpc`, which includes reconstruction): reconstructs MMR leaves from instruction data +
    sysvar-streamed block entropy (Yellowstone gRPC), with no dependence on emitted events. Derives
    supersede/produced-public handles directly; fails closed on incomplete plans.
5. **Off-chain proof service — solana-proof-service** (`solana-proof-service/`): ingest (atomic, gap-free,
   fail-closed), decode (incl. `emit_cpi!` op-event resolution for born-public handles), replay, and
   `build_verified_proof` cross-check against confirmed peaks (a wrong record surfaces as
   `PeaksDiverged`/`CorruptCache`, never a bad proof).
6. **ABI / IDL golden** (`scripts/check-zama-host-idl.sh`, `plan_contracts.rs`): vendored IDLs and the
   Borsh golden manifest must match the freshly-built Anchor IDLs; EVENT_VERSION consistency across
   zama-host / confidential-token / host-listener is asserted (a mismatch would silently drop events).
7. **End-to-end** (`.github/workflows/solana-e2e.yml`, `full-vertical.sh`): the Yellowstone-only
   path feeds ordinary computation facts through Yellowstone gRPC reconstruction while retaining only
   the narrow produced-public lifecycle batch required by solana-proof-service
   reconstruction. It drives the **decrypt vertical** against a local validator + full coprocessor/KMS
   stack — compute → public-decrypt (solana-proof-service MMR proof) → user-decrypt (current) → historical-user-decrypt
   (superseded handle + live MMR proof) — exercising all three authorization paths. Operator execution
   is intentionally representative rather than exhaustive: the live vertical retains one example for
   encrypted/encrypted and encrypted/scalar binary wiring, unary type conversion, ternary selection,
   bounded randomness, and each distinct composite encoding (`Sum`, `IsIn`, `MulDiv`). Exhaustive
   operator contract belongs to pure conformance; Mollusk and direct real-TFHE add representative SBF
   and cryptographic evidence. The live vertical also retains token composition through wrap → burn →
   public release → redeem (witness-bound KMS certificate) and `disclose_secp` (stateless host `verify_public_decrypt`, DD-040). `token_mollusk` owns the
   broader negative matrix (including after-supersession, redeem consume-once, disclosure idempotency, and foreign-proof rejection).

## Reconstruction parity strategy

The rewrite's central correctness bet is that off-chain consumers reproduce on-chain MMR state exactly.
The e2e `reconstruct=true` arm exercises host-listener reconstruction against the full stack, while
`build_verified_proof` cross-checks reconstructed peaks against final chain state. A divergence fails
closed rather than yielding a wrong proof, which the KMS then re-verifies against confirmed peaks anyway
(DD-035).

## Confirmed-view operations

An equal-leaf-count proof mismatch is retried as
`classification=confirmed_equal_count`; a proof ahead of the KMS view is retried as
`classification=confirmed_proof_ahead`. Both use the ordinary configured decryption budget
(`max_decryption_attempts`, default 20) and fast event polling interval (default 3 seconds). Actual
wall-clock exhaustion also depends on batch load and processing time; there is no separate hidden
fork-retry loop. Deterministic mismatches remain fail-closed, but the classification distinguishes
them from ordinary proof-ahead catch-up in logs.

A persistent proof-service `corrupt_cache` response means the PostgreSQL store snapshot disagrees with
the confirmed on-chain peaks after Yellowstone ingest / bounded RPC recovery. Recovery is operational,
not an authorization fallback: stop `solana-proof-service`, reset its Postgres volume / DB, and restart
it so Yellowstone (plus configured `recovery.bootstrap_slot` RPC recovery when needed) rebuilds the
durable store. Proof HTTP remains read-only against that store and fails closed (`lagging` /
`corrupt_cache` / readiness gates) until history is complete; the KMS never trusts the store without
re-verifying the proof against live chain state.

## Deliberately deferred (filed as follow-ups, not gaps in the merge)

- **Explicit Mollusk CU-trace assertions.** CU fit is currently implicit (Mollusk enforces the budget,
  so passing = fits) and bounded by the liveness audit's op-count analysis (≤80 SHA-256/supersession,
  leaf-count-independent). An explicit `compute_units_consumed` assertion per hot instruction would turn
  the estimate into a measured, regression-guarded number.
- **Cross-language Rust↔TS vectors.** No TS harness exists yet; a shared fixture set (leaf commitments,
  handle derivation, MMR proofs) checked against a TS reimplementation would guard the SDK/relayer
  language boundary. Tracked as a follow-up.
- **litesvm gate** (zama-ai/fhevm#3045): a lighter-weight in-process runtime alongside Mollusk.
