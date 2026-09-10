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
reorg and catchup payloads cannot supply it. These manual bounds remain fixed
across drift restarts. A queued live backlog means this reference can be behind
the RPC head. Inverted ranges are rejected. Absolute bounds do not have to be
below the live reference. Listener-core retains its existing behavior: it caps
the end at its actual RPC head and skips a range starting above that head.
Future scheduled replay is not supported.

The consumer declares its delivery queues before registering contracts.
It registers all contracts in one atomic WATCH registration, so a fresh subscription
cannot publish a live block with only part of the contract set active.
It starts the flows, waits for the live reference, and publishes the resolved
range using the existing listener-core catchup protocol. Request publication failure or
consumer failure stops the process with an error; live processing does not wait
for replay to finish.

Without manual flags and without a recorded drift, startup requests no replay.
Delivery queues are declared so automatic drift recovery is always available.

## Drift restart

Drift recovery restarts live and replay processing after database cleanup and
requests the missing history. It does not depend on another host listener
rebuilding the database.

The consumer polls `drift_revert_signal` even when no blocks arrive. Every block
handler also checks that its subscription's database drift checkpoint is current
before ingesting. Query failures stop or retry work rather than bypassing the
check; each drift query has a timeout.

On a new signal, a changed boundary, or a signal becoming unfinished again:

1. Cancel both live and replay intake. Cancel active ingestion and wait for its
   local barrier to drain, then await the consumer tasks. Cancelling ingestion
   drops its transaction; the database driver rolls it back.
2. Publish UNWATCH for the retired subscription's contracts.
3. Wait for cleanup to be Done. Pending, Reverting and Failed keep ingestion
   stopped. Cleanup is still owned by the existing gw-listener revert runner;
   a Failed signal requires the existing operator retry/acknowledgement flow.
4. Create a new subscription with fresh dependency caches, queues and registration.
5. Observe its first live block H and request replay through H − 1 while live
   processes H onward. The start is the earliest observed drift point or the
   original manual start, whichever is lower. A higher explicit manual end is
   retained. A drift start at or above H needs no replay below H unless the
   manual range also requires it.

A pending signal lowered to an earlier block is picked up while waiting for
cleanup. Later signals never raise the earliest required replay start during
this process. Repeated observation of the same completed signal does not cause
another restart. If drift occurs again during recovery, the loop repeats.

At process startup, an existing Done drift signal also triggers replay from its
boundary. This is conservative recovery for a recorded drift. General automatic
catchup from the last persisted block, when no drift signal exists,
is not implemented. Use explicit manual bounds when such a shutdown gap needs
repairing.

The consumer does not declare replay completion: chunks can finish out of order,
so receiving the last block is insufficient. Both flows remain active; neither
waits for another listener's database tip. Retired-stack protections still apply.

## Subscription identities and resources

The runner constructs a consumer ID as `<service>.<chain>.<session>.s<subscription_id>`.
Each process creates a fresh UUID session ID. The subscription counter starts at
0 and increments after each drift-revert; a fresh subscription is then created. The listener library derives delivery
queue names from this consumer ID, for example:

```text
<service>.<chain>.<session>.s0.new-event
<service>.<chain>.<session>.s0.catchup-event
```

The complete consumer ID owns its filters. Each process has independent live
and replay delivery. Old messages cannot enter the new subscription, including
messages published after cancellation. A process restart creates a new session
and does not reuse its old queues. Ingestion remains idempotent.

UNWATCH is asynchronous and best effort on shutdown; failure is logged. Old
queues, streams and dead-letter resources are not deleted automatically, and
in-flight core work is not synchronously cancelled. Retire these resources
through broker maintenance. Old replay requests use ordinary catchup semantics:
once filters are removed, core skips publication instead of waiting for those
filters to return. A future acknowledged retirement protocol can replace this
resource lifecycle. Raw broker metrics retain subscription-specific topic labels;
application metrics remain keyed by chain ID.

When upgrading from stable queue identities, retire the old shared registration
and queues only after all consumers using them have stopped.

## Deployment and validation

Deploy the updated listener-core before deploying this host-consumer: startup
requires atomic WATCH support, even without manual catchup. Atomic WATCH uses
an array of ordinary filter commands on the existing WATCH topic. Core still
accepts legacy single commands; atomic registrations add filters rather than replacing them.
Existing partial registrations are not deactivated while an atomic registration is pending.

Each subscription declares its delivery queues before publishing its complete
contract set atomically. Its first live block therefore proves the registration
is active; catchup uses `request_catchup()` with that same subscription identity.
No schema change or new core cancellation protocol is required.

Unit tests cover replay bounds, drift checkpoints and cancellation/draining:

```sh
SQLX_OFFLINE=true cargo test -p host-listener --lib consumer::
```

The ignored drift tests require Docker for a disposable PostgreSQL database and
an isolated Redis or RabbitMQ broker. A control peer stands in for listener-core
RPC fetching, with ABI-encoded TFHE and ACL events exercising real ingestion.

Drift tests exercise the subscription runner:

```sh
DRIFT_RECOVERY_BROKER_URL=redis://127.0.0.1:6379 SQLX_OFFLINE=true cargo test -p host-listener --lib consumer::runner::tests -- --ignored
```

They cover recovery with and without manual flags, lower drift boundaries,
rejection of old-subscription messages, and recovery after the production revert
SQL deletes computations and permissions. The latter checks that replay restores
the exact original handles in both tables.

For full-stack detection, cleanup, RPC replay and compute/decrypt validation, run
`./fhevm-cli test ciphertext-drift-consumer-recovery` from `test-suite/fhevm`
after booting a `two-of-three` stack with this host-consumer and listener-core.
This profile temporarily stops legacy host listeners and pollers on every
operator and host chain. See the test-suite README for CI instructions.

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
material. Downloads do not block broker acknowledgement. The worker stops with
its delivery subscription during drift recovery and shutdown, and respects stack
retirement. No additional CLI option is required.

The consumer subscribes to live, catchup, finalized and finalized-catchup flows.
Live and finalized contract filters are registered atomically. Finalized payloads
are re-ingested idempotently, then their exact chain/block hash is finalized in
the same transaction, including orphan cleanup and deferred fallback grants.
Only a successful commit permits acknowledgement; contradictory ancestry retries.
Ordinary catchup never implies finality.

The KMS worker checks candidate block status in PostgreSQL. It processes only its
own chain, including compressed migration of an existing key, and requires no
RPC connection. Legacy listener/poller callers retain their additional RPC gate.

Configure `blockchain.finality_active: true` in listener-core. With
`blockchain.finality_tag: true`, core uses the chain's RPC `finalized` tag;
otherwise core uses its configured `finality_depth`. The consumer trusts the
selected policy and has no separate finality flag. Disabling core finality leaves
pending activations waiting; the consumer does not infer finality from live depth.

Manual and drift replay request both ordinary and finalized catchup for the same
resolved bounds. Core clamps finalized replay to its finality boundary. All four flows
and the KMS worker stop together during drift recovery and shutdown.

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
