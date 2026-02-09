# Solana Host Listener Learning Log

Date opened: 2026-02-09
Branch: `codex/solana-host-listener-discovery`
Status: Active

## How to use this file

- Record facts, not guesses.
- When a hypothesis is invalidated, keep the old entry and mark it as invalidated.
- Link each decision to evidence from code, logs, or a reproducible run.

## Current Facts (confirmed)

1. Gateway stays on EVM as payment/enforcement layer.
2. Existing `host-listener` is deeply EVM-coupled.
3. Coprocessor compute path downstream is mostly DB-driven.
4. Handle metadata compatibility with Gateway checks is mandatory.
5. Fast learning requires a self-contained local e2e loop.
6. A local hybrid (`logs + PDAs`) Solana host demo exists at `/Users/work/code/zama/solana-symbolic-host-demo` and is useful as a fast-loop reference.

## Open Questions

1. Is log-driven ingestion (`msg!`) sufficiently deterministic for our indexing and replay needs?
2. Do PDA receipts reduce ambiguity enough to justify additional complexity?
3. What is the smallest Solana operation model that maps cleanly to current DB schema?
4. Which parts of the current listener should eventually become chain-agnostic, if any?
5. Do we need any transaction-sender behavior changes for Solana-host-only local tests?

## Hypotheses

### H1

A separate `solana-listener` will let us validate feasibility faster than refactoring `host-listener` now.

Status: Pending validation

Evidence to collect:

- Time-to-first-e2e.
- Number of EVM listener files that would otherwise require invasive abstraction.

### H2

A log-driven PoC can reach first e2e faster than a PDA-receipt PoC.

Status: Pending validation

Evidence to collect:

- Implementation time and complexity.
- Failure modes during replay/restart.

### H3

PDA receipts will offer better replay/index guarantees for long-term robustness.

Status: Pending validation

Evidence to collect:

- Idempotency behavior under restarts.
- Simplicity of detecting missed operations.

## Experiment Journal

### Experiment 0: Repo architecture audit

Date: 2026-02-09
Objective: Determine if adapter vs new service is lower risk for initial exploration.
Result: `host-listener` appears deeply EVM-shaped; separate `solana-listener` is currently favored for PoC.
Confidence: Medium-high
Notes:

- Final decision deferred until at least one Solana e2e run exists.

### Experiment 1: Fast feedback loop + parity scoping

Date: 2026-02-09
Objective: Define a spec-driven host+listener-first loop and enumerate EVM features to replicate.
Result: Added `FAST_FEEDBACK_LOOP.md` and `HOST_LISTENER_PARITY_MATRIX.md`.
Confidence: High
Notes:

- Initial PoC boundary is now explicit (handle metadata, op request ingestion, allow ingestion, finality/cursor/idempotency).

### Experiment 2: Solana architecture decomposition

Date: 2026-02-09
Objective: Make separation boundaries explicit (host programs, adapter/listener, shared core, gateway), including ownership and CPI options.
Result: Added `SOLANA_ARCHITECTURE.md` with component and sequence diagrams.
Confidence: Medium-high
Notes:

- Decided to favor monolithic host program for first loop, while keeping internal module seams compatible with later split (ACL/HCU programs).

## Decision Log

### D0

Date: 2026-02-09
Decision: Keep Gateway on EVM for the exploration.
Why: Fixed project constraint and aligns with protocol/payment responsibilities.
Status: Locked for this exploration phase.

### D1

Date: 2026-02-09
Decision: Prioritize separate `solana-listener` PoC before attempting listener abstraction.
Why: Minimizes blast radius and improves feedback speed.
Status: Active (revisit after Track 1 and Track 2 results).

## Next Update Triggers

Update this file after any of the following:

1. First successful local Solana -> DB -> worker e2e run.
2. Any invalidated hypothesis.
3. Any architecture decision affecting adapter vs separate service.
