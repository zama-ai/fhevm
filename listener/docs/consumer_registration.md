# Consumer registration

`ListenerConsumer::register_contracts(&contracts)` registers all live contract
filters atomically. For a fresh subscription, core sees either no filters or the
complete set, so the first published block cannot contain only part of that set.

## Startup order

1. Declare the live queue with `ensure_consumer()`.
2. Declare the replay queue with `ensure_catchup_consumer()` if replay is needed.
3. Call `register_contracts()` once with the complete contract address list.
4. Consume events and request replay as needed.

The method returns after publishing the command, not after core commits it.
Queues must already exist when activation makes the subscription visible.
See the [live example](../crates/example/src/live_events.rs).

## Wire format

The existing chain-namespaced `control.watch` topic accepts a legacy single
filter object or an array of filter objects (`WatchCommand::Atomic`):

```json
[
  {
    "consumer_id": "host.1",
    "log_address": "0x0000000000000000000000000000000000000001"
  },
  {
    "consumer_id": "host.1",
    "log_address": "0x0000000000000000000000000000000000000002"
  }
]
```

Omitted `from` and `to` fields mean no address restriction on the transaction;
omitted `filter_type` means LIVE. Chain scope comes from the topic namespace.
The atomic array must be nonempty. Core validates every member before opening
the insertion transaction. Invalid commands are dead-lettered without writes;
a database error rolls back all additions and retries the complete command.
Existing filters and duplicate registrations are retained without duplication.

This is atomic addition, not subscription replacement: previously registered
partial filters remain active while additions are pending. It does not repair
messages already published with those filters.

## Compatibility and scope

Deploy updated listener-core before callers use the new `register_contracts()`.
Older core accepts only a single object and cannot process atomic arrays. New
core continues to accept single objects from older clients. No schema migration
is introduced by this change, and Redis and RabbitMQ use the same handler.

`register_filter()` still supports individual custom filters. UNWATCH and
`unregister_contracts()` still remove filters individually. The separate
`register_final_contracts()` helper is unchanged and sends individual commands;
the safe default described here applies to live contract registration.

Host-consumer uses atomic registration even when manual catchup is disabled.

## Verification

Protocol validation tests run with `cargo test -p primitives watch_batch_tests`.
The repository rollback/retry test requires a disposable PostgreSQL database:

```sh
SQLX_OFFLINE=true cargo test -p listener_core batch_watch -- --ignored
```

Set `BATCH_WATCH_DATABASE_URL` to that disposable database before running the
command. The test applies migrations and temporarily installs a trigger that
rejects the second insert, then verifies rollback and idempotent retry. Do not
point it at a shared or production database.

### Combined live and finalized registration

`ListenerConsumer::register_contracts_with_finality(&contracts)` registers both
sets in one atomic WATCH command. Declare live, catchup, final, and final-catchup
queues first. A live delivery then proves the complete filter set is active.
Use `consume_final` and `consume_final_catchup` for finalized deliveries, and
unregister both sets on shutdown. Finality policy belongs to listener-core;
consumers do not choose an independent depth or RPC tag.
