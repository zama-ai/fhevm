# Solana Host Listener PoC Plan

Date: 2026-02-09
Branch: `codex/solana-host-listener-discovery`

## Goal

Validate whether we can run a minimal end-to-end FHEVM flow with a Solana host chain while keeping Gateway on EVM.

This is an exploratory PoC. Code can be discarded after each experiment if the learning is captured.

## Scope

- Investigate integration shape for Solana host events -> coprocessor jobs.
- Prioritize fast feedback loops and clear validation over production readiness.
- Capture decisions and corrections in `LEARNING.md`.

## Non-goals

- No big refactor of current EVM `host-listener`.
- No protocol redesign.
- No production hardening.

## Current Baseline (from repo audit)

- Gateway remains EVM and is a payment/enforcement layer.
- Existing `host-listener` is strongly EVM-specific in transport, decoding, and ingestion.
- Scheduler/worker path is mostly DB-driven and can be reused if DB rows are compatible.
- Handle metadata format must remain compatible with Gateway checks.

## Main Decision to De-risk

Choose one path based on evidence:

- Path A: Adapter/sub-service inside current `host-listener`.
- Path B: New `solana-listener` service writing equivalent DB records.

Current expectation: Path B is lower risk for fast learning.

## Experiment Tracks

### Track 1: Log-driven listener (`msg!`)

Question: Can deterministic Solana logs be a reliable source of operation intents?

Validation target:

- Parse and map a minimal operation set from logs.
- Insert rows that trigger downstream coprocessor execution.
- Prove one local end-to-end pass.

### Track 2: PDA receipt-driven listener

Question: Are PDA receipts/state accounts a more reliable source than logs for indexing and retries?

Validation target:

- Persist operation intents in PDA accounts.
- Index receipts and map them to the same DB operation model.
- Prove one local end-to-end pass.

## E2E Feedback Loop Requirements

Each track must be self-contained and scriptable on a laptop.

Minimum loop:

1. Start local stack components (existing coprocessor + Gateway EVM + Solana validator).
2. Submit one host operation from Solana side.
3. Listener ingests and writes DB operation rows.
4. Scheduler/worker process operation.
5. Observe expected state transition and artifacts in DB/logs.

A run is valid only if all 5 steps are reproducible with documented commands.

## Execution Plan

1. Define minimal operation subset for PoC (`add` only, then 1-2 more ops).
2. Formalize mapping from Solana signal -> existing DB schema.
3. Implement Track 1 PoC with a small listener spike.
4. Capture learnings and decide whether Track 1 is sufficient.
5. Implement Track 2 PoC if uncertainty remains.
6. Compare both tracks with explicit criteria.
7. Decide architecture direction for next phase.

## Fast Loop Artifacts

- Fast feedback loop design: `FAST_FEEDBACK_LOOP.md`
- 1:1 feature parity matrix: `HOST_LISTENER_PARITY_MATRIX.md`

## Comparison Criteria

- Determinism and replay safety.
- Idempotency and deduplication behavior.
- Finality/conflict handling strategy.
- Operational simplicity in local/dev environments.
- Compatibility with existing coprocessor model.
- Effort to evolve toward production.

## Deliverables for This Branch

- This `PLAN.md`.
- `LEARNING.md` updated as experiments run.
- Optional throwaway PoC code used only to validate hypotheses.
- A final recommendation: adapter vs new `solana-listener`.

## Exit Criteria for Initial Exploration

- At least one verified self-contained e2e run with Solana host signal.
- Clear written tradeoff between log-driven and PDA-driven approaches.
- Decision recorded on whether to keep listener separate.
