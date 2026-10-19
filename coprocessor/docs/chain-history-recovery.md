# Host history recovery

The host poller replays recent history after restart to repair omitted events.
Replay is background repair: workers, uploads, Gateway ingestion and transaction
submission continue draining admitted work. An unavailable host RPC or a rejected
replay block does not itself halt other chains or either stack.

## Replay and readiness

The default restart window is 64 committed blocks. `--replay-window` accepts
1 through 10,000. Pending ranges survive restart and are adopted by replacement
pollers. Overlapping ranges are combined, with an absolute 10,000-block bound;
a larger obligation requires operator recovery. Each startup gets a generation,
so an older replica cannot clear a newer replay obligation.

Replay does not lower the durable cursor. `--seed-start-block` only initializes
an absent cursor. Startup resolves the cursor's actual schema under the cutover
fence, including a concurrent Green promotion. Discarding a Green schema resets
its replay generation without discarding Blue obligations or confirmed halts.

Pending replay blocks upgrade readiness/promotion until repair completes. It
**does not block ordinary runtime writes**. A failed replay may therefore delay
an upgrade while existing workloads continue. Completion wakes both host and
Gateway readiness gates without requiring another block; the gates also recheck
every five seconds. The websocket listener does not
perform the poller's bounded restart replay. It exits on rejected block data;
exhausted database retries rewind the iterator and refetch in-process. Neither
path acknowledges the failed block or silently skips it.

## Confirmed conflicts

Websocket startup catchup uses the observed head and configured finality lag;
it does not mark recent blocks finalized merely because they were fetched.
Host admission compares retained committed block/parent identities under a
per-chain transaction lock, including deferred finalization. Unknown legacy
parent hashes are not contradictions. Replay also checks immutable computation
metadata. Legacy NULL operand-boundary masks remain NULL and retain the worker's
legacy inference; replay compares masks only where a mask was already recorded.
Other recorded execution metadata remains immutable for legacy rows too.
A confirmed identity, execution-metadata or committed-inventory contradiction records a durable halt
in `public.chain_recovery_halts`. A halt blocks guarded writes and submissions
across this database, including both stacks: dependencies can cross host chains.
It is not cleared by restarting a service. Automatic drift rollback also waits
for audited recovery, preserving its pending signal for retry. SNS retains a dequeued upload while
contained; Gateway ingestion fails without advancing its durable cursor.

## Repairs after commitment

An omitted computation or ACL event can be repaired automatically while the
affected inventory has no upgrade state hash or manifest seal. Once either
exists, a replay that adds computations, upgrades their eligibility or changes
the sealed producer inventory rolls back and records a recovery halt. The halt
names the affected producer block, including when its ACL arrives in a later
block. Replaying unchanged events and repairing only dependency-chain scheduling
remain permitted. Admitting a different unfinalized fork does not change the
sealed inventory of its sibling.

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

Hash creation, manifest selection/sealing and ingestion share the per-stack,
per-host-chain history fence. Manifest selection skips a busy history fence;
selected publication retains it through its bounded S3 operation. Publication
also observes the database-wide recovery halt under the cutover fence. Deploy
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
