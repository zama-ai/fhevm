# coproc-mngr

Upgrade orchestrator for the FHEVM coprocessor stack. Implements the FSM and
signaling described in [RFC-021-blue-green-upgrad] 
§ `coproc-mngr`.

What works today:

- LISTENs on the `event_upgrade` channel; falls back to polling
  `upgrade_events` on a configurable interval.
- Drives the GCS FSM `OFFLINE -> SNAPSHOTTING -> REPLAYING -> READY -> SIGNALING`
  on `proposedUpgrade`.
- Drives the BCS FSM `LIVE -> DRAINING -> STOPPED` and the GCS FSM
  `SIGNALING -> CUTTING_OVER -> LIVE` on `CoprocUpgraded`.
- Inserts a row into `signal_ready_pending` carrying the (placeholder)
  `stateCommitment`. The future tx-sender SignalReady op will drain it.
- Per-stage `pg_notify` on
  `event_coproc_mngr_{snapshot_start,replay_start,dry_run_on,dry_run_off,signal_ready,bcs_drain,gcs_promote}`.
- DB-persisted FSM in `service_state` with restart-safe transitions.

Explicitly **out of scope** for v1:

- Real `stateCommitment` derivation (fixed zero-hash placeholder).
- Submitting `signalReady` on-chain (handed off via `signal_ready_pending`,
  but no consumer yet).
- Any handling in other services. They will read `pg_notify` channels in
  follow-up iterations.

`pg_dump | pg_restore` against the snapshot table set is implemented and runs during the `OFFLINE -> SNAPSHOTTING` transition. The BCS DB pool is opened only for the readiness check that precedes it, and is dropped immediately after. `pg_dump` and `pg_restore` are spawned as child processes via `tokio::process::Command` and pipe-linked at the OS level (no in-memory buffering); both are spawned with `kill_on_drop(true)` so a cancelled coproc-mngr does not leak them.

## Tables introduced

See migration `20260430120000_create_coproc_mngr_tables.sql`:

- `service_state` - per-stack FSM. Bootstrapped with `BCS=LIVE`, `GCS=OFFLINE`.
- `upgrade_events` - ingress queue. The future gw-listener routing path
  inserts `proposedUpgrade` / `CoprocUpgraded` rows here. Trigger fires
  `NOTIFY event_upgrade`.
- `signal_ready_pending` - coproc-mngr -> tx-sender handoff.

## Testing manually

Until gw-listener writes to `upgrade_events` automatically, you can drive
the FSM by inserting rows yourself:

```sql
INSERT INTO upgrade_events
    (kind, proposal_id, version, snapshot_block, eval_block, block_number, transaction_hash)
VALUES
    ('proposedUpgrade',
     decode('aabb...','hex'),
     'v0.8.0',
     100, 105,
     12345, decode('cc...','hex'));
```

The trigger fires `NOTIFY event_upgrade`; coproc-mngr picks it up and runs
the full GCS lifecycle, ending in SIGNALING.

To trigger cutover:

```sql
INSERT INTO upgrade_events
    (kind, proposal_id, state_commitment, block_number, transaction_hash)
VALUES
    ('CoprocUpgraded',
     decode('aabb...','hex'),
     decode('0000000000000000000000000000000000000000000000000000000000000000','hex'),
     12350, decode('dd...','hex'));
```

The same coproc-mngr instance handles both BCS-drain and GCS-promotion paths
based on which `service_state` rows look non-trivial.

## CLI

```
coproc-mngr \
    --database-url postgres://.../greencs \
    --bcs-database-url postgres://.../bluecs   \ 
    --upgrade-event-channel event_upgrade      \
    --poll-interval 10s                        \
    --readiness-timeout 30m                    \
    --readiness-poll-interval 5s               \
    --health-check-port 8080                   \
    --metrics-addr 0.0.0.0:9100
```

## Metrics

Exposed on the `--metrics-addr` HTTP server (Prometheus text format):

- `coproc_mngr_inbound_event_total` - events dispatched.
- `coproc_mngr_inbound_notification_total` - NOTIFYs received.
- `coproc_mngr_inbound_poll_total` - polling-fallback ticks.
- `coproc_mngr_event_success_total{stage}` - successful handler stages.
- `coproc_mngr_event_fail_total{reason}` - handler failures.
