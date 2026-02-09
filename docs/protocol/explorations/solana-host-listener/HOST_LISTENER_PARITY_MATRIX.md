# Host Listener Parity Matrix (EVM -> Solana)

Date: 2026-02-09
Status: Draft

## Purpose

List which EVM-host features must be replicated 1:1, and the Solana options for each. This is the contract for exploration scope.

## Legend

- `Must now`: required for first host+listener feasibility loop.
- `Soon`: likely needed before full end-to-end.
- `Later`: can be deferred during initial PoC.

| Feature | EVM Baseline | 1:1 Invariant to Keep | Solana Options | PoC Choice | Priority |
|---|---|---|---|---|---|
| Handle metadata format | `host-contracts/contracts/FHEVMExecutor.sol` (`_appendMetadataToPrehandle`) + `gateway-contracts/contracts/libraries/HandleOps.sol` | 32-byte handle with parseable chain-id/type/version bytes | A) keep exact byte positions, custom prehandle hash; B) new format + gateway change | A | Must now |
| Symbolic op request semantics | `host-contracts/contracts/FHEEvents.sol` + listener op mapping in `tfhe_event_propagate.rs` | host emits enough data to reconstruct operation/dependencies/result handle deterministically | A) logs only; B) PDA receipt only; C) hybrid | C | Must now |
| ACL allow (persistent) | `host-contracts/contracts/ACL.sol` + `Allowed` event handling in listener | listener can ingest allow state and queue PBS work exactly once | A) logs only; B) PDA ACL state + logs | B (with logs as fast path) | Must now |
| ACL allowForDecryption | `ACL.sol` + `AllowedForDecryption` handling | equivalent public-decrypt permission signal reaches ingestion | A) explicit log/event; B) PDA flag transition | A now, B later | Soon |
| ACL delegation events | `ACL.sol` delegation + listener `insert_delegation` | delegation/revocation can be ingested with ordering and replay safety | A) event/log; B) delegation PDA journal | defer | Later |
| ACL transient allowance | `ACL.sol` `allowTransient` via transient storage | per-transaction temporary access control for on-chain symbolic execution | A) instruction-context only; B) ephemeral PDA/session account | A | Soon |
| HCU gating | `host-contracts/contracts/HCULimit.sol` | per-tx complexity limits prevent abuse of symbolic pipeline | A) protocol-level metering account/PDA; B) rely only on Solana compute budget; C) off-chain policy only | B for first loop; evaluate A | Soon |
| Input verification surface | `FHEVMExecutor.verifyInput` + `VerifyInput` event currently ignored by ingestion | keep clear contract of what host-listener must ingest vs ignore | A) keep ignored in first PoC; B) add dedicated ingestion path | A | Later |
| Chain identity binding | listener chain-id checks in `poller/mod.rs` + DB tenant mapping | one listener instance only writes for intended host chain tenant | A) map Solana cluster/program to protocol chain_id; B) redesign tenant keying | A | Must now |
| Finality + reorg handling | EVM websocket/poller catchup (`cmd/mod.rs`, `poller/mod.rs`) | no silent data loss on transient forks/restarts | A) finalized commitment + slot cursor + backfill; B) processed commitment only | A | Must now |
| Catchup cursor | `host_listener_poller_state`, `host_chain_blocks_valid` | resumable catchup with explicit cursor | A) store slot/signature cursor in new table/state; B) reuse existing cursor table shape | B (extend semantics) | Must now |
| Idempotent ingestion | `ON CONFLICT DO NOTHING` in computations/allowed/delegations | replaying the same signal must not duplicate work | A) deterministic event keys + UPSERT; B) in-memory dedup only | A | Must now |
| Dependence chain scheduling | `dependence_chain` updates from ingestion | dependency ordering remains coherent for worker scheduling | A) compute from canonical dependencies exactly as today; B) disable temporarily | A for ops included in PoC | Soon |
| Health/metrics contract | listener health checks and metrics | liveness/readiness must expose chain and DB status | A) copy same health shape; B) minimal logs only | A (minimal first) | Soon |
| Gateway compatibility | `GatewayConfigChecks.onlyHandleFromRegisteredHostChain` and gateway allow/add flows | gateway accepts host chain handles without protocol-level changes | A) register Solana host chain id and preserve handle metadata; B) gateway contract changes | A | Must now |

## Immediate PoC Boundary

To move fast, first Solana host+listener loop will cover:

1. handle metadata compatibility
2. symbolic op request ingestion (`add` first)
3. persistent allow ingestion
4. finality/cursor/idempotency

Everything else is intentionally deferred.

## Notes from Existing Local Solana Demo

Local repository `/Users/work/code/zama/solana-symbolic-host-demo` already validates a hybrid `logs + PDAs` model and idempotent fulfillment flow. That project is not protocol-complete, but it is a useful template for quick feedback loop mechanics.
