# listener_core — Claude Code Context

## What This Crate Does

Production EVM block listener. Fetches blocks from RPC nodes, validates chain continuity, detects reorgs, filters transactions per consumer, and publishes events to a message broker (Redis Streams or RabbitMQ).

## Pipeline

```
RPC Node
  → SemEvmRpcProvider (semaphore rate-limited)
    → EvmBlockFetcher (5 strategies, parallel)
      → AsyncSlotBuffer (out-of-order insert → in-order read)
        → Cursor (parent-hash chain validation, sequential)
          → Publisher (FilterIndex matching → per-consumer payloads)
            → Broker (Redis | AMQP, with ensure_publish)
              → DB insert (PostgreSQL, canonical block)
```

## Core Algorithms

### Cursor (`core/evm_listener.rs`)
Producer-consumer over `AsyncSlotBuffer`. Producer spawns one tokio task per block in range, fetching in parallel. Consumer reads sequentially, validates `block.parent_hash == expected_hash`. On mismatch → `CursorResult::ReorgDetected`, cancels producer. On success → publish events → insert block as CANONICAL.

Dedup guard: Lock flow is handling the dedup by design, it will be able to discard message if there is some processing and duplicated tasks.

### Reorg Backtrack (`core/evm_listener.rs`)
Three-phase crash-safe algorithm:
1. **Walk + Publish (read-only)**: Fetch reorged blocks by parent_hash, publish events (`BlockFlow::Reorged`), collect lightweight metadata. DB untouched.
2. **Batch Commit (atomic)**: Single DB transaction — upsert all walked blocks as CANONICAL, previous ones become UNCLE. Rollback on any failure.
3. **Resume**: Publish `FETCH_NEW_BLOCKS` to restart cursor.

Crash at any phase → DB untouched or fully committed → retry re-walks safely (at-least-once).

### Slot Buffer (`core/slot_buffer.rs`)
`AsyncSlotBuffer<T>`: fixed-size vec of `Mutex<Option<T>>` + `Notify`. Producers call `set_once(index, value)` in any order. Consumer calls `get(index)` sequentially, awaits `Notify` if slot empty. Cancel-safe.

### Block Fetcher (`blockchain/evm/evm_block_fetcher.rs`)
Five strategies, all with retry + cancellation:
1. **BlockReceipts** — 2 parallel tasks (block + `eth_getBlockReceipts`). Most efficient.
2. **BatchReceiptsFull** — Single batch JSON-RPC for all receipts.
3. **BatchReceiptsRange** — Chunked parallel batches (`batch_receipt_size_range`).
4. **TransactionReceiptsParallel** — One task per receipt.
5. **TransactionReceiptsSequential** — One at a time, rate-limit friendly.

Retry classification: Unrecoverable (fail) | RateLimited (exponential backoff) | Recoverable (fixed interval).

### Block Computer (`blockchain/evm/evm_block_computer.rs`)
Optional cryptographic verification: transaction root, receipt root, block hash. Handles L2-specific encodings (Optimism deposit tx type 126, Arbitrum internal type 106).

### Publisher (`core/publisher.rs`)
`FilterIndex` — inverted index for O(1) transaction matching:
- `by_from`, `by_to`, `by_pair`, `by_log`, `unfiltered` (wildcard consumers)
- Builds per-consumer `BlockPayload` with filtered transactions/logs
- **Publish-before-commit**: events sent before DB insert. If publish fails → no DB mutation → cursor retries same block. Guarantees zero missed events.
- Per-consumer infinite retry until broker ACK.

### Consumer Routing
- Internal: `FETCH_NEW_BLOCKS`, `BACKTRACK_REORG`, `WATCH`, `UNWATCH`, `CLEAN_BLOCKS`
- External: `{consumer_id}.new-event` — dynamic from filters DB
- All topics namespaced by `chain_id_to_namespace(chain_id)`

## File Map

| File | What |
|------|------|
| `src/main.rs` | Entry point, wiring |
| `src/lib.rs` | Public API exports |
| `src/core/evm_listener.rs` | Cursor + reorg algorithms |
| `src/core/slot_buffer.rs` | AsyncSlotBuffer |
| `src/core/publisher.rs` | FilterIndex + event publishing |
| `src/core/workers.rs` | Broker message handlers (FetchHandler, ReorgHandler, etc.) |
| `src/core/filters.rs` | Filter lifecycle (add/remove) |
| `src/core/cleaner.rs` | Old block deletion |
| `src/blockchain/evm/evm_block_fetcher.rs` | 5 fetching strategies |
| `src/blockchain/evm/evm_block_computer.rs` | Block verification |
| `src/blockchain/evm/sem_evm_rpc_provider.rs` | Semaphore-based RPC provider |
| `src/config/config.rs` | YAML + env config schema |
| `src/store/repositories/block_repo.rs` | Block DB operations (upsert, batch upsert) |
| `src/store/repositories/filter_repo.rs` | Filter DB operations |
| `src/store/models/block_model.rs` | Block, BlockStatus, UpsertResult |

## Hard Invariants

1. **ZERO EVENTS MISSED.** Duplication acceptable (at-least-once), but every matching event must reach its consumer. Publish-before-commit enforces this.
2. **Atomicity & crash resilience.** DB failures, Redis failures, broker failures must not corrupt state. Reorg uses read-only walk then atomic batch commit. Cursor publishes before inserting.
3. **HPA compatible.** No duplicate processing of main flow messages. `is_empty_or_pending()` dedup guard + `prefetch=1` prevents concurrent cursor runs per chain.
4. **Broker parity.** Redis Streams and RabbitMQ must produce identical behavior. `ensure_publish` maps to `WAIT 1 500` (Redis) / `confirm_select` (AMQP). Same handler interface for both.
5. **Correct consumer routing.** Events published only to consumers with matching filters. FilterIndex must be rebuilt from DB on each cursor iteration.
6. **Chain ID segregation.** All topics namespaced by chain_id. DB queries filtered by chain_id. One canonical block per (chain_id, block_number).
7. **Keep up with fastest chains.** Parallel fetching via SlotBuffer, configurable batch sizes, 5 RPC strategies, semaphore-controlled concurrency.

## Error Handling

Handlers classify errors for the broker:
- **Transient** (`HandlerError::Transient`): DB errors, RPC failures, broker publish failures, payload build errors → broker retries (max 5).
- **Permanent** (`HandlerError::Permanent`): Invariant violations, deserialization failures → dead-letter queue, no retry.

## Known Accepted Limitations

- RPC stall blocks cursor until timeout (accepted, exponential backoff mitigates).

## Skills to load

- if available locally, load the skill /karpathy-guidelines
- for planning, or when benchmarking, if available locally, load the skill /brainstorming
