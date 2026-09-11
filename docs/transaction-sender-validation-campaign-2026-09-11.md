# Transaction sender validation campaign — 2026-09-11

Candidate: `c353ee772`, on transport `67762d493` and retry accounting `0fab89ce1`.
Protocol: [proof retry reliability](transaction-sender-http-proof-retry-test-protocol.md).

## Execution started

Artifacts are retained locally at `/tmp/fhevm-validation-c353ee772/`.
The Gateway credential file is read directly by the harness; credentials are
not copied into this document or passed as command-line arguments.

| Lane | Scope | Artifact |
| --- | --- | --- |
| Sender regression | Full transaction-sender package, release/locked, two test threads | `full-sender.log` |
| Stack | Standard e2e profiles; candidate binary copied into local sender container and SHA-256 matched | `e2e-standard.log` |
| Extended retry matrix | HTTP 408/429/500/502/503/504 at estimate, nonce read and send; 18 cases | `retry-matrix.log` |
| Long outages | Two five-minute submission outages, removal enabled and disabled, max retries 15 and initial count 14 | `retry-matrix.log` |
| Gateway performance | Pilot, HTTP batches 1/10/40/80 with two loops, batch 128 with one loop; WSS comparisons at 10/40/128; three-key HTTP; ten-minute HTTP soak | `performance-status.log`, individual run logs |

Retry cases audit every proof-row update/deletion with a database trigger,
require unchanged retry eligibility during the fault, observe repeated attempts,
check bounded backoff and require the matching contract event after recovery.
The recovery deadline is 20 seconds after fault removal. These tests use real
sender code with Anvil and PostgreSQL; they do not inject faults into Conduit.

Gateway preflight confirmed chain 10900 and latest = pending on all three
accounts. Performance runs use zero-value transfers, 20-second warmup and
60-second measurement except the short pilot and 600-second soak. Single-key
runs use account index 1; the multi-key run uses all three. A campaign-specific
copy of the retained driver enforces attempt and reserved gas-spend limits,
drains active tasks at the end, suppresses raw error samples and reports
successful receipt throughput. The original driver remains untouched.
The aggregate spend ceiling is 0.08 ETH, with a maximum reservation of 0.03 ETH
per run. The runner stops on failed tests or an account with unmined work at rest.
Account balance/nonce snapshots are retained without keys or endpoint URLs.

## Interpretation and remaining gates

Execution has started; this record is not a passing release gate. Results will
be recorded after completion. Transfer benchmarks measure transport/submission
capacity; they do not establish mixed proof/ciphertext capacity on Conduit.
Error counters from the retained driver include warmup and drain; measured
successful receipt throughput is restricted to the measurement window.

The extended matrix does not cover the entire protocol: baseline negative
controls, selective-failure fairness, database-preserving restarts across mixed
operations, client/relayer expiry and retry races, and deployed OTLP inspection
still require dedicated cases. Standard e2e success must not be substituted for
those fault scenarios. Accepted-but-unmined nonce recovery remains deferred.

## Interim status

Standard e2e passed in 839 seconds. The sender suite passed 32 tests before
one AWS KMS gas-estimation failure stopped the run; the failed test and
remaining proof suites are queued for rerun. The extended retry harness had
a usize/u64 compilation mismatch; corrected, with execution queued again.

HTTP measured goodput: 189.3 tx/s at batch 10 × 2, 492.6 at 40 × 2,
533.1 at 80 × 2, and 482.1 at 128 × 1. All these HTTP runs had zero
errors, including warmup and drain. WSS batch 128 × 1 measured 3.07 successful
receipts/s and 609 send timeouts across warmup, measurement and drain.
It left pending work at the eight-second rest check, stopping the campaign.
A later read confirmed all three accounts had drained naturally; no nonce
repair was performed. The three-key run and HTTP soak are now resuming,
retaining the original aggregate spend accounting.

## Latest completed results

- All 70 distinct sender regression tests passed across the initial run and
  targeted continuation. The previously failing AWS KMS estimation test passed
  on rerun; its initial intermittent failure remains recorded, not erased.
- Extended retry matrix: 20/20 passed in 898.94 seconds, including both
  five-minute outages with database transition auditing.
- Three-key HTTP: 552.825 successful receipts/s, 44,160 successful transactions
  including warmup/drain, zero errors.
- The intended ten-minute soak was invalidated by the harness reservation
  ceiling: 59,520 successful transactions, zero RPC errors, then submission
  stopped before the measurement timer ended. Its 92.733 tx/s aggregate is
  not a valid sustained-load throughput measurement. A complete ten-minute
  soak at the intended concurrency remains outstanding.
- Total Gateway spend through this attempt: 0.05171878324 ETH. Nonce deltas
  account for 244,265 transactions; all three accounts had latest = pending
  at the final snapshot.

The launched processes have finished. No release-ready verdict is established:
the full-duration soak and the dedicated protocol gaps above remain open.

## Continuation with reviewed commits locked

The reviewed commits are `4b14c0a9f`, `d2fbe5142`, and `2d335dd89`.
All continuation changes are test harnesses and documentation on top of that
history. Runtime source is unchanged. Artifacts for this continuation live in
`/tmp/fhevm-validation-locked/`.

The revised performance budget retains the maximum reservation for every
outstanding or ambiguous submission. A mined receipt replaces its reservation
with `gas_used * effective_gas_price`; an estimation failure releases its
reservation because no submission was attempted. Transactions exceeding the
assumed maximum gas limit are not submitted. The accounting unit test checks
that confirmed receipts refund headroom while ambiguous work stays reserved.
The soak rerun has a 0.028 ETH cap within the original campaign's remaining
budget. It is a 20-second warmup plus 600 measured seconds at batch 10 × 2.
Builds for other validation lanes share this host, so its throughput is not an
isolated capacity measurement.

### Finding: selective failures block subsequent proof batches

`gateway_mixed_campaign_tests` failed its fairness assertion after completing
all cleanup/recovery checks. The fault proxy returned HTTP 503 only for gas
estimation of proof IDs 1–10. There were initially 30 proofs, another 10 added
on each of two restarts, and 10 ciphertext digests per cycle. Proofs included
verification and rejection responses and started one retry below exhaustion.

- All ten affected proofs retained retry count 14 of 15. A database trigger
  observed no update/deletion of those proofs during the fault.
- Ciphertext work drained during each outage cycle.
- Healthy later proof completions were `[0, 0, 0]` across the three cycles.
- After fault removal, all 50 distinct proof IDs had matching contract events
  and the proof queue drained within the declared 60-second recovery bound.

The cause is selection by `ORDER BY zk_proof_id LIMIT batch_limit` together
with unchanged eligibility for the earliest failing batch. Restarts preserve
that ordering. This is a failed selective-failure fairness gate, not proof loss
or a cross-operation ciphertext stall. The review must explicitly address this
limitation; no production change was made to hide or resolve the failure.

The relayer's existing `input_proof_v2_test` suite passed 49/49 tests in 24.68
seconds, including timeout, deduplication, and retry-after-failure. It runs with
a mock Gateway. The application's deployed SDK version and finite retry policy
are still needed to define full-stack client-recovery acceptance.

### Harness corrections and additional controls

One continuation soak accidentally selected the old Rust test executable. Its
preflight filter ran zero tests but returned success; that run stopped at the
old reservation ceiling after 55,550 successful transactions (0.0116655 ETH).
It is invalid as a sustained soak. The runner now requires exactly one passing
reservation test before any Gateway calls. A negative check confirmed that it
rejects the old executable. A new run uses the corrected executable, whose
SHA-256 and driver source hash are saved in `soak-confirmed/metadata.json`.
This separate load-driver binary was built with Rust 1.97.1; the release
regression builds use the engine's pinned Rust 1.91.1.

To cover the repeated run, the campaign's self-imposed aggregate ceiling was
raised to 0.10 ETH, with 0.028 ETH reserved for the new soak. The user-funded
accounts had sufficient balance; no refill was requested.

The baseline worktree and candidate initially shared Cargo artifacts, producing
invalid compile failures. After the baseline finished, the candidate's common
library was explicitly rebuilt before rerunning validation. Future baseline
builds should use a separate `CARGO_TARGET_DIR` and the pinned toolchain. These
compile failures are harness problems, not runtime regression findings.

The transport-only negative control at `4b14c0a9f`, with candidate test/proxy
files copied into its worktree and runtime code untouched, failed all four
selected preservation assertions for submission HTTP 500/502/503/504. Rows
reached retry count 15 rather than remaining at 14, so they were ineligible.
The cases include both removal settings. The assertions fail upon exhaustion;
this control does not independently wait for the subsequent deletion pass.

An additional relayer test passed in 4.63 seconds: identical input deduplicates
before server expiry; after expiry it creates a new job that succeeds, while
the original job remains failed. It uses a two-second server timeout and
one-second scheduler cadence with a mock Gateway. This adds one distinct
passing case to the earlier 49, without claiming deployed-client acceptance.

## Continuation results

| Check | Result |
| --- | --- |
| Corrected ten-minute HTTP soak, batch 10 × 2 | Passed: 107,400 measured successful transactions, 178.999 tx/s, p50 98.3 ms, p99 259.2 ms |
| Soak warmup + measurement + drain | 111,280 successful transactions, zero errors/reverts/timeouts, 0 unmined; cost 0.0233688 ETH |
| Additional audited fault/recovery cases | 6/6 passed in 219.58 seconds: unanswered estimate, local send timeout, Conduit deadline, RPC unavailable, HTTP disconnect before estimate/send forwarding |
| Extended retry campaign total | 26 distinct passing cases including the earlier 20 and both five-minute outages |
| JSON and actual OTLP/gRPC export | Passed: positive control present in both sinks, no synthetic credential markers |
| Transport/privacy regression rerun | 8/8 passed |
| Relayer input-proof tests | 50 distinct passing tests across the original suite and the new expiry/resubmission case |
| Negative control | All four selected baseline cases failed the preservation assertion as intended |
| Selective-failure fairness | Failed; preservation, ciphertext progress, and post-outage recovery passed |

Total Gateway campaign spend, including invalidated attempts, is
**0.08675308324 ETH** across **411,095 transactions by confirmed nonce delta**.
The final account snapshots show latest = pending on all three accounts and
0.33632946026 ETH remaining in total. No refill was needed.

The Python soak runner now also requires all outcome counters to be present,
zero errors, and a successful-receipt count matching confirmed nonce progress.
These acceptance checks were verified against the completed soak's captured
outcomes/account snapshots; they do not require another funded run.

The campaign is not an all-green release gate: selective failures can starve
later proofs, and deployment-specific client/SDK retry guarantees remain
unvalidated pending the actual client policy. Local mock-relayer and OTLP
checks do not replace full-stack client-outage races or inspection of the
actual deployment's collector configuration. Production-path mixed contract
capacity and deferred nonce reconciliation are also outside these results.

The mixed-workload test was repeated with the pinned Rust 1.91.1 toolchain:
it reproduced the same fairness failure in 29.94 seconds, with all 50 proofs
recovered after fault removal and later completions `[0, 0, 0]` during faults.
Formatting with the pinned formatter, Python syntax checks, and diff whitespace
checks passed. Harness usage is documented in the sender's `scripts/README.md`.

## Deployed SDK/readiness clarification

The deployment owner supplied `@zama-fhe/sdk@3.5.1` → `@fhevm/sdk@0.13.2`
(commit `07fb05fb7`), relayer v0.13.4 and rollback v0.13.0. This resolves the
version/configuration question above. Source comparison confirms that the
SDK async-request implementation matches this branch, and that the readiness
loop and example retry settings match all three relayer tags v0.13.0/.2/.4.
The SDK throws on the relayer's terminal 503; its one-hour deadline does not
retry that result. Approximately 225 seconds describes the relayer readiness
budget, not an application retry limit (74 three-second sleeps plus RPC time).

The prior input-proof expiry test covers a different state machine. The test
protocol now includes section C for decryption readiness, the final-attempt
race, actual SDK terminal-error behavior and explicit fresh calls after
recovery, for both relayer versions. Those full-stack scenarios remain unrun;
no automatic application resubmission guarantee is inferred from SDK polling.
This clarification does not change the passing sender preservation results
or resolve the observed selective-failure starvation gate.

Focused readiness regression: `test_readiness_timeout_returns_503_with_correct_label`
passed (1/1, 0.91 seconds) with Rust 1.91.1 and `--features integration-tests`.
It uses a mock Gateway and two attempts separated by 50 ms, asserting HTTP
503 and the exact readiness label. It does not exercise the SDK or the deployed
window. The first invocation omitted the integration-test feature and failed
to compile the optional mock dependency; the corrected invocation passed.
Artifact: `/tmp/fhevm-validation-locked/relayer-readiness-boundary.log`.
