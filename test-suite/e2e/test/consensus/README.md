# Consensus agreement coverage

This layer checks canonical byte and digest agreement within one software,
backend and hardware class. CPU and CUDA bytes are not compared with each other;
expected decrypted plaintext provides the correctness oracle on each backend.

## Cases

- Materialization across same-block and cross-block transaction boundaries,
  intra-transaction fan-out, complete transaction counts and no errors.
- Same-sourcing aliases sharing canonical values, with mixed-sourcing operations
  minting distinct handles.
- Classified comparator canaries, complete participant evidence, key/context and
  provenance checks, gateway commitment bindings, and decrypted plaintext.
- Distinct replacement blocks retaining identical sourcing and canonical bytes,
  with evidence that listeners observed the replacement.
- Competing branches: colliding handles agree; different branch content has
  distinct handles; an orphan-only permission cannot authorize canonical decryption.
- Single-GPU scheduling diversity supported by executed scheduling counters.

The inventory at `test-suite/fhevm/consensus/inventory.yaml` defines required and
explicitly deferred cases. Multi-device execution attribution and extracted-key
SNS experiments are not claimed as required regression coverage.

## Execution

From `test-suite/fhevm`, boot the appropriate source-built stack, then use:

```sh
./scripts/run-materialization-consensus.sh --suite materialization
./scripts/run-materialization-consensus.sh --suite reorg
# Requires --scenario three-of-three-fork:
./scripts/run-fork-consensus.sh --case main
```

For scheduling, boot `three-of-three-heterogeneous-scheduling`, provide the matching
per-operator GPU tuning, then use `gpu-consensus-workers.sh build`, `start`, and
`run-materialization-consensus.sh --heterogeneous`. Finish with
`gpu-consensus-workers.sh stop` to restore the original container owners.

The manual `test-suite-consensus` workflow provides `harness`, `rust-regression`,
`smoke`, `standard`, `full`, `byte-agreement`, `fork`, and `gpu` selections. `full`
means all required cases present in this layer; it does not claim fault-campaign
coverage. CPU-only selections do not provision GPU hardware. Ordinary GPU E2E CI
is a separate container-based workflow and does not depend on this campaign.

## Evidence and cleanup

Runners require process exit status and structured assertion evidence. Missing
participants, unreadable required evidence, invalid preconditions, failed cleanup,
and incomplete selections cannot establish a complete passing run. Run identity
records the actual images/binaries as well as the source revision.

Private journals restore canary changes and mining state after interrupted suites.
Process ownership, deadlines and cleanup support are shared by all runners; they
are required even for a normal agreement run. Docker/systemd log access must match
the worker ownership in use. Do not discard a failed-cleanup record to continue.

## Separate failure-mode delivery

Interrupted TFHE execution, expired leases, service-specific faults, degraded
availability/backlog convergence, GPU lifecycle fault injection, and fork
stale-child repair/replay belong to the following PR. Their cases and manual CI
selections are introduced there. The ordinary agreement oracle and its required
negative controls remain enabled in this layer.
