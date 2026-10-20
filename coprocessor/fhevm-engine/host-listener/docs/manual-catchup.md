# Persisted manual catchup

`host_listener_consumer` can replay historical blocks while its live and finalized
subscriptions continue. Deploy listener-core with catchup identities (#4116)
and atomic WATCH before enabling this feature. Host-consumer declares delivery
queues first, then registers its complete live/finalized contract set in one
transaction, avoiding replay while only part of the contract set is visible.

```sh
--catchup-from-block=100 --catchup-up-to-block=200
--catchup-from-block=-1000
--catchup-from-block=-1000 --catchup-unfinalized
```

Both bounds are inclusive. Nonnegative values are absolute block numbers;
negative values are offsets from the first live block this process observes,
saturating at genesis. The omitted end is `-1`. An inverted range is rejected.
By default, listener-core clamps the end to its finalized head; finality must
be enabled in core. `--catchup-unfinalized` instead replays up to the current
chain head, including blocks which can still reorg. Both modes are bounded
replays, not requests to wait for future blocks.

## Retaining requests

`host_consumer_catchups` stores the original signed parameters, the absolute
bounds resolved once, the `finalized` mode, and the UUID sent as `catchup_id`.
A restart with the same parameters and mode reuses the stored UUID and bounds, without waiting for another live
block. Two replicas serialize their changes in PostgreSQL, so matching requests
reuse one ID even when their first live blocks differ.

A newly created request and its creator's initial lease are committed in one
transaction. Another listener can never observe that new request as unowned.
Reusing matching parameters preserves an existing owner's lease.

The broker identity remains `host-listener-consumer.<chain-id>`, independent of
the telemetry service name. Existing consensus-version group suffixes and group
retirement are preserved. Replicas sharing the identity and database share the
replay work; different consumer identities do not take over each other's requests.

| Startup | Behavior |
|---|---|
| No replay flags, no retained request | Consume live/finalized blocks; create no replay request. |
| No replay flags, retained requests | Claim unowned or expired requests and re-send their stored IDs and bounds; leave active owners alone. |
| Matching replay flags | Reuse the ongoing ID and bounds. |
| Same bounds, different finality mode | Select a different request identity; suppress the previous request unless `--no-override-catchup` is set. |
| Different replay flags | Create a request and suppress/cancel other ongoing requests. |
| Different flags with `--no-override-catchup` | Add the request and retain the other ongoing requests. |

Unowned requests created by another replica are discovered by the command worker
within five seconds. Each replay delivery checks the shared state instead of an ID set
captured at startup. A suppressed or unknown ID is acknowledged without ingesting
its block, as is an ID delivered on the wrong catchup flow. Database failures remain retryable. Suppression is serialized with the
block's ingestion transaction, so an old-request block cannot commit after the
suppression transaction has committed.

## Publication and cancellation

The database records command intent before publication. `request_pending` is
cleared after the broker acknowledges a request. Suppression sets
`cancel_pending`, which is cleared only after publishing the cancel. A restart
or another replica retries commands left pending by a crash after acquiring the
lease. Acquiring an ongoing request re-sends its ID once; renewing ownership
does not re-send it. Core deduplicates already-dispatched requests. The IDs and
bounds never change on a retry. Cancellation commands precede request commands
within each owner's batch.

The persisted mode selects `request_final_catchup` / `cancel_final_catchup_request`
or `request_catchup` / `cancel_catchup_request`. A start without replay flags
resumes claimable requests of either mode. Both delivery queues remain subscribed,
so retained requests do not depend on this process's startup mode. The request
worker runs separately; the consumer waits for all four read flows before cleanup.

## Ownership and takeover

`owner_id` and `lease_until` form a renewable coordination lease. Unowned rows
have both fields null. Each acquisition batch receives a fresh UUID owner token,
including reacquisition by the same process. A token identifies an ownership
grant, independently of the stable consumer ID. The database clock grants a
60-second lease. The command worker checks ownership every five seconds and
extends a valid lease only when 30 seconds or less remain, giving roughly
30-second renewal intervals with up to five seconds of polling delay. Other
polls read current command state without extending the deadline. This heartbeat
measures coordinator/database availability, not block-processing progress: a
stuck catchup can retain its lease while its coordinator remains healthy.
Pending cancellations are leased too, so a surviving owner can finish them.

Claims are atomic and skip locked rows. Another replica cannot claim an active
lease. Once a lease expires, another worker can claim that same catchup ID and
its original bounds. An expired token cannot be renewed. The old worker detects
missing renewals and stops coordinating those requests. Request/cancellation
acknowledgements also require the current, unexpired token, so delayed command
callbacks cannot update a successor's state. A fresh token on
reacquisition prevents older callbacks from becoming valid again.

A stopped or stalled process cannot react until it runs again. A command already
sent to the broker cannot be recalled; retries retain the same idempotent
catchup ID. Broker publication is bounded to 15 seconds, and publication or
database errors trigger bounded consumer shutdown and return an error so the
process supervisor can restart it. Coordinator panics or unexpected exits do
the same. On shutdown, leases expire naturally; takeover happens after expiry
on a subsequent poll.

Leases govern request coordination. They do not restrict
block ingestion to one process: replicas in the same broker group continue
sharing delivery, including deliveries already in flight during takeover.
Operator replacement still suppresses other ongoing requests immediately in
shared state; their owners publish the persisted cancellation on their next
poll, or a successor does so after lease expiry.

Core cancellation does not stop work already in flight. Local suppression drops
that late delivery by ID. A process detected as retired at startup does not start the command worker or
readers. At cutover, the replay reader stops and drains with the live/finalized
readers before groups are released. Cancellation here does not perform drift cleanup or autonomously
restart replay after cleanup. Replay waits while drift cleanup is unfinished;
live ingestion retains its existing drift recovery policy.

## Status and completion

`ONGOING` means the consumer retains and accepts the request. `SUPPRESSED` means
it no longer accepts it. The store supports an idempotent, consumer/chain-scoped
`ONGOING` to `DONE` transition that releases ownership,
and clears pending request publication without overwriting suppression.
A `DONE` request is no longer resumed at startup, but
its queued deliveries remain accepted so another replica can finish draining.
Matching replay flags reuse only ongoing requests; after `DONE`, explicitly
starting with those flags creates a new request once a live reference is available.

The handler caches each catchup's immutable `block_end` by `catchup_id` when it
first receives a catchup payload. After successfully ingesting a block
whose number equals that bound, it marks that request `DONE`. Any replica can
record this observation, including one which does not own the command lease.
Failed ingestion does not mark completion; database errors during completion
leave the delivery retryable. Suppressed requests cannot become `DONE`.

This status is informative: other replicas can still be processing earlier
deliveries. It relies on listener-core publishing the last block last.
If core clamps the request to a finalized or current head below the stored `block_end`,
the request remains `ONGOING`. `consume_final_catchup()` continues reading the
shared queue after an individual request completes; subscription cancellation
or errors do not mark requests done.

## Validation

From `coprocessor/fhevm-engine`, supply PostgreSQL and a Redis broker:

```sh
cargo test -p host-listener --lib consumer::catchup::
CATCHUP_BROKER_URL=redis://127.0.0.1:6379 cargo test -p host-listener --test consumer_catchup_tests -- --ignored
cargo test -p host-listener --test consumer_catchup_store_tests -- --ignored
```

The integration tests use the existing test harness (Docker by default). Its
localhost reset mode must only point to a disposable database, and must run with
one test thread. The tests verify parameter/ID reuse, concurrent replicas,
replacement, retained cancellation commands, startup without replay flags,
lease exclusion/renewal/takeover, stale-owner fencing, out-of-order replay,
finalization, and live ingestion alongside replay.
