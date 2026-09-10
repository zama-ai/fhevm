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

## KMS processing

The required `--kms-generation-address` specifies the KMS contract to monitor.
A supervised KMS activation worker always runs. Every five seconds it retries the shared legacy processor: cancel orphaned
candidates, activate finalized ready keys/CRS, and download and verify pending
material. Downloads do not block broker acknowledgement. The worker stops on
shutdown, skips work while a drift-revert is in progress, and respects stack
retirement. No additional CLI option is required.

The consumer always subscribes to live and finalized flows. Manual catchup also
subscribes to finalized-catchup. Live and finalized contract filters are
registered atomically. Finalized payloads are re-ingested idempotently, then
their exact chain/block hash is finalized in the same transaction, including
orphan cleanup and deferred fallback grants. Only a successful commit permits
acknowledgement; contradictory ancestry retries. Ordinary catchup never implies
finality.

The KMS worker checks candidate block status in PostgreSQL. It processes only its
own chain, including compressed migration of an existing key, and requires no
RPC connection. Legacy listener/poller callers retain their additional RPC gate.

Configure `blockchain.finality_active: true` in listener-core. With
`blockchain.finality_tag: true`, core uses the chain's RPC `finalized` tag;
otherwise core uses its configured `finality_depth`. The consumer trusts the
selected policy and has no separate finality flag. Disabling core finality leaves
pending activations waiting; the consumer does not infer finality from live depth.

Manual replay requests finalized catchup for the resolved bounds. Core clamps
finalized replay to its finality boundary. Live, finalized, optional
finalized-catchup, and the KMS worker stop together on shutdown.

The library test `consumer::finalization_tests` exercises real PostgreSQL and a
mock material store: pre-final downloads, quiet-chain retries, missed-live CRS
recovery, duplicate final delivery, orphan cancellation, chain isolation, and
transaction rollback on contradictory ancestry. The broker catchup tests also
exercise both finalized queues with real PostgreSQL ingestion.

## Optional contract parity

ProtocolConfig and ConfidentialBridge addresses remain optional, as in the
legacy listener. ProtocolConfig proposals are processed only on the configured
canonical chain. If that role is enabled without a ProtocolConfig address, the
consumer emits the same warning as legacy.

Broker integration tests cover registration, omitted and incorrect addresses,
canonical-chain filtering, bridge source-chain and destination-handle validation,
and deferred fallback synthesis on finalized catchup. Legacy tests remain in place.
Run with Docker and an isolated Redis or RabbitMQ broker:

```sh
CATCHUP_BROKER_URL=redis://127.0.0.1:6379 SQLX_OFFLINE=true cargo test -p host-listener --test consumer_optional_contracts_tests -- --ignored
```
