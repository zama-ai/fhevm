# Minimal v0.13 hotfix: final scope and validation handoff

This is the current release plan. The new static-gas option has been removed from
this hotfix. Earlier experiments and the broader main/v0.14 remediation remain in
[the validation report](hotfix-validation-results.md) and
[the full review](hotfix-review-and-amendment-plan.md); their static-gas sections
are historical, not current release instructions.

## Final implementation

The hotfix retains:

- Pending-based nonce seeding and ordered broadcast. The mutex covers nonce lookup
  and submission acknowledgment, with inclusion awaited outside it.
- Separate bounded nonce-lookup, submission, gas-estimation and receipt phases.
  The final receipt lookup shares the receipt budget; transport errors retain
  their type so `BackendGone` still reaches the existing stop path.
- Shared admission control across both operations, covering each active attempt
  through its receipt wait. This bounds active attempts, not all unmined chain
  transactions: a receipt timeout releases its permit while the transaction may
  remain pending.
- Batch draining after send and preparation errors, handled proof-address parse
  errors, and preservation of fatal backend errors while collecting results.

The hotfix no longer includes:

- The new `--gas-limit` flag or binary wiring. The binary passes `None` and estimates
  gas on every first attempt and retry, retaining existing terminal-error detection.
- Retry-counter-based selection between static gas and estimation.
- The static-gas regression suite, the added static-base helper test, or the load
  harness's `LOAD_STATIC_GAS` switch.

The pre-existing internal `gas: Option<u64>` API and overprovisioning support are
preserved, with their original behavior for library callers. The existing
`--gas-limit-overprovision-percent` flag also remains. New static-gas functionality
can be reviewed separately for main/v0.14 if there is measured demand for it.

## Compilation and evidence status

Compilation only was requested for this trim. Run from `coprocessor/fhevm-engine`:

```sh
SQLX_OFFLINE=true cargo check --locked -p transaction-sender --all-targets
```

Result on 2026-09-10: passed, including compilation of the remaining test targets.
Cargo emitted the existing non-root-profile warning for stress-test-generator.
No tests, RPC probes, load
runs or benchmarks were executed locally for this trim. The earlier report's test
counts include the now-removed static-gas tests and must not be presented as a test
result for the trimmed tree. Existing static-disabled load results remain useful
prior evidence, but the final tree still needs the checks below on the larger host.

The history is organized into three commits: implementation, tests/harness, and
plans/documentation. Previous candidate hashes in historical reports identify the
pre-trim experiments, not the final release artifact.

## Frozen configuration for final validation

Validate these settings together, using the supplied production settings for the
other flags:

| Option | Value |
|---|---|
| `--verify-proof-resp-batch-limit` | `10` |
| `--add-ciphertexts-batch-limit` | `10` |
| `--max-inflight-sends` | `32` |
| `--send-txn-sync-timeout-secs` | `4` |
| `--txn-receipt-timeout-secs` | `30` |
| `--gas-estimation-timeout-secs` | `20` |
| `--gas-limit-overprovision-percent` | `300` |

Do not supply `--gas-limit`: it is not a supported CLI option in this release.
The receipt flag was formerly an alias of the submission flag. Replace any old
`--txn-receipt-timeout-secs=4` argument and set both deadlines explicitly.

At 10/10 batch limits, the two operations offer at most 20 active attempts, so
admission 32 is non-binding. It is not permission to raise batch sizes independently:
those changes activate additional concurrency and require joint revalidation.
Each nonce lookup and submission has its own four-second budget; neither includes
queue waiting or gas estimation. Preserve the actual eight-second shutdown grace
in the restart experiment.

## Required larger-host validation

The release sender now requires HTTP/HTTPS and keeps asynchronous raw submission
with ordered acknowledgments. The WSS-retention vendor patch and its dedicated
regressions have been removed. The receipt builder still has a finite internal
timeout, in addition to the outer deadline and bounded fallback: watcher retention
is independent of transport. See the
[HTTPS evaluation and release protocol](https-evaluation-and-validation.md) for
transport policy, deployment migration, comparison, fault tests and rollback.

Run `hotfix_http_transport_tests` and the dropped-transaction watcher regression
on the larger machine. Both are compile-checked only on this host.

Use the existing Docker/test DB, Anvil, solc and signer-test prerequisites, then run:

```sh
cd coprocessor/fhevm-engine
cargo test --locked -p transaction-sender
```

This runs the remaining regression suites; the load gate remains ignored unless
explicitly selected. Confirm bounded nonce lookup, ordered submissions before
inclusion, receipt deadlines through the fallback, preserved receipt-stage
`BackendGone`, real failed receipts, drain after preparation failure, delayed
acceptance with distinguishable operations, and outstanding work after receipt
timeouts through the admission-controlled method.

For the removed static-gas behavior, no replacement static-gas test is required.
Instead, confirm the actual operation paths estimate on both fresh and pre-retried
rows and retain the existing already-added/verified/rejected and configuration-error
handling. The original internal explicit-gas support is not a new release feature.

Run the final mixed-workload bundle with first a fresh and then a pre-retried backlog:

```sh
LOAD_OUT=/tmp/txn-hotfix-fresh.csv LOAD_BACKLOG=1024 LOAD_ARRIVAL=2 \
LOAD_DEADLINE=600 LOAD_FAULT_SECS=90 LOAD_MEASURE_SECS=600 \
LOAD_VP_BATCH=10 LOAD_ADD_BATCH=10 LOAD_MAX_INFLIGHT=32 \
LOAD_RECEIPT_TIMEOUT=30 LOAD_BLOCK_TIME=1 LOAD_PRERETRIED=0 \
cargo test --locked -p transaction-sender --test hotfix_load_gate -- --ignored --nocapture
```

Repeat with `LOAD_PRERETRIED=1`, a distinct `LOAD_OUT`, and then `LOAD_RESTART=1`.
`LOAD_SEED` labels repetitions; it does not make them deterministic replays.
`LOAD_STATIC_GAS` has been removed and must not be used to describe a run variant.

Before issuing a final accounting verdict, ensure the harness has actually stopped
arrivals and terminated the sender, and obtain settled measurements. A shutdown or
probe failure must not be treated as a clean zero. Attribute remaining discrepancies
rather than accepting a small unexplained residual. Preserve the per-operation rates,
phase-separated estimation latency, actual inserted work and final outcomes alongside
all effective arguments and the final candidate SHA/image digest.

Then run the same bundle against production-equivalent Nitro, real Gateway contracts
and HTTPS. The Anvil/HTTP harness does not establish acknowledgment, pending-state, TLS
or connection-recovery semantics on that infrastructure. Exercise:

1. Fresh and entirely pre-retried backlogs with continuing arrivals on both paths.
2. HTTP keep-alive closes, abrupt disconnects, connection recovery and stalled receipt RPCs.
3. Acceptance delayed before forwarding, lost acknowledgments, and lost receipts
   after acknowledgment, with work accounting rather than nonce continuity alone.
4. Cold restart with a fresh provider after the old sender terminates, while
   transactions are demonstrably still pending.
5. Real-contract estimation and terminal errors, plus the supported input sizes
   and quorum positions. Static-limit sizing is outside this release.

Require both operations to recover and meet the agreed deadline. For the historical
fixture, B=1024 per class, continuing arrivals=2/s per class, and D=600 s imply a
combined target of at least 7.41 useful completions/s; also evaluate the actual
backlog at restoration and each operation's recovery. Do not equate estimate-only
latency with full preparation latency, or claim comfortable capacity margin from a
single favorable run. The final production decision needs representative repetitions.

## Intended proof expiry and deferred scope

Proof responses exhausting six retries expire by design: the client has already
timed out and is expected to resubmit. The pre-retried fixture's retry-cap deletion
is therefore expected expiry, not an unexpected hotfix loss. Record those expiries
separately from useful completions; queue shrinkage through expiry is not throughput.

The release criterion is recovery without unexpected work loss while preserving the
existing proof-expiry policy. Ciphertext commitments retain their effectively unlimited
retry policy. Do not expand this hotfix into proof-retention redesign merely to force
zero expiry in the deliberately near-cap fixture.

Durable reconciliation, operation supervision, malformed-row quarantine and multi-process
nonce coordination remain deferred. One active sender per signing key is required,
including during rollout. After the final infrastructure gates pass, canary one operator
while monitoring useful completion rates, queue age, retries and proof expiries.
