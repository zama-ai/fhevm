# Consensus campaign runbook

This campaign checks agreement, correctness and bounded recovery across
case-specific coprocessor topologies. Byte agreement is within one software,
backend and hardware class. The [inventory](inventory.yaml) is the
case contract; the [workflow](../../../.github/workflows/test-suite-consensus.yml)
defines which backend CI requires for each case. A green campaign covers that
inventory at the recorded revision and topology, not every failure mode of the
coprocessor fleet. Campaign results are recorded separately from these contracts;
see the [validation notes for e9180f14c](VALIDATION-e9180f14c.md) for the older
13-case campaign. Those results do not validate this branch's expanded cases.
The [delivery ledger](COVERAGE-DELIVERY.md) maps the audit packages to prepared
checks, declared exclusions and pending execution.

## Local object-store routing

The managed stack discovers MinIO through its published port on the Docker
IPv4 bridge gateway. A stopped container releases its leased address, which a
restarting worker can acquire before MinIO returns. The gateway endpoint stays
stable across that sequence and is reachable from both the host and containers.
Its numeric address preserves path-style S3 requests in released SNS workers.
Discovery rejects an ambiguous gateway or a port published only on host loopback.
Existing saved discovery keeps its original address; use a fresh stack to
validate this routing behavior.

## What the cases establish

| Cases | Checks and evidence | Limits |
| --- | --- | --- |
| MAT-01 | Same-block and cross-block dependencies, fan-out, raw bytes, compute/SNS digests, key identity, provenance, exact completed transaction counts, and authorized gateway quorum. | Agreement is within one execution class. Equal bytes alone do not establish plaintext correctness. |
| MAT-02 | Same-sourcing aliases retain one canonical value and storage row despite multiple producers; mixed sourcing produces distinct handles. | The fixture exercises a selected graph, not all operation/alias combinations. |
| MAT-03 | Fixture outputs decrypt to expected arithmetic values. | This checks the fixture's values through the relayer/KMS path, not general cryptographic correctness. |
| MAT-05 | A fixed manifest of named comparator and canary contracts must appear exactly once in Mocha's passing results. Missing classes, skipped tests, or unrelated passing tests cannot satisfy it. | Synthetic comparator evidence and mocked database interactions; it does not substitute for the live canaries. |
| REORG-01 | Receipt-identified replacement blocks differ, preserve sourcing, are ingested by every operator, and retain matching bytes and authorized quorum. | Controlled local Anvil replacement, not arbitrary reorg depth or production network behavior. |
| FORK-01 | Different branch blocks share the EVM-visible handle preimage, mint the same handle, agree on bytes/digests, and reach distinct authorized-member quorum. | Cross-branch provenance is deliberately excluded: producing blocks/transactions legitimately differ. |
| FORK-02 | Successful branch receipts mint different handles. Canonical operators pass a later sentinel without acquiring the fork-only handle. | Absence is bounded by observed ingestion progress; it is not a claim about all future history. |
| FORK-03 | The forked operator first observes an orphan-only allow, then ingests the receipt-identified block containing a fresh sentinel on canonical history and completes that exact transaction/handle. The orphan handle has no quorum during the observation window and canonical decryption rejects it at the ACL check; a canonical positive control decrypts correctly. | The block check identifies the later sentinel block, not the initial replacement block. Does not claim that orphan ACL rows are physically deleted, or test stale-child repair/replay. F2 supplies the orphan handle, but F3 must produce its own recovery evidence. |
| SCH-01 | All materialization checks run under deliberately different scheduling settings. Running configuration, executed batch/window measurements and single-versus-overlapping GPU permits must demonstrate diversity; bytes/digests must still agree. | One GPU is sufficient. No multi-device coverage, throughput guarantee, or exhaustive scheduling interleaving coverage is claimed. |
| REG-01 / REG-02 | Selected Rust regressions exercise listener pool rebinding/reconciliation and daemon error exit behavior. | These runs do **not** revert production fixes. They make no mutation-sensitivity claim. |
| HAR-02 / HAR-03 | Inventory/result aggregation and readiness contracts. | Unit/contract coverage does not replace a fresh stack boot or live ownership/readiness checks. |

Materialization and FORK-01 each run two live canary arms through their own
comparison. The digest arm corrupts the stored compute digest and must fail as
`compute-digest`. The raw-byte arm changes one operator's ciphertext **and**
recomputes its digest, preserving local hash validity; it must fail as
`raw-bytes` for that handle. A timeout, database error, wrong mismatch class, or
unrelated handle cannot satisfy the raw-byte control. Each arm requires clean
agreement before mutation and restored agreement afterward. Mutation waits for
the victim operator's own publication, not just another operator's quorum.

Neither an assertion name nor a PASS marker proves a property by itself.
Stackless records summarize the selected executed contracts; live records consume
run/case-bound assertion receipts. Review new receipts at the assertion site and
add a negative control when extending a claimed property.

## CPU, GPU, and selection boundaries

CI requires CPU records for MAT-01/02/03, REORG-01, and FORK-01/02/03. **SCH-01, SCH-04 and
GPU-01/02/03/04 require GPU execution.** The byte and plaintext fixtures run inside SCH-01,
but their results are not separately recorded as GPU MAT cases from that
heterogeneous session. Homogeneous local GPU runs can record GPU MAT/reorg/fork
results where the inventory permits them; these do not replace the backend
required by `aggregate --ci`.

CPU and GPU ciphertexts are not compared against each other. Each run checks
agreement within its class and uses expected plaintext as its correctness
oracle. GPU squash format is checked explicitly (format 21); matching format
alone does not establish byte agreement. Host GPU units replace the Docker
TFHE/SNS/zkproof consumers; setting `CUDA_VISIBLE_DEVICES` on CPU binaries would
not constitute a GPU run. Retain the binary/build manifest and observed device
and worker activity evidence.

`smoke` selects CPU byte agreement, typed boundaries and verified inputs.
`standard` adds harness, Rust regressions, CPU scheduling, bridge, fork, degraded,
crash-retry, service/database faults, backlog, host RPC, broker redelivery, object
integrity, storage backpressure and submission partitions. `full` also includes
GPU scheduling, reservation pressure and lifecycle cases. CPU stack jobs and GPU provisioning always
wait for successful harness checks, including stack-only selections whose
requested coverage does not include the harness. A Rust-regression-only
selection remains independent. `full` with `build=false` is PARTIAL by design:
published production images cannot validate this checkout's production changes.
The workflow still builds this checkout's test-suite image in published mode.
The ordinary container-based GPU E2E workflow is separate from this host-worker
consensus campaign.

These failure suites are part of this branch's inventory. Long operator suites,
blue/green and stateful key-migration rollouts remain separate campaigns; an
inventory `full` PASS does not validate them. SCH-02 multi-device attribution
remains deferred and is not required by single-GPU CI. SCH-03 now requires a
generated-key CPU squash regression; the old extracted-key experiments remain
optional diagnostics.
Optional probes also have limits: when the E2E container cannot access Docker,
the runner announces that it omits attestation readiness; it still runs the
byte/digest, quorum and plaintext checks. That omission is not attestation
coverage.

## Preparing and running a campaign

Use a dedicated local stack and retain its resolved version lock. From
`test-suite/fhevm`, inspect scope before provisioning:

```sh
bun scripts/consensus-inventory.ts validate
bun scripts/consensus-inventory.ts plan full
```

Use each case's declared topology: most materialization and fault cases use
three operators and threshold three, majority/partition cases use threshold
two, and the bridge case uses two operators and two host chains. Blue/green
acceptance uses three operators, threshold two and two host chains. Runners read
the active generated state and inspect running listener,
poller and consumer routes; live suites read gateway membership and threshold.
Environment labels must agree with the case and that evidence. A deployment
with the wrong threshold or a fork
label applied to canonical-only listeners cannot be relabeled into coverage.
The fork scenario routes operator 2 to `fork-anvil` and isolates its Redis
consumer from the canonical stream.

For source validation, follow the workflow's `checkout-build-receipt.ts`
begin/build/finish sequence around `fhevm-cli up --build`. Record running image
identities with `record-run-identity.sh`, attach the build receipt, and pass the
result through `CONSENSUS_ARTIFACT_IDENTITIES_FILE`. A checkout SHA alone is not
runtime provenance. Registry authentication or the documented local
`--e2e-public-runtime` option is needed for the selected build bases. Rebuild the
test-suite image after suite edits: E2E source is baked into the image, not
mounted from the checkout, and runners reject a source mismatch.

Run the applicable commands on the corresponding source-built topology:

```sh
# Active scenario: three-of-three
./scripts/run-materialization-consensus.sh --suite materialization
./scripts/run-materialization-consensus.sh --suite reorg

# Active scenario: three-of-three-fork
./scripts/run-fork-consensus.sh --case main
```

For SCH-01, boot `three-of-three-heterogeneous-scheduling`, then use
`gpu-consensus-workers.sh build` and `start`. Match the workflow's per-operator
window/component settings (100/20, 1/1, 200/64) and stream budgets (4, 1, 2);
scenario boolean tuning is carried
into each unit's environment. Explicit launcher overrides are appended to a
private per-unit environment file after the inherited settings; systemd and the
scheduling report use that same resolved file. This avoids `EnvironmentFile`
overriding values supplied through systemd's `--setenv`. Restart uses the saved
unit tuning and host environment, not the caller's current overrides. The
launcher checks this handoff. Run
`run-materialization-consensus.sh --heterogeneous --suite materialization` and
finish with `gpu-consensus-workers.sh stop`. Three independent operators can
share the selected GPU. `fhevm-cli down` alone does not stop host systemd workers.

Use a distinct run ID for each execution. Preserve structured results, suite
logs, topology, runtime identity, scheduling evidence and final cleanup outcome.
For CI-equivalent acceptance use the workflow's aggregate arguments, including
`--ci` and the checkout build requirement. For extra local backend runs, aggregate
against their actual backend without claiming CI coverage. Do not combine old
revision records with new ones to fill missing cases.

## Failure attribution and recovery

Results remain staged until EXIT cleanup finishes. Finalization preserves
already-staged sibling PASS records when a separate FAIL/INVALID/NOT_RUN
represents the runner's error and shared gates and cleanup succeeded. The fork
runner uses this for individually attributed case failures. Materialization
treats any nonzero suite exit as a shared failure and marks all its cases
unsuccessful; it does not preserve passing bodies from that failed suite.
Materialization can preserve siblings when the suite exits successfully but a
case marker is missing and gets its own NOT_RUN record. The overall run still
fails in either situation. A failed shared hook, validity gate, refused record,
or failed recovery invalidates provisional sibling passes.
A killed process that never publishes results is missing coverage, not success.
F1, F2 and F3 carry separate workload, block and observation-time receipts; F1's
receipt cannot stand in for either later case.

Canary originals are journaled durably under row locks before mutation. The raw
arm restores ciphertext and digest together and verifies both before deleting
its journal. Mining settings are also journaled. Stop suite processes before
replaying recovery, retain journals after failed cleanup, and recover before
reusing the stack. Journals contain connection URLs and must not be uploaded as
public artifacts or removed merely to unblock a run.

When validating changes to these checks, include negative paths: stopped fork
operator after F2; replacement ingestion without completed sentinel work;
missing comparator class despite many unrelated passes; consistently rebound
raw-byte poison; wrong deployed threshold; missing case receipt; and shared-hook
versus individual-case failure. Unit tests for these paths are useful but do not
replace live execution of the revised suites and a fresh CPU/GPU boot.

## Changes visible to ordinary CLI users

Queue ownership preflight runs for ordinary `fhevm-cli up`, not only consensus
runners. It checks Docker and local TFHE/SNS/zkproof consumers. If a host worker's
database cannot be identified from its command line or `/proc/<pid>/environ`
(for example, because process information is unreadable), ownership cannot be
established and `up` fails closed. On a shared machine, another user's worker can block
startup even when it might belong to another stack. Use an environment where
ownership is inspectable; do not interpret that refusal as a consensus failure.

Coprocessor services use `restart: on-failure:10`. Readiness rejects a nonzero
container restart count, even if the container happens to be running when
polled. A single boot-time crash is thus a failed `up`, not a recovered ready
service. Review its logs and restore a clean service lifecycle before retrying;
healthy uptime does not itself renew the restart budget. These are shared CLI
semantics and should be stated in the PR description, alongside the CI backend
and `build=false` limitations above.

## Failure-campaign review gates

Crash-retry checks authorized Gateway quorum for **every** recovered target,
including the cross-block dependent. Completion of only the producer cannot
satisfy CR-01/02/03. REG-03 shares CR-01's workload and requires a failpoint build
even when selected alone; these IDs do not count as independent executions.

Before a fault, the crash, matrix, request-recovery and degraded runners reconcile
the generated scenario with running listener routes and deployed Gateway
membership/threshold. A reused 2-of-3 stack cannot publish a 3-of-3 result.
DEG-06's generated configuration is recognized only with its three explicit
auto-revert overrides; its normal Gateway quorum still has to be observed.

FM-UPGRADE-CONTROLLER remains a Rust/database subprocess regression, not a live
blue/green interruption campaign. Its verdict requires the exact successful
`tests::cutover_recovers_after_process_kill` result as well as the controller
suite's execution floor. Ordinary passing controller tests cannot substitute
for a skipped process-kill construction.

## Verified-input agreement


`./scripts/run-materialization-consensus.sh --suite input` runs INPUT-01/02/03
on the homogeneous `three-of-three` profile. CPU CI executes it in the
byte-agreement leg. A local homogeneous GPU invocation is supported but has no
GPU execution claim until its own campaign is recorded.

Each compact list contains zero, maximum and repeated maximum for one of ebool,
euint8/16/32/64/128/256. The largest list contains 768 bits, within the 2048-bit
SDK CRS capacity. The oracle reads input storage directly before contract use;
it checks the exact row set, bytes, type/version and public handle derivation
from blob hash, index, ACL and chain. It does not require an SNS or computation
output row. Public Gateway acceptance must name the exact handle list and have
an authorized distinct-sender quorum. Every input is then authorized by a real
contract call and decrypted against the independent list.

The replay arm submits the original encoded blob through Gateway again and
requires stable, unique input storage. An independently encrypted equal list
must establish its own acceptance/byte agreement and correct plaintexts. The
negative arms submit a truncated serialized proof and the original proof with
a different user binding. Only an explicit authorized rejection satisfies the
negative; timeouts cannot pass. Existing material must remain unchanged. Direct
Gateway controls require the isolated profile's zero fee policy and deployer
credentials. No private key or raw proof is printed in results.

Installed key IDs must agree before the input run and remain unchanged. The
adapter does not parse key metadata from TFHE serialization; successful
independent decryption supplies the usable-key check. Input continuity across a
protocol upgrade is a separate upgrade campaign. This is a single-chain matrix,
not an exhaustive mixed-type/maximum-size proof stress test.

These tests have static/unit validation only; live acceptance remains pending.

## Typed transaction boundaries


`./scripts/run-materialization-consensus.sh --suite typed` runs
MAT-06-TYPED-BOUNDARIES. Each width has a local graph and a later-block graph
reading the persisted producer. Integer arithmetic (including wrap, reversed
subtraction, multiply, scalar divide/remainder), shifts and rotates stop at
u128 as required by the production operation table. u256 uses supported
bitwise, comparison/select and truncation. Boolean outputs cross their own
transaction boundary. Every intermediate has an exact computation count,
receipt/chain/block provenance, fleet byte/digest agreement, authorized quorum
and independent plaintext expectation. This is a pairwise matrix; it does not
claim every type/operator/operand-sourcing combination or an exact
rerandomization-transcript security proof.

## Bridged dependencies and transcript sensitivity


`./scripts/run-materialization-consensus.sh --suite bridge` uses the existing
confidential bridge flow on `two-of-two-multi-chain` (the CPU `bridge` CI leg).
It produces a computed source, consumes it locally, bridges that same source,
and consumes the destination handle in a later transaction. The association
oracle requires the exact source and destination receipts, canonical finalized
headers, approval/association identities, independent public handle derivation,
and identical source/destination serialized bytes within each operator and
across the fleet. It intentionally does not invent a producing computation for
the association. Both actual computations must complete atomically, reach quorum
and decrypt to their independently calculated values. The isolated mock LayerZero
endpoint is required; this is not evidence for real LayerZero transport latency.

REG-04 exercises the production Rust rerandomizer with fresh keys. Fixed inputs
and transcript repeat byte-for-byte; changing the handle, opcode, ciphertext
order or ciphertext representation must change the result. Plaintexts and scalar
operands remain unchanged. Together with MAT-02 and MAT-06, this covers public
alias/operand sourcing plus a lower-layer transcript sensitivity check. REG-04
is a Rust regression, not an additional full-stack execution. No cross-protocol
or CPU/GPU ciphertext equality is required.

## Blue/green continuity and quiet hosts

Blue/green tests use the contracts compiled into the E2E image. The background
traffic streams and retained-material probes share its artifact directory and
must run with Hardhat compilation disabled. `hardhat test --no-compile` also
skips the custom test task's compilation steps, so concurrent streams cannot
prune fixtures needed by later continuity checks. Rebuild the E2E image after
changing contracts; do not compile into that directory during a campaign.

The successful proposal records committed `DryRunStarted` transitions in a
transactional test audit before polling. Every operator must provide a transition
for both configured hosts, the candidate version, proposal 2, and the same proposal
block still identified by its upgrade state. This evidence survives fast promotion;
a `LIVE` state alone is insufficient. The observer does not delay cutover, and its
trigger and table are removed on success or failure. Application traffic started
after this observation may run after promotion when cutover is especially fast;
retained pre-proposal values and post-promotion computations remain required.

The ordinary `./fhevm-cli test blue-green` profile now keeps a separate
receipt-identified pre-cutover mint on every configured host chain. It requires
a dependent transfer on each chain after promotion is observed and verifies
the model balance. The earlier stress transfers remain supplementary: they
cannot substitute for the post-promotion transfers or silently reduce the
required number of chains.

For the quiet-chain success arm, boot
`blue-green-two-of-three-multi-chain`, then run:

```sh
BLUE_GREEN_QUIET_SYNTHETIC=1 ./fhevm-cli test blue-green
```

This retains the existing synthetic-disabled timeout/reset control, then
allows no application traffic during the successful dry-run window. Gateway
input traffic exercises its track; synthetic Gateway input can also anchor it.
Each host track must have recorded synthetic transaction IDs whose three
computations completed successfully in Green. A test-only statement trigger
records the actual computation rows deleted by cutover, together with that
track's full marker list, in the same transaction. This evidence survives
promotion clearing the markers and dropping Green; it does not depend on a
poll winning a race with promotion. Missing, incomplete, or errored work cannot
satisfy the check. Audit objects are removed on success or ordinary failure,
and a cleanup error fails the command. Promotion and the
identified post-promotion transfers must succeed on both host chains. The
mode rejects a topology other than three operators, threshold two and two host
chains. It is a separate, stateful, single-use profile; it is not counted as
another case in the consensus workflow's `full` selection. Report faults and controller interruption require separate isolated runs;
their prepared arms and the compatible-release checks are documented below.

## Upgrade interruption boundaries (separate stateful runs)


Build Green with `FHEVM_CONSENSUS_TEST_FEATURES=upgrade-controller/test-failpoints`
and boot `blue-green-two-of-three-multi-chain`. On a fresh stack, run
`BLUE_GREEN_INTERRUPT=before-cutover-commit ./fhevm-cli test blue-green`.
The other supported values are `dry-run-started` and `after-cutover-commit`;
each needs its own fresh stack. The controller reports its compiled protocol
version and hook capability before the profile arms a fault. Production builds
exclude the hooks.

The supervisor installs a version-bound, once-only control in operator 1,
waits for the actual controller acknowledgment, checks the durable live version
on the appropriate side of commit, kills that process and requires automatic
supervisor replacement. Acknowledged controls are skipped by the replacement.
The ordinary real-stack upgrade assertions then require all operators to reach
LIVE, retire the old role and preserve the identified dependent application
work on both chains. The ledger and boundary receipt survive failure; parent
EOF clears the control and restores service ownership. An absent hook, wrong
commit side or missing replacement fails the profile.

The post-commit arm proves interruption after durable promotion. It does not
claim to freeze every listener before it consumes the atomic notification;
that ordering is not under the controller's ownership. These stateful upgrade
runs remain outside the consensus workflow inventory. Their live execution,
like the other new feature cases, is pending.

## Key migration: lagging recipient and candidate selection


The existing stateful rollout accepts the optional
`RFC029_MIGRATION_FAULT=lagging-recipient`. It stops the second operator's Green
host-listener roles before requesting compressed material for the existing key.
It requires generation and operator 0's activation to complete while the held
operator still has legacy-only material and no matching activation. Releasing
the hold must let that operator apply the original activation through normal
catch-up. Existing final checks then compare material/event bindings, unchanged
key IDs and legacy bytes, restart the workers and decrypt through legacy serving.

Run this only on a fresh rollout stack:

```sh
RFC029_MIGRATION_FAULT=lagging-recipient ./fhevm-cli rollout run \
  rollouts/v0.14-to-v0.15-gpu-key-migration/run.ts
```

The outage is owned by a child supervisor whose stdin closes if the CLI exits;
normal release, failure and parent exit restore the stopped services. A durable
ledger under the rollout directory remains available if recovery fails. This
arm covers delayed application/catch-up, not a mid-download kill, malformed
material, duplicate activation or GPU consumption of compressed material.
The force-legacy safeguard remains in place.

Historical rollout defaults are still historical. Set
`RFC029_ACCEPTANCE_MODE=candidate`, an explicit full `RFC029_TARGET_FHEVM_SHA`,
and an explicit `RFC029_BASELINE_FHEVM_TAG` for candidate work. Candidate mode
refuses the historical target and implicit baseline. Record the baseline's
resolved source and image digests: supplying a tag does not prove it represents
the latest release branch. The rollout receipt, resolved locks and actual
locally built Green revision must agree before claiming candidate acceptance.

## CPU scheduling diversity

SCH-06 reuses SCH-01's bounded backlog, byte/model oracle and actual persisted
batch/window measurements on CPU. Run `./scripts/run-materialization-consensus.sh
--heterogeneous` on `three-of-three-heterogeneous-scheduling` with CPU workers.
This is execution-policy diversity, not a fairness bound or proof that GPU
reservation pressure was exercised. It requires no second GPU.

## Bounded replay, backlog and combined recovery

The CPU `backlog` leg boots `three-of-three-backlog`, where the live worker
acquisition limit and poller page limit must both be four. FM-HOST-LONG-OFFLINE
isolates all redundant victim ingestion paths, submits twelve receipt-identified
transactions spanning at least three pages, then restores the poller from its
durable cursor. FM-DURABLE-BACKLOG instead proves all twelve computations are
persisted and pending before killing the held worker and requiring supervisor
replacement. Both require every original output to agree, remain unique,
reach quorum and decrypt to 12; fresh work must also succeed. Neither case is
a throughput benchmark or a claim about broker delivery attribution.

FM-STORAGE-WORKER-RESTART extends the noisy-storage case: an actual upload
failure must occur, then the SNS owner is killed and automatically replaced
while storage is still stopped. Only then is storage restored, and the original
work must become retrievable and decryptable. It shares the existing restoration
ledger and deadline. This is one bounded combined fault, not a chaos matrix.

Gateway-specific transport/reorg expansions are omitted at the user's request.
The current protocol's input verification and ordinary quorum completion remain
necessary to drive and validate application work.

## Produced-object integrity

The CPU `object-integrity` leg runs `STORAGE-01-INTEGRITY` on a fresh
three-of-three stack. Five separately produced, never-decrypted targets cover
missing objects, truncated bodies, another real handle's object/attestation,
a changed key ID, and a changed format. Every serving bucket is changed so a
healthy replica cannot hide the fault. A durable journal retains original bytes
and all metadata before the first mutation. Public GETs confirm injection;
handle-attributed rejection logs and the original request state are required
before restoration. Missing objects and invalid signed attestations are retried
by relayer readiness while the request is queued. Truncated bytes pass the
attestation check and fail digest verification in KMS after submission. Every
mode must recover the same accepted request without another POST and decrypt
to the independent model. Invalid signatures are discarded and may be replaced
on a later round; the terminal disagreement path instead requires conflicting
valid attestations and is not exercised here. The runner uses explicit MinIO
metadata headers because `mc --attr` strips quotes from JSON attestations.

Wrong-key and wrong-format arms intentionally invalidate the signature. They
prove that unsigned metadata substitution is rejected, not that a malicious
signer with a valid key cannot attest to bad material. This covers the current
RFC023 object layout, not compatibility with an older release's layout. Cleanup
restores and reads back every original, including after partial injection.
Failed recovery preserves the journal and fails the run; discard the isolated
stack until recovery succeeds. Live execution of these new arms is pending.

## Single-GPU reservation pressure

SCH-04 uses `GPU_CONSENSUS_TEST_FAILPOINTS=1` when building/starting the host
workers. CI opts into this feature only when SCH-04 is selected; a standalone
SCH-01 selection retains production features. The compiled hook reads an
expiring per-process zero-admission budget under `/tmp`; it neither allocates
memory nor fabricates an execution error. The ordinary reservation loop must
acknowledge a real nonzero admission and reach its real deadline. The worker's
retry log must identify the target output. Healthy operators complete the same
work while the victim retains a pending, error-free computation without stored
output. Removing the budget must recover that original output and fresh work,
with fleet byte agreement, ordinary quorum and independent plaintexts.

Run `./scripts/run-gpu-pressure.sh` on the existing heterogeneous scheduling
scenario after starting hook-enabled GPU workers. It is single-device coverage,
not an OOM experiment or proof of fairness within one fully blocked reservation
pool. The control expires in at most ten minutes even if the runner dies; a
changed worker PID, missing acknowledgment, or expired negative checkpoint fails
the case. Admission/retry receipts survive for CI artifacts. Live execution is
pending, including confirmation of the configured reservation/lease margins.

## Key application interrupted inside its transaction

`RFC029_MIGRATION_FAULT=application-interruption` selects a separate fresh
migration rollout. A private, key-scoped PostgreSQL trigger holds the real
compressed-key UPDATE after download and validation. An independently visible
ungranted advisory lock identifies that boundary; an uncommitted trigger row
would not suffice. The selected material must still be absent to other readers.
The supervisor kills and observes replacement of all redundant recipient
listener roles, proves the interrupted transaction published no compressed
material, removes its private gate and restores the roles. The ordinary rollout
then requires the original activation to apply on every recipient, preserving
key identity, legacy bytes and successful legacy-serving decryptions.

This is an application-transaction interruption and replay through normal
catch-up, not a claim about interruption in the middle of an HTTP download.
The lock-owning session releases on EOF; trigger removal and service recovery
are mandatory cleanup. The trigger SQL is executed against an isolated disposable
PostgreSQL database in the harness, with termination/rollback/replay assertions.
Live migration execution remains pending. This mode does not require weakening
key validation or adding hooks to production listener binaries.

## Host RPC retry and catch-up

The CPU `host-rpc` leg runs five separate cases on `three-of-three-backlog`:
`./scripts/run-host-rpc-recovery.sh 429` (or `503`, `408`, `reset`, `stall`). An owned
HTTP proxy receives only operator 1's explicit poller route. Alternate
listener paths stay stopped through verification. Actual `eth_getLogs` failures
must be observed both before arming and after all twelve receipt-identified
transactions are outstanding. After normal forwarding resumes, every original
output and fresh work must complete, agree and decrypt correctly. The observed
four-block page bound makes this catch-up span multiple pages.

HTTP 408 is an explicit request-timeout response, not a blackholed socket or a
claim about the client's own timeout setting. Reset closes the actual connection.
The separate `stall` arm sends no response headers or body and requires an
observed client cancellation followed by another request while the fault stays
active. Releasing the proxy cannot supply that evidence. The poller now bounds
each HTTP attempt at 30 seconds (connection establishment at three seconds),
allowing its existing retry policy to run against a silent peer. A local socket
regression fails on the prior unbounded client and passes with the deadline.
The other RPC methods pass through so health/chain identity traffic alone cannot
satisfy fault evidence. The original poller command and immutable image must be
restored, then the proxy and alternate listener holds must be cleaned up.
Recovery configuration stays in a private directory; CI uploads request/phase
evidence without container environment or connection credentials. Live execution
is pending. TLS trust changes and all Gateway-specific transport/reorg campaigns
are outside this selection.

## Committed broker redelivery

The CPU `broker-redelivery` leg builds `host-listener/test-failpoints` and runs
`./scripts/run-broker-redelivery.sh`. A narrowly scoped broker hook holds the
selected live block after its handler succeeds and before XACK. The runner
independently requires its computation row to be committed and the exact Redis
stream entry to remain pending. Pending-drain reads can increment Redis's count
without dispatching another handler, so the runner records a baseline instead
of assuming the count remains one. After killing the consumer
and observing supervisor replacement, the same queue, entry ID and full payload
must be handled again. A second XPENDING query must report a higher delivery
count; the broker metadata's fallback value cannot satisfy this check. Removing
the gate must let the entry be acknowledged. The ordinary failure oracle then
checks completion of the original output, uniqueness, bytes and quorum. It also
decrypts the retained output and a fresh post-recovery output to the independent
7+5=12 model, requiring fleet byte agreement for the fresh output as well.

Local operators share one Redis service, so each operator and Blue/Green role
must have a separate consumer service name and therefore a separate queue.
Generated Compose supplies these identities while retaining the original
primary queue. Explicit scenario `--service-name` overrides are preserved and
must remain distinct for independent recipients. Sharing a name distributes
blocks between consumers; healthy WebSocket and polling paths can conceal that
misconfiguration until a test isolates the broker route.

Alternate host ingestion remains stopped through the replacement handler and
confirmed acknowledgement. Only then does the runner restore those paths for
the shared recovery oracle, which also checks the poller cursor. A live/final
duplicate, a separately republished message or a fresh transaction cannot
substitute for the pending entry. The hook expires automatically and is
absent from production builds. This covers the managed Redis backend, not AMQP.
Live broker execution remains pending; lower-layer hook checks do not establish
that the complete redelivery path has run.

## Generated-key SNS determinism

SCH-03 is now a required CPU Rust regression in `rust-regression`. It generates
its own keyset, computes noisy u8 7+5, verifies plaintext 12, compresses once,
and independently reconstructs and squashes the same input twice under each
of one- and two-thread policies. All four serialized outputs must agree. A
one-byte mutation must fail that same agreement check. The stackless runner
requires the exact successful test name, so unrelated passing tests cannot
substitute for it.

This checks fixed-input/key CPU reconstruction and thread-policy invariance.
It does not compare different keysets, CPU with GPU, or two GPU models, and it
does not claim full-stack SNS delivery. The older ignored extracted-key
experiments remain optional diagnostics. The generated-key regression has
passed locally; live SNS fault and recovery cases remain pending.

## One missing host upgrade report

`BLUE_GREEN_WITHHOLD_HOST=1 ./fhevm-cli test blue-green` adds UPG-02 to the
specialized blue/green profile. Boot `blue-green-two-of-three-multi-chain`
with `FHEVM_CONSENSUS_TEST_FEATURES=consensus-detector/test-failpoints` and
local candidate builds. The ordinary first negative arm still uses quiet host
chains when this option is absent.

With this option, synthetic work is enabled on both hosts. Operator 1 computes
eligible state hashes but withholds publication for the first host chain and
proposal 1 only. The owned control expires and is compiled out of production.
It acknowledges the real eligible upload boundary; unrelated proposals, chains
and blocks cannot acknowledge it. Before accepting the omission, the runner
requires the same selected block/hash on all three operators, upload by both
peers but not the victim, and a matching uploaded block on the other host track
from all three. These checks distinguish a missing report from missing compute
or a second divergent track.

Ordinary threshold-two compute/decrypt traffic runs on both hosts. Promotion
must still time out into the exact failed/reset state with the old version
active. The supervisor then drops the control, and fresh proposal 2 must pass
the existing full promotion and per-chain continuity checks. Parent exit,
unobserved fault, mismatched peer evidence and failed cleanup cannot pass.
This does not test withholding a Gateway report, a forged report, a software-only
upgrade, or a post-retirement writer. Those are separate properties. Unit SQL
and cleanup contracts pass; live blue/green execution remains pending.

## Detector interruption before report publication

`BLUE_GREEN_INTERRUPT_DETECTOR=1 ./fhevm-cli test blue-green` uses the same
three-operator, threshold-two, two-host topology and detector failpoint build.
Run it separately from the other upgrade faults. It targets successful proposal
2, holds one operator's real eligible host report after computation, and checks
the same peer/other-track evidence as the omission case. It then kills that
operator's GCS detector, requires automatic supervisor replacement, and removes
the gate so publication can resume from durable state. Promotion and the
existing exact per-chain post-cutover dependencies must complete afterward.

The receipt identifies the selected chain/block and old/new process identities.
It proves interruption of a pending report and recovery of the upgrade, not
that the final unanimity anchor must use this particular block: choosing a later
eligible anchor is legitimate. Cleanup contracts cover replacement failure and
parent exit. Actual detector/cutover execution remains for the live campaign.

## Bounded storage write backpressure

The CPU `storage-pressure` leg runs `./scripts/run-storage-backpressure.sh` on
`three-of-three`. It holds SNS while the selected noisy nine-plus-seven output
is computed, then redirects only operator 1's SNS endpoint through an owned
proxy. Only PUTs to that handle's exact `coproc-1/ct128/<handle>/1` object receive
HTTP 507. Other objects and read requests continue through the real store.
Signed request headers and bytes are preserved during forwarding. The proxy
uses its network namespace owner's numeric IPv4 address, preserving S3
path-style addressing; a hostname would introduce unresolvable bucket-prefixed
DNS names before the selected upload reaches the proxy.

The proxy must observe at least two rejected PUTs. At two checkpoints while
rejection remains active, the durable squash must still exist and the SNS digest
must remain unpublished. After release, a successful PUT for the same object,
fleet byte agreement, authorized quorum, original plaintext sixteen and fresh
work are required. Existing noisy-SNS recovery cases now also decrypt their
original value rather than relying only on fresh traffic for plaintext checks.

This models a bounded storage-capacity response through the real upload client.
It does not fill the host disk or claim enforcement of a specific MinIO quota
configuration. The original worker image, command and environment must be
restored; private recovery snapshots are excluded from public artifacts. The
proxy/route contracts pass locally; live storage recovery remains pending.

## Compute versus submission participation

The CPU `submission-partition` leg runs `./scripts/run-submission-partition.sh`
on `two-of-three`. All three operators compute the original named output while
two transaction senders stay stopped. A progressing-chain observation window
must contain exactly one authorized distinct submitter and no quorum. A second
sender then returns; that same handle must reach threshold two while the third
sender remains stopped. Full rejoin must preserve the original transaction and
unique canonical output, decrypt it to twelve, and process/decrypt its fresh
dependent value nineteen.

This separates compute participation from submission visibility and exercises
an asymmetric rejoin. It does not partition host or Gateway networking, forge
signatures, or make Byzantine-safety claims. The no-quorum oracle rejects any
additional sender outside the declared partition. Per-phase logs and original
receipts are retained, and both senders must be restored before PASS publication.
Live execution is pending; existing quorum observation contracts cover the
negative-window validity checks.

## One versus multiple GPU execution permits

SCH-01 uses per-operator stream budgets 4/1/2 in CI, all on a single GPU. Local
launches must also set distinct budgets with at least one equal to one, for
example `GPU_CONSENSUS_STREAMS_PER_DEVICE_0=4`, `_1=1` and `_2=2` (use the full
variable prefix for each). The launcher must observe these actual settings.

The scheduler records `coprocessor_gpu_execution_permits` after each successful
limiter acquisition, labelled with its actual device and capacity. SCH-01
brackets the identified backlog with metric snapshots. Every worker must
acquire permits, the capacity must match its observed launch setting, a
one-permit worker must show only single occupancy, and a larger-capacity worker
must record at least two overlapping acquisitions during this window. Earlier
overlap, distinct flags without overlap, missing metrics and counter resets
cannot satisfy the gate. The existing byte/model oracle still applies.

These are scheduler execution permits, not measured simultaneous CUDA kernels.
Sampling can miss a transient overlap if a peer releases first; an insufficient
observation invalidates the run rather than establishing concurrency. This
adds no multi-device requirement and makes no throughput or fairness claim.

## Divergent host commitment during upgrade

`BLUE_GREEN_DIVERGE_HOST=1 ./fhevm-cli test blue-green` runs separately from
withholding and detector/controller interruption, on the same three-operator,
threshold-two, two-host-chain rollout. It requires GCS detectors built with
`consensus-detector/test-failpoints`. The normal production feature set contains
no publication mutation.

For proposal one, the owner selects operator 1 and one host track. The hook
flips one bit in each eligible published state commitment inside that exact
proposal/version/window. It preserves computed DB state and the real consensus
comparison. The supervisor requires equal original computed commitments on all
operators, publication by every operator, and HTTP readback of the exact differing
32-byte victim report and both unchanged peer reports. It also checks matching
published work on the other host track. Healthy or unavailable objects cannot
satisfy this observation. This models a faulty report, not a cryptographic hash
collision or a demonstrated divergent FHE execution.

Ordinary threshold-two application work must still compute and decrypt on both
host chains. The failed upgrade must time out, retain the old active version,
clear its latches and reset the GCS schema. The fault is then removed and a fresh
proposal must promote with the normal per-chain dependent-value checks. Gateway
services and their reports remain healthy; this is not a Gateway fault campaign.

Before each altered upload, the hook persists original bytes and block metadata
in a private control-table journal outside the reset GCS schema. Cleanup stops
the uploader, expires the control, restores every journaled object and verifies
its public bytes and metadata before removing the control and restarting the
service. Failed restoration retains the journal and fails the run; discard the
rollout stack until recovery is completed.

### Retained material and migrated-key GPU continuation

The key-migration rollout compiles its application fixtures against the Solidity
library from `RFC029_BASELINE_FHEVM_TAG` (default `v0.14.1`). That ref must exist
in the local Git clone. The runner resolves it to an immutable commit before
boot, reapplies the library after every test-container recreation, and records
the commit, library tree, and archive SHA-256 in `fixture-solidity` receipts.
Only the test container's library and compiled fixtures change; production
contracts, candidate binaries, and checkout sources keep their selected versions.

This keeps the same baseline-compatible applications usable throughout the
upgrade. A current library can require a newer executor ABI (for example,
`checkHandleType(bytes32,uint8)`) before the legacy executor supports it. All
input verification, computation, public/user decryption, retained-value and
GPU assertions still run. These rollout passes demonstrate compatibility of
baseline applications across migration; they do not establish that a current
Solidity application can run on an older executor.

Blue/green tests now seed a retained external input and a computed output on
**each host chain** before cutover. After promotion they require identical
verified-input rows, key identities and original serving-object bytes/metadata,
then user/public decryption of both retained values and computations combining
them with a fresh verified input. Snapshots live under
`runtime/retained-material`; reseeding an existing baseline is refused.

The `v0.14-to-v0.15-gpu-key-migration` rollout seeds an additional fixture while
actual baseline release images are running. Its final CPU restart consumes that
fixture. This tests release-produced object lookup and retained input use, not
just serialization of synthetic historical bytes.

For the separate GPU continuation, on a clean committed checkout with a CUDA
build environment, run:

```sh
./scripts/gpu-consensus-workers.sh build
# Uses the native build receipt and immutable image IDs. GPU_RUNTIME_BASE can
# select a compatible public runtime where private-registry access is absent.
./scripts/gpu-consensus-workers.sh package-migration
export RFC029_GPU_CONTINUATION=1
export RFC029_GPU_IMAGES="$FHEVM_STATE_DIR/runtime/gpu-consensus-workers/migration-images.json"
# Run the ordinary v0.14-to-v0.15-gpu-key-migration rollout with these variables.
```

The GPU continuation selects `Default` parameters and explicitly sets
`kms.insecureTestKeygen: true`. Threshold `Test` parameters use drift noise
reduction unsupported by CUDA; secure preprocessing with `Default` is too
expensive for this campaign. In this isolated mode a per-party gRPC adapter
selects the pinned KMS releases' insecure preprocessing and key-generation APIs.
It preserves request IDs, protobuf messages and signed responses, and forwards
all other RPCs unchanged. The four-party topology, on-chain verification,
existing-key migration and decryption checks remain in use. This validates key
migration and GPU consumption, **not secure distributed key generation**.
Use this mode only with disposable test keys; it intentionally reconstructs
secret material during test key generation. The CPU fault arms continue using
secure threshold generation with `Test` parameters. The pinned v0.14.0-1 and
v0.15.0-0 KMS releases support these APIs and existing-share migration; older
KMS releases are not covered by this opt-in.

Set `FHEVM_STATE_DIR` explicitly for this recipe. The continuation replaces all
three worker roles on both promoted operators with those GPU images on **one
physical device**. It requires compressed-XOF loading from every role, retained
and fresh-input decryption, within-GPU-class byte agreement and format 21 for new
squashed outputs, then repeats after worker restart. Original CPU serving objects
must remain unchanged; newly computed CPU and GPU bytes are not compared.
The original key ID and legacy material must survive. Parent EOF or normal exit
restores the original CPU images, commands and environments; failed restoration
fails the rollout and retains its private journal. Neither an image label alone
nor successful container startup is the GPU-use oracle: the compressed loading
logs, completed computations and GPU-only squash format are required.

These rollout checks remain outside the consensus inventory's `full` verdict.
They are prepared coverage until the corresponding live rollout completes.

`./scripts/run-host-rpc-recovery.sh tls-trust` adds the HTTPS arm. It first
requires successful log polling with an isolated trusted CA, then replaces the
endpoint certificate with an untrusted one on the same host/port. Actual TLS
client errors, no accepted log requests and the twelve named unprocessed targets
must coexist. Restoring the trusted certificate must recover all original pages
and fresh work. Persistent certificate errors can exhaust the poller's bounded
RPC failure budget and trigger its normal supervisor restart. The test allows
that restart backoff, but never manually restarts the faulted poller: the same
routed container must recover after trust is restored. Before/after restart
counts and poller logs accompany the request evidence. The untrusted certificate
is an otherwise valid server leaf, not a CA certificate used as an endpoint.
The worker receives only a public CA through its normal trust
store environment; verification is never disabled. Cleanup removes the override
and restores the original poller environment as well as its command and image.
The CA and private endpoint keys are unique to this test and expire after two
days. Local proxy contracts exercise certificate rejection and recovery; live
poller execution remains part of the deferred E2E campaign.

SCH-04 now scopes its expiring reservation control to the actual heavy
transaction ID. The scheduler carries that identity only for synchronous
partition execution, with an unwind-safe guard; unrelated transactions retain
normal admission. Before installing the selector, the owner briefly freezes the
victim so the named transaction cannot escape the fault. The original all-work
control format remains supported for lower-layer callers.

Three freshly submitted short transactions must finish on **the same worker**
while all sixteen heavy outputs and their real cross-transaction child remain
pending, error-free and unpublished. Each short transaction has a 150-second
watchdog within the control's 600-second lifetime. After release, every exported
intermediate and the child must agree across operators and decrypt correctly.
This tests completeness at the successful completion checkpoint; it does not
assert that the protocol forbids publishing any successful operation before an
unrelated operation in the same transaction finishes. The public pipeline has no
supported malformed opcode/type injection: terminal internal scheduler errors
remain lower-layer tests, not SQL-fabricated application traffic.

### Key-download faults

The migration rollout accepts `RFC029_MIGRATION_FAULT=download-interrupt`,
`download-wrong-digest`, `download-malformed`, or `download-wrong-key`. Build its
Green host listeners with
`FHEVM_CONSENSUS_TEST_FEATURES=host-listener/test-failpoints`. A one-hour, exact-key
control routes only that compressed-key GET through an owned proxy; other keys,
legacy material and the original MinIO objects are untouched. The normal AWS
client, digest validator, parser and activation transaction remain in use.

The interruption arm observes a nonempty partial HTTP body before replacing all
recipient listener owners and proves no compressed key was installed. Corruption
arms require the actual digest error and an unstaged activation row. Every arm
then requires successful fetching through the same endpoint and activation of
the original migration. The enclosing rollout checks original key identity,
retained inputs, objects and decryptions. Cleanup removes the fixed-path control
using the test binary's narrow control command, including on distroless images.

For a valid independently generated wrong-key payload:

```sh
# From coprocessor/fhevm-engine; creates a new file and never installs the key.
SQLX_OFFLINE=true cargo run --release -p host-listener --features test-failpoints \
  --bin migration_test_key -- /absolute/path/independent-key.bin
export RFC029_WRONG_KEY_FILE=/absolute/path/independent-key.bin
export RFC029_MIGRATION_FAULT=download-wrong-key
```

A malformed HTTP body fails its committed digest before parsing. A separate
isolated-database regression supplies malformed bytes **with a matching digest**
to the real download/validate/stage function and requires parser rejection with
no staged bytes. It also redelivers the identical decoded activation through the
production insertion function and requires one activated event, one logical key
and no second application. These are explicitly lower-layer parser/idempotency
checks; the live HTTP arms do not claim an authenticated malformed payload or a
second broker-delivery campaign.

### Compatible software replacement and retired writers

Set `RFC029_SOFTWARE_BASELINE_TAG=v0.14.0-7` to prepend a compatible software
replacement to the migration rollout (whose target baseline defaults to
`v0.14.1`). The release binaries are pulled and inspected before replacement:
all five observed service roles must report the same compiled `0.14.0` protocol
identity, and at least one binary hash must differ. Released 0.14 uses
`STACK_VERSION` for this decision; it does not expose the later numeric
consensus-version flag. The ordinary CLI group-upgrade path must adopt the exact
candidate images without starting Green or changing the versioning singleton.
Retained inputs/objects and fresh values must still work. The later ordinary
blue/green phase separately requires a strictly newer compiled numeric consensus
version and a real promotion. This is a concrete release-pair compatibility
check, not a certification of every patch release or hypothetical downgrade.

Set `RFC029_RETIRED_WRITERS=1` to challenge the actual old release after promotion.
An owned supervisor stops promoted SNS workers and restarts the retired SNS
binaries, requiring their real retirement rejection. A named computation is
prepared for squash by the live TFHE workers. A nested owner then holds promoted
TFHE workers, restarts their retired binaries and submits a second named
computation. Across three checkpoints spanning at least thirty seconds, the
first stays computed but unpublished, the second stays uncomputed, and neither
object may exist in any serving bucket. This makes the SNS external-write check
load-bearing: it has real prepared ciphertext to publish. Restoring Green must
finish and decrypt both original workloads. Retired probes are stopped; promoted
services are restored on normal completion or parent loss. These are retirement
checks for compute and object publication, not Gateway submission faults.

For the deferred rollout validation, use a fresh stack for each download fault.
One healthy rollout should enable software replacement, retired-writer checks
and the GPU continuation together. Run the lagging-recipient,
application-interruption and four download fault modes separately. The required
REG-06 inventory gate covers parser staging and decoded-event idempotency in an
isolated database; those checks do not require another live broker campaign.

### Companion versions for released blue/green baselines

The released `v0.14.0-7` baseline uses TFHE 1.6.3. Bootstrap it with a
compatible KMS core, such as `CORE_VERSION: v0.14.0-1` in the resolved version
lock, before starting the blue/green campaign. A current core can generate key
encodings that the released baseline cannot deserialize, even though both
fleets start and the key rows satisfy readiness. Reusing those keys after
changing only the core image does not repair the stack; generate fresh material
on a fresh stack. Candidate acceptance still uses the branch-built candidate.

KMS releases before 0.15 publish the insecure test runtime as `core-service`;
0.15 and newer publish it as `core-service-insecure`. The CLI selects the
repository for centralized and threshold deployments from the release pin.
Unversioned source tags use the current repository naming convention.


### Threshold KMS epoch ownership during the 0.14 to 0.15 upgrade

KMS 0.15 needs an explicit context-to-epoch mapping to migrate existing PRSS
storage. The key-migration rollout reads the active association from
`ProtocolConfig`, checks that each party has the corresponding legacy context
and PRSS objects, and records it in `rollout/kms-epoch-migration.json`. This
scenario creates one context and epoch; it does not rotate epochs before the
upgrade and is not coverage for migrating a multi-epoch deployment.

The upgrade API persists the mapping separately for each upgraded operator.
Generated migration configuration is mounted only into those operators, so
older nodes in a mixed fleet retain their original configuration. Runtime
regeneration preserves the mapping for subsequent restarts. Missing storage or
ambiguous mappings fail the rollout rather than synthesizing replacement PRSS.


The GPU continuation selects threshold KMS `Default` parameters at baseline
creation. Threshold `Test` parameters use drift noise reduction and are rejected
by CUDA; merely having compressed key bytes does not establish GPU compatibility.
CPU-only migration/fault variants retain the smaller `Test` parameters. The GPU
variant consequently needs more key-generation time and memory. It must generate
and retain its original ciphertexts with `Default` from the start: switching key
parameters after the CPU checks would not prove consumption of the original
migrated key. GPU success still requires the live retained-material and restart
checks; parameter selection alone is not evidence of success.
