# Conduit Gateway acknowledgment-throughput protocol

The current release uses HTTPS. Apply the transport policy and comparison rules in
[the HTTPS validation protocol](https-evaluation-and-validation.md). References to
WSS below describe the preserved comparison candidate, not the current binary.

## Question and scope

Measure whether the hotfix's ordered submission acknowledgment phase has enough
capacity for one production signing key. The sender waits for acknowledgment of
nonce N before submitting N+1, but waits for their receipts concurrently.

This protocol needs a Gateway endpoint and a small standalone driver, not a full
coprocessor deployment or database. It does not replace the mixed-operation,
retry, restart and fault-recovery release gates in
[the minimal hotfix plan](minimal-v013-hotfix-plan.md).

**Driver status:** the repository's `hotfix_load_gate` and pipelining tests create
Anvil/test fixtures. They are not external-endpoint benchmarks. Do not redirect
them at Conduit or assume their `LOAD_*` variables select an external endpoint.
The driver and timing hooks described below must be implemented on the test
machine before executing this protocol. This document specifies the experiment;
it does not add an executable benchmark or claim a result.

## 1. Inputs from infrastructure

- Production-equivalent Conduit Gateway WSS URL and authentication through the
  normal secret mechanism. Record a redacted endpoint identity, not credentials.
- Dedicated funded key, exclusively assigned to this experiment. No other sender
  or benchmark may use it until all submitted work has been reconciled.
- Confirmation of routing, RPC limits, sequencer forwarding and any differences
  from the operator endpoint. An HTTP endpoint is useful for observation, but
  measured submissions must use WSS.
- A test host/pod in the sender's deployment region and network path, with the
  same signer type (private key for the supplied production configuration).
- Agreed maximum transaction rate, RPC rate, total gas spend and test window.
  Include observation/receipt traffic in the RPC budget.
- For the later contract stage: deployed contract addresses, an authorized test
  identity and distinct valid inputs for both operations. Funding alone does not
  confer contract permissions.

Before starting, record the exact source SHA, driver patch/diff, lockfile hash,
build profile, host/region, signer address, chain ID, effective provider retries,
timeouts, connection settings and endpoint limits. Query the chain ID and verify
it is the intended network. Confirm the test account has no unexplained pending
work. A matching latest/pending count alone cannot exclude a request still in
transit from a previous run; reconcile that run first.

Use a dedicated ordinary recipient with no code for the transfer stage. Send zero
value if the endpoint accepts it; otherwise use a small fixed value and include it
in the spend budget. Estimate gas on this chain rather than assuming Ethereum's
21,000-gas transfer limit. Size funding from a pilot's actual fees, the planned
transaction count and an agreed margin.

## 2. Driver requirements

Reuse this branch's `NonceManagedProvider`, wallet and
`FillersWithoutNonceManagement`, with the production WSS connection/retry setup.
Do not add another nonce filler. Use one shared provider/nonce sequence per run;
clones must share it. Build in release mode. Keep fee filling, signing and actual
raw submission in their existing positions inside the nonce lock.

Two modes are required:

| Mode | Invocation and scheduling | What it measures |
|---|---|---|
| A: ordered-phase capacity | Call `send_transaction_sync(tx, 4s, 30s)` with a continuously replenished, bounded worker pool. Pre-estimate a transfer gas limit; leave normal fee filling/signing enabled. | Capacity with estimation outside the measured work; receipt waits remain concurrent. This method bypasses the admission semaphore, intentionally. |
| B: release provider configuration | Call `send_sync_with_overprovision(tx, 300, 4s, 20s, 30s)` with gas unset. Two producers each launch ten tasks, drain that batch, then offer the next; share admission 32. | Actual estimation and batch/admission effects, with at most 20 active attempts. Transfers still do not reproduce operation SQL/preparation costs. |

Pre-setting gas in A is an experimental control using the existing library API,
not a proposal to restore the removed static-gas CLI feature. Do not pre-sign all
transactions or pre-fill fees and call the result the hotfix's ordered-phase cost.

For A, sweep worker counts 8, 20, 32, 64, increasing further only within the agreed
load budget. A larger worker pool is diagnostic, not a release-setting change.
The nonce lock must have waiting work for a capacity measurement. If workers are
all awaiting receipts and the lock is idle, increase workers within the budget
or report that the experiment is receipt/worker-limited. A rate limiter hitting
the agreed ceiling likewise establishes a lower bound, not maximum capacity.

Use unique logical IDs for every attempted transaction. Capture signed transaction
hash and nonce before the network request, including requests that never return
an acknowledgment. In the clean capacity experiment, do not automatically create
a new transaction for a failed/uncertain transfer: retain its outcome as unknown
and reconcile it. Transfers do not have the contracts' duplicate-work checks.

Enforce separate ceilings for active attempts, acknowledged-but-unresolved work,
total attempts, wall time and spend. A receipt timeout releases an attempt but
does not prove its transaction disappeared. Stop offering work if the unresolved
ceiling is reached; do not let timed-out receipts silently create unlimited load.

## 3. Instrumentation

Add test-only timing hooks around the actual provider phases, preserving their
behavior. Instrumentation can live in a recorded benchmark patch; it need not be
part of the release. A timer around the public send method alone is insufficient.

Use monotonic timestamps for durations. For each logical attempt record:

1. Work offered; admission requested/acquired where applicable.
2. Estimate start/end, including failure or timeout.
3. Nonce lock requested/acquired.
4. Pending lookup start/end when the sequence is cold.
5. Submission wrapper start/end (includes fillers and signing).
6. Raw RPC request start and matching response/error/timeout; signing start/end
   and fee-RPC intervals where instrumentation permits.
7. Nonce lock released.
8. Receipt observed, receipt status/block/hash, or receipt-stage error/timeout.

Capture raw-RPC timing client-side on the measured WSS connection, through
middleware/hooks that retain structured transport errors. A separate instrumented
proxy changes the path; if used, measure its overhead and label those runs.
Never log keys, authentication or raw signed transaction bytes. Record hash,
nonce, logical ID, error category and sanitized error detail instead.

Compute separately:

- Lock queue time and lock hold time. The latter includes a cold nonce lookup
  when present; report cold/reseed samples separately from steady warm sends.
- Raw submission acknowledgment latency and complete submission-wrapper latency.
  If subphase hooks are unavailable, label wrapper time honestly; do not call it
  network RTT or isolate signing cost by assumption.
- Full preparation time: admission acquired through raw-send start. Also report
  work-offered through raw-send start to expose admission wait. Do not substitute
  estimate latency for either measurement.
- Acknowledgments/sec, successful receipts/sec, failures/timeouts by phase,
  active attempts, lock utilization, and acknowledged-but-unresolved count.
- Latencies as mean/p50/p90/p99/max, with timeout counts alongside them rather
  than silently excluding failed attempts from the conclusion.

Keep one-second samples plus per-attempt events. Maintain receipt observation
while sending; acknowledge that its RPC traffic is part of the workload. Use a
separate bounded observation connection for reconciliation so instrumentation
does not park behind a faulted sender connection. Record probe failures as unknown.

## 4. Execution sequence

### Pilot and accounting check

Send 20 transfers at 1/s. Verify chain ID, expected signer/recipient, successful
receipts, nonce/hash mapping and actual fees. Verify that at higher concurrency
the next raw request starts only after the previous acknowledgment, while receipt
waits can overlap. Confirm timing events and stop ceilings function before load.

### Clean capacity sweep (mode A)

For each worker count, warm up for 60 seconds and measure for 300 seconds. Ramp
offered load within the agreed ceiling until either the lock is continuously busy,
another resource limits output, or the approved rate is reached. Keep a ready
queue bounded; never spawn an unlimited backlog of tasks.

Measure only the declared steady window, with warmup, measurement and final drain
reported separately. After each run stop arrivals, join bounded active tasks, then
continue bounded receipt reconciliation before starting the next run.

Repeat the useful saturation point at least three times, preferably at different
times during the agreed window. Report the spread and observed slowest run;
neither is a guarantee of future capacity. Do not extrapolate 200 TPS from a
lower-rate test. If 200 TPS is a comparison objective, infrastructure must approve
enough load to test it directly.

### Release configuration (mode B)

Run the two batches of ten with admission 32 and the deadlines above. First offer
the required steady arrival rate; then maintain a ready backlog for saturation.
Use the same 60-second warmup/300-second measurement and at least three repetitions.
Keep per-producer counts, though transfers cannot establish real-operation fairness.

Repeat with representative valid Gateway calls when permissions/inputs are
available. Each call must represent fresh work; repeatedly submitting the same
already-completed input measures revert handling instead of useful capacity.
Record payload sizes, operation mix, gas estimates/limits/used and successful
contract outcomes. This stage requires fixtures, not the full upstream pipeline.

### Optional transport follow-up

Only after clean capacity is established, repeat the selected configuration while
closing the test client's WSS connection normally and abruptly. Keep the fault
and recovery windows separate. Do not reset shared infrastructure. Observe whether
the provider returns `BackendGone` and needs reconstruction rather than assuming
transparent reconnect. A fresh provider must not overlap the old sender on the key.

Ambiguous acceptance, retries of logical work and cold restart with pending work
belong to the broader recovery protocol. They must not be reported as validated
merely because clean transfer throughput or a reconnect succeeded.

## 5. Accounting and stop conditions

Stop new work on budget/rate-limit breaches, loss of observation, growing unresolved
work beyond its ceiling, persistent nonce errors or insufficient funds. Retain
logs and label the run limited/failed rather than discarding it as warmup.

At the end, stop arrivals and terminate/join sending tasks before the final
observation. Resolve each captured hash as successful, reverted, still pending,
or unknown; also record attempts definitely not submitted. A missing receipt is
not proof of rejection. If a nonce was consumed but the captured hash is absent,
inspect that signer's transactions to attribute the outcome. Do not automatically
resubmit an unknown transfer under a new nonce.

Set a reconciliation deadline before running. If work remains unresolved at that
deadline, save the ledger, mark accounting incomplete and avoid reusing the key
for another experiment until its state is understood. Pending-count differences
are supporting observations, not a transaction ledger.

Report both cohort results (outcomes for work offered in the measurement window)
and wall-clock rates (events observed during that window). Do not divide all
warmup/drain completions by a short measurement window. Useful output excludes
reverts, duplicate work and unresolved attempts.

## 6. Decision criteria

Agree required capacity before execution:

`required useful tx/s = normal arrival tx/s + backlog at restoration / recovery deadline`

For the historical nominal fixture this is `4 + 2048 / 600 = 7.4133 tx/s`.
Use actual production demand and actual backlog at restoration for the release
decision; arrivals during an interruption may increase the latter. Apply the
criterion per operation as well as combined when testing real calls. Agree a
capacity margin in advance; merely exceeding the target is not comfortable margin.

- **Ordered phase adequate in this environment:** repeated mode-A runs sustain
  the target plus agreed margin, successful inclusion keeps up, unresolved work
  does not grow persistently, and no errors are hidden by accounting gaps.
- **Ordered phase insufficient:** the nonce lock is continuously busy and useful
  capacity is below target, with lock timing accounting for the limit. Split raw
  acknowledgment, filling/signing and reseeding before attributing it to Conduit.
- **Different bottleneck:** the lock is often idle while workers await receipts,
  estimation, admission or the load limiter. This does not establish an
  acknowledgment ceiling. Compare A against B and report the observed constraint.
- **Release configuration adequate for tested workload:** B also meets the target
  and margin with stable outstanding work. Transfer-only evidence remains
  provisional until representative contract calls and recovery gates pass.

At 200 submissions/s, the average serialized budget is about 5 ms per transaction.
This is a budget check, not a prediction from median RTT. Prefer measured sustained
rates and lock utilization; do not fit a block-time capacity equation or infer
maximum throughput from only two points.

## 7. Deliverable

Retain the manifest, exact driver source/instrumentation patch, per-attempt event
ledger, one-second CSV, phase-specific summaries and final accounting. Summarize:

| Run | Mode/payload | Workers/batches | Offered rate | Ack/s | Successful inclusion/s | Lock hold mean/p99 | Raw ack mean/p99 | Receipt p99 | Final unresolved | Limiting resource |
|---|---|---|---|---|---|---|---|---|---|---|

State the measured answer explicitly: whether ordered acknowledgment has enough
capacity for the agreed target on this endpoint, what limits it, and what remains
unvalidated. Do not claim full release readiness from this experiment alone.
