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
| **Live cursor** | Tip read / filter read / insert → `DatabaseError`/`FilterFetchError` → transient → infinite redelivery. Lock acquire → transient. **Stalls, self-heals**; retried blocks re-publish to consumers that already got them (duplicates). | Consumers auto-reconnect internally (Redis `force_reconnect`, AMQP reconnection loop) — the process does **not** exit. In-flight message stays pending; continuation publish fails → transient. **Stalls, self-heals.** | Head fetch → `ChainHeightError` → transient. Block fetches retry *inside* the fetcher forever (fixed interval; rate limits back off exponentially, capped). Stalls holding the message; claim-sweeper duplicates are Ack-skipped by the FlowLock. | Fan-out loop spins on the missing consumer → cursor frozen, **DB tip stops advancing**. No loss; resumes when the queue appears. |
| **Reorg backtrack** | Same as cursor. The batch commit is one atomic transaction — a crash leaves the DB fully unchanged; the walk is re-run safely. | Same as cursor. | Walk fetches (by hash) retry forever → stalls. | Same fan-out spin (reorged blocks go through the same helper). |
| **Live catchup** | No DB writes; the per-consumer filter read → transient → sub-range retried. | Same reconnect/stall. An orchestrator retry re-publishes already-sent sub-ranges (duplicates; downstream dedupes by block number/hash). | Orchestrator head fetch → transient; range fetches retry forever → the sub-range stalls. | Spins on that consumer's `catchup-event` queue. No FlowLock — see head-of-line note below. |
| **Finality** | Identical to the live cursor, against `final_blocks`. Fully isolated: own lock, queue, and tip — a stalled finality flow never touches the live flow, and vice versa. | Same as cursor. | `get_final_block_number` → `ChainHeightError` → transient; block fetches retry forever. | Fan-out spin → **finality tip frozen** (the final cleaner simply idles at the tip). Live flow unaffected. |
| **Final catchup** | Same as live catchup (FINAL filter query). | Same as live catchup. | Final-head fetch → transient. | Same as live catchup, on `final-catchup-event`. |
| **Cleaners** (both) | Delete errors are **swallowed** (logged, iteration skipped); the loop continues and reschedules. Lock error → transient. | Reschedule publish fails → transient → message unacked → loop recovered by redelivery. | Not used — unaffected. | No destination queues — unaffected. |
| **Watch / Unwatch** | DB error → transient → command retried until the watcher registers. | Reconnect; the command stays pending. | Not used. | N/A (writes to Postgres, not to queues). |

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
