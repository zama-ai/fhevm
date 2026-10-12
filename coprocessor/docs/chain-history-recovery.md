# Chain history recovery

This policy applies to main and the v0.15 backport of CFI-0002 and CFI-0012.
The coprocessor automatically replays unchanged host history to repair omitted
events. A change to committed host or Gateway history requires operator recovery.

## Automatic recovery and limits

The host poller re-fetches its last 64 committed blocks on every restart, plus
any unfinished replay range recorded by another replica or an earlier process. Set
`--replay-window` to a value between 1 and 10,000 for a larger bounded replay.
`--seed-start-block` remains an initialization anchor, used only when no saved
cursor exists. It does not rewind an existing cursor. Replay does not lower the
durable cursor; a crash during replay causes the recorded window to start again.
Overlapping restarts atomically adopt their combined range, capped at 10,000
blocks. Every start records a new completion generation. An older replica cannot clear
a newer obligation, even if both ranges are identical. Exceeding
that absolute bound keeps readiness closed and requires operator recovery.

Host admission checks the block and both retained committed neighbors in the
same transaction as its writes, under a per-chain lock shared by listener
replicas. This also checks continuity at batch boundaries. A replacement
poller adopts unfinished replay even when its cursor is absent. Discarding a
green schema during upgrade rollback invalidates that schema's replay
generation in the same exclusive transaction; Blue replay obligations and
genuine history halts are retained.

Host startup reads the cursor and registers replay in one fenced transaction,
using the cursor's actual schema to determine replay ownership. A seed resolved
outside that transaction cannot replace a cursor created or promoted meanwhile.
The completion token retains the resolved owner and generation, so a process
queued behind promotion cannot leave an unowned Green replay after restart.

Replay repairs omissions only where the canonical block identity has stayed
unchanged. Derived workers and transaction submission wait until the recorded
replay completes. The host and Gateway upgrade readiness checks also remain
closed. Raw ingestion continues so it can finish the replay.

The poller checks its committed tip on every iteration, including idle ones,
and checks stored committed/finalized identities before replaying each block.
The Gateway listener pins every block in a fetched range before admitting its
events, checks log/header identity and header parent linkage, and rechecks the
saved tip even when no new block exists. It retains the tip and at least 64
preceding checkpoints; a range that has not committed progress remains pinned.
Gateway block batches are limited to 1 through 1,000 blocks.

A hash change at a stored committed checkpoint halts the whole coprocessor
database, across host chains and both stacks. The Gateway policy also contains
changes to admitted but unfinished ranges: it cannot prove that their earlier
events have not already been consumed. No automatic deletion, replacement of
old proofs, or reversal of published results is attempted.

Legacy Gateway cursors above genesis have no recorded block hash. Their first
start after this migration requires an operator to establish a verified baseline;
the listener records a halt instead of trusting today's RPC response over data
accepted under the old cursor. Plan this step during the v0.15 rollout.

Detection depends on a trustworthy canonical RPC endpoint. A hash-consistent
response can omit events without exposing the omission at that time. Restart
replay is bounded; omissions outside the configured host window, unavailable
historical RPC data, and an unknown legacy baseline require operator recovery.
This policy does not implement arbitrary rollback of computed or published
history. It does not validate TFHE arithmetic.

## Durable containment

Deploy the repaired builds to every writer, sender and controller before relying
on this fence. Older binaries do not read the containment tables. A mixed fleet
can continue accepting work after a halt, so stop legacy services during rollout.

`public.chain_recovery_halts` records the listener, stack role, chain, height,
expected/observed hashes, reason and detection times. Repeated observations of
one active boundary update the same record. These shared safety tables survive
green-schema recreation and are not merged or discarded at cutover.

Containment takes the exclusive cutover advisory lock. Writers and Gateway
transaction submissions already holding its shared form finish first; subsequent
guarded ingestion, result acceptance and publication cannot proceed once the
halt commits. A transaction submitted before containment commits may therefore
already exist externally even if its local receipt bookkeeping was blocked.
Containment cannot undo it. In-flight work and RPC detection latency are explicit
limits of this policy.

The halt closes host/Gateway readiness, revokes the persisted Gateway dry-run
release latch and blocks cutover. Old proof/computation rows remain as evidence
and cannot be accepted through the guarded production paths. A restart does not
clear the halt. Migration or safety-table query failures fail closed.

Inspect the state with:

```sql
SELECT id, source, stack_role, chain_id, block_number,
       encode(expected_hash, 'hex') AS expected_hash,
       encode(observed_hash, 'hex') AS observed_hash,
       reason, detected_at, last_detected_at
FROM public.chain_recovery_halts
WHERE resolved_at IS NULL;

SELECT * FROM public.listener_replay_state WHERE pending;
```

## Operator recovery procedure

1. Stop the entire coprocessor fleet for this database, including listeners,
   workers, uploaders, the transaction sender and the upgrade controller. Capture
   a database snapshot, the halt records, process logs and canonical RPC evidence.
2. Establish the first changed block and a trustworthy earlier canonical
   checkpoint. Compare affected proof requests, permissions, computations and
   published Gateway transactions against canonical history. Include both
   stacks and every affected descendant, rather than only the reported height.
3. Restore a coherent database from a verified checkpoint before the affected
   history, or audit/rebuild the affected state using the deployment's approved
   recovery tooling. Run database migrations before starting repaired binaries.
   If previously published/consumed state cannot be reconciled with the protocol,
   keep the fleet stopped and escalate for coordinated protocol recovery.
4. Establish the canonical hash at each retained Gateway cursor, including the
   blue and green cursors when both exist. Populate the checkpoint table only
   after verifying the baseline's stored proofs and externally consumed effects.
   For a legacy cursor, an unverified current block hash alone is insufficient.
5. While services remain stopped, take the exclusive advisory lock and record
   the verified baseline and recovery audit reference. Resolve only the halt IDs
   covered by that recovery. Reset stale replay markers only after their baseline
   has been verified. Do not clear a halt merely to make readiness green.
6. Start the host poller with a replay window covering the recoverable retained
   range. Confirm replay finishes, all active halts are resolved, and readiness
   reflects the canonical history before releasing the rest of the fleet. Check
   retained output/ciphertext identities and reconcile external receipts before
   resuming submission. If history changes again, containment must trigger again.

The following is a transaction template for step 5, not a standalone recovery.
Replace placeholders using the independently verified baseline and audit record.
The advisory-lock identity is `0x4648455643555456` from `CUTOVER_LOCK_ID`.

```sql
BEGIN;
SELECT pg_advisory_xact_lock(x'4648455643555456'::bigint);

INSERT INTO public.listener_block_checkpoints
    (source, stack_role, chain_id, block_number, block_hash)
VALUES ('gateway', '<BCS or GCS>', <gateway_chain_id>, <verified_cursor>,
        decode('<verified canonical hash without 0x>', 'hex'))
ON CONFLICT (source, stack_role, chain_id, block_number)
DO UPDATE SET block_hash = EXCLUDED.block_hash;

UPDATE public.chain_recovery_halts
SET resolved_at = NOW(), recovery_note = '<recovery audit reference and verified baseline>'
WHERE id IN (<reviewed halt ids>) AND resolved_at IS NULL;

-- Only for an audited, rebuilt baseline; normal restart replay clears itself.
UPDATE public.listener_replay_state SET pending = FALSE, updated_at = NOW()
WHERE source = '<audited source>' AND stack_role = '<audited role>'
  AND chain_id = <audited chain>;
COMMIT;
```
