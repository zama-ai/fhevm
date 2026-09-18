# Consensus campaign runbook

This campaign checks agreement and correctness for three coprocessors running
one software, backend and hardware class. The [inventory](inventory.yaml) is the
case contract; the [workflow](../../../.github/workflows/test-suite-consensus.yml)
defines which backend CI requires for each case. A green campaign covers that
inventory at the recorded revision and topology, not every failure mode of the
coprocessor fleet.

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
| FORK-03 | The forked operator first observes an orphan-only allow, then ingests the exact canonical replacement block and completes a fresh sentinel transaction/handle. The orphan handle has no quorum during the observation window and canonical decryption rejects it at the ACL check; a canonical positive control decrypts correctly. | Does not claim that orphan ACL rows are physically deleted, or test stale-child repair/replay. F2 supplies the orphan handle, but F3 must produce its own recovery evidence. |
| SCH-01 | All materialization checks run under deliberately different scheduling settings. Running configuration and executed batch/window measurements must demonstrate diversity; bytes/digests must still agree. | One GPU is sufficient. No multi-device coverage, throughput guarantee, or exhaustive scheduling interleaving coverage is claimed. |
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

CI requires CPU records for MAT-01/02/03, REORG-01, and FORK-01/02/03. **SCH-01 is
the only required GPU case.** The byte and plaintext fixtures run inside SCH-01,
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

`smoke` selects CPU byte agreement; `standard` adds harness, Rust regressions and
fork; `full` adds GPU scheduling. `full` with `build=false` is PARTIAL by design:
published production images cannot validate this checkout's production changes.
The workflow still builds this checkout's test-suite image in published mode.
The ordinary container-based GPU E2E workflow is separate from this host-worker
consensus campaign.

Service fault matrices, degraded availability, expired leases, interrupted-work
recovery, GPU lifecycle failures, fork stale-child repair/replay, and the long
operator suites are outside this layer's `full` claim. SCH-02 multi-device
attribution and SCH-03 extracted-key SNS experiments are explicitly deferred.
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

The runners require three operators, a 3-of-3 gateway threshold, and one logical
host chain. They read the active generated state and inspect running listener,
poller and consumer routes; live suites read gateway membership and threshold.
Environment labels must agree with that evidence. A 2-of-3 deployment or a fork
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
window/component settings (100/20, 1/1, 200/64); scenario boolean tuning is carried
into each unit's environment. The launcher checks this handoff. Run
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

Results remain staged until EXIT cleanup finishes. An individual recorded
FAIL/INVALID/NOT_RUN keeps successful siblings intact when shared gates and
cleanup succeeded; the overall run still fails. A failed shared hook, validity
gate, refused record, or failed recovery invalidates provisional sibling passes.
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
