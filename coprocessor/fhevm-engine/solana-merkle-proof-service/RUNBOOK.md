# Solana Merkle record runbook

The Merkle indexer (`solana_merkle_indexer`) rebuilds every encrypted store's RFC 035 leaf record
into the `solana_merkle` database, and the Merkle proof server (`solana_merkle_proof_server`)
serves inclusion proofs from it to the KMS connectors. The record is never trusted: each KMS
connector verifies every proof against the peaks it reads on chain, and asks the next
coprocessor when a proof is missing or wrong. A wrong record therefore costs availability at one
coprocessor, never a wrong decryption.

This runbook covers one fault: the record disagrees with the chain.

## The alert

| Alert | Severity | Query |
|---|---|---|
| Merkle record disagrees with the chain | critical, pages on-call | `max(solana_merkle_indexer_quarantined_stores) > 0` |
| Merkle proof path does not reach the recorded peaks | critical, pages on-call | `increase(solana_merkle_proof_server_leaves_total{outcome="inconsistent"}[5m]) > 0` |
| A coprocessor served a proof that does not verify | critical, pages on-call | `increase(kms_connector_worker_solana_proof_answers_counter{outcome="invalid"}[5m]) > 0` |

- The first two fire at the coprocessor whose record is wrong. The second covers a wrong path
  below correct peaks, which the store check does not compare. Leaves refused because their store
  is quarantined count as `quarantined` and do not page a second time.
- The third fires at a KMS connector. Its `source` label names the coprocessor that served the
  proof. When that coprocessor is a partner, tell the partner.

Infra defines the Grafana rules. Each carries `severity=critical`, which pages the "Protocol
Oncall L1" escalation in Grafana IRM as the coprocessor consensus-drift alerts do, and this
runbook as `runbook_url`. [The metrics reference](../../../docs/metrics/metrics.md) lists every
metric of the indexer and the proof server.

## What happened

Every 10 minutes (`--store-check-interval-secs`), and once when it starts, the indexer reads each
recorded store from Solana and compares the chain's peaks with the record's peaks at the chain's
leaf count. A store whose peaks differ is a mismatch:

- the indexer logs `encrypted store's recorded peaks differ from the chain's` with the store
  address, the chain's leaf count and the record's leaf count, or `encrypted store account is not
  a valid store` with the reason;
- it writes the store into `quarantined_stores`;
- the proof server answers every query for that store with `upstream_transient` "leaf record
  inconsistent", which is retryable, so the connectors take the proof from another coprocessor.

Decryptions keep working while at least one other coprocessor serves the store. Decryptions of a
store fail only when every coprocessor has it quarantined: then open an incident at once.

Usual causes: a dump restored from a diverged database, a manual edit, a disk fault, or an indexer
bug. An indexer bug is the one to rule out first, because a restore does not fix it.

## Heal: restore the record from a dump

The record has no in-place repair. Replace the whole database with a dump taken before the
divergence.

1. **Find a dump.** Take the newest `pg_dump` of the `solana_merkle` database from Zama's
   coprocessor. Restore only a dump from Zama: `pg_restore` runs the SQL the dump contains. The
   dump holds its own checkpoint.
   - When Zama's own record is the wrong one, take Zama's newest dump older than the last store
     check that completed with no quarantined store. The divergence can predate the first
     mismatch log line by up to one check interval, longer if the indexer was down, and a dump
     taken in that gap carries it.
   - With no dump to trust, rebuild from the chain instead (below).
2. **Stop the writer, then the server.** Scale `<release>-solana-merkle-indexer` to 0 first,
   because it is the record's only writer. Then scale `<release>-solana-merkle-proof-server` to 0.
   The connectors use the other coprocessors meanwhile. When Argo CD auto-syncs the release, it
   reverts `kubectl scale` at the next sync: pause auto-sync for the application, or set the
   replica counts in its values, until the last step.
3. **Replace the database.** Drop and recreate `solana_merkle`. `DROP DATABASE` needs the
   database owner's rights and no open connection, which step 2 ensures. Then
   `pg_restore --no-owner --no-privileges --exit-on-error --single-transaction --dbname=<solana_merkle URL> <dump>`.
   `pg_dump` must be the server's major version or newer, and `pg_restore` the version of the
   `pg_dump` that made the archive or newer.
4. **Start the indexer.** Scale `<release>-solana-merkle-indexer` to 1. It resumes after the
   dump's checkpoint. The first block it replays must carry the checkpoint's hash; when it does
   not, the indexer stops, and the dump is from another fork: take an older dump.
5. **Wait for a clean store check.** Re-admit only when all three hold:
   - the indexer's lag alarm has cleared: `time() -
     solana_host_follower_applied_block_timestamp_seconds{service=~".*-solana-merkle-indexer"}`
     stays under 120 for every `host_chain_id`;
   - `solana_merkle_indexer_store_check_completed_timestamp_seconds` is later than the time the
     lag alarm cleared;
   - `solana_merkle_indexer_quarantined_stores` is 0.

   A check that runs while the record is far behind finds most stores `behind` and compares
   none of their peaks, so only a check after the catch-up proves the restore.
6. **Start the proof server.** Scale `<release>-solana-merkle-proof-server` back up.

## Rebuild from the chain

A rebuild needs no dump: the indexer replays the whole host history from the archive RPC
(DD-066). It takes hours on mainnet, during which the lag alarm fires and the proof server
answers only what the record holds so far.

1. Stop the indexer, then the proof server, as in step 2 above.
2. Drop and recreate `solana_merkle`, empty.
3. Start the indexer. On an empty database it starts at
   `solanaHostListener.merkleIndexer.startSlot` (`--start-slot`), a slot before the first
   encrypted store was created, such as the zama-host deployment slot.
4. Wait for a clean store check and start the proof server, as in steps 5 and 6 above.

## When it comes back

A mismatch again within 24 hours of a restore is a bug, not a fault: the same input produced the
same wrong record. Keep the proof server down, open an incident, and attach the mismatch log
lines, the dump's checkpoint and the indexer version. Do not restore again in a loop. Rebuild
from the chain once to rule out the dump: a mismatch after a rebuild from the chain is an indexer
bug.

## Not covered yet

- The dump schedule and storage. Infra owns the `pg_dump` job, and its cadence must be shorter than
  the gap the indexer can catch up through the archive RPC in reasonable time.
- A data-only export that a partner could import without running another operator's SQL.
