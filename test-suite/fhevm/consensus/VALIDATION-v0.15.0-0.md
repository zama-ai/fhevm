# v0.15.0-0 release validation

The selected local freeze validation completed on October 3, 2026 at
20:15 UTC: all 70 selected inventory cases, eight standalone blue/green modes
and seven key-migration modes passed. Published CPU/H100 acceptance also passed.
This is local validation evidence, not approval of an external deployment.
September campaigns remain historical evidence and are not counted here.

## Result summary

| Validation scope | Observed result | Evidence under the local campaign root |
| --- | --- | --- |
| Selected inventory, including lower-layer checks | 70/70 PASS: 60 CPU, four harness and six GPU cases | `selected-inventory-evidence.json` |
| Published CPU image acceptance | 7/7 PASS | `published-cpu-acceptance.json` |
| Published H100 image acceptance | Scheduling, byte/plaintext agreement and compressed output PASS | `published-gpu-acceptance.json` |
| Standalone blue/green upgrade modes | 8/8 PASS | `blue-green-evidence/index.json` |
| Key-migration modes | 7/7 PASS, including the targeted wrong-key retry | `migration-status.json`, `migration-evidence/index.json` |
| Migrated-key H100 continuation | PASS before/after restart; exact CPU restoration verified | `migration-gpu-runtime-observations.json` |
| Final execution evidence reconciliation | PASS; strict selected CI/checkout aggregate and indexed hashes verified | `final-evidence-observer.json`, `final-inventory-evidence.json` |

These rows overlap in coverage and are not an additive test count. Published
artifact acceptance and stateful rollouts are separate executions from the
selected inventory. PASS applies only to the stated scope; exclusions and
external deployment requirements are listed below.

## Source and scope

- Freeze: `v0.15.0-0`, `9bcd19f5a9e35314d7c07498369af61fa122c432`.
- Validation branch: `antoniu/validate-v0.15.0-0`, initially
  `fee7ade7c0528097584d95ea4d58853842072d34`. Two cherry-picked fixture fixes
  change six E2E files only; production source matches the freeze.
- Published-container and migration harness: `23221c338` on an isolated worktree. It observes
  actual GPU containers and visible devices instead of assigning them the CPU
  execution class. The main checkout stayed fixed at `fee7ade7c` through the
  final aggregate; the validation branch then incorporated the harness fixes
  through `7c587cf1d` and this report. Test records retain their execution SHAs.
- Corrected migration harness: `6fecc25fe0eb9063c7c09e147d23fbbccb461386`,
  with the per-host contract plan restored. Production sources remain freeze-identical.
  The first migration is recorded at `23221c338` plus this explicit supplement;
  five following modes passed at `6fecc25fe`; the final wrong-key retry uses
  `7c587cf1d` after the setup failure described below.
- Candidate upgrade pair: `v0.14.2` to `v0.15.0-0`. Production currently uses
  `v0.13.5`; its hop to the new `v0.14.2` testnet candidate is a separate handoff.
- Selected inventory: 70 cases (69 required plus one supplementary case), with
  CPU and GPU legs; eight blue/green modes and seven key-migration modes are
  separate required campaigns.
- User exclusions: expensive operator suites, including typed-boundary sweep;
  new Gateway-specific resilience campaigns; experimental HTTP relayer; optional
  one-worker/two-device scheduling configuration. Ordinary Gateway integration
  remains part of input, decryption and upgrade validation.

## Completed checks

See `initial-checks.json` for execution revisions and log hashes.

- CLI typecheck and initial unit suite: 919 passed, 9 skipped.
- Inventory harness: four cases passed, including 203 comparator/fault-oracle
  tests with a separately provisioned PostgreSQL oracle.
- Rust inventory: seven cases passed; strict CI subset aggregate passed.
- Detector database tests: 236 passed. Upgrade-controller tests: 54 passed;
  one ignored subprocess entry point is exercised by its process-kill parent.
- Ciphertext-attestation: 62 passed with all features. Block-manifest: 24 passed.
- Host-listener producer-block/synthetic classification and missing-parent repair:
  two isolated real-DB integration regressions passed.
- Worker containment: 29 scheduling/result/epoch checks plus one real-DB
  transaction-barrier test passed on the production feature set.
- Scheduler CPU: 23 passed. Clippy with warnings denied passed for scheduler,
  detector, upgrade-controller, host-listener and SNS library targets.
- GPU production and fault-enabled native builds passed, with separate binary
  hashes and build manifests. These builds are supplementary instrumentation,
  distinct from the published release H100 images.
- Published GPU binaries start and see their assigned H100 via NVIDIA runtime
  injection. This preflight is not FHE execution evidence.
- Published chart `coprocessor:0.13.24` matches all 53 archived freeze files;
  `Chart.yaml` differs only in Helm serialization. Lint and default rendering
  pass with the CI-pinned Helm 3.16.4. Approved deployment values remain absent.
- Eleven fresh local contract versions match freeze source; proxy implementation
  addresses and code hashes are recorded. This is not the external deployed
  baseline.
- Independent compressed wrong-key fixture generated and deserialized; its
  exact bytes were served in the passing wrong-key rejection/recovery mode.

The isolated GPU-container and published-migration harness has 935 unit passes,
10 skips, a passing typecheck, 14 focused backend/recording checks and six
focused migration-provenance tests. The additional skipped
Hardhat check requires prebuilt local artifacts; these results do not replace
live E2E execution.

## Live execution and provenance

The initial three-operator CPU cold build and startup completed. Its receipt
binds running image IDs to the unchanged checkout. All three extracted TFHE
binaries contain the parallel scheduler implementation. All 24 copro services
were running with zero boot restarts. An orchestration argument error after the
receipt completed was repaired by rerunning only the image audit; the existing
stack was reused without repeating the build or any test case.

CPU input cases INPUT-01, INPUT-02 and INPUT-03 passed, covering bool through
uint256 input lists, identity and fleet byte checks, plaintext decryption,
exact replay, independent encryption, and explicit rejection of malformed or
wrongly bound proofs. The first seven-case CPU leg (input, materialization and reorg) passed its strict CI checkout-receipt aggregate. The degraded, dependency-interruption, worker-interruption, smoke, object-integrity
and storage-backpressure legs also passed their strict per-leg gates. The latest
snapshot records 70/70 selected cases passing with no failure records; the
combined strict CI/checkout aggregate passes (`selected-inventory-evidence.json`). Majority availability, detector recovery,
submission partition and all five fork cases now pass their strict per-leg gates.
Fork evidence includes both corruption canaries, canonical sentinel recovery,
the gated-child negative control and observed cursor replay with successful
service restoration. Durable backlog and long-offline ingestion also passed
their original-workload and cleanup checks. All six host-RPC cases also passed
the strict checkout aggregate, with same-container recovery, original-output
validation and route restoration; see `host-rpc-evidence/index.json`. MAT-08
also passed with two operators and two host chains, checking local and bridged
reuse, exact output bytes/provenance, quorum and model plaintexts. Both operators
reached peer manifest consensus for four material-bearing blocks. Bridge transport
uses local LayerZero mock endpoints; external bridge-network delivery is not certified.
Broker redelivery also passed: commit-before-ACK death, replacement delivery of
the same payload with an independently queried Redis delivery count, original
and fresh output correctness, byte/quorum checks and strict cleanup aggregate.
All three crash/retry boundaries passed, including the TFHE-crash and supervised
daemon aliases. The original interrupted work recovered byte-identically with
no stranded work, and strict checkout/cleanup gates passed. Database outage and
stall also passed their original-workload recovery and cleanup/ownership checks,
followed by the strict checkout aggregate. CPU scheduling diversity passed with
observed 8.33 versus 1.00 transactions per batch and matching outputs. All 60 CPU
and four harness cases passed a combined strict CI/checkout subset aggregate
(`cpu-selected-aggregate.json`). Published CPU acceptance subsequently passed
all seven materialization/input/reorg cases and its strict CI aggregate. The
before/after runtime identity check passed for all 27 operator-role containers,
including completed migration containers, against the approved release manifests.
All three operators reached peer consensus for six material-bearing manifests.
Published H100 acceptance also passed. The five-case native production GPU leg
(GPU-01/02/03/04 and SCH-01) passed its strict CI/checkout aggregate. The separate fault-enabled reservation-pressure case also passed: a named
16-operation transaction reached reservation failure/retry while the same worker
completed fresh short work; pending outputs and their dependent had no premature
publication, then recovered with correct fleet bytes and every intermediate
plaintext. Cleanup and strict aggregation passed. Its expiring admission control
is test-only and does not represent physical VRAM exhaustion. See
`release-fee7ade7c-gpu-three-of-three-heterogeneous-scheduling-all-fault-hooks-status.json`
and its corresponding `-aggregate.log`.

The ordinary selected inventory requires checkout receipts. Separate acceptance
runs use the published CPU images and the user-published
`v0.15.0-0-cuda12.8-sm90` images. Their records use published build mode and are
not presented as cold-checkout receipts. Published CPU startup initially hit a local
identity-check error: Docker returned image-list IDs rather than the pinned
amd64 child IDs. A digest-bound index/child check plus exact config/rootfs and
source-label comparison verified the original artifacts, with a wrong-image
negative control. Acceptance resumed on the existing stack before any suite
had run; see `published-cpu-provenance-recovery.json`. The three GPU operators are assigned
to devices 0, 1 and 0; no single worker spans both GPUs.

Published H100 acceptance passed its strict CI aggregate at harness `23221c338`,
with unchanged before/after image and device identities. The byte/alias/plaintext
fixtures ran within SCH-01: all operators agreed on format-21 outputs, expected
plaintexts and zero watchdog divergence. Observed transaction batch means were
8.33/1.00/1.00, with GPU permit capacities 4/1/2 and observed overlapping
acquisitions 23/0/1 out of 26 per operator. This is published-image execution,
separate from the selected inventory's checkout-receipt run.

Before this suite began, its external orchestration used an unsupported scenario
source basename and a short scenario label. The saved source was relocated
byte-identically to the canonical filename; only two source-path references
changed, with normalized configuration equality asserted. The correct scenario
label and live topology probe then passed, without replacing containers or keys.
See `published-gpu-scenario-relocation.json` and the preserved initial failure
log. This was a local orchestration correction, not a production runtime change.

GPU image metadata declares CUDA 12.8 and sm90, but the published Dockerfile
omits source-revision labels. User confirmation and release tags establish the
intended build source; immutable registry digests and image IDs bind actual
execution. The harness uses the observed image set as runtime identity when
source labels are absent rather than substituting its own checkout SHA.
This does not constitute an independent signed source attestation. Migrated-key
GPU continuation uses the same pinned published images after a read-only
preflight checks image identity, CUDA/SM labels and source equivalence outside
`test-suite/`. Local checkout receipts retain their stricter source-label rule.

All three operators also published and reached peer consensus for six
material-bearing manifests after the first CPU leg, with no drift findings.
This is live healthy-path coverage of manifest publication/verification.
The separate on-chain detector fault case also passes: exactly one victim drift
signal completed recovery; original and control bytes agree fleet-wide, and
honest operators received no extra signals. This does not certify the distinct
S3/ct64-healing path. Post-case snapshots record one verified ct64 mismatch
marked contained and healed on the victim. That observation is retained, but it
does not isolate healing from the simultaneously exercised revert and cleanup.

The first blue/green baseline has 18 observed operator-role containers bound to
v0.14.2 source revision `07b4abd6a6452e2095b5ee853619405bcee710ad` and the
approved Linux/amd64 manifests (`blue-green-healthy-baseline-images.json`).
Immutable index/child relationships were checked where Docker returned image
index IDs. The healthy upgrade mode subsequently passed end to end. A separate
read-only post-bootstrap probe checked all 17 expected contract versions across
both host chains and the Gateway against freeze source, recording proxy
implementation addresses and code hashes. KMSGeneration is canonical-host-only
by the existing discovery contract. See
`blue-green-healthy-contract-observations.json`; this does not certify external
governance or initializer state.

Healthy blue/green passed at `fee7ade7c`: the incomplete proposal was rejected
and reset on all three operators, then the complete proposal promoted all three
to consensus 2. Six background traffic iterations across two chains completed
without retries or failures. Retained input/object verification, fresh work and
post-promotion dependent transfers passed on both chains; every identified old
value survived and ciphertext counts increased from 7 to 58 on each operator.
Logs and identity evidence: `blue-green-evidence/healthy/index.json`. The first
mode's standalone retained JSONs were pruned by normal teardown before archival;
the full successful test log remains. Subsequent modes archive those files before
teardown. This validates the local published-v0.14.2 to checkout-built freeze pair.

Quiet blue/green also passed: all three operators rejected and reset the first
proposal, then promoted with synthetic-only evidence and no background traffic.
Retained inputs, objects, fresh work and decryption passed on both host chains;
every identified old value survived (7→20 ciphertexts per operator). All 24
observed green roles matched the completed production-feature checkout receipt,
and 18 baseline roles matched the approved v0.14.2 image identities. Both seed
snapshots and consumption receipts survived the automatic teardown archive.
Completed modes, timings and verified hashes: `blue-green-evidence/index.json`.

Withheld-host-report mode passed with `consensus-detector/test-failpoints`.
The receipt identifies operator 1, chain 12345 and block 2196; both peers and
the other host track continued publishing. Threshold-two computation/decryption
passed on both chains during the fault, but upgrade unanimity correctly failed
and reset all three operators. After fault release, all promoted, six traffic
iterations completed with no retry/failure, and retained/fresh work passed on
both chains. All identified old values survived (15→66 ciphertexts per operator).
The 18 observed baseline roles and 24 green roles matched their image evidence;
fault receipts and retained snapshots/consumption records are archived.

Divergent-host-report mode passed with the same detector fault feature. The
supervisor read the divergent published bytes for operator 1, chain 12345,
block 1940. Ordinary threshold-two application work passed on both chains;
upgrade unanimity rejected/reset all operators. Restoring and reading back the
original reports permitted a fresh proposal to promote all three operators.
Six traffic iterations completed without retries/failures; retained/new work
passed on both chains and every identified old value survived (15→66 ciphertexts
per operator). Image observations, fault receipts and retained evidence are
included in the verified mode index.

Detector-interruption mode passed: operator 1's report was held on chain 12345,
block 2253, its detector was killed, and a replacement process was observed
(restart count 0→1). All three operators subsequently promoted to consensus 2;
GCS reached LIVE/completed and temporary schemas were removed. Six traffic
iterations had no retries/failures. Both chains passed retained/fresh-work
checks with zero watchdog divergence/stalls, and all identified old values
survived (7→58 ciphertexts per operator). The 18 baseline and 24 green role
identities were verified before injection. Fault and retained-material receipts
are included in the mode index.

Dry-run-start controller interruption passed with
`upgrade-controller/test-failpoints`. The supervisor observed the committed
boundary, interrupted the controller while the live version remained v0.14,
and verified a replacement process. All operators subsequently promoted to
consensus 2, completed their upgrade state and removed temporary schemas.
Six traffic iterations completed without retries/failures. Both chains passed
retained/fresh-work checks with zero divergence/stalls; every identified old
value survived (7→58 ciphertexts per operator). The boundary receipt, image
observations and retained evidence are hash-indexed with the mode log.

Before-cutover-commit interruption passed with the controller fault feature.
The observed boundary and live v0.14 state precede interruption; the supervisor
verified a replacement process. All operators then promoted to consensus 2,
completed their upgrade state and removed temporary schemas. Six traffic
iterations had no retries/failures; both chains passed retained/fresh-work
checks with zero divergence/stalls. Every identified old value survived
(7→58 ciphertexts per operator). The mode index preserves the boundary receipt,
image observations, retained evidence and passing log.

After-cutover-commit interruption passed. The fault receipt observes v0.15.0
already live before the controller is killed; a replacement process was verified.
All operators reached consensus 2, LIVE/completed state and schema cleanup.
Six traffic iterations had no retries/failures, and retained/fresh work passed
on both chains with zero divergence/stalls. Every identified old value survived
(7→58 ciphertexts per operator). Image, boundary and retained-material evidence
is preserved in the completed-mode index.

**Blue/green complete: 8/8 modes PASS**, at `fee7ade7c`. The controller exited 0;
all mode identities, expected feature sets, final assertions and evidence hashes
were checked. This certifies the stated local published-v0.14.2→freeze-equivalent
checkout upgrade scope, not a deployed environment or post-cutover downgrade.
The first migration baseline has 12 observed running roles bound to the approved
`v0.14.2-0` amd64 manifests and peeled source tag, with zero observed restarts
(`migration-none-v0.14.2-0-baseline-images.json`). This is a point-in-time identity
check. The software-only replacement to `v0.14.2` subsequently passed: ten
role binaries reported protocol `0.14.0`, two binary instances changed, and
retained/fresh workloads passed on both host chains without a protocol-state
change (`migration-software-prelude.json`). Twelve observed target roles match
the approved `v0.14.2` manifests.

**Key migration: 7/7 modes PASS.** The first (`none`) completed at 11:28 UTC
on October 3, including protocol cutover, retired-writer fencing, same-key
compressed-material activation, and retained/fresh workloads on both host chains.
The original key identity and legacy bytes were preserved. All six published
H100 workers consumed compressed-XOF keys; retained/fresh workloads passed both
before and after restarting those workers. Their original CPU images, commands
and environments were restored exactly and independently checked.

The fault modes initially used corrected harness `6fecc25fe`. The final
wrong-key retry uses `7c587cf1d` as described below. `lagging-recipient`
completed at 12:44:52 UTC and `application-interruption` at 13:58:55 UTC on
October 3. `download-interrupt` completed at 15:14:29 UTC and
`download-wrong-digest` at 16:28:46 UTC and `download-malformed` at
17:41:57 UTC. `download-wrong-key` completed at 20:15:13 UTC after its targeted
fixture-preflight fix. The first mode keeps its actual `23221c338` execution
identity and host-contract repair supplement. The old queue controller was
intentionally retired after the successful child exit; its SIGTERM is not a test
failure. `release-v015-migration-tail` stopped on the final setup failure; the
separate wrong-key retry controller completed successfully. Both attempts
remain in the evidence; no completed mode was rerun to conceal a failure.

The application-interruption fault observed a blocked advisory-lock waiter,
replaced the listener owners, and confirmed rollback before restoration. All
operators subsequently applied identical compressed material. Final input,
public/user decryption, and both original retained workloads passed with no
watchdog divergence or stalled work. The boundary and complete receipt are indexed.

The interrupted-download proxy recorded 11 half-body responses and interrupted
closes, then a full response with the original hash for the same key. Both
operators applied identical compressed material. Final input/public/user
decryption and both original retained workloads passed; five interrupted
listener owners were restored and temporary routes cleared. The evidence index
verifies fault-before-recovery ordering, response sizes, and original-byte hashes.

Wrong-digest recovery recorded two altered responses, then original bytes through
the same route. Its oracle required `Invalid Key digest` and no compressed-key
activation before recovery. Both operators subsequently applied identical
material; cleanup, final application checks, and original retained workloads
on both chains passed. This proves integrity rejection, not validation of
malicious material accompanied by its matching published digest.

Malformed-download recovery served three 31-byte invalid bodies, then all
2,665,566 original bytes for the same key. Fault responses preceded recovery;
the original SHA256 matched the recovered body. The digest guard prevented
activation during the fault. Both operators then applied identical compressed
material, all five temporary routes were cleared, and final input/public/user
decryption plus both original retained workloads passed with zero divergence
or stalled work. Its 17 contract versions matched the freeze without repair.
This is an integrity-rejection case, with the same matching-digest limitation.

Evidence: `migration-status.json`, `migration-evidence/index.json`,
`migration-gpu-runtime-observations.json`, `migration-controller-transition.json`
and the 18 archived retained-material files indexed under
`retained-evidence/23221c338-migration-none/index.json`.

## Limits and remaining evidence

- All 70 selected inventory cases and published CPU/H100 acceptance pass.
  The strict combined CI/checkout aggregate and all eight blue/green modes pass.
  Normal migration with published-H100 continuation, lagging-recipient recovery,
  application interruption, interrupted download, wrong-digest recovery and
  malformed-download and wrong-key recovery passed. The final aggregate used
  `--ci --require-build-mode checkout --revision fee7ade7c0528097584d95ea4d58853842072d34 --partial`.
  It reports all selected cases passing and PARTIAL against the unfiltered
  inventory because of the user-approved exclusions; it does not claim that
  excluded operator or optional two-device tests ran.
- Manifest observations are read-only cumulative runtime snapshots. From job 0
  onward they also retain handle/digest/timestamp details; older files are unchanged. They can
  establish publication and peer verification, but do not by themselves prove
  an intentionally injected fault or its healing. Containment/healing currently
  has component/database evidence; review live fault coverage separately.
- The malformed/wrong-key download arms substitute bytes while retaining
  the original published digest. Their rejection oracle is `Invalid Key digest`
  with no compressed material activated, followed by recovery through the same
  route. They do not establish deeper validation of malformed/unrelated keys
  accompanied by a matching digest. See `migration-download-fault-scope.json`.
- Public Debian packaging is used for local checkout builds because Chainguard
  login is unavailable. Published acceptance uses the actual Chainguard-based
  release artifacts. Neither proves hosted workflow execution.
- Local topology, chain state and key generation differ from deployment.
  Test-only Default-key setup is not a secure distributed-key-generation audit.
- Post-cutover database/PITR restoration, approved recovery ownership and target
  Helm values are not yet verified. A Helm rollback is not evidence of safe
  protocol, database, contract and key rollback.
- Contract versions were checked against freeze source on the fresh local stack
  and the two-host-chain healthy upgrade stack, with implementation addresses
  and code hashes recorded. The external deployment's implementations,
  initializer state, governance actions and source deployment lock remain
  handoff inputs.
- Evidence is local to this machine until an approved durable destination is
  supplied. No PR, issue or external page has been published automatically.

The published H100 pins also resolve independently as registry manifests:
`published-gpu-manifest-identities.json` distinguishes manifest and config
digests. This host reports the manifest digest as the GPU image ID; equal
`digest` and `imageId` fields in the acceptance receipt are therefore expected.
No additional source attestation is implied.

## Evidence entry points

Local campaign root: `/home/ubuntu/release-validation-v0.15.0-0`.
Delivery draft: `/home/ubuntu/coprocessor-delivery-v0.15.0-0.md`.

Key files: `STATUS.md`, `requirements.json`, `plan.json`,
`source-provenance.json`, `initial-checks.json`, `results/`,
`published-results/`, `published-gpu-images.json`,
`published-gpu-exec-preflight.json`, `chart-evidence.json`,
`contract-source-audit.json`, `migration-delta-v0142.txt`,
`selected-inventory-evidence.json`, `published-acceptance-evidence.json`,
`blue-green-evidence/index.json`, and per-leg build receipts, logs and manifest
snapshots. Final execution reconciliation is recorded in
`final-evidence-observer.json`, `final-inventory-evidence.json` and
`final-selected-aggregate.log`. `final-documentation-evidence.json` binds the
repository report commit and local delivery draft; `delivery-audit.json`
distinguishes completed local work from unverified deployment requirements.

### Migration host-contract plan correction

The tested correction is available locally at
`/scratch/release-v015/migration-contract-fix/test-suite/fhevm/rollouts/v0.14-to-v0.15-gpu-key-migration/run.ts`
at `7c587cf1d`, incorporating the contract-plan fix from `6fecc25fe` and
fixture preflight. The delivery draft links to that current local version;
the freeze migration script lacks both corrections. Publication remains a separate handoff.

A read-only version probe during the first migration found FHEVMExecutor still
at 0.6.0, while the freeze requires 0.7.0. The migration runbook's explicit
contract stage covered Gateway and canonical-host KMSGeneration but omitted
the bootstrap plan for contracts on every host chain. The separate eight-mode
blue/green campaign used that complete bootstrap path and is unaffected.

Harness fix `6fecc25fe` reuses the per-host bootstrap plan on each configured
chain, including unchanged-reinitializer handling. Fifteen focused tests and
CLI typechecking passed. The same helper upgraded the existing first-migration stack. All 17 live
contract versions then matched the freeze; the affected input and public
decryption checks passed again. The first scenario completed successfully at
11:28 UTC on October 3; the six fault modes use the corrected harness.
This is an observed rollout-harness coverage defect, not a production Rust
failure. The supplemented first execution remains identified as such; it
is not relabelled as a clean execution at the new revision.

Evidence: `migration-host-contract-hold.json` and
`migration-host-contract-repair.jsonl`. Repair and affected rechecks passed (`migration-none-contract-observations.json`
and `migration-contract-recheck.jsonl`). All seven migration modes passed;
the final wrong-key retry includes the fixture-preflight correction below. See
`migration-evidence/index.json` and `migration-status.json`.

The next mode, `lagging-recipient`, independently completed the corrected
contract phase at `6fecc25fe` without a supplemental repair: all 17 versions
matched the freeze, and input/compute/decrypt, public decryption and all eight
user-decryption tests passed while the coprocessors still ran v0.14.2.
Evidence: `migration-lagging-recipient-contract-observations.json` and
`migration-lagging-recipient-contract-checkpoint.json` in the local evidence
directory. The full lagging-recipient mode subsequently passed at 12:44:52 UTC
on October 3, with original retained handles verified on both chains.

## Wrong-key fixture setup failure and targeted retry

The first final-mode attempt at `6fecc25fe` passed baseline compatibility,
contract upgrades, mixed-KMS rejection, all-party checks and nested blue/green
cutover. It then failed before requesting key migration: its cold native
fixture-helper build took 4m52s inside the supervisor's three-minute setup
budget. The helper successfully deserialized the independent key, but setup
was already cancelled. All listener routes were cleared during cleanup.
This was a harness timing failure; no wrong-key fault or migration request
was exercised by that attempt.

Fix `7c587cf1d27d2ab1cfb51fc5898d0766e4698ab5` builds and validates the fixture
before touching the rollout stack, retaining immediate pre-delivery validation.
Seven focused tests (35 assertions) and CLI typechecking passed. Only the final
wrong-key scenario was rerun, with a new baseline and state directory;
its original in-memory pre-upgrade key hashes could not be recovered after
the failed run. The `pre-cutover` schema does not copy shared key material.
The other six migration outcomes, eight standalone blue/green outcomes and
70 selected inventory results are preserved.

Retry controller: `release-v015-migration-wrong-key-retry.service`, started
2026-10-03 at 19:02:33 UTC and exited successfully at 20:15:13 UTC. The final
receipt is `66. complete`; all seven migration modes passed.
`migration-wrong-key-retry.json` hashes the original failed attempt, and
`migration-fixture-preflight-checks.json` records the fix validation.
The retry independently observed 12 baseline roles with zero restarts, all 17
contract versions, and 24 green roles including six running forced-legacy key
workers. The warm fixture-helper build took 0.46s. The proxy served the exact
444,303,721-byte independently validated fixture at 20:06:15.244 UTC, then
2,665,566 unchanged original bytes at 20:06:17.635 UTC for the same key.
The digest guard prevented activation during the fault. Both operators then
applied identical compressed material; all five listener routes were cleared.
Final input/public/user decryption and both original pre-upgrade retained
workloads passed with zero reported divergence or stalled work. The HTTP fault
remains an integrity-rejection test, not matching-digest malicious-key validation.

The final evidence audit passed at 20:15:13 UTC. It verified all seven migration
modes, the eight standalone blue/green modes, published acceptance evidence,
the supporting-check hashes and the strict 70-case selected aggregate. The
original failed setup is archived and hashed separately from the passing retry.
