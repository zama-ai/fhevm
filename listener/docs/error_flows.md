# Error Flows — Failure Behavior Map

Reference for how every listener flow behaves under infrastructure failures:
Postgres outage, broker outage, RPC node outage, and a missing destination
(consumer) queue. All behaviors below are derived from the code paths named in
each section.

The listener runs seven message-driven flows per chain:

| Flow | Queue(s) | Dedup lock | Delivery queue |
|---|---|---|---|
| Live cursor | `fetch-new-blocks` | FlowLock (raw chain_id) | `{consumer_id}.new-event` |
| Reorg backtrack | `backtrack-reorg` | FlowLock (shared with cursor) | `{consumer_id}.new-event` |
| Live catchup | `catchup` → `range-catchup` | none (by design) | `{consumer_id}.catchup-event` |
| Finality | `fetch-final-block` | FlowLock (FINALITY salt) | `{consumer_id}.final-event` |
| Final catchup | `final-catchup` → `range-final-catchup` | none (by design) | `{consumer_id}.final-catchup-event` |
| Cleaner / Final cleaner | `clean-blocks` / `clean-final-blocks` | FlowLock (own salts) | — |
| Watch / Unwatch | `control.watch` / `control.unwatch` | — (DB unique index) | — |

All control queues are chain-namespaced (`chain-id-{id}.<key>`); delivery
queues are per-consumer and unscoped.

## The three shared safety rails

Every behavior in the matrix derives from three mechanisms:

1. **Publish-before-commit** (live cursor `cursor_processing`, finality
   `final_processing` — `core/evm_listener.rs`). Events are published *before*
   the block is written to `blocks` / `final_blocks`. Any failure leaves the
   DB tip unchanged, so the retry re-processes the same block. Consequence:
   **events can be duplicated, never lost** (at-least-once, everywhere).

2. **Transient errors retry forever; only permanent errors dead-letter**
   (`classify` in `core/workers.rs`; broker behavior in
   `shared/broker/src/redis/consumer.rs` and `amqp/consumer.rs`). DB, RPC,
   broker, and slot-buffer errors are transient — both broker backends
   redeliver transient messages *infinitely, without incrementing the delivery
   count*. Only `InvariantViolation` (task panic, missing boot anchor) is
   permanent and dead-letters after `max_retries`. An outage therefore parks a
   loop message; it never kills it. Malformed payloads are the one
   non-transient input: deserialization/validation failures dead-letter
   immediately (deterministic — a retry cannot help).

3. **Queue-existence gate on every delivery**
   (`publish_payload_to_consumer`, `core/publisher.rs`). Before each publish,
   `broker.exists({consumer_id}.<suffix>)` is checked:
   - queue missing + `publish_stale: true` (**shipped default**) → retry
     forever, once per `publish_retry_secs` (1s). The handler stalls holding
     its message.
   - queue missing + `publish_stale: false` → after
     `publish_no_stale_retries` attempts, **skip that consumer and continue**
     — that consumer silently misses the block (error-logged).
   - broker error during the check or the publish → propagate immediately →
     handler transient → the whole block is retried.

## Failure matrix

| Flow | Postgres down | Broker down | RPC node down | Consumer queue missing (`publish_stale: true`) |
|---|---|---|---|---|
| **Live cursor** | Tip read / filter read / insert → `DatabaseError`/`FilterFetchError` → transient → infinite redelivery. Lock acquire → transient. **Stalls, self-heals**; retried blocks re-publish to consumers that already got them (duplicates). | Consumers auto-reconnect internally (Redis `force_reconnect`, AMQP reconnection loop) — the process does **not** exit. In-flight message stays pending; continuation publish fails → transient. **Stalls, self-heals.** | Head fetch → `ChainHeightError` → transient. Block fetches retry *inside* the fetcher forever (fixed interval; rate limits back off exponentially, capped). Stalls holding the message; claim-sweeper duplicates are Ack-skipped by the FlowLock. A block whose receipts do not match it (count, tx hashes or `blockHash`: the chain moved between the block and receipts calls) **re-arms** the cursor instead of a transient error: the range is cancelled, the DB tip stays at the last block published and inserted in order, the next iteration resumes from it (`listener_rearms_total{flow="live"}`). See [Stalls and recovery per flow](#stalls-and-recovery-per-flow). | Fan-out loop spins on the missing consumer → cursor frozen, **DB tip stops advancing**. No loss; resumes when the queue appears. |
| **Reorg backtrack** | Same as cursor. The batch commit is one atomic transaction — a crash leaves the DB fully unchanged; the walk is re-run safely. | Same as cursor. | Walk fetches by hash. A reorged block can vanish from the RPC for good (a newer reorg replaced it), so `NotFound` is capped at `strategy.reorg_not_found_max_retries` (default 5) on every call of a by-hash fetch (block and receipts). On the cap, or on receipts not matching the block (count, tx hashes or `blockHash`), the walk is **abandoned before any DB write**, waits `loop_delay_ms`, and the cursor is re-armed (`FETCH_NEW_BLOCKS` + Ack, no transient error); the cursor re-detects the reorg from fresh chain data if it is still real (`listener_rearms_total{flow="reorg"}`). Transport errors still retry forever. | Same fan-out spin (reorged blocks go through the same helper). |
| **Live catchup** | No DB writes; the per-consumer filter read → transient → sub-range retried. | Same reconnect/stall. An orchestrator retry re-publishes already-sent sub-ranges (duplicates; downstream dedupes by block number/hash). | Orchestrator head fetch → transient; range fetches retry forever → the sub-range stalls. | Spins on that consumer's `catchup-event` queue. No FlowLock — see head-of-line note below. |
| **Finality** | Identical to the live cursor, against `final_blocks`. Fully isolated: own lock, queue, and tip — a stalled finality flow never touches the live flow, and vice versa. | Same as cursor. | `get_final_block_number` → `ChainHeightError` → transient; block fetches retry forever. | Fan-out spin → **finality tip frozen** (the final cleaner simply idles at the tip). Live flow unaffected. |
| **Final catchup** | Same as live catchup (FINAL filter query). | Same as live catchup. | Final-head fetch → transient. | Same as live catchup, on `final-catchup-event`. |
| **Cleaners** (both) | Delete errors are **swallowed** (logged, iteration skipped); the loop continues and reschedules. Lock error → transient. | Reschedule publish fails → transient → message unacked → loop recovered by redelivery. | Not used — unaffected. | No destination queues — unaffected. |
| **Watch / Unwatch** | DB error → transient → command retried. Atomic WATCH rolls back all additions before retry; invalid members reject the entire command without writes. | Reconnect; the command stays pending. | Not used. | N/A (writes to Postgres, not to queues). |

## Stalls and recovery per flow

How each flow reacts to every class of fetch problem. "Stall" means the flow
holds its message and makes no progress until the cause clears; nothing is
lost (the DB tip, or the catch-up sub-range, does not move past an
unpublished block).

### RPC and data-integrity errors

| Problem | Live cursor | Reorg walk | Live catch-up / final catch-up | Finality | Startup validation / init block / final anchor |
|---|---|---|---|---|---|
| Transport error, timeout, 5xx | Retried forever inside the fetcher (fixed `retry_interval`). Stall, self-heals. | Same. | Same; the sub-range stalls. | Same. | Same (boot blocks). |
| Rate limited (429) | Exponential backoff, capped by `max_exponential_backoff_ms`. | Same. | Same. | Same. | Same. |
| `null` result (`NotFound`), by number | Retried forever (normal head lag clears it). | — | Retried forever (a pruned node returning `null` for old blocks stalls the sub-range). | Retried forever. | Retried forever. |
| `null` result, by hash | — | Capped at `reorg_not_found_max_retries` (default 5, i.e. 6 attempts) per call, block and receipts alike → **abandon + re-arm**. | — | — | — |
| Receipt count ≠ tx count, or a tx without receipt | **Re-arm** (no transient error, no circuit breaker). | **Abandon + re-arm.** | Transient → sub-range redelivered. | Transient → redelivered. | **Panic** (strategy incompatible). |
| Receipt `blockHash` ≠ block hash | **Re-arm.** | **Abandon + re-arm.** | Not checked: published as before. | Not checked. | Not checked. |
| Block computer mismatch (`compute_block: true`) | Transient → redelivered (`CouldNotComputeBlock`). | Transient → redelivered (the walk is retried). | Transient. | Transient. | Panic. |
| Unrecoverable RPC error (`UnsupportedMethod`, `DeserializationError`, `BatchUnsupported`) | Fetcher stops at once → transient → redelivered (same result until config/node changes). | Same. | Same. | Same. | Panic. |
| DB, broker, missing consumer queue, FlowLock | See the [failure matrix](#failure-matrix). | | | | |

`—` = the flow never makes that call.

### Receipt integrity checks per flow

`build_fetched_block` (`blockchain/evm/evm_block_fetcher.rs`) is shared by
every flow. The flags below are set per call site in `core/evm_listener.rs`.

| Check | Live cursor | Reorg walk | Catch-up / final catch-up / finality | Startup / init / anchor |
|---|---|---|---|---|
| Receipt count = tx count | yes | yes | yes | yes |
| Every tx has a receipt | yes | yes | yes | yes |
| Receipt `blockHash` = block hash (`verify_receipt_block_hash`) | **yes** | **yes** | no | no |
| Block recomputation (`compute_block`) | config | config | config | config |

Why the `blockHash` check only in live and reorg: those two flows work at the
chain head, where the block and its receipts can come from two different
branches (a reorg between the calls, or two load-balanced upstreams). Receipts
are never fetched by block hash (strategy 1 uses `eth_getBlockReceipts(number)`
even in the by-hash path; strategies 2–5 fetch by tx hash), so a same-tx sibling
block would otherwise pass the count and coverage checks with receipts that
are not its own. Catch-up reads historic ranges and finality reads finalized
blocks, where such a race is unlikely; they deliberately keep their exact
pre-existing behaviour (no new failure mode on historic data).

### What "re-arm" means

- No gap: the cursor publishes and inserts strictly in order, so the DB tip is
  the last block fully handled (live: possibly some blocks of the cancelled
  range; reorg abandon: the DB is untouched). Nothing of the failing block was
  published, since the integrity checks run before publish.
- The handler publishes `FETCH_NEW_BLOCKS` and Acks (same as a normal
  iteration end). No transient error, no circuit breaker, no claim delay.
- The listener waits `loop_delay_ms` first (live arm and reorg abandon alike),
  so a persistent mismatch cannot hot-loop.
- The next iteration starts at `tip + 1` with fresh chain data. A reorg that
  is still real is re-detected at any depth; one that vanished is simply
  followed on the new canonical chain.
- Consumers may receive duplicates (blocks of an abandoned walk are published
  again by the next walk), never gaps.
- An abandon does **not** retract what the walk already published: block N
  (`Live`) and the walked blocks (`Reorged`) stay delivered but are never
  committed. If the chain settles back on the branch the DB already holds,
  those events belong to blocks that never became canonical. Nothing is
  missing: the DB blocks at those heights were published when first inserted.
  This is the same exposure as any reorg flip-flop; finality events are the
  canonical signal.
- Counted by
  `listener_rearms_total{flow="live"|"reorg"}`; each walk also counts in
  `listener_reorgs_total`, so an abandoned and re-detected reorg counts twice.

### Known stalls not covered by re-arm

- A pruned node returning `[]` for an old non-empty block: receipt count
  mismatch → catch-up stalls (transient, redelivered) until the upstream serves
  the data. A `null` instead is `NotFound`, retried forever inside the fetcher
  (same stall). Use an archive upstream for deep catch-ups.
- A node answering a missing block with a JSON-RPC *error* (e.g. "header not
  found") instead of `null`: classified as transport and retried forever, so
  the by-hash cap does not apply. An error message containing "does not exist"
  is classified `UnsupportedMethod` instead (unrecoverable → transient at the
  handler → redelivered).
- A permanent receipt quirk on one block (node or chain always returning
  receipts that do not match it): the live cursor re-arms on it forever and
  the DB tip stops there. This is not a transient error, so it shows only in
  `listener_rearms_total{flow="live"}` and the re-arm WARN, not in
  `listener_transient_errors_total`; watch sync lag and the re-arm rate.
- A block computer mismatch on a block the reorg walk needs: the walk is
  retried transiently; set `compute_block_allow_skipping` for chains with
  unsupported tx types.

## Missing consumer queue — blast radius on other consumers

The flows differ most here, because of **head-of-line blocking**:

- **Live and finality (fan-out flows)**: `publish_block_events` /
  `publish_final_block_events` iterate all matching watchers *sequentially*
  per block. If consumer X's queue is missing, consumers ordered before X
  already received the block; everyone after X receives **nothing** — and
  since the flow stalls, *all* watchers on that flow stop receiving subsequent
  blocks. One absent consumer freezes delivery for the whole flow on that
  chain (live and finality independently). When the queue appears, delivery
  resumes with zero loss; intermediate transient retries duplicate the block
  for the earlier consumers.
- **Live/final catchup (per-consumer flows)**: each sub-range targets one
  consumer, so the spin is scoped to that consumer's replay. However, the
  `range-catchup` / `range-final-catchup` queues are shared per chain with
  `range_prefetch: 1` — on a single pod, a stuck sub-range queues every other
  consumer's catchup behind it. Under HPA, other pods keep draining other
  sub-ranges, and the claim sweeper (idle > `claim_min_idle_secs`) re-claims
  the stuck message to another pod, which — having no FlowLock on catchup —
  runs it concurrently (more duplication, still no loss).
- **`publish_stale: false` flips the trade**: no stalls anywhere — the missing
  consumer is silently skipped after bounded retries instead. For catchup this
  means whole replayed ranges evaporate for that consumer. This knob decides
  "stall everyone" vs "lose delivery for the absentee".
- **Redis nuance**: "queue exists" means the stream key exists, which only
  happens once the consumer's `consume*()` future has run (`XGROUP CREATE …
  MKSTREAM` happens on the consumer side). `ensure_*_consumer()` declares
  topology on AMQP but is a **no-op on Redis** — so consumers should start
  consuming promptly after registering watchers or requesting a catchup.

## Edge cases

- **Claim-sweeper lock/Ack erasure race** (FlowLock'd flows: fetch, finality,
  cleaners): a handler stalled longer than `claim_min_idle` under its lock
  gets its message re-claimed by another pod, which sees the lock held and
  **Acks** — deleting the message from the pending list. If the original pod
  then crashes or errors, that loop's message is gone and the loop halts until
  a pod restart re-seeds it (gated by `automatic_startup` / the flow's
  `active` flags). A missing consumer queue on the fan-out flows is precisely
  the stall that makes this reachable.
- **Boot with RPC or Postgres down**: `validate_strategy_and_init_block` /
  `validate_and_init_final_block` panic deliberately → crash-loop-backoff
  until the dependency returns.
- **Readiness**: a Postgres or broker outage also fails `/readyz`
  (`SELECT 1` + broker health check); the flows themselves recover on their
  own via transient retries — readiness only affects traffic routing.
- **Finality inactive** (`finality_active: false`): final-catchup requests and
  sub-ranges arriving anyway are Ack-dropped with identifying logs
  (deliberate; re-send after enabling). The finality and final-cleaner loops
  never run at all.

## Summary

Every infrastructure failure stalls the affected flow and self-heals with
duplicates-but-no-loss. The only silent-loss paths are opt-in
(`publish_stale: false`) or explicit drops (finality inactive). A missing
consumer queue is the most contagious failure: the default stall semantics
freeze the entire fan-out flow it belongs to — isolated to that flow, never
crossing between live and final.
