# Byte-consensus tests

Opt-in E2E gates proving that independent coprocessors running the **same
software revision on the same backend/hardware class** persist byte-identical
ciphertexts, and that identified work survives the failures the fleet is
expected to survive.

See the [agreement campaign runbook](../../../fhevm/consensus/RUNBOOK.md)
for the prerequisite agreement layer and its limits. This PR adds the failure
campaigns described below.

What each green establishes is written down, case by case, in
`test-suite/fhevm/consensus/inventory.yaml`. Read that first: it is the inventory
of coverage, and it is what the runners select from and report against.

```sh
cd ../../../fhevm
bun scripts/consensus-inventory.ts list
bun scripts/consensus-inventory.ts show FORK-04-STRANDED-CHILD
```

## The materialization oracle

Transaction-boundary materialization means:

- intra-transaction intermediates are forwarded in memory (never
  compress/decompress round-tripped) — the fixture's `selected → sum/diff`
  fan-out;
- cross-transaction consumers always read the producer's persisted canonical
  compressed bytes — the fixture's same-block `stageInputA → deriveFromAAndB`
  edge and the next-block `consumeFanout` reader;
- whole transactions execute and complete atomically (exact per-transaction
  computation counts, no errors).

The oracle compares, across every coprocessor database: raw ciphertext bytes,
type/version, producing operation, transaction/block provenance, gateway key id,
and the Keccak digest bindings up to the on-chain
`AddCiphertextMaterialConsensus` quorum. CPU and GPU topologies are never put in
one byte comparison; their shared oracle is user-decrypted plaintexts. The staged,
derived, independent and terminal graphs also bind their database attribution to
the actual named transaction receipts and selected host chain, so unanimous
attribution to a wrong transaction cannot satisfy the oracle.

## The shared comparator, and what it will not do

`comparator.ts` owns the semantics. Every suite calls it, and it fails with a
CLASSIFIED reason (`raw-bytes`, `compute-digest`, `sns-digest`,
`sns-evidence-missing`, `key-identity`, `provenance`, `value-multiplicity`,
`storage-row-uniqueness`, …) rather than a bare boolean. Three things follow
from putting it in one place, and each of them replaces a way a green could be
vacuous:

- **A handle may have several producing computations.** Under the
  minted-in-transaction discriminant two transactions with identical operand
  sourcing alias to one handle, so `computations` legitimately holds one row per
  producing transaction while `ciphertexts` holds exactly one row for the value.
  The comparator normalizes the producing set and compares that, and asserts
  storage-row uniqueness separately. Comparing `rows[0]` across operators, as an
  earlier version did, is a coin toss for an aliased handle.
- **The SNS digest is durable, so a missing one is missing evidence.**
  `transaction-sender`'s `delete_ct128_from_db` deletes the raw squashed
  ciphertext from `ciphertexts128` once `AddCiphertextMaterial` has landed, but
  `set_txn_is_sent` only stamps `txn_is_sent`/`txn_hash`/`txn_block_number` on
  `ciphertext_digest` — the digest columns stay. Excusing an absent digest as
  post-submission cleanup let two matching digests out of three report as
  unanimous.
- **A comparison never narrows after seeing that something is absent.** Missing
  evidence fails as missing evidence.

`comparator.test.ts` falsifies each class against synthetic rows, and requires
the comparator to ACCEPT the shapes that are legitimately different — an aliased
handle whose producing transactions arrive in a different order on each operator
being the one that matters.

## The canary rule

Each suite class runs one deliberately-poisoned arm and must go red, every time,
or that suite's greens count for nothing. The rule exists because this
repository has already shipped two comparisons that compared nothing: a GPU
byte-gate that measured one stream against itself, and a fork precondition that
compared zero against zero. Both were green.

Two things make a canary worth having:

- it drives **the comparison that suite actually calls**, not a private query;
- it requires the **specific mismatch class** the tamper should produce. A
  timeout, a database outage or a failed read all fail the canary, because none
  of them is the comparator catching a poisoned digest.

The tamper is applied to an already-published handle, so it can never damage a
gateway commitment, and it is restored and re-verified afterwards. Both these
canaries and the deliberate pre-submission detector fault journal the original
digest under the mutation row lock before writing poison. After an aborted
phase, the host verifies journal recovery before resuming senders. The private
recovery journal contains connection URLs and must stay out of uploaded artifacts. Once the
host records that the detector's poisoned sender may be released, an abort keeps
the journal and marks cleanup failed: local restoration cannot undo a queued
Gateway transaction or delayed automatic revert. Keep the original workload,
handshake and private journal. Recovery must complete the same detector verdict
and verify phase (signal done, original/control agreement, no extra signals)
before this fence is retired; do not delete it to start another case. If that
recovery cannot be established, replace the isolated stack.

## Run-validity gates

A run that cannot produce trustworthy numbers aborts as INVALID rather than
reporting numbers. `validity.ts` gates the preconditions of measurement:

- **Key material.** Every required operator holds usable material, and they all
  hold the same ACTIVE key — read the way the workers read it
  (`ORDER BY sequence_number DESC LIMIT 1`), not as a minimum over historical
  rows. "Usable" is capability-specific: `sns-worker`'s `keyset.rs` falls back to
  the legacy `sns_pk` large object when `compressed_xof_keyset` is absent, except
  under `--features gpu` where the legacy encoding is refused outright.
- **Deferred transactions** drain to zero, so a wedged scheduling window is not
  measured as a result.
- **The chain advances**, so a suite cannot pass by measuring a stalled world.
- Every read is bounded. A stalled endpoint fails its gate rather than
  outliving it.

The unanimous degraded case starts its four-minute no-quorum window only after
the healthy survivors submit the identified handle with the agreed key and
digests. It samples Gateway progress inside that same window and rejects a
30-second stall; progress after the window cannot rescue the observation.

Two gates need host access and live in `scripts/consensus-validity.sh`:

- **`locks`** — whether any tfhe-worker lost dependence-chain locks in the run
  window. It reads the ACTIVE execution backend (systemd journals under a GPU
  session, where the worker containers are deliberately stopped), requires a run
  window, and treats an unreadable source or a missing expected worker as
  INVALID rather than as a quiet clean.
- **`exclusivity`** — one worker per queue, judged by the QUEUE. A process with
  a matching executable name against an unrelated database is not a conflict;
  a second worker on one of this stack's databases is one whatever it is called.

## Host daemon access

Fault injection is driven host-side by `test-suite/fhevm/scripts/*.sh`, not from
inside the runner container, and the suites and runners exchange the identity of
the work under test through a handshake file (`handshake.ts`) that the runner
reads with `docker exec cat`.

That is a deliberate split, and the earlier comments in this directory
contradicted each other about why. The generated compose DOES mount the host
Docker socket into `test-suite-e2e-debug` when one is present
(`dockerSocketRuntime` in `test-suite/fhevm/src/generate/compose.ts`, which
reads the socket's group id from the socket itself). What the runner image does
not have is a `docker` client, and a CI runner that grants socket access through
an ACL rather than group ownership is not covered by a supplementary group
either. So socket availability is a host fact that varies, and the suites do not
depend on it: they submit work and read databases, and the host scripts break
things. `run-materialization-consensus.sh` probes the capability
(`docker exec … docker info`) and omits only the RFC-023 attestation-readiness
probe when it is unavailable, saying so.

## Fault cases: what a green means

Every failure case has the same shape, and it is the shape the previous version
of this suite did not have:

1. **Identify the work.** The suite publishes the transaction hashes, handles or
   request ids whose fate the case asserts.
2. **Prove the precondition.** The relevant stage is observed — a dependence
   chain `processing` under a `worker_id`, an unsquashed digest, an unsubmitted
   commitment, an unverified proof, blocks behind an ingestion watermark, a
   gated child with a non-zero dependency count. A whole-database pending count
   is not a stage: it includes fixture setup and unrelated traffic.
3. **Apply and observe the fault.** Both the injection's status and an
   independent postcondition.
4. **Recover the SAME work.** Not a fresh fixture. A successful fresh
   computation after restarting a service is a recovery smoke test, and the one
   case that is exactly that says so (`FM-SMOKE-CELLS`, acceptance `smoke`).
5. **Assert the full contract**, with safety, liveness, quorum, bytes/digests
   and cleanup stated separately.

`worker_id` is what makes the retry provable: it is a fresh UUID per worker
PROCESS (`daemon_cli.rs`), so the same dependence chain acquired again under a
different `worker_id` is the restarted process reaching it through the
acquisition query's `expired_lock` arm.

## Quorum

Quorum is a three-valued mode — `required`, `forbidden`, `not_checked` — and the
threshold it is judged against is read from the RUNNING gateway
(`getCoprocessorMajorityThreshold`, `getCoprocessorTxSenders`), not from the
scenario file. A boolean defaulted to off cannot distinguish "quorum must not
form" from "quorum was not checked", and the same `0` ran on a 2-of-3 topology
where quorum should form.

A negative quorum claim additionally requires the gateway to have been reachable
and advancing throughout the observation window: an RPC error is not proof of
absence.

## The equivocation residual

Across a fork, handles collide only when the replacement block shares the parent
AND timestamp of the block it replaces: a slashable same-slot double proposal on
Ethereum L1, or an ordinary unsafe-head reorg on fixed-block-time chains
(OP-stack derives the timestamp from the height). Historically a collision with
mixed input sourcing was the one case canonical materialization could not bridge
by construction: `decompress(compress(x))` is bit-inexact for noisy ciphertexts
(see the ignored `compression_round_trip_bit_exactness_survey` test), and the
consuming operation's PBS modulus-switch rounding absorbs that delta only when it
stays below a rounding boundary. The boundary bits close that case
deterministically — colliding handles now imply identical sourcing, hence
identical bytes.

Building that case takes care in two places, and both were wrong before:

- **On one chain** (`reorgConsensus.ts`), the replacement block must be a
  DIFFERENT block. Same parent, same timestamp, same single transaction and the
  same resulting state produce the same header — so the sibling carries one
  extra unrelated transfer, from a different account, touching no FHEVM
  contract. The hashes are then asserted to differ.
- **Either way**, the fleet has to be shown to have OBSERVED the replacement
  before anything is compared. Rows for the handle already exist from the
  original inclusion, so "the bytes are unchanged" was true whether or not any
  listener had seen the sibling. `host_chain_blocks_valid` records ingested
  block hashes per operator, and the comparison waits for the replacement's hash
  to appear there on every operator.

## Deliberately not covered

Recorded here so the suite's claims stay bounded. The inventory carries the same
statements per case, with a reason.

- **Device independence.** `SCH-02-DEVICE-SPLIT` remains an optional local
  experiment. CI has one GPU, so this case is excluded from CI acceptance.
  `SCH-01-HETEROGENEOUS` exercises distinct scheduling configurations on that
  single device and measures committed transactions per execution batch. It
  requires a full transaction of separation above the narrow configured batch
  capacity and enough identified work for two such batches; a tiny average
  difference or partial metric coverage cannot establish scheduling diversity.
- **Full rollout deployment.** `FM-UPGRADE-CONTROLLER` runs the real controller
  reconciliation path in separate processes against PostgreSQL. It kills a
  process while it owns the cutover write fence, checks rollback, and verifies
  that a new process completes the transition idempotently. Deploying two
  complete version bundles remains the existing blue-green rollout harness's
  responsibility.
- **Squash determinism as a gate.** `SCH-03-SQUASH-DETERMINISM` stays a
  diagnostic: the experiments are `#[ignore]`d, acquire fixtures from a
  developer path, and print distinct digest counts rather than asserting.
- **CPU/GPU byte equality**, heterogeneous GPU architectures, and incompatible
  software or key versions. Those are not the byte-consensus execution class
  this suite establishes; the CPU/GPU boundary is plaintexts.
- **Universal claims.** This is a finite suite. It does not establish Byzantine
  tolerance, arbitrary hardware independence, or every possible failure
  interleaving.

## Running one gate by hand

The runners discover everything from the running stack, which is the point:
assembling the environment by hand is how a run ends up measuring the wrong
thing. Prefer them.

```sh
cd test-suite/fhevm
./scripts/run-materialization-consensus.sh --suite materialization
./scripts/run-crash-retry-consensus.sh --victim 1 --boundary before-commit
./scripts/run-failure-matrix.sh --case FM-SNS-CRASH
bun scripts/consensus-inventory.ts aggregate --run <run-id> --select full
```

`--network staging` is not incidental where a suite is invoked directly: it
reads `RPC_URL`, whereas `localCoprocessor` is hardcoded to `localhost:8746` for
running Hardhat on the host against a forwarded port, and inside the test
container that fails with HH108 before any test body runs.

`helpers.test.ts`, `comparator.test.ts`, `validity.test.ts` and
`materializationFixture.test.ts` are plain unit tests of the harness itself and
run without a stack.
