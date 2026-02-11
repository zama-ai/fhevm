# Host Listener Parity Matrix (Discovery)

Date: 2026-02-09
Last synced: 2026-02-11
Status: Active reference (discovery complete, parity expansion in progress)

## Outcome

Recommended direction for PoC speed and low blast radius:
- Build a separate `solana-listener`.
- Preserve existing canonical DB contracts (`computations`, `allowed_handles`, `pbs_computations`).
- Keep Gateway unchanged by preserving handle metadata semantics.

## Checkpoint Update (2026-02-10)

1. Discovery scope from #1028 is complete and closed.
2. Parity expansion is active under #1031/#1032.
3. First non-ADD op (`SUB`) is now implemented in Solana listener decode + DB mapping.
4. Explorer-visible flow is reproducible with one command, and local explorer decode is now enabled by default via auto IDL publish in runner.

## Core Findings

| Concept | Finding (precise) | Solana implication | Code evidence |
|---|---|---|---|
| Handle metadata compatibility | Handle metadata includes host chain id + type + version in fixed bytes; Gateway parses this for host-chain validation. | Keep handle byte semantics compatible; replace EVM chain-id source with deterministic Solana host id derivation. | [FHEVMExecutor metadata write](https://github.com/zama-ai/fhevm/blob/main/host-contracts/contracts/FHEVMExecutor.sol#L780), [Handle parsing](https://github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/libraries/HandleOps.sol#L24), [Gateway host-chain check](https://github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/shared/GatewayConfigChecks.sol#L107) |
| Listener coupling | Current host-listener chain transport and decode path is Ethereum-specific (RPC blocks/logs + ABI decode). | Implement Solana ingestion separately rather than refactoring EVM listener first. | [Poller loop](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/poller/mod.rs#L211), [ABI contract bindings](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/contracts/mod.rs#L1), [Log decode + ingest](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/database/ingest.rs#L63) |
| DB ingestion contract reusability | Canonical effects are persisted in DB with idempotent conflict handling. | Preserve same row semantics and idempotency keys in Solana listener output. | [Computations insert (idempotent)](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/database/tfhe_event_propagate.rs#L299), [Allow insert](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/database/tfhe_event_propagate.rs#L683), [PBS trigger insert](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/database/tfhe_event_propagate.rs#L660) |
| Chain identity binding | Current listener startup enforces configured chain identity match; upstream DB model is moving from `tenant` to `host_chains`. | Keep one-listener-one-configured-host-chain model for Solana too. | [Poller chain mismatch check](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/poller/mod.rs#L143), [Main chain mismatch check](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/cmd/mod.rs#L1051), [PR #1856 host_chains cache](https://github.com/zama-ai/fhevm/blob/f991b40c0c8f0e73abf768d37506323a3175ee04/coprocessor/fhevm-engine/fhevm-engine-common/src/host_chains.rs#L1) |
| Finality + replay safety | Canonical ingestion uses finality gating and persisted cursor/catchup. | Use finalized Solana source and persisted slot/lane-seq cursor with backfill. | [Finality lag safe tip](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/poller/mod.rs#L229), [Poller cursor table](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/db-migration/migrations/20251015000000_host_listener_poller_state.sql#L1), [Reorg/missing ancestor recovery](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/cmd/mod.rs#L725) |
| ACL parity (v0-relevant) | Persistent allow flow drives both permission persistence and downstream PBS queuing. | Reproduce both side effects 1:1 for Solana `allow` path in v0. | [ACL allow](https://github.com/zama-ai/fhevm/blob/main/host-contracts/contracts/ACL.sol#L191), [Allowed event handling](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/database/tfhe_event_propagate.rs#L521) |
| Dependency correctness | Scheduler correctness depends on explicit dependency relations, not chain type. | Keep explicit dependency metadata in canonical Solana events; do not rely on lane order alone. | [Dependence chain derivation](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/host-listener/src/database/dependence_chains.rs#L44), [Scheduler dependency graph build](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/scheduler/src/dfg.rs#L326) |
| Schedule ordering contract | Work selection/execution uses stable order fields and source ordering assumptions. | Derive deterministic Solana source order from canonical stream (not websocket arrival order). | [Worker query ordered by schedule_order](https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/tfhe-worker/src/tfhe_worker.rs#L377), [Ordering change context PR #1901](https://github.com/zama-ai/fhevm/pull/1901) |

## Strict v0 Scope (Host + Listener)

Must implement now:
1. Handle metadata compatibility for Gateway checks.
2. Minimal symbolic ops (`add`, `sub`) -> canonical `computations` insertion.
3. Persistent `allow` -> canonical `allowed_handles` + `pbs_computations` insertion.
4. Finality gating + persisted cursor + idempotent replay.

Explicitly deferred:
1. Full op catalog parity.
2. Delegation and all ACL edge paths.
3. Full worker/gateway throughput tuning.
4. Production indexer infra hardening.

## Model Update (2026-02-09 from PR #1856)

What changed upstream:
- `tenants` is renamed to `keys`; tenant API key and tenant-centric fields are removed.
- Chain metadata is moved to a dedicated `host_chains` table.
- CRS is split into its own `crs` table.

Why it matters for Solana discovery:
- `tenant_id` is not a stable concept to build Solana integration around.
- Our parity framing should target `host_chain_id` + key metadata compatibility.
- The “one listener instance per host chain” assumption still holds; naming/model now aligns with `host_chains`.

Evidence:
- [PR #1856 migration](https://github.com/zama-ai/fhevm/blob/f991b40c0c8f0e73abf768d37506323a3175ee04/coprocessor/fhevm-engine/db-migration/migrations/20260128095635_remove_tenants.sql#L1)
- [PR #1856 host chain cache type](https://github.com/zama-ai/fhevm/blob/f991b40c0c8f0e73abf768d37506323a3175ee04/coprocessor/fhevm-engine/fhevm-engine-common/src/host_chains.rs#L1)
- [PR #1856 key cache type](https://github.com/zama-ai/fhevm/blob/f991b40c0c8f0e73abf768d37506323a3175ee04/coprocessor/fhevm-engine/fhevm-engine-common/src/db_keys.rs#L1)

## Decision for #1028

Discovery recommendation: proceed to PoC with a separate `solana-listener` service writing the same canonical DB effects as the EVM host-listener.
