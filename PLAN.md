# Ciphertext Drift E2E Implementation Plan

## Overview
Add one focused end-to-end test path that proves the Rust `gw-listener` drift detector fires on an intentionally corrupted ciphertext digest in the existing e2e stack. Keep it compatible with the current `2-of-2` coprocessor workflow and reuse the existing Hardhat tests instead of building a new scenario from scratch.

## Goals
- Enable the Rust drift detector in the existing e2e coprocessor stack.
- Add one deterministic drift-injection workflow that fits the current `fhevm-cli test` flow.
- Verify detector output via metrics, not brittle log matching.

## Non-Goals
- No new general-purpose fault-injection framework.
- No changes to the existing consensus watchdog behavior outside the dedicated drift test path.
- No new CI profile or topology change beyond what the current workflow already deploys.

## Assumptions and Constraints
- Current workflow deploys `2` coprocessors with threshold `2`.
- In `2-of-2`, one corrupted coprocessor means no consensus; this test should assert drift detection, not successful consensus.
- The existing JS `consensusWatchdog` would intentionally fail this scenario, so the dedicated drift test run must disable it.
- Host-side scripts can use `docker exec` against the shared Postgres container.

## Requirements

### Functional
- `coprocessor-gw-listener` in e2e must receive `ciphertext_commits` and `gateway_config` addresses.
- A script must mutate one ready-but-unsent `ciphertext_digest` row on a chosen coprocessor DB.
- A dedicated `fhevm-cli test` target must orchestrate the injection and run an existing Hardhat test that produces ciphertext work.
- The test must fail if the Rust `gw-listener` drift metric does not increment.

### Non-Functional
- Keep the change surface small and local to the existing e2e workflow.
- Avoid adding new runtime services or test dependencies.
- Keep the workflow deterministic enough for CI.

## Technical Design

### Data Model
- No schema changes.
- Reuse `ciphertext_digest(handle, ciphertext, ciphertext128, txn_is_sent, created_at)`.

### Architecture
- Extend the e2e `coprocessor-gw-listener` command with:
  - `--ciphertext-commits-address`
  - `--gateway-config-address`
- Add a host-side drift injector script that:
  - targets one coprocessor DB
  - waits for a new ready row with `txn_is_sent = false`
  - flips one byte in `ciphertext`
- Add a host-side runner script that:
  - stops one transaction sender
  - starts the injector
  - runs one existing Hardhat grep with the JS watchdog disabled for that process
  - restarts the transaction sender
  - checks `gw-listener` metrics for `coprocessor_gw_listener_drift_detected_counter`

---

## Implementation Plan

### Serial Dependencies (Must Complete First)

#### Phase 0: E2E Wiring
**Prerequisite for:** Drift injection and runner script

| Task | Description | Output |
|------|-------------|--------|
| 0.1 | Add detector address args to e2e `coprocessor-gw-listener` compose command | Drift detector enabled in e2e stack |
| 0.2 | Add `GATEWAY_CONFIG_ADDRESS` to coprocessor env template used for multi-copro copies | Address available in all coprocessor env files |
| 0.3 | Add a small plan file documenting the scoped implementation | `PLAN.md` |

---

### Parallel Workstreams

#### Workstream A: Drift Injection
**Dependencies:** Phase 0
**Can parallelize with:** Workstream B

| Task | Description | Output |
|------|-------------|--------|
| A.1 | Add a host-side script to wait for a new ready `ciphertext_digest` row in one coprocessor DB | `inject-coprocessor-drift.sh` |
| A.2 | Mutate one byte in `ciphertext` for the selected handle and print the handle | Deterministic DB-level drift injection |

#### Workstream B: Test Orchestration
**Dependencies:** Phase 0
**Can parallelize with:** Workstream A

| Task | Description | Output |
|------|-------------|--------|
| B.1 | Add a host-side runner that pauses one transaction sender, launches the injector, runs one existing Hardhat test, and checks metrics | `run-ciphertext-drift-e2e.sh` |
| B.2 | Add one `fhevm-cli test ciphertext-drift` entrypoint that calls the runner | Existing CLI can trigger the new path |

---

### Merge Phase

#### Phase 1: Validation
**Dependencies:** Workstreams A, B

| Task | Description | Output |
|------|-------------|--------|
| 1.1 | Run targeted checks on the changed scripts and CLI wiring | Verified local changes |
| 1.2 | Document how to run the new e2e path and what it proves | Clear operator/developer usage |

---

## Testing and Validation

- Verify `coprocessor-gw-listener` command in e2e compose includes both new addresses.
- Verify the injector script can discover and mutate a new unsent row for a chosen DB.
- Verify the runner script disables the JS consensus watchdog only for this one intentional drift run.
- Verify the runner fails if `coprocessor_gw_listener_drift_detected_counter` does not increase.

## Rollout and Migration

- No migration.
- Hard cutover for the new e2e path: once merged, `fhevm-cli test ciphertext-drift` becomes the supported drift test entrypoint.

## Verification Checklist

- `bash -n test-suite/fhevm/scripts/inject-coprocessor-drift.sh`
- `bash -n test-suite/fhevm/scripts/run-ciphertext-drift-e2e.sh`
- `rg -n "ciphertext-commits-address|gateway-config-address" test-suite/fhevm/docker-compose/coprocessor-docker-compose.yml`
- `rg -n "ciphertext-drift" test-suite/fhevm/fhevm-cli`

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| The injector races before a ready row exists | Medium | Medium | Poll for a new ready row and gate on `txn_is_sent = false` |
| The existing JS watchdog fails the intentional drift run | High | High | Disable it only for the dedicated drift test command |
| Metric polling hits the wrong listener | Low | Medium | Check a specific gw-listener container and assert the exact counter name |

## Open Questions

- [ ] Whether we want a second scenario later for `3-of-5` where consensus still succeeds with one bad coprocessor.

## Decision Log

| Decision | Rationale | Alternatives Considered |
|----------|-----------|------------------------|
| Keep the first e2e on `2-of-2` | Matches current workflow with the least change | Adding a new CI topology now |
| Use DB-level digest mutation | Smallest realistic fault injection point | Patching workers or faking on-chain events |
| Assert via Rust metrics | More stable than log text matching | Parsing logs or relying only on the JS watchdog |
