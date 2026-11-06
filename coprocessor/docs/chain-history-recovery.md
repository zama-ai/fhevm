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
metadata. Legacy NULL block numbers remain unknown; replay does not backfill them
or treat them as conflicting heights. Legacy NULL operand-boundary masks remain
NULL and retain the worker's legacy inference; replay compares heights and masks
only where those values were already recorded.
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

An ordinary database rollback preserves computation admission metadata for
blocks referenced by upgrade hashes or manifest state, including their original
eligibility. It clears execution results and resets the retained computations
for replay; the listener then re-arms their dependency chains. Identical replay
can therefore restore ciphertexts without adding inventory underneath existing
commitments. Uncommitted computation rows are deleted as before. This applies
to both the manual rollback script and automatic drift rollback. Hashes, seals,
signed manifests and producer history are retained. This is not permission to
rewrite published commitments: a genuinely new computation or eligibility
change still triggers containment and requires the audited recovery below.

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

## Gateway history containment

The Gateway listener's durable cursor is a height, so a height alone cannot
distinguish the block it previously scanned from a same-height replacement.
The Gateway checkpoint table records each admitted block hash and parent hash
by source, BCS/GCS role, chain and height. The listener fetches canonical
headers for every bounded log range, rejects logs whose block hash disagrees,
and compares each header with retained checkpoints and adjacent parents under
the per-chain history lock. Checkpoint admission and cursor updates share the
cutover fence. A random cursor revision prevents a range fetched before a
rewind or promotion from committing an obsolete watermark. The listener keeps
the current cursor and its preceding 64 blocks; the extra anchor supports the
64-block restart replay. An unfinished replay retains its older checkpoints
and predecessor anchor until that obligation is completed.

On startup, a nonzero cursor without a checkpoint is treated as an unverified
legacy baseline and halts for operator recovery. With a verified cursor, the
listener requests the last 64 blocks of InputVerification request and final
response events, retaining any older unfinished range. Startup and the steady
loop process that obligation in chunks of at most 64 blocks; a surviving
replica can adopt it without creating a new generation, even on an idle chain.
Matching history is safe to replay through the existing idempotent proof
handlers. Readiness and cutover remain closed until the durable obligation
completes. Completion retains its original stack role across promotion. A changed checkpoint or parent records the
expected and observed hashes in `public.chain_recovery_halts`, revokes the
Gateway dry-run latch, closes readiness and prevents cursor advancement. The
listener does not delete or replace an existing proof row: the service cannot
prove from its schema whether that row has already been consumed or published.
That conservative choice requires audited operator recovery for changed
history.

GCS window alignment pins the rewind anchor before lowering its cursor. During
cutover, GCS checkpoints are promoted with the cursor; an identity conflict
records a halt and aborts the merge. Gateway readiness requires verified
checkpoints for both the GCS and BCS cursor in addition to the existing window,
replay and unresolved-proof checks.

### Gateway rollout baseline

The migration creates the checkpoint table but cannot infer trusted identities
for existing cursors. Before starting binaries with this policy, stop the
Gateway listeners and controllers, take a database snapshot, and verify each
nonzero BCS and GCS cursor against an independently trusted canonical RPC for
the configured Gateway chain. Record the block hash and parent hash together
with the RPC/source and audit reference. Insert one `source='gateway'` row per
role at its verified cursor height. Do not populate a checkpoint from an
unreviewed RPC response: that would bless a possible fork as the baseline. If
the cursor identity or already-consumed work cannot be reconciled, keep the
fleet stopped and rebuild from an audited checkpoint. A zero or absent cursor
does not need a legacy checkpoint.

After migration and verified baseline seeding, deploy the repaired listeners,
workers and controllers together. All participants sharing the database must
honor the durable halt. A pre-migration binary does not enforce it.

### Gateway recovery limits

Automatic replay covers only the latest 64 cursor blocks and depends on the
historical RPC returning canonical, complete logs and headers. History older
than the retained checkpoint window is not audited automatically. A process
can have exposed a proof to downstream work before a contradiction is seen;
containment cannot undo consumed proofs, submitted transactions or published
artifacts. An unfinished replay remains readiness-blocking until a replacement
listener completes it or an operator resolves it as part of a verified rebuild.
Do not clear a halt or replay row solely to restore readiness. For a confirmed
fork, follow the audited recovery procedure below, reconcile external effects,
and record the verified baseline before resolving the halt.

These checks do not certify all historical data. Replay is bounded and depends
on trustworthy historical RPC responses. Work can be consumed or published
before a contradiction is discovered. Containment drains transactions already
holding the shared cutover fence; upgrade hash uploads retain it through their
database bookkeeping, with a 15-second S3 deadline. An in-flight submission can
finish before the halt commits. A timed-out or cancelled upload remains unmarked
in the database; its request may already have reached S3. Containment cannot undo
external transactions or previously published results. A pending legacy fork row is not independently proof of committed
canonical identity; ambiguous pre-upgrade history requires investigation.

## Deployment and recovery

Use the existing migration and upgrade process. Apply the safety-table migration
before starting the new binaries. For a legacy Gateway deployment, complete the
verified baseline procedure above before enabling the listener. All
writers/controllers sharing the database must use the repaired build before
relying on containment; older binaries do not honor the halt.

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

## CFI-0023 persisted self-edge recovery

The split-batch fix prevents new self-dependency edges. It does not rewrite
`dependence_chain` rows created by older binaries. A self-edge is an invalid
scheduler dependency: the chain is waiting on itself even though the executor
already established the transaction's internal log order.

Check for retained self-edges after deploying the fix and before relying on the
affected database:

```sql
SELECT encode(c.dependence_chain_id, 'hex') AS chain_id,
       c.status, c.worker_id, c.dependency_count,
       cardinality(array_positions(c.dependents, c.dependence_chain_id))
           AS self_edge_occurrences
FROM public.dependence_chain c
WHERE c.dependence_chain_id = ANY(c.dependents)
ORDER BY c.dependence_chain_id;
```

If any rows are returned, stop the host listener and all TFHE workers sharing
that database, preserve a database snapshot, and record the query output. Do
not repair a row while it is processing or owned by a worker. A row may be
repaired directly only when it is `updated`, unowned, has at least one pending
dependency, and contains exactly one self-edge. In that case the self-edge is
one invalid gate; remove it and decrement the matching count atomically under
a row lock:

```sql
BEGIN;
SELECT dependence_chain_id, status, worker_id, dependency_count,
       cardinality(array_positions(dependents, dependence_chain_id))
           AS self_edge_occurrences
FROM public.dependence_chain
WHERE dependence_chain_id = decode('<chain-id-hex>', 'hex')
FOR UPDATE;

UPDATE public.dependence_chain
SET dependents = array_remove(dependents, dependence_chain_id),
    dependency_count = dependency_count - 1,
    last_updated_at = NOW()
WHERE dependence_chain_id = decode('<chain-id-hex>', 'hex')
  AND status = 'updated'
  AND worker_id IS NULL
  AND dependency_count >= 1
  AND cardinality(array_positions(dependents, dependence_chain_id)) = 1
RETURNING dependence_chain_id, status, worker_id, dependency_count, dependents;

SELECT pg_notify('work_available', '');
COMMIT;
```

Require exactly one row from the `UPDATE`; otherwise roll back and investigate.
For a processing/owned/processed row, multiple self-edge occurrences, or any
count/edge inconsistency, do not edit the row with this template. Keep the
services stopped and use an audited database recovery with the coprocessor
team. After repair, verify the self-edge query is empty, replay the affected
block, and check that the chain is eligible only after its remaining real
producer gates have discharged. This procedure changes scheduler bookkeeping
only; it does not authorize changing computation metadata or ciphertexts.
