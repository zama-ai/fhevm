# tx-sender hotfix: validation results

> Historical results, before the final static-gas trim. The release no longer
> exposes `--gas-limit` or selects gas based on retry counters. Static-gas tests
> and load variants described below were removed; their historical results are
> retained as experiments. Use [the final minimal plan](minimal-v013-hotfix-plan.md)
> for the current scope, compilation status and required final validation. No
> runtime results below are claimed as a fresh run of the trimmed tree.
>
> Policy clarification: proof responses expire after six retries by design,
> because the client times out and resubmits. The measured retry-cap expiry is
> expected; distinguish it from unexpected work loss and from useful throughput.

Runtime validation of the hotfix stack, executed on the 32-core / 235 GB bench
host. Closes the "validation remains outstanding" item in
[the minimal v0.13 plan](minimal-v013-hotfix-plan.md).

## Tree mapping

The bench host and the pushed branch carry the same content under different
hashes, because the commits were re-created on push. Trees are identical in
every case (`git diff` between them is empty).

| Content | Pushed (reviewed) | Bench-host original |
|---|---|---|
| regression suite and load gate | `0b78f8311` | `d86c1ca05` |
| first validation record | `1b727f508` | `f85b65de4` |
| ordered broadcast / receipt tracking | `e11f919f7` | `02cf3c614` |
| revalidation record | `edb7c51cc` | `79933aedc` |
| receipt-path fixes | `cc2d862fe` | `b21a2ebae` |

This round is based on `cc2d862fe`. Comparison builds: `8463c45ef`
(pre-amendment), `8234efc` (unpatched `origin/release/0.13.x`).

## Environment and deviations from the plan

| Plan requirement | What was used | Impact |
|---|---|---|
| Production Gateway/Nitro | anvil (`foundry v1.3.5`) | **Acknowledgment timing, pending-state behaviour, reconnect behaviour and the real capacity ceiling are not settled here.** |
| Real gateway contracts | mock `CiphertextCommits` / `InputVerification` | Real-contract execution and estimation behaviour **not** validated. |
| Method-aware RPC fault proxy | `tests/support/mod.rs`, HTTP | Method-level faults are faithful; WS closes and `BackendGone` are driven by dropping anvil. |
| WS transport | HTTP for fault-injected paths | Normal WS closes and reconnects **not** covered. |
| Recovery deadline D | **D = 600 s for B = 1024 per class**, set before the runs | Required output = input + 2B/D = **7.41/s**. |

Instrumentation bypasses the proxy: an early run had the sampler read the nonce
through the faulted path, and the pending-count blackhole stalled the sampler
itself for 181 s.

## Test results

**74 tests pass, 0 failures.** The load gate is ignored by default.

## The amendments, against the build where each defect reproduces

| Amendment | Candidate | Pre-amendment (`8463c45ef`) |
|---|---|---|
| 1 — bounded nonce lookup | distinct `eth_getTransactionCount(pending) timeout` inside its own phase deadline; mutex released; both loops resume | the send never returns within 5× the send timeout while the pending count is blackholed |
| 2 — drain on preparation error | 20 proofs → **20 mined, 20 retired, 1:1** | **32 mined for 5 retired (6.4×)** in 120 s |
| 2 — handled address error | the verify loop survives | an unparsable `user_address` panics it; `run()` joins only after cancellation, so the process reports healthy with the verify path dead |
| 3 — estimation on retries | static gas only when every retry counter is zero | a retried row still skips estimation and reuses the static limit |

## Ordered broadcast separated from receipt waiting

The serialized design measured **1.00/s** at a 1 s block time against 7.41/s
required, unchanged by static gas, admission limit or batch limits. The bound
was the sender's own mutex, which spanned nonce selection, submission *and*
inclusion. The mutex now covers nonce selection and submission acknowledgment
only.

Submission and receipt uncertainty are handled differently: a **submission**
error or timeout leaves acceptance unknown, so the sequence is invalidated and
re-seeds from `pending`; a **receipt** timeout concerns an already-acknowledged
transaction whose successors hold valid higher nonces, so the sequence is left
alone.

Two defects in that change were found in review and fixed in `cc2d862fe`: the
final receipt lookup was unbounded (its budget is now carved out of
`receipt_timeout`), and receipt errors were reformatted into strings, which
erased `BackendGone` for the whole receipt stage (transport errors now pass
through intact).

## Harness corrections this round

Four validation claims exceeded what the harness established. All four are now
fixed, and one of them changed a result.

### Dropped proofs: the claim was not proven, and is now attributed

The final accounting compared insertion counters against the last pre-shutdown
sample — observations from different times, so a residual could not be
distinguished from a dropped item. Accounting is now taken at **quiescence**:
arrivals stopped, sender terminated, chain settled, then a fresh read.

That immediately surfaced a reproducible residual, and a differential run
attributes it:

| Run | Backlog | Retry-cap deletion | Proofs unaccounted | Adds unaccounted |
|---|---|---|---|---|
| d1 | fresh | on | **0** | 0 |
| d2 | pre-retried | on | **1** | 0 |
| d3 | pre-retried, cold restart | on | **1** | 0 |
| e3 | pre-retried, repetition 2 | on | **1** | 0 |
| e1 | pre-retried | **off** | **0** | 0 |
| e2 | pre-retried, repetition 2 | **off** | **0** | 0 |

With retry-cap deletion disabled the identity balances exactly in both
repetitions, and `e2` shows one proof sitting *at* the cap — retained precisely
because deletion was off. So the residual is attributed: it is the six-attempt
budget deleting a row seeded near the cap, which is the documented data-loss
limitation, not a new defect and not a sampling race.

**The honest claim is therefore narrower than before:** zero dropped proofs is
proven for a fresh backlog; with a pre-retried backlog containing rows near the
cap, exactly one proof is dropped per run, reproducibly, by retry-cap deletion.

### The outstanding-work test bypassed admission control

It called `send_transaction_sync` directly, but permits are acquired in
`send_sync_with_overprovision`, so the configured limit was never exercised. The
test now routes through the admission-controlled method and asserts that every
attempt in each wave abandons its receipt, then that outstanding work reaches
exactly `LIMIT × WAVES` — above the limit.

### Delayed acceptance proved forward progress, not work accounting

It resubmitted the same transfer and checked mined nonces for uniqueness and
contiguity, which the chain enforces regardless. It now uses two
**distinguishable** operations (distinct transfer values), retries the loser as
the operation layer would, and asserts each logical operation is applied
**exactly once** on chain.

### The cold restart did not assert pending work

Graceful shutdown could finish the outstanding batch before the restart, so the
scenario might never occur. Inclusion is now held across the restart, and the
run **fails** if nothing is left unmined. Observed at the restart point:
**20 transactions unmined, 697 rows unsent**, after which the fresh sender
recovered and drained.

## Load gate

B = 1024 per class, arrivals 2/s per class, D = 600 s, required **≥ 7.41/s**.
Fault window 90 s then restore: estimate contention through one service channel
at 40 ms, five suppressed acknowledgments, five held past the client deadline,
and a 20 s pending-count blackhole.

| Run | Batches | Admission | Total | add | verify | Peak outst. | Est. p50/p90 **fault** | Verdict |
|---|---|---|---|---|---|---|---|---|
| serialized build | any | 1/8/32 | 1.00/s | 0.01–0.53 | 0.47–0.99 | 1 | — | never drains |
| d1 · fresh | 10/10 | 32 | 19.62/s | 9.64 | 9.98 | 20 | 374 / 734 ms | pass |
| d2 · pre-retried | 10/10 | 32 | 19.79/s | 9.81 | 9.98 | 20 | ~374 / 734 ms | pass |
| d3 · cold restart | 10/10 | 32 | 16.70/s | 8.25 | 8.45 | 20 | ~404 / 736 ms | pass |
| c2 | 64/64 | 32 | 17.75/s | 8.87 | 8.88 | 32 | **695 / 1106 ms** | pass |
| c3 | 64/64 | 64 | **52.55/s** | 25.42 | 27.12 | 48 | **1897 / 2363 ms** | pass |
| e2 · repetition | 10/10 | 32 | 7.63/s | — | — | 20 | — | pass |

Repetition spread at the bundle setting is wide: 19.62, 19.79, 19.75, 19.23 and
7.63/s. `LOAD_SEED` is a run label and does not seed `rand::random`, so these
are repetitions under the same distribution, not reproducible replays. Treat
**~7.6/s as the conservative figure**, which still clears 7.41/s but only just.

### Throughput and prep latency trade against each other

Phase-separated (the proxy previously accumulated samples across the whole run,
so the clean recovery phase dominated the percentiles):

| Active attempts | Fault-phase estimate p50 | Gate 1 (p50 < 500 ms) |
|---|---|---|
| 20 (10/10, adm 32) | 374 ms | pass |
| 32 (64/64, adm 32) | 695 ms | **fail** |
| 64 (64/64, adm 64) | 1897 ms | **fail** |

Fault-phase p50 tracks roughly `active_attempts / 2 × service_time` (40 ms in
this model), fitting all three points. The highest-throughput configuration
misses the gate by nearly 4×.

These percentiles measure **`eth_estimateGas` service time observed at the
proxy**. They are not full preparation latency, which also includes signing, fee
filling and the mutex wait, and which the sender still does not instrument. That
narrower label is deliberate.

### Gate by gate

| # | Criterion | Result |
|---|---|---|
| 1 | prep p50 < 0.5 s with a deep verify queue | **conditional** — passes at ≤ ~20 active attempts (374 ms); fails at 32 and 64. Estimation service time only |
| 2 | add-path throughput comparable | **pass** — 9.64 vs 9.98/s |
| 3 | backlog drain strictly decreasing | **pass** — including a cold restart with 20 transactions unmined and 697 rows unsent |
| 4 | zero `gas estimation timeout` | **pass** |
| 5 | zero out-of-gas | **not covered** — mock contracts |
| 6 | realized multiplier ≈ 3.0 | **partial** — wire gas limit asserted at 750 000; ratio needs real contracts. Not a release blocker with static gas unset |
| 7 | `nonce too high` = 0 | **pass** — zero nonce-error responses, counted per occurrence at the proxy |
| 8 | every mined transaction has a DB transition | **pass within the tested scenarios**, against 6.4× pre-amendment. Not general across DB failures or arbitrary restart timings |
| 9 | zero dropped proofs | **conditional** — proven zero for a fresh backlog; exactly one proof dropped per run with a pre-retried backlog, attributed by differential to retry-cap deletion |

## Frozen release bundle

These values were validated together and must be revalidated together.

| Setting | Value |
|---|---|
| `--verify-proof-resp-batch-limit` | `10` |
| `--add-ciphertexts-batch-limit` | `10` |
| `--max-inflight-sends` | `32` |
| `--send-txn-sync-timeout-secs` | `4` |
| `--txn-receipt-timeout-secs` | `30` |
| Gas estimation | every attempt; the new `--gas-limit` option is deferred |

> **Admission 32 is not headroom.** With these batch limits, active attempts are
> capped at 20 (two operations × 10) and the admission limit is non-binding.
> Raising the batch limits would activate the higher concurrency that already
> **failed** the modeled estimation-latency gate — 695 ms at 32 active
> attempts, 1897 ms at 64, against 500 ms. Batch and admission changes require
> joint revalidation.

> **Breaking flag change.** `--txn-receipt-timeout-secs` used to be an *alias* of
> `--send-txn-sync-timeout-secs`. It is now a separate phase, and the new default
> of 30 does **not** override a value already present in a deployment. An
> existing `--txn-receipt-timeout-secs=4` now means a four-second inclusion
> deadline, which will abandon receipts and retry needlessly. Set both flags
> explicitly.

## Release path

Revalidate the final trimmed tree per the minimal plan, then run **this
exact bundle** against production-equivalent Nitro, Gateway contracts and
WebSocket transport, covering:

- backlog recovery for both operations, measuring useful throughput and recovery
  per operation;
- normal WebSocket closes and reconnects;
- ambiguous acceptance;
- stalled receipts;
- cold restart with transactions still pending.

Then a single-operator canary.

Still unvalidated, and not closed by anything here: real-contract execution and
estimation behaviour, and full preparation latency. With the static-gas option deferred,
static-limit sizing is outside this release. Durable reconciliation, proof retention and
operation supervision remain accepted deferred work — implementing them is not a
prerequisite, but their limitations stay explicit:

1. A dead operation loop is never surfaced: `run()` joins its operations only
   after cancellation, and `/health` reports only connectivity.
2. A malformed row starves later rows in its batch; pinned by a test.
3. The six-attempt proof budget deletes rows at the cap — the measured source of
   the one dropped proof per pre-retried run.
4. One active sender per key is still required, including during rollout.
