# Fast Feedback Loop (Host + Listener First)

Date: 2026-02-09
Status: Active (v0 baseline validated)

## Objective

Get a repeatable red/green loop in minutes for Solana host-listener behavior before plugging workers, transaction-sender, or full Gateway flows.

Primary value for this phase:

- Feasibility signal quickly
- Learning capture with low blast radius
- Spec-first confidence (not ad hoc experimentation)

## Design Constraints

- Gateway remains EVM payment/enforcement layer.
- We are exploring Solana as a new host listener input source.
- Keep current EVM `host-listener` stable during exploration.
- Prefer throwaway PoC code over early abstractions.

## Spec-Driven Loop Model

Use a small contract between host-side signals and ingestion behavior.

For each feature we track:

1. `Spec`: expected semantics and payload
2. `Adapter`: Solana -> canonical host event model
3. `Assertion`: DB/state effects and idempotency guarantees

If `Spec` changes, tests and adapters must change in the same commit.

## Loop Ladder

### L0: Parser/Mapping Loop (seconds)

Purpose:

- Validate parsing and mapping without chain or DB.

Input:

- fixture logs/receipts (JSON)
- fixture expected canonical events

Checks:

- operation mapping (`add`, `sub`, `mul`, etc.)
- handle byte-level invariants (length + metadata bytes)
- ACL event mapping

Pass criteria:

- deterministic parser outputs for all fixtures
- no panics on malformed/unknown data

### L1: Listener+DB Loop (1-3 min)

Purpose:

- Validate ingestion semantics without running a validator.

Input:

- replay fixtures fed into listener adapter

Checks:

- inserts into `computations`, `allowed_handles`, `pbs_computations`, `host_chain_blocks_valid`
- no duplicates after replaying same batch (`ON CONFLICT` behavior)
- stable `host_chain_id` + key-id routing

Pass criteria:

- expected row counts and keys
- idempotent replay

### L2: Host+Listener Localnet Loop (3-10 min)

Purpose:

- Validate real Solana signal source with minimal infra.

Stack:

- `solana-test-validator`
- minimal host program (symbolic only)
- `solana-listener` PoC + Postgres

Checks:

- request tx emits/updates expected signal source
- listener ingests and writes expected rows
- listener restart does not duplicate completed ingestion

Pass criteria:

- one deterministic request -> one deterministic DB insertion set
- restart-safe replay

### L3: End-to-End Extension (later)

Purpose:

- add worker/scheduler and then Gateway-facing flows.

Not required for initial feasibility.

## Canonical First Slice

Start with one operation and one ACL flow:

1. `FheAdd` equivalent request -> `computations` insertion
2. `allow` equivalent -> `allowed_handles` + `pbs_computations`

Why:

- hits both TFHE and ACL ingestion pipelines
- enough to validate most listener architecture decisions

## Solana Signal Strategy for Fast Feedback

PoC baseline source:

- canonical: finalized RPC logs/events + durable cursor replay
- optional fast hints: confirmed websocket logs (wake-up only)
- event encoding variants to compare: `emit!` vs `emit_cpi!`

Rationale:

- lowest state cost and fastest implementation
- keeps replayability by using finalized catchup + idempotent DB writes
- avoids early rent/state complexity from per-tx PDA or journal designs
- lets us measure event-delivery reliability tradeoff early without changing DB contract

## Minimum Assertions Per Run

Every L2 run must output:

1. `tx_signature` (or equivalent canonical tx id)
2. derived `handle` (hex/base58 + raw 32 bytes)
3. ingestion summary (`inserted_computations`, `inserted_allowed`, `inserted_blocks`)
4. replay summary (`replayed_events`, `new_rows=0`)
5. event mode summary (`emit` or `emit_cpi`)

If any of these are missing, the run is not considered valid.

## Suggested Runbook Skeleton

1. Start Postgres and run migrations.
2. Start `solana-test-validator`.
3. Deploy/start minimal symbolic host program.
4. Start `solana-listener` in verbose mode.
5. Submit one request tx (`add`) using event mode A (`emit!`).
6. Query DB assertions.
7. Repeat with event mode B (`emit_cpi!`).
8. Restart listener.
9. Re-run ingestion/backfill.
10. Re-check DB for idempotency.

## What We Explicitly Defer

- full op catalog parity
- delegation propagation
- transaction-sender integration
- worker execution and ciphertext material publication
- production-grade indexer infra (Geyser/custom pipeline)
- on-chain receipt/journal cleanup design

## Exit Criteria for This Feedback Harness

- We can run L2 locally end-to-end in under 10 minutes.
- We can run L1 from fixtures in under 3 minutes.
- We can detect regressions by failing assertions, not manual log reading.

Current checkpoint:

1. finalized RPC ingestion + DB assertions: validated
2. `emit!` vs `emit_cpi!` DB-contract equivalence: validated
3. replay idempotency (`new_rows=0`): validated
4. worker e2e compute + decrypt sanity (`emit!` and `emit_cpi!`): validated
5. ACL gate behavior (`request_add` blocked until `allow`): validated

## v0 Binary Acceptance Checklist (Agreed)

1. `request_add` path: one Solana request produces one canonical op record and exactly one `computations` row.
2. `allow` path: one Solana allow signal produces exactly one `allowed_handles` row and exactly one `pbs_computations` row.
3. Deterministic ordering: `schedule_order` is derived from `slot_time + tx_index + op_index` and replaying the same finalized range preserves ordering.
4. Replay/idempotency: restarting listener and reprocessing the same finalized range inserts `0` new canonical rows.
5. Finality safety: canonical ingestion/cursor advancement only from finalized data; confirmed/log stream is hint-only.
6. Scope discipline: no required refactor/regression in existing EVM listener path.
7. Event mode comparison: same payload semantics and DB effects for `emit!` and `emit_cpi!`.

## Deferred State-Cost Experiments

Not in the first PoC baseline:

1. per-tx receipt PDA lifecycle and cleanup
2. sharded journal/lane segment designs

These are evaluated only after baseline logs path passes replay/correctness gates.
