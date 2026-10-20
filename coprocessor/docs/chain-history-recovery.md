# Host history recovery

The host poller replays recent history after restart to repair omitted events.
Replay is background repair: workers, uploads, Gateway ingestion and transaction
submission continue draining admitted work. An unavailable host RPC or a rejected
replay block does not itself halt other chains or either stack.

## Replay and readiness

The default restart window is 64 committed blocks. `--replay-window` accepts
1 through 10,000. Pending ranges survive restart and are adopted by replacement
pollers. Overlapping ranges are combined without dropping unfinished history;
adopters process at most 10,000 blocks per chunk and persist the remaining range.
Each startup gets a generation, so completion of an older replay cannot clear a
newer obligation. Running pollers also adopt pending generations, without creating
another generation: a pod that dies during startup cannot strand the fleet's replay.

Replay does not lower the durable cursor. `--seed-start-block` only initializes
an absent cursor. Startup resolves the cursor's actual schema under the cutover
fence, including a concurrent Green promotion. Discarding a Green schema resets
its replay generation without discarding Blue obligations or confirmed halts.

Pending replay blocks upgrade readiness/promotion until repair completes. It
**does not block ordinary runtime writes**. A failed replay may therefore delay
an upgrade while existing workloads continue. Completion wakes both host and
Gateway readiness gates without requiring another block; the gates also recheck
every five seconds. The websocket listener does not
perform the poller's bounded restart replay. It exits on rejected block data
unless a durable containment halt is present;
exhausted database retries rewind the iterator and refetch in-process. Neither
path acknowledges the failed block or silently skips it.

## Confirmed conflicts

Websocket startup catchup uses the observed head and configured finality lag;
it does not mark recent blocks finalized merely because they were fetched.
Host admission compares retained committed block/parent identities under a
per-chain transaction lock, including deferred finalization. Unknown legacy
parent hashes are not contradictions. Sparse catchup may require recovery of an
empty canonical replacement. These finalizer-requested repairs are authoritative
observations below the confirmation boundary, so contradictions are contained
rather than discarded as delayed live forks.

The websocket listener and HTTP poller inspect disconnected finalized segments
once at startup, limited to the 1,000 block heights ending at the database's
finalized tip (not the RPC head). Both sides of the discovery query are bounded.
Adjacent legacy rows with NULL or empty parent hashes remain one segment;
missing heights and known parent mismatches still split segments. Older
disconnected history is explicitly trusted until replay touches it. This is
not a whole-database integrity audit.

After that audit, verification reads only the current committed tip and checks
its RPC hash; it does not repeat segment discovery on each block. Losing the
verified tip, changing stack roles or replacing the history relation invalidates
the baseline and repeats the bounded audit. Process restart also repeats it.
RPC verification holds no database transaction. Successful startup checks survive
retries in batches of at most 64, with identities checked under the history lock.
Successful header reads refresh liveness; failed reads do not.

All finalized admissions, including broker deliveries and deferred finalization,
must attach to an adjacent finalized header under the per-chain history lock.
Only an empty chain can establish an independent initial anchor. An exact replay
of an existing finalized row remains allowed. A historical backfill can attach
backwards to a known successor; unknown linkage must first be recovered from a
coherent header. Missing history returns an explicit recovery request without
inserting the original block or acknowledging its delivery.

Websocket and HTTP recovery ingest complete canonical blocks, including empty
ones, in batches of at most 32 before yielding. A long gap may span more than
1,000 blocks: the audit limit does not cap actual gap recovery. The original
cursor/page acknowledgement advances only once its block commits. Broker
consumers request missing finalized blocks on their separate final-catchup queue
and retry the original delivery. This prevents sparse traffic from establishing
new disconnected finalized anchors without repeatedly scanning retained history.

Replay also checks immutable computation
metadata. Legacy NULL operand-boundary masks remain NULL and retain the worker's
legacy inference; replay compares masks only where a mask was already recorded.
Other recorded execution metadata remains immutable for legacy rows too.
A confirmed identity, execution-metadata or committed-inventory contradiction records a durable halt
in `public.chain_recovery_halts`. A halt blocks guarded writes and submissions
across this database, including both stacks: dependencies can cross host chains.
It is not cleared by restarting a service. Automatic drift rollback also waits
for audited recovery, preserving its pending signal for retry. SNS retains a dequeued upload while
contained; Gateway ingestion fails without advancing its durable cursor.

The poller and websocket listener observe halts before ingestion startup, after
failed writes, and with a five-second polling interval while idle. They pause with healthy
`/liveness` and failing `/healthz`, rather than repeatedly exiting. A paused
listener requires restart after audited recovery, even after the halt is
resolved: it must reload cursors and discard cached or buffered history.
Transient observer errors, including a pool closed during reconnect, are retried
against the current pool, with a 20-second deadline covering retries and hung
reads. Failed observations do not refresh ingestion liveness. Expiring the
deadline exits the listener even when probes are disabled. Database errors fail
readiness; they are not evidence of a history conflict. A confirmed halt still
keeps liveness healthy while awaiting audited recovery.

For ordinary host work, "finalized" is the application's configured confirmation
depth (`--finality-lag` for the poller), not the RPC's Ethereum finalized tag.
A small lag alone does not halt anything; a later canonical hash change at an
already finalized block does. A reorg within the unfinalized window does not
trigger this policy. Choose the lag for each host chain with the database-wide
containment consequence in mind.

## Alerting

Readiness includes recovery state, but also fails during ordinary bounded
replay; it is not a dedicated containment alert. The poller's existing metrics
endpoint now exports `host_listener_recovery_paused`: zero during normal
operation/replay, one after containment until restart. Both listener modes emit
the structured error `event="chain_recovery_paused"` when they pause. A newly
committed halt emits `event="chain_history_halt"` with its source, stack role,
chain, block number and reason.

The coprocessor Helm chart provides the opt-in critical alert
`CoprocessorChainHistoryContained`. Enable `chainRecoveryAlerts.enabled` and
`hostListenerPollerShared.serviceMonitor.enabled`, with a poller enabled for the
deployment. The alert covers this release's pollers in its namespace and fires
on the first evaluation with a paused poller. Configure
`chainRecoveryAlerts.additionalLabels` to match the Prometheus rule selector and
route the alert through Alertmanager. Outside Helm, alert on
`host_listener_recovery_paused == 1` using the deployment's target labels.
Websocket-only deployments must route the structured error events to their log
alerting system; the websocket binary does not expose the poller's metrics
endpoint. See the [Prometheus Operator alerting documentation](https://prometheus-operator.dev/docs/developer/alerting/).

## Repairs after commitment

An omitted computation or ACL event can be repaired automatically while the
affected inventory has no upgrade state hash or manifest seal. Once either
exists, a replay that adds computations, upgrades their eligibility or changes
the sealed producer inventory rolls back and records a recovery halt. The halt
names the affected producer block, including when its ACL arrives in a later
block. Replaying unchanged events and repairing only dependency-chain scheduling
remain permitted. Roots orphaned by canonical admission are excluded from the
inventory comparison; retracting their fallback grants is normal fork retirement.
Their existing seals and signed archives remain retained.

This deliberately contains at **hash creation / manifest sealing**, even if the
database has not recorded successful publication. An S3 PUT can succeed before
its database transaction fails. Upgrade peers cache fetched commitments, and a
later manifest can already include a block that has no publication of its own.
The existing schema has no durable publication-intent/revision protocol that
would make an automatic rewrite safe. Original hashes, signed manifests and
their history remain unchanged; the repair is not reported as successful.
Refreshing an unpublished digest retains its previous descriptor count as
evidence of sealing. Older rows whose seal was cleared are also protected when
a published descendant includes them in its history.

Hash creation, manifest preparation/sealing and ingestion share the per-stack,
per-host-chain history fence. Manifest selection skips a busy history fence.
Preparation commits its seals and range commitments before upload, releasing
history admission for ingestion. A separate publisher lock spans both phases on
one database connection. Upload rechecks the selected seal and stack role, and
retains the cutover fence and publication row lock through the bounded S3 operation.
Publication also observes the database-wide recovery halt. Deploy
the listener and consensus-detector changes together before relying on this
protection. No additional migration is introduced.

Recovery must reconcile the commitment history as well as local ciphertexts.
For an affected upgrade, use an audited fresh upgrade attempt/checkpoint rather
than overwriting an S3 hash peers may have cached. For manifest history, retain
the signed archives and use an audited rebuild of mutable publication state for
a new verification epoch; changing the epoch alone does not release old
commitments. Do not merely clear `manifest_published` or rewrite the existing
immutable revision. A general manifest revision protocol remains outside this
change.

Gateway history containment, checkpoint bootstrapping and Gateway reorg detection
are outside this change. No Gateway checkpoint insertion is required at rollout.

These checks do not certify all historical data. Replay is bounded and depends
on trustworthy historical RPC responses. Work can be consumed or published
before a contradiction is discovered. Containment drains transactions already
holding the shared cutover fence; an in-flight submission can finish before the
halt commits. It cannot undo external transactions or previously published
results. A pending legacy fork row is not independently proof of committed
canonical identity; ambiguous pre-upgrade history requires investigation.

## Deployment and recovery

Use the existing migration and upgrade process. Apply the safety-table migration
before starting the new binaries. All writers/controllers sharing the database
must use the repaired build before relying on containment; older binaries do
not honor the halt. No additional Gateway rollout phase is required.

Inspect obligations and conflicts:

```sql
SELECT * FROM public.listener_replay_state WHERE pending;
SELECT id, source, stack_role, chain_id, block_number,
       encode(expected_hash, 'hex') AS expected_hash,
       encode(observed_hash, 'hex') AS observed_hash,
       reason, detected_at, last_detected_at
FROM public.chain_recovery_halts WHERE resolved_at IS NULL;
```

For a confirmed conflict:

1. Stop all services sharing the database. Preserve a DB snapshot, halt records,
   logs and canonical RPC evidence.
2. Verify the first inconsistent boundary, dependent work on every affected
   chain/stack, retained material and external submission receipts.
3. Restore a coherent verified checkpoint or rebuild affected state with the
   deployment's approved recovery procedure. If external effects cannot be
   reconciled, keep the fleet stopped and escalate for protocol recovery.
4. With services stopped, resolve only the audited halt IDs under the exclusive
   cutover lock, recording the recovery reference. Do not clear halts merely to
   make readiness green. Reset replay obligations only for an audited rebuilt
   baseline; ordinary replay clears itself.
5. Start the poller, confirm repair and retained/new-work checks, then resume the
   remaining services. Reconcile external receipts before resuming submission.

Transaction template for step 4 (not a substitute for recovery):

```sql
BEGIN;
SELECT pg_advisory_xact_lock(x'4648455643555456'::bigint);
UPDATE public.chain_recovery_halts
SET resolved_at = NOW(), recovery_note = '<verified baseline and audit reference>'
WHERE id IN (<audited halt IDs>) AND resolved_at IS NULL;
COMMIT;
```
