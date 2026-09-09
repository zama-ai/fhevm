# Testing Strategy — Solana EncryptedValue + MMR ACL

How the MMR-ACL rewrite and the confidential-token flows are tested, layer by layer, and what is
deliberately deferred. Companion to [`MMR_ACL_MVP.md`](./MMR_ACL_MVP.md) (the model) and
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (the rationale).

## The two leaves (the core invariant surface)

Every decrypt authorizes through exactly one leaf proof against the account's confirmed peaks, and
each has dedicated coverage:

| Leaf | Check | Where tested |
|---|---|---|
| **allow** (`HistoricalAccessLeaf(handle, key)`) | MMR proof vs live peaks; the current handle and a replaced one alike | `zama-solana-acl` unit (`authorize_historical`, `mmr_verify`); `host_mollusk` write-then-prove; `kms-worker` `solana_` (`handle_binding`, `proof`); host-listener `solana_leaves_tests` |
| **public** (`PublicDecryptLeaf(handle)`) | MMR proof vs live peaks, exact handle | `zama-solana-acl` (`authorize_public`); `token_mollusk` burn→redeem and `disclose_secp` after-update; `host_mollusk` `verify_public_decrypt` negatives (DD-040) |

Negative coverage for each: wrong key, wrong handle, a proof from a foreign encrypted value account,
invalid/forged proof, a record that is behind — all fail closed (see the `*_rejects_*` mollusk tests
and the connector's `ProofRecordBehind` / `NoLeaf` classification).

## Test layers

1. **Shared-crate unit** (`solana/crates/zama-solana-acl`): MMR append/verify (incl. peak-cap
   `append_at_peak_cap_fails_without_mutating`), prefix-separated leaf commitments, the two
   `authorize_*` functions, encrypted value account reconstruction, the seed list, and the
   `resource_bounds_match_liveness_doc` doc-sync guard (keeps `MMR_ACL_MVP.md`'s liveness numbers
   honest).
2. **On-chain integration — Mollusk** (`solana/runtime-tests/tests/{host,token}_mollusk.rs`): runs the
   **real compiled `.so`** against Mollusk. Covers both leaf kinds, program verification of the
   output authority (`EncryptedValueAuthorityNotProgramPda`), the deny list by application, the rand
   nonce, the full token flows (wrap / transfer / burn→redeem / disclose), the produced-public
   lifecycle event (zero/one/multiple/max-size), the sequential pending-burn redeem/cancel act-once
   lifecycle, and handle update. Token disclosure is the thin `disclose_secp` consumer of the host
   `verify_public_decrypt` verifier (DD-040); `token_mollusk` covers its happy path (amount +
   balance), after-update consume, idempotency / no-replay-marker, foreign-proof rejection surfaced
   from the host, and scope binding. The verifier's own negatives (destroyed context, sub-threshold
   cert, handle/proof mismatch, non-canonical context, survives-update) live in `host_mollusk`
   (#3220). The token and batcher Mollusk suites explicitly set `compute_unit_limit = 1_400_000`;
   the host suite uses Mollusk's default per-instruction budget (stricter than 1.4M). In all cases,
   every passing test is an implicit CU-fits assertion at the configured budget. The specimen
   suites (`counter_mollusk`, `dep_chain_mollusk`) prove the kit onboarding and the load shape.
3. **Handle-derivation / lifecycle transport** (`zama-host` lib unit): the maximum
    `MAX_FHE_EXECUTION_STEPS`-record execution's exact CPI envelope — 32 records, 2,133 bytes
    (21 bytes of framing + 66 per record), asserted against DD-038's 10,240-byte limit in
    `event_transport.rs` — and signer/readonly event-authority metadata, plus handle-derivation
    determinism.
4. **Off-chain reconstruction — host-listener** (`coprocessor/fhevm-engine/host-listener`, feature
    `solana-reconstruct`): reconstructs compute rows and MMR leaves from instruction data +
    sysvar-streamed block entropy (Yellowstone gRPC), with no dependence on emitted events for the
    leaves. Derives update/produced-public handles directly; fails closed on incomplete executions.
    `tests/solana_leaves_tests.rs` (real Postgres) round-trips the leaf record through the
    migration, moves the checkpoint, and answers `POST /v1/solana/leaf-proofs` with proofs that
    verify against the recorded peaks; a test keeps the committed OpenAPI document in sync with the
    routes.
5. **KMS connector** (`kms-connector/crates/kms-worker`, `solana_` tests): the pipeline as pure
   functions of `(typed request, snapshot, proofs, deployment, now)` — envelope signature, window,
   deployment identity, the two-read snapshot ordering, pause, watermark, scope (`(program, scope)`
   against `allowedScopes`), peer proof verification and one selective retry, handle
   binding, delegation freshness — plus the committed byte vectors under
   `solana/test-fixtures/` that the TypeScript SDK asserts against too.
6. **ABI / IDL golden** (`scripts/check-zama-host-idl.sh`, `execution_contracts.rs`,
   `scripts/check-pda-seeds.py`): vendored IDLs and the Borsh golden manifest must match the
   freshly-built Anchor IDLs; EVENT_VERSION consistency across zama-host / confidential-token /
   host-listener is asserted (a mismatch would silently drop events); every handwritten TypeScript
   PDA seed matches its Rust counterpart.
7. **End-to-end** (`.github/workflows/solana-e2e.yml`, the bun:test scenario suite under
   `test-suite/fhevm/e2e/scenarios/`): the Yellowstone-only path feeds computation facts and leaves
   through host-listener reconstruction against a local validator + full coprocessor/KMS stack. It
   drives the **decrypt vertical** through the `encrypted-counter` specimen — write → user decrypt
   (allow leaf) → update → user decrypt of the replaced handle and of the current one — and the
   token composition wrap → burn → public release → redeem (certified public decrypt) and
   `disclose_secp` (stateless host `verify_public_decrypt`, DD-040), the confidential-transfer arc,
   delegated decrypt with a Squads multisig delegator, and the `dep-chain` load smoke. Operator
   semantics beyond the counter's operations are not exercised live: the pure conformance layer
   owns the full operator contract, and Mollusk plus direct real-TFHE supply representative SBF
   and cryptographic evidence. This leaves an integration-coverage gap for the other operators
   through reconstruction, scheduling, and decrypt. Restore that coverage through a program-owned
   specimen; the removed wallet-signed driver cannot supply the required PDA authority.
   `token_mollusk` owns the
   broader negative matrix (including after-update, redeem consume-once, disclosure idempotency,
   and foreign-proof rejection).

## Reconstruction parity strategy

The rewrite's central correctness bet is that off-chain consumers reproduce on-chain MMR state exactly.
The solana-e2e scenarios exercise host-listener reconstruction against the full stack, and every
proof the connector fetches is verified against the peaks it read on chain. A divergence fails
closed rather than yielding a wrong proof: a record that has sealed at least as much history as the
chain shows and holds no such leaf is a terminal refusal; a record that is behind is a retry.

## Confirmed-view operations

The connector reads the account at `confirmed` commitment (once, or twice when an entry is
delegated; the second read decides) and then reads the leaf proofs from every configured
coprocessor as one batch. Each query accepts any candidate that verifies against that snapshot.
An older proof may still verify against an unchanged peak; an ahead proof is shortened to the
snapshot's required height before verification. Only unresolved retryable queries are fetched
once more. A failed refresh does not discard successful queries. After that, the ordinary
decryption budget (`max_decryption_attempts`, default 20) and event polling interval (default
3 seconds) decide. An invalid path or an index ahead of the snapshot is retryable; it may reflect
different confirmed observations. A `NoLeaf` response from a record claiming sufficient history
is terminal unless another peer supplies a valid candidate or a retryable outcome.
Local HTTP tests cover peers stalling before headers and during body reads, with a healthy peer
still able to authorize, and RPC requests terminating at the same configured deadline.

The leaf record is derived in the same database transaction as the compute rows, so it cannot
disagree with them about which blocks were applied. An account first seen through an update has
`history_complete = false`: no leaf of it is stored and no proof is served for it until the
listener is replayed from before its creation. Recovery is operational, not an authorization
fallback: the connector never trusts the record without verifying the proof against live chain
state, and a record behind or incomplete only delays a decrypt. The optional `--start-slot`
bootstrap reuses inclusive Yellowstone replay within provider retention. Listener tests cover
an unapplied anchor across reconnects and the transition to an applied checkpoint. They do not
establish that a deployed Yellowstone provider retains all historical sysvar updates: validate
that with the actual plugin before relying on recovery operationally.

## Deliberately deferred (filed as follow-ups, not gaps in the merge)

- **Explicit Mollusk CU-trace assertions.** CU fit is currently implicit (Mollusk enforces the budget,
  so passing = fits) and bounded by the liveness audit's op-count analysis (leaf-count-independent).
  An explicit `compute_units_consumed` assertion per hot instruction would turn the estimate into a
  measured, regression-guarded number.
- **litesvm gate** (zama-ai/fhevm#3045): a lighter-weight in-process runtime alongside Mollusk.
  Blocked on two dependency walls, both recorded in `solana/runtime-tests/Cargo.toml`: litesvm
  pulls `solana-program-runtime >= 4.1` where this workspace is pinned to `=4.0.0-rc.0`, and
  mollusk 0.15.0 does not build on rustc 1.91.1 (E0658). Neither is worked around by test code —
  whoever picks the issue up starts from those two, not from scratch.
