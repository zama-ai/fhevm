# Fast Feedback Loop (Host + Listener First)

Date: 2026-02-09
Status: Draft for Experiment 1

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
- stable `chain_id`/tenant routing

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

Use hybrid source immediately:

- `msg!`/program logs for low-latency trigger and observability
- PDA receipt/state for replay/recovery source of truth

Rationale:

- logs-only is fast but fragile on reconnects
- PDA-only is robust but slower to iterate on UX and instrumentation
- hybrid gives fast local loop while preserving restart safety

This matches existing local learning in `/Users/work/code/zama/solana-symbolic-host-demo/README.md` and `/Users/work/code/zama/solana-symbolic-host-demo/docs/notes/logs-vs-pdas.md`.

## Minimum Assertions Per Run

Every L2 run must output:

1. `request_id` (`signature` or equivalent)
2. derived `handle` (hex/base58 + raw 32 bytes)
3. ingestion summary (`inserted_computations`, `inserted_allowed`, `inserted_blocks`)
4. replay summary (`replayed_events`, `new_rows=0`)

If any of these are missing, the run is not considered valid.

## Suggested Runbook Skeleton

1. Start Postgres and run migrations.
2. Start `solana-test-validator`.
3. Deploy/start minimal symbolic host program.
4. Start `solana-listener` in verbose mode.
5. Submit one request tx (`add`).
6. Query DB assertions.
7. Restart listener.
8. Re-run ingestion/backfill.
9. Re-check DB for idempotency.

## What We Explicitly Defer

- full op catalog parity
- delegation propagation
- transaction-sender integration
- worker execution and ciphertext material publication
- production-grade indexer infra (Geyser/custom pipeline)

## Exit Criteria for This Feedback Harness

- We can run L2 locally end-to-end in under 10 minutes.
- We can run L1 from fixtures in under 3 minutes.
- We can detect regressions by failing assertions, not manual log reading.
