# `example` — a reference consumer

A downstream consumer of `listener_core`. It subscribes to a token contract's
`Transfer` events on all four delivery flows (live, final, catchup,
final-catchup), keeps durable bookkeeping of the catchup requests it owns, and
exposes a small HTTP control plane so you can drive and observe catchups by
hand.

No RPC, no database. It talks only to the broker. `listener_core` must be
running against the same broker and `CHAIN_ID`.

## Binaries

| Binary       | What it does                                                                |
| ------------ | --------------------------------------------------------------------------- |
| `example`    | Token-filtered consumer with catchup bookkeeping and the control plane.     |
| `full_block` | Wildcard consumer — every block, twice (live and final). No catchup state.  |

```bash
cargo run -p example                         # the token consumer
cargo run -p example --bin full_block        # the wildcard consumer
```

## Configuration

Everything is environment variables; every one has a default.

| Variable             | Default                                      | Notes                                                                 |
| -------------------- | -------------------------------------------- | --------------------------------------------------------------------- |
| `BROKER_URL`         | `redis://localhost:6379`                     | Append `/5` to use Redis logical DB 5 and keep a test run isolated.   |
| `CHAIN_ID`           | `1`                                          | Must match the listener's `blockchain.chain_id`.                      |
| `TOKEN_ADDRESS`      | `0xA12CC123ba206d4031D1c7f6223D1C2Ec249f4f3` | Zama ERC-20 on mainnet. Point it at a local contract if you have one. |
| `CONSUMER_ID`        | `token`                                      | Prefix of the four delivery queues. Two ids are two independent consumers. |
| `CONTROL_ADDR`       | `127.0.0.1:8088`                             | The control plane is unauthenticated — exposing it is a deliberate act. |
| `CATCHUP_STATE_PATH` | `./catchup-state.json`                       | Durable record of the catchup ids this consumer owns.                 |
| `CATCHUP_START` / `CATCHUP_END`             | unset                 | Pin the live-catchup range instead of deriving it from head.          |
| `FINAL_CATCHUP_START` / `FINAL_CATCHUP_END` | unset                 | Same, for the final-catchup flow.                                     |

If no range is configured and no record exists, the boot range is
`head - 100 ..= head` per flow.

## Control plane

| Route                   | Method | Body                             | Returns                  |
| ----------------------- | ------ | -------------------------------- | ------------------------ |
| `/stats`                | GET    | —                                | per-flow counters        |
| `/catchup/request`      | POST   | `{"block_start":1,"block_end":500}` | `{"catchup_id": "..."}`  |
| `/catchup/cancel`       | POST   | —                                | `{"cancelled_id": "...", "catchup_id": null}` |
| `/final-catchup/request`| POST   | `{"block_start":…,"block_end":…}`| `{"catchup_id": "..."}`  |
| `/final-catchup/cancel` | POST   | —                                | `{"cancelled_id": "...", "catchup_id": null}` |

Requesting a range different from the one currently owned retires the old
catchup and mints a new id — a cancel plus a request, never an id reused with a
different range.

A cancel answers with `cancelled_id`, the id it retired, so the request is
greppable in the listener logs and in `catchup_requests` without having to have
recorded it beforehand. It is read back from the tombstone, not from what was
current before the call: a cancel on a flow that owned nothing returns
`"cancelled_id": null` and reports honestly that it was a no-op. `catchup_id` is
always `null` on these routes — that is what a cancel means.

## Running it locally

### 1. Infrastructure

```bash
cd listener
docker compose up -d postgres redis      # from listener/docker-compose.yaml
```

### 2. A chain

Any EVM RPC will do. For a disposable one:

```bash
anvil --port 8546 --chain-id 31339 --block-time 1 --silent &
# mine some history to catch up over
curl -s -X POST -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","method":"anvil_mine","params":["0x7D0"],"id":1}' \
  http://127.0.0.1:8546
```

### 3. The listener

Copy `crates/listener_core/config.yaml`, point `chain_id`, `rpc_url`,
`database.db_url` and `broker.broker_url` at the above, then:

```bash
cd crates/listener_core
cargo run -p listener_core -- --config /path/to/your.yaml
```

The flag is space-separated (`--config <path>`), and migrations resolve
relative to the working directory, so run it from `crates/listener_core`.

Use a scratch database for test runs — `block_start_on_first_start: "current"`
only applies to an empty database, so a leftover cursor from another chain will
be resumed instead.

```bash
docker exec listener-postgres psql -U postgres -c "CREATE DATABASE listener_scratch;"
```

### 4. The consumer

```bash
BROKER_URL=redis://localhost:6379/5 \
CHAIN_ID=31339 \
CONSUMER_ID=token-dev \
CATCHUP_STATE_PATH=/tmp/catchup-state.json \
cargo run -p example
```

## Driving a catchup

On boot each flow waits for its first block to learn where the chain is, then
requests `head - 100 ..= head` unless a range is configured or a record already
exists. So a freshly started consumer mints two catchups on its own, one per
flow, before you touch anything.

To drive a specific range, use the control plane — it takes an explicit range
and does not wait for a head:

```bash
curl -X POST -H 'Content-Type: application/json' \
  -d '{"block_start":1,"block_end":500}' http://127.0.0.1:8088/catchup/request
# {"catchup_id":"01a0d32c-a4b5-78b0-a107-0dbab1f271e7"}

curl -s http://127.0.0.1:8088/stats
# {"catchup":{"active_id":"01a0d32c-…","delivered":601,"dropped_stale":0}, …}
```

`delivered` is cumulative across every catchup this process has served — 601
here is the 101 blocks of the boot catchup plus the 500 just requested.

Watch it land in the listener's database:

```bash
docker exec listener-postgres psql -U postgres -d listener_scratch -x -c \
  "SELECT catchup_id, status, block_start, block_end, fanned_end,
          sub_ranges, covered, terminal_at
   FROM catchup_requests;"
```

A finished request looks like this — `covered` has coalesced to a single range
spanning `[block_start, fanned_end + 1)`, and the status has flipped:

```
status      | COMPLETED
block_start | 1
block_end   | 500
fanned_end  | 500
sub_ranges  | 5
covered     | {[1,501)}
```

`fanned_end` is the end the fanout actually used — `min(block_end, chain
height)`. Completion is measured against it, not against what you asked for, so
a request that runs past the head still completes.

### Things worth trying

**Cancel mid-flight.** Request a range big enough to take a few seconds, then
cancel it:

```bash
curl -X POST -H 'Content-Type: application/json' \
  -d '{"block_start":1,"block_end":6000}' http://127.0.0.1:8088/catchup/request
sleep 4
curl -X POST http://127.0.0.1:8088/catchup/cancel
# {"cancelled_id":"01a0d32c-a4b5-78b0-a107-0dbab1f271e7","catchup_id":null}
```

The row goes `CANCELLED` with partial coverage, and
`listener_catchup_subrange_discarded_total` counts the sub-ranges the fetcher
threw away unread. Sub-ranges already claimed when the cancel landed still run
to completion — cancellation is prompt, not instant.

**Restart after completion.** Kill the consumer and start it again with the
same `CATCHUP_STATE_PATH`. It re-sends the catchup id it owns; the listener
recognizes the id as terminal and acks without publishing anything. No rows are
added, no blocks are re-delivered, `delivered` starts from 0.

**Request above the head.** A `block_start` past the chain height is recorded
`SKIPPED` with `fanned_end` null and no sub-ranges — the request is answered,
not silently dropped.

## Metrics

The listener exposes these on `telemetry.metrics_port` (`9091` in the sample
config):

```bash
curl -s http://127.0.0.1:9091/metrics | grep listener_catchup
```

| Metric                                     | Meaning                                                        |
| ------------------------------------------ | -------------------------------------------------------------- |
| `listener_catchup_active_requests`         | Gauge of ACTIVE requests, refreshed every 15s.                 |
| `listener_catchup_completed_total`         | Requests that covered everything they fanned out.              |
| `listener_catchup_cancelled_total`         | Requests cancelled by their owner.                             |
| `listener_catchup_cancel_rejected_total`   | Cancels refused because the id belongs to another consumer.    |
| `listener_catchup_skipped_above_head_total`| Requests whose `block_start` was above the head.               |
| `listener_catchup_subrange_discarded_total`| Sub-ranges dropped unread because their request went terminal. |

The consumer's own `/stats` reports `delivered` and `dropped_stale` per flow.
`dropped_stale` counts blocks that arrived stamped with a catchup id this
consumer has since retired.

## Catchup state file

```json
{
  "catchup": {
    "retiring": null,
    "current": { "id": "01a0d32e-…", "block_start": 3000, "block_end": 3300 },
    "cancelled": null
  },
  "final_catchup": { "…": "…" }
}
```

`current` is what this consumer owns, `retiring` is set across a
cancel-then-request so a crash mid-swap is recoverable, and `cancelled` is the
tombstone that stops a bare cancel from being undone by the next boot deriving
a fresh range from head.

The listener never retires a catchup on the consumer's behalf, which is why
this file exists. Deleting it and restarting will mint new ids for whatever
range is then configured.

One process per state file. Replicas sharing a `CONSUMER_ID` need to share the
durable state or run as a singleton.

## Tests

```bash
cargo test -p example
```

Covers the range-resolution precedence rules (env, record, tombstone, head
fallback), concurrent reconciliation of the two flows, the stats counters, and
the control routes.
