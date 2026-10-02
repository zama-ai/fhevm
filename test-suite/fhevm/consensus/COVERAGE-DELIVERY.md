# v0.14 feature and failure coverage delivery ledger

This ledger maps the v0.14 feature audit and delivery plan to the preparation
on `antoniu/pr/14-failure-mode-coverage`. **Prepared means implemented and checked
at the indicated lower layer, not accepted by a live E2E run.** Live campaigns
are deliberately a separate step. The older 13-case validation report must not
be used as evidence for the expanded branch.

The original 17 September source audit files supplied alongside the checkout had the following hashes. These identify the historical planning inputs, not the subsequently updated reports:

| Source | SHA-256 |
| --- | --- |
| `coprocessor-v014-e2e-delivery-plan.md` | `b902934b0c23836b96134378c19d4ea7c925c21959858af19b4deb3022b3c07b` |
| `coprocessor-v014-e2e-inventory.csv` | `d5c3084a53ea24046c9acd7c5fd9ec083138bf83843a6b14447d0993329e8e82` |
| `coprocessor-v014-features-e2e-audit.md` | `41059bf8c87772d8f2c37b98a0c1a3dd2134ea52d083f8e1a0d84c039a0c2827` |

The [inventory](inventory.yaml) defines executable consensus case contracts and
backend requirements. The [runbook](RUNBOOK.md) documents commands, faults and
limits. Specialized rollout commands are outside the consensus inventory's
`full` verdict. Package names below are planning groups, not additional PASS
records or substitutes for case evidence.

## Prepared checks

| Package | Implemented checks | Boundary of the claim |
| --- | --- | --- |
| D0: identity | Checkout build receipts, observed running images and GPU worker settings; explicit candidate SHA and baseline tag for key migration. | Published-image mode is partial. A historical baseline is not automatically the latest supported release. Cold-start receipt evidence is included in the later completion audit; see the execution update below. |
| D1: compact input | INPUT-01/02/03 compare verified input-list storage before computation; bool and unsigned widths through 256 bits, selected zero/max/repeated values, replay, independent blobs, wrong user and invalid proof; plaintext oracle. | Bounded representatives, not every proof/parameter combination. CPU is the required inventory backend; do not claim an unexecuted GPU input campaign. |
| D2: typed operations and bridge | MAT-06 covers typed pairwise local and persisted dependencies with expected intermediate values. MAT-08 covers real two-chain bridge source/destination associations and values. REG-04 generates real keys and checks rerandomization transcript sensitivity. | Supported operations only; no exhaustive Cartesian matrix or public multi-output operation. Transcript regression is lower-layer coverage. |
| D3: upgrades | Quiet synthetic-progress control, per-host retained verified inputs, immutable serving objects and new dependent values before/after promotion, withheld and divergent eligible host reports, ordinary threshold-two service during failed unanimity, exact failed-proposal reset and fresh-proposal success. | Divergence changes published commitments through a test-only hook; it does not demonstrate divergent FHE computation. An optional actual-release software-only replacement requires matching compiled legacy protocol identities, changed binary bytes, unchanged versioning and retained material before the distinct numeric-protocol promotion. Retired TFHE/SNS probes must reject prepared work without canonical or serving-object publication. |
| D4: key migration | Lagging recipient while a peer applies; observed validated-key application blocked inside its real transaction, process interruption, rollback and replay; original key identity and legacy bytes retained; opt-in GPU consumption before/after restart with compressed loading, new format-21 outputs and original input/object preservation. | Separate download arms observe an interrupted HTTP body or wrong-digest, malformed and independent wrong-key bodies, then recover through the same route. Matching-digest parser rejection and decoded-event replay are isolated-database checks, not additional live transport faults. The CPU control remains forced legacy; GPU continuation is separately required for its claim. |
| D5: scheduling and resource pressure | SCH-06 observes CPU batch/window diversity. SCH-01 observes GPU batch/window diversity plus one-versus-multiple execution permits. SCH-04 denies real nonzero GPU reservations for one named 16-operation transaction, observes timeout/retry, and requires three fresh short transactions on the same worker while the heavy chain and a real child remain pending; every original intermediate must recover. | Permit overlap is not simultaneous CUDA kernel evidence. Reservation denial models admission pressure, not physical exhaustion. This is bounded same-worker progress under transaction-scoped pressure, not a throughput or arbitrary starvation bound. |
| D6b: SNS determinism | Required SCH-03 creates keys, computes noisy ciphertext and compares reconstructed CPU squash outputs under one/two threads; a changed result must fail the same byte gate. | Rust gate, not GPU SNS determinism or an additional E2E campaign. |
| D7: interruptions | Full blue/green controller interruption at observed dry-run, pre-cutover-commit and post-cutover-commit boundaries; separate detector replacement with a durable unpublished host report. | Post-commit does not mean before every listener consumed its notification. The migration continuation separately challenges retired compute and SNS publication against prepared work; it does not add a Gateway submission fault. |
| D8: host transport and replay | Real host RPC 429/503/408/reset and silent-response cancellation/retry; valid HTTPS, observed untrusted-certificate rejection and recovery on the same route; twelve retained transactions exceed the configured catch-up page; long offline recovery plus fresh work. REG-05 checks the actual HTTP deadline and same-client recovery. | HTTP 408 and client timeout are distinct arms. No new Gateway transport or reorg campaign. |
| D9: serving-object integrity | STORAGE-01 mutates freshly produced objects across all serving buckets: absent, truncated, wrong real handle, changed key metadata and changed format. It requires intended rejection and completion of the same pending decryption request after restoration. | Changed signed metadata has an invalid signature; this is not a valid malicious attestation. The migration rollout separately seeds actual baseline-release objects and requires their preservation and consumption after upgrade/restart; STORAGE-01 alone does not prove historical compatibility. |
| D10: durable and combined faults | Named durable backlog; witnessed Redis redelivery after DB commit and before ACK with entry/payload identity and delivery-count increase; exact ciphertext PUT capacity rejection and recovery; one/then-two/then-three submission participation while all compute; storage outage combined with worker replacement. | Bounded operational faults, not arbitrary Byzantine behavior or disaster recovery. Proxy HTTP 507 does not prove object-store quota enforcement. Submission isolation does not fault the Gateway. |

## Implemented boundaries

The previously implementable gaps now have explicit prepared checks: retained
inputs and release-produced objects, migrated-key GPU consumption, compatible
software replacement and retired writers, actual key HTTP faults, same-worker
progress under pressure, and HTTPS trust recovery. Their subsequent live completion is recorded in [VALIDATION-2026-09-24.md](VALIDATION-2026-09-24.md). Parser rejection with a matching digest and decoded activation replay
are required lower-layer database regressions (REG-06), not additional live
transport or broker campaigns.

## Deliberate exclusions and prerequisites

- No new Gateway-specific transport, reorg, forged-submission or finality tests:
  the Gateway is being retired. Existing protocol quorum/input dependencies and
  previously present cases remain. A healthy Gateway dependency is not a claim
  of Gateway fault coverage.
- SCH-02 multi-device attribution is deferred. Required CI must work on one GPU;
  adding a second device is not a substitute for proving actual workload placement.
- D11 cannot be an application E2E until a real production multi-output operation
  and public caller exist. Lower-layer grouping tests must retain their lower-layer
  classification; no invented opcode or SQL-fabricated application result counts.

## Preparation and execution evidence

Preparation checks on 2026-09-21 included CLI typechecking, 873 CLI tests
(nine isolated Docker integration checks skipped), 187 standalone consensus
contracts on Node 22 including disposable PostgreSQL oracles, workflow and shell
lint, and focused Rust builds/tests/clippy. Real generated-key CPU squash and
rerandomization checks ran; they are not full-stack E2E. The silent HTTP deadline
regression also failed for the intended timeout reason with the original
unbounded client temporarily restored.

The E2E package-wide typecheck passes after building and linking this checkout's
SDK locally and fixing ERC20 clear-value narrowing and generated-handle typing.
The local SDK runtime and declarations were built; this does not substitute for
the eventual source-built E2E container or its cold-start receipt.

At the preparation checkpoint described above, no live CPU/GPU consensus campaign, blue/green rollout, key migration or cold-start checkout receipt campaign had run for these additions. The later completion update below supersedes that status. Acceptance
requires a new revision-bound report, including every selected required case,
intended fault/negative-control evidence and successful restoration. Unit checks
and a successful inventory plan cannot substitute for that report.

The completion pass ran 875 passing CLI tests (nine opt-in Docker integration
checks skipped), successful CLI/E2E typechecks and Solidity compilation, real
HTTP/TLS proxy contracts, the scoped reservation unit test, and the migration
parser/replay regression against a fresh disposable PostgreSQL container.
Focused GPU scheduler/worker and host-listener clippy checks and workflow lint
also passed. These preparation results do not claim execution of the newly
prepared live fault, software-replacement, retirement or GPU migration arms.

## Execution update — 24 September 2026

The campaign owner reports **70/70 required inventory cases, 8/8 blue/green
modes and 7/7 key-migration modes PASS**, with a clean final evidence audit.
See the [completion report](VALIDATION-2026-09-24.md) for source-equivalence
provenance, affected download-mode reruns, archived failure/cleanup history and
evidence filenames. Rebased commit hashes alone do not invalidate that report.

Execution was local with public runtime images, a pinned compatible service
bundle, locally built coprocessor targets and one H100. Hosted CI/private pulls,
separate operator suites and optional two-GPU attribution were not exercised.
GPU Default-key generation used the documented disposable test-only setup.
These limits and the existing Gateway/multi-output exclusions remain; full
completion does not extend the tested scope. This documentation refresh did not
rerun or independently inspect the remote campaign artifacts.
