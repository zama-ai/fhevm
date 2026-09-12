# Host consumer

`host_listener_consumer` receives host-chain blocks from listener-core through
Redis or RabbitMQ and writes supported contract events to the coprocessor
database. Listener-core handles RPC fetching; host-consumer handles ingestion.

## Manual Catchup

Manual catchup replays an inclusive historical block range while live processing
continues. The consumer keeps running after the requested range has been replayed.

Add these flags to the usual `host_listener_consumer` command:

```sh
--catchup-from-block=100 --catchup-up-to-block=200
```

Or replay relative to the first live block observed at startup:

```sh
--catchup-from-block=-1000 --catchup-up-to-block=-10
```

`--catchup-from-block` enables manual catchup. `--catchup-up-to-block` requires
it and defaults to `-1`: one block before the first observed live block. Both
bounds are inclusive. For example, with a live reference of 1000,
`--catchup-from-block=-100` replays `[900, 999]`.
Negative values subtract from that same observed block, saturating at block zero
(including the default end when the reference is genesis).
The reference is captured once from a `LIVE` payload on the configured chain;
reorg and catchup payloads cannot supply it. A queued live backlog means this
reference can be behind the RPC head. Inverted ranges are rejected. Absolute
bounds do not have to be below the live reference. Listener-core retains its
existing behavior: it caps the end at its actual RPC head and skips a range
starting above that head. Future scheduled replay is not supported. Startup
without a start flag does not request or consume manual replay.

The consumer declares its live and catchup queues before registering contracts.
It registers all contracts in one atomic WATCH registration, so a fresh subscription
cannot publish a live block with only part of the contract set active.
It starts both flows, waits for the live reference, and publishes the resolved
range using the existing listener-core catchup protocol. Request publication failure or
consumer failure stops the process with an error; live processing does not wait
for replay to finish.

## Independent instances

The consumer ID is `<service-name>.<chain-id>`, shared by live delivery,
replay requests, and filter registration. The listener library derives delivery
queue names from this ID, including `<consumer-id>.catchup-event` for replay.
Use distinct service names for consumers that need independent replay delivery.
Reusing the same consumer ID after a restart reuses its durable queues;
pending replay and the new startup request may overlap. Ingestion is idempotent.

## Deployment and scope

Deploy the updated listener-core before deploying this host-consumer: startup
now requires atomic WATCH support, even without manual catchup. Atomic WATCH uses
an array of ordinary filter commands on the existing WATCH topic. Core still
accepts legacy single commands; atomic registrations add filters rather than replacing them.
Existing partial registrations are not deactivated while an atomic registration is pending.

This feature does not declare replay completion: different chunks can finish
out of order, so seeing the last block is insufficient. The catchup consumer
stays active alongside live consumption. Drift recovery and finalization/KMS
changes are separate work. Live retains its existing drift checks. Manual replay acknowledges and discards
blocks at or above the earliest drift boundary observed during this run, even
when that signal is already Done. Earlier blocks may still be ingested. Manual
replay does not update live's recovery target. Restarting replay after cleanup
is deferred to a later commit, so this version does not complete manual recovery
across a drift. Retired-stack protections still apply.

## Manual catchup tests

From `coprocessor/fhevm-engine`, use Docker for a disposable PostgreSQL database
and supply an isolated Redis or RabbitMQ broker:

```sh
CATCHUP_BROKER_URL=redis://127.0.0.1:6379 SQLX_OFFLINE=true cargo test -p host-listener --test consumer_catchup_tests -- --ignored
```

These tests call the public consumer entry point. They check historical replay
while live processing continues, out-of-order and duplicate delivery, and an
explicit inclusive end that keeps live consumption running. ABI-encoded TFHE
and ACL events must produce the exact expected handles in both database tables.
A control peer stands in for listener-core RPC fetching.
