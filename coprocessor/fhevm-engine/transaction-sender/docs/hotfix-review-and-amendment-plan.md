# v0.13 sender hotfix review and amendment plan

> Historical review and broader main/v0.14 backlog. The final v0.13 scope defers
> the new static-gas CLI and its retry-specific behavior; see
> [the current minimal plan](minimal-v013-hotfix-plan.md). Static-gas findings and
> recommendations below describe earlier candidates, not required release work.
> Proof expiry after six retries is intentional because clients time out and
> resubmit; retention redesign is not a prerequisite for this hotfix.

Reviewed 2026-09-09 at `50e0ab9e4`, against merge base
`8234edfec7a542ccdd819dbff82fcc792d1e7344` of `origin/release/0.13.x`.
Scope: the two sender commits, their tests, the operation loops, deployment configuration,
Gateway contract behavior, and the locally installed, lockfile-pinned Alloy provider 1.1.2.
Incident figures are supplied observations, not independently verified measurements.

Following the request to minimize the v0.13 release, the separate
[minimal hotfix amendment plan](minimal-v013-hotfix-plan.md) proposes the release scope and
explicitly accepted existing limitations. This full document retains the broader remediation
and validation backlog for main/v0.14; not every amendment below is a v0.13 release requirement.
The minimal amendments have since been implemented and compile-checked; their exact behavior
and outstanding runtime validation are recorded in that plan. Findings below describe the
original reviewed commit unless otherwise stated.

## Supplied production configuration (follow-up)

The user supplied the deployed argument list after the initial review. Treat this as the
configuration for this deployment, not proof that every incident operator used identical flags.
The apparent newline/quote around `--error-sleep-max-secs=4` is treated as a paste artifact;
check rendered arguments when preparing rollout manifests.

| Setting | Supplied deployment | Same arguments with this branch's binary |
|---|---|---|
| Signer | Private key | Private key; AWS signing is not on this deployment's path |
| Proof / ciphertext batch limits | 128 / 10 | 128 / 10 selected rows, with shared provider admission limit 8 by default |
| Static gas base | Unset | Unset: estimation remains enabled |
| Gas multiplier | 300% | 300%; adding base 250000 would give limit 750000 |
| Send timeout | 4 s via `--txn-receipt-timeout-secs` | 4 s; alias still selects the same send option |
| Estimation timeout | No separate supplied option | New default 20 s |
| Provider retries / interval | 4294967295 / 4 s | Same; unsuitable as an application-level deadline |
| Proof retry budget / deletion | 6 / enabled | Same; exhausted proofs remain disposable |
| Ciphertext limited-retry budget | 2147483647 | Same; effectively unlimited for this incident |
| DB pool / polling | 10 / 1 s | Same |
| Error backoff | 1 s initial, 4 s maximum | Same |
| Graceful shutdown | 8 s | Same; shorter than even one possible 20 s estimate |
| End-to-end histogram ranges | 0.1–60 s, step 0.1 s | Same; unsuitable for resolving the reported long tail |
| Gauge update interval | 10 s | Same; does not establish that metrics are scraped |

Consequences for the review:

- A binary-only upgrade bounds concurrent preparation and serializes sends, but **does not
  remove estimation**. Benchmark this exact configuration as its own candidate before testing
  static gas. The failed-receipt regression in finding 1 becomes a release blocker when
  static gas is enabled; it is not introduced by keeping this argument list unchanged.
- The multiplier uncertainty is resolved for this deployment. The maximum safe gas requirement
  across operations/payloads is still unvalidated; 750000 is a candidate, not a validated limit.
- AWS signing latency/failure cannot explain this private-key deployment. Retain those tests
  only for operators that actually use KMS. The incomplete preparation-error draining remains
  a code defect, but the KMS-specific trigger is not applicable here. The incident's AWS
  signing-warning timing marker is not a usable preparation metric for this deployment;
  use explicit phase instrumentation and verify which operators supplied those measurements.
- The very large provider retry budget reinforces the need for a bounded pending-nonce lookup.
  The 8 s shutdown grace also makes restart/ambiguous-outcome reconciliation a required test;
  draining task results alone cannot promise that every loaded batch finishes on shutdown.
- Use 128/10, admission 8, multiplier 300%, private-key signing, 4 s send timeout and 20 s
  estimation timeout as the exact branch-with-unchanged-arguments test. Keep the supplied
  baseline configuration for the old binary; it does not accept the new options. Then vary
  static gas, batch sizes and deadlines independently. Do not assume incident mitigations
  such as proof batch 10 or timeout 16 s are present in this supplied deployment.

**Decision: amend before production rollout.** The direction is useful, but this branch does
not yet establish recovery from an interrupted service with a backlog. In particular, static
gas changes retry semantics, nonce lookup can block both paths outside the new timeout, and
sibling cancellation remains possible during batch preparation. The load experiment below
is a release gate after these amendments, not evidence that the current branch is safe.

## What to keep, and what is conditional

| Change | Assessment |
|---|---|
| Seed from `pending` | Keep. Avoids deliberately ignoring visible pending transactions. Does not resolve ambiguous submission outcomes or inconsistent RPC backends. |
| Serialize allocation and submission | Keep as a conservative initial design. Holding the lock until a receipt is returned additionally caps throughput; that part needs a measured capacity gate. |
| Shared preparation semaphore | Keep. Bounds concurrent estimation across both paths, including when static gas is disabled. It does not bound all spawned tasks or proof-signature preparation. |
| Drain task results after errors | Keep, but complete it for preparation failures as well. |
| Static gas option | Useful mitigation, conditional on failed-receipt reconciliation and gas validation for every operation/input class. It is not enabled by default. |
| Separate estimation timeout | Sensible for the estimation mode. The specific 20-second default and send timeout must be selected experimentally. |

None of the supplied observations establishes that every part of the measured preparation
delay was `eth_estimateGas`. The correlation supports that hypothesis; controlled method-level
measurements are needed to distinguish RPC queuing, fee lookup, signing, and admission waits.

## Findings and required amendments

### 1. High: static gas bypasses the error path used to retire completed work

Sources: `src/ops/add_ciphertext.rs:74,134,175`, `src/ops/verify_proof.rs:147,213`,
`gateway-contracts/contracts/CiphertextCommits.sol:132` and
`gateway-contracts/contracts/InputVerification.sol:446` (Gateway paths relative to repo root).

Today, estimation can report `CoprocessorAlreadyAdded`, `CoprocessorAlreadyVerified`, or
`CoprocessorAlreadyRejected` as a decoded RPC error. Those branches retire the local row.
With static gas, the transaction can instead be submitted, mined, and return a receipt with
`status=false`. Both operation handlers treat that receipt as a generic retryable failure;
they do not recover the custom error. Non-retryable configuration errors and
`VerifyProofNotRequested` are affected by the same distinction.

Concrete trigger: transaction A succeeds but its response is lost; its row is retried at a
new nonce. The duplicate reverts on-chain. If the endpoint returns the failed receipt, the
ciphertext remains eligible for effectively unlimited paid retries, while a proof can exhaust
its six-attempt budget. This directly undermines the intended backlog recovery.

A local Anvil probe during this review installed an always-reverting contract and submitted
the same call through estimation and a signed raw synchronous send. Estimation returned RPC
error code 3 (`execution reverted`); `eth_sendRawTransactionSync` returned **no RPC error**,
receipt `status=0x0`, `gasUsed=0x520e`. This demonstrates the receipt distinction, not the
production Nitro endpoint's exact behavior or an end-to-end sender reproduction.

**Amendment:** add bounded reconciliation on ambiguous outcomes and failed receipts. On the
failure path, a bounded `eth_call` with the original sender/calldata and sufficient simulation
gas can recover recognized terminal errors; alternatively use authoritative sender-specific
contract state/events. Do not infer this node's completion from aggregate consensus alone.
Mark complete only on positive evidence. Preserve unknown cases for retry/reconciliation and
retain receipt hashes. Keep these extra RPCs off the normal successful path and under the
same load controls. Test the existing terminal/config-error cases with static gas both on
and off, including lost success responses and a DB update failure after successful mining.

### 2. High: a cold nonce lookup can stall both operations outside the send timeout

Source: `src/nonce_managed_provider.rs:158`, especially lines 167–170.

`next_nonce().await` holds the shared nonce mutex and calls `eth_getTransactionCount(pending)`
**before** `with_timeout` starts. The path is taken at startup and after every send error.
A non-answering lookup therefore blocks all following sends, regardless of a 4- or 16-second
send deadline. Provider retry/reconnection settings make relying on implicit termination
particularly inappropriate. Draining batches now means an operation also waits for that task.

**Amendment:** bound nonce lookup while holding the lock, preferably with an explicit nonce
RPC deadline or a clearly documented total active-attempt budget covering lookup and send.
Ensure all exits release the mutex/permit and leave uncertain state invalidated. Do not just
wrap the outer operation in a timeout that cancels a possibly submitted transaction without
recording uncertainty. Measure admission wait separately from active-attempt time.

**Regression:** blackhole only `eth_getTransactionCount`, after both startup and a send failure;
leave other RPC methods healthy. Verify deadline enforcement, bounded retries, shutdown, and
recovery of both operations after restoring lookup responses.

### 3. High: batch preparation can still cancel siblings

Sources: `src/ops/verify_proof.rs:302–355`, `src/ops/add_ciphertext.rs:376–409`.

Tasks are spawned incrementally. Later rows can still exit the enclosing function through
`?` before the new drain loop. In particular an AWS proof-signature error at `sign_hash().await?`
can drop an existing `JoinSet` while previous rows are sending. Malformed row conversions
also exit early. The proof signature happens outside the provider semaphore and is not bounded
by either new RPC timeout. Thus “all tasks are awaited” holds only once preparation completes.

**Amendment:** put each row's fallible preparation and send into the drained task lifecycle,
with bounded signing/concurrency, or collect preparation errors without leaving the function
until already-started tasks have drained. Replace row-data panics with handled errors on these
paths. Preserve the first error without losing a later fatal `BackendGone` classification.
Keep shutdown cancellation explicit: the existing 8-second grace period can still abort
a loaded batch, so restart reconciliation remains necessary.

**Regression:** block the first send after acceptance, fail a later proof-signature request,
then release the first send and verify its DB result is recorded. Repeat with invalid row
data in both operations and with shutdown during a populated batch.

### 4. Release gate: serialization and timeout advice are not capacity-validated

Sources: `src/nonce_managed_provider.rs:167–172`, `src/transaction_sender.rs:94–123`,
and `docs/prep-latency-and-nonce-hotfix.md` sections 2.2 and 4.

The mutex covers filling, wallet signing, synchronous RPC submission and receipt return.
Static gas still leaves Alloy's EIP-1559 fee estimation when fees are unset
(`alloy-provider-1.1.2/src/fillers/gas.rs:105–118`). Preparation is therefore not entirely local.
The semaphore is held while waiting for this mutex, so a large verify batch can also delay
new ciphertext work; it supplies no per-operation latency guarantee.

For successful calls with average lock occupancy S, the ideal total capacity is at most 1/S
transactions/second across **both** operations. Faults, signer latency, lookup and loop backoff
reduce it further. For illustration, if each success occupies the lock for one second, the
upper bound is 60 transactions/minute, before failures. A larger machine cannot remove that
serialization bound. A configured 16-second timeout does not impose 16 seconds on successes.

The old concurrent sender's observed error rate cannot be multiplied by a proposed serialized
timeout to prove 16 seconds is invalid: changing concurrency/timeouts changes the error rate.
Nor does proof inflow alone determine headroom; ciphertext inflow shares the same signer.

**Amendment:** withdraw categorical “4 or 6, NOT 16” advice. Compare 4/6/8/16 seconds with the
same workload and fault schedule. Log the effective timeout, multiplier, static gas, batch
limits, admission limit, build SHA, and redacted endpoint identity at startup. If measured
capacity does not provide recovery headroom, split ordered broadcast from receipt tracking
as a separately reviewed amendment, retaining bounded outstanding work and explicit handling
of uncertain broadcasts. Do not simply restore concurrent nonce allocation.

### 5. Release gate: one static gas number has not been justified for all paths

Sources: `src/bin/transaction_sender.rs:103`, `src/config.rs:3`, both operation builders,
and `gateway-contracts/contracts/InputVerification.sol:226–305`.

The same base applies to add, verify and reject. Verify accepts variable-length handles and
extra data and has different consensus/finalization branches. A sampled worst-case ciphertext
cost does not bound these other paths or rollup data-fee conditions. The actual code default
multiplier is **120%**. The supplied production arguments explicitly override it to **300%**,
so adding `250000` there gives **750000**. A fresh configuration using the code default would
instead give **300000**; rollout snippets should specify the multiplier explicitly.

**Amendment:** benchmark the deployed contract/proxy version, full supported payload envelope,
pre-quorum/finalizing/post-quorum transactions and applicable rollup fee conditions. Choose
operation-specific limits if one value is wasteful or unsafe; a common limit is acceptable if
validated. Validate nonzero limits and checked multiplication/conversion (the existing `as u64`
can truncate an oversized configured product). Log the final applied limits. Any rollout
manifest must explicitly pin the multiplier and the validated base value(s).

### 6. High for outage recovery: exhausted proofs remain disposable

Sources: `src/ops/verify_proof.rs:106–119,286–297`, `src/config.rs:54–56`.

This predates the hotfix, but six infrastructure failures still make an otherwise valid proof
ineligible and, with defaults, deleted. Fully draining failed batches can now record failures
that cancellation previously hid. It improves accounting without preventing exhaustion.

**Amendment:** retain exhausted responses with sufficient replay/user correlation data and
alert on them. A minimal change can retain the existing rows, exclude them from automatic
selection, and provide a reviewed requeue procedure after reconciliation; a new dead-letter
table is not required just to stop loss. Distinguish transport ambiguity from permanent
application failure, and bound retries/backoff during an outage. Disabling deletion alone
does not make exhausted rows eligible again. Verify CLI behavior when exposing an explicit
false value: the existing default-true clap bool must be tested, not assumed to support it.

## Limits of the incident explanation

- Within a single normal running process, each operation awaits `execute()` before polling
  again. Missing row claims do not by themselves cause that loop to reselect still-running
  tasks. Aborted-but-submitted transactions, restarts, or multiple processes are different
  cases and need reconciliation or ownership control.
- A WebSocket supports multiplexed requests; one socket alone does not prove they serialize
  on the wire. Measure outstanding methods and server/provider queuing before attributing
  all contention to the connection.
- A pending count is a snapshot, not an authoritative answer about an ambiguous send. A
  delayed original request can arrive after the lookup, and another backend can have a
  different pool view. Test these schedules. Scope a minimal hotfix to one active process
  per key with a consistent submission/nonce backend; otherwise add stronger coordination.
- No gaps among included nonces cannot rule out temporary submission gaps. A mined sequence
  naturally conceals gaps that were later filled.
- Reducing batch size helped in the supplied account, but configuration changes and upstream
  recovery overlap. This does not prove a universal minimum batch of 10 or an exact tipping
  point at 128.

## Implementation order and acceptance

1. Fix failed-receipt/ambiguous-outcome reconciliation and bound nonce lookup. Add small
   deterministic regression tests first; these do not need a large machine.
2. Complete task draining across preparation errors; bound signing and test cancellation.
3. Retain exhausted proofs and expose replay/alerting. Add effective-config logging and phase
   timing. Ensure `ConfigSettings.max_inflight_sends` is either applied at the library
   construction boundary or removed from there: currently the binary configures the provider
   separately, so setting this config field alone has no effect.
4. Establish gas bounds and replace the existing deployment recommendations with measured
   settings. Check rendered chart arguments, including the two aliases for the same send
   timeout; pin the actual hotfix image digest.
5. Run the load gates below. Publish raw results, configuration and fault schedules with the
   rollout decision. If receipt serialization misses capacity, amend the submission design.

## Larger-machine validation protocol

**Environment.** Use an isolated Gateway/Nitro deployment matching production, real v0.13
contracts and PostgreSQL schema, one sender key per process, and a JSON-RPC-aware WebSocket
proxy. The proxy must distinguish request delay before forwarding from response loss after
acceptance. A generic TCP bandwidth limit alone cannot reproduce those cases. Start with
private-key signing to isolate RPC behavior, then run the critical matrix with real AWS KMS
in a test account and injected signer delay/failure only for deployments using KMS. The
supplied production deployment uses a private key, so that is the primary release gate.
Localstack alone does not validate KMS latency. Use five identities and threshold three for
finalization/gas/quorum tests.

Provision enough CPU/RAM/disk for the pinned Rust build, DB, Nitro and workload generator;
record saturation so a generator or DB bottleneck is not blamed on the sender. Running the
sender-only DB workload avoids needing expensive new FHE computation for every load run.
Use valid captured/synthetic eligible rows whose Gateway requests, signatures and ciphertext
material state match; inserting arbitrary proof rows only measures reverts. Maintain an
independent manifest of every logical work item and its expected outcome.

**Build controls.** Compare baseline `8234edfec`, nonce/concurrency-only `08a21ced4`, current
`50e0ab9e4` with static gas off/on, and the amended candidate. Replay the same seeded workload
and fault schedule with fresh chain/DB state. First hold flags constant; then compare selected
rollout configurations separately so a batch-size change is not attributed to code.
Run the crate tests on the larger host with Docker, Anvil and solc available:

```sh
cd coprocessor/fhevm-engine
cargo test --locked -p transaction-sender --test nonce_sequence_tests
cargo test --locked -p transaction-sender --test overprovision_gas_limit_tests
cargo test --locked -p transaction-sender --test add_ciphertext_tests --test verify_proof_tests
```

Extend tests for the amendments; the current static-gas test only tests the helper's output,
and the current nonce tests cover sequential timeout/rejection, not mixed concurrent backlog,
stale pending views, task cancellation, or DB reconciliation. Gas sizing must use the actual
Gateway contracts, not the simplified Solidity mocks in this crate.

**Workload.** Obtain actual peak combined ciphertext/proof arrival rates and payload sizes.
Until available, report capacity curves rather than claiming production headroom. Exercise
proof backlogs of 0, 10, 128, 129, 1024 and 10000 with a simultaneous ciphertext backlog and
continuing arrivals; also exercise ciphertext-only, proof-only and mixed workloads. Verify
rows include accepted and rejected proofs, small and maximum supported payloads, and all
quorum positions. Sweep offered load through 0.5x, 1x and 2x the measured production peak.
Keep the offered rate externally paced, independent of sender completion.

Use batch sizes 10/128, shared admission limits 1/4/8/16, and send deadlines 4/6/8/16 seconds.
Avoid a full Cartesian explosion: screen healthy capacity, choose viable configurations,
then apply the full fault suite to finalists and boundary configurations. Run at least three
repetitions with identical seeds across builds. Suggested stages: 10-minute warmup, 15-minute
healthy baseline, 15-minute repeated faults, and recovery until drained or the declared
deadline. Separately test an outage long enough to exhaust the old six-attempt policy.

| Scenario | Injection and required observation |
|---|---|
| Estimate contention | Delay only estimates by 50 ms / 1 / 5 / 20 s; cap proxy upstream workers; measure method queues. Static mode must issue zero `eth_estimateGas` calls during normal sending. Estimation mode must respect admission and its deadline. |
| Fee and signer cost | Independently delay fee RPCs, wallet signing and proof signing. Identify costs left after static gas and confirm bounded recovery. |
| WS interruption | Graceful normal close every 60 seconds, abrupt close, and unanswered keepalives; continue arrival load. Confirm reconnect/recovery and no lost logical work. |
| Ambiguous success | Accept/mine, then suppress the send response; also lose the response before mining. Recover the original outcome without an unbounded duplicate/revert loop. |
| Definite rejection/gap | Reject nonce N before acceptance while many tasks wait; then restore. Subsequent sends must not create a persistent hole. |
| Delayed original/stale pending | Hold a raw request until after the timeout and pending lookup; release it later. Route lookup to a stale backend; also evict a pending transaction. Observe reconciliation or declare the topology unsupported. |
| Nonce lookup blackhole | Drop only pending-count responses after an error. Both operations must recover after the deadline/fault recovery. |
| Sibling failure | Fail a later signer/preparation task while earlier work is accepted. Every earlier result must be recorded or explicitly reconciled. |
| Restart/DB failure | Fail DB persistence after mining, SIGTERM during backlog, and restart with accepted pending transactions. Reconcile all items. Verify deployment prevents overlapping owners of a key. |
| Reverts and gas | Already completed/not requested/config errors, deliberate OOG, maximum payload, quorum-finalizing calls, and rollup fee variation. Distinguish each outcome; none may become an indefinite hot loop. |
| Long outage | Continue arrivals beyond the old retry budget, then restore. Retain/requeue exhausted proofs, alert, and drain both work classes. |

**Instrumentation.** Record per-operation distributions for selection-to-terminal-result,
admission wait, proof signing, gas estimation, nonce-lock wait/hold, pending lookup, fee
filling, wallet signing, RPC submission/receipt response, and DB persistence. Use monotonic
clocks for durations. Record method request counts, outstanding requests and error classes;
logical attempts, known hashes/nonces, receipts and receipt status; ready/exhausted queue
counts and oldest age; successful distinct work/sec and retries per distinct work item.
Use bounded metric labels (operation/outcome); put hashes and proof IDs in structured logs.
Include failures/timeouts in observations, not only successful samples. Scrape continuously
and fail the test when telemetry is missing. Use buckets extending to the intended recovery
window, with useful resolution for both subsecond phases and hours of backlog.

**Pass/fail gates, declared before each run:**

- Correctness: every input-manifest item is completed with positive chain evidence or retained
  in an explicit actionable failure state. No silent proof loss, unexplained pending nonces,
  or unresolved DB/chain discrepancies after recovery. Count reverted/duplicate transactions
  as attempts, not completed useful work. During shutdown, temporary uncertainty is allowed;
  indefinite uncertainty after recovery is not.
- Capacity: after faults stop, each operation sustains output above its continuing input rate
  while backlogged. Target at least 2x measured peak input for each operation (proposed recovery
  margin, to be agreed against the service recovery objective). If backlog B must drain in D,
  require output >= input + B/D. Evaluate over stable multi-minute windows, not two samples
  or a requirement that every individual lag observation decreases.
- Fairness: report ciphertext latency and useful throughput while the proof queue stays deep;
  reject configurations that starve either class even when aggregate throughput looks good.
  Set the absolute latency/recovery SLO from production needs before choosing a winner.
- Fault containment: outstanding work stays bounded, nonce lookup respects its deadline,
  and request/retry rate returns to the healthy range after recovery. No repeated paid
  duplicate-revert loop. All retained exhausted proofs have an alert and a tested requeue path.
- Static gas: no estimates on the ordinary send path and no unexpected OOG throughout the
  supported payload/fee envelope. Use trace/replay evidence to classify OOG; a failed receipt
  with `gasUsed/gasLimit >= 0.95` is **not** a reliable OOG classifier. Report gas usage by
  operation/quorum/payload. A fixed limit does not imply a fixed 3x realized multiplier.
- Phase latency: use the supplied <0.5-second preparation p50 as a provisional diagnostic,
  measured with an explicit phase definition. Also gate p95/p99 queue wait and end-to-end
  recovery against the SLO; low preparation latency alone cannot pass a serialized sender
  with an ever-growing admission queue.

If the baseline fails to reproduce the reported signature, record which mechanism each
synthetic test covers. Do not present an artificially slow estimate alone as reproduction
of the entire incident, or a healthy idle test as backlog validation.

## Rollout and remaining inputs

Canary a single operator after gates pass, preserving quorum margin according to actual fleet
health. Confirm a single active sender per key, explicit validated gas/multiplier/timeouts,
working scrape/alerts and proof retention. Observe a representative peak and induced staging
recovery before expanding. Roll back code/config on growing queues, repeated failed-receipt
loops, unexpected OOG or reconciliation failures; preserve DB state and reconcile pending
transactions before changing ownership. Do not delete queue state to obtain a clean canary.

Supplied flags are recorded above. Remaining inputs: image digests and rollout strategy,
any differences across operators;
peak combined arrival rates and recovery SLO; largest supported proof payloads; deployed
Gateway/Nitro versions and endpoint pending/sync semantics; ability to run an RPC fault proxy
and, for KMS-using operators, test KMS; whether keys are shared across replicas or other services. These are measurement
inputs, not prerequisites for fixing findings 1–3.

Validation performed in the initial review: source/diff inspection and the local Anvil RPC probe
described above. No Rust suite or backlog benchmark was run: the workspace filesystem had
approximately 1.1 GiB free, insufficient headroom to assume a safe Rust integration-test
build. The branch document's earlier `cargo check` claim is not a fresh result from this
review. That initial pass added documentation only. The subsequent minimal implementation
and successful compilation-only check are recorded in the minimal plan; runtime validation
of those amendments remains outstanding.
