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
| Merkle record disagrees with the chain | critical, pages on-call | `solana_merkle_indexer_quarantined_stores > 0` |
| Merkle proof path does not reach the recorded peaks | critical, pages on-call | `increase(solana_merkle_proof_server_leaves_total{outcome="inconsistent"}[5m]) > 0` |
| A coprocessor served a proof that does not verify | critical, pages on-call | `increase(kms_connector_worker_solana_proof_answers_counter{outcome="invalid"}[5m]) > 0` |

- The first two fire at the coprocessor whose record is wrong.
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

- the indexer logs `encrypted store disagrees with the chain` with the store address, the
  chain's leaf count and the record's leaf count;
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
   `pg_dump` major version must match the Postgres server's. The dump holds its own checkpoint.
   - When Zama's own record is the wrong one, take Zama's newest dump older than the first
     mismatch log line. The indexer then catches up from that dump's checkpoint.
2. **Stop the writer, then the server.** Scale `<release>-solana-merkle-indexer` to 0 first,
   because it is the record's only writer. Then scale `<release>-solana-merkle-proof-server` to 0.
   The connectors use the other coprocessors meanwhile.
3. **Replace the database.** Drop and recreate `solana_merkle`, then
   `pg_restore --no-owner --dbname=<solana_merkle URL> <dump>`.
4. **Start the indexer.** Scale `<release>-solana-merkle-indexer` to 1. It resumes after the
   dump's checkpoint. The first block it replays must carry the checkpoint's hash; when it does
   not, the indexer stops, and the dump is from another fork: take an older dump.
5. **Wait for a clean store check.** Re-admit only when both hold:
   - `solana_merkle_indexer_store_check_completed_timestamp_seconds` is later than the restart;
   - `solana_merkle_indexer_quarantined_stores` is 0.

   The record does not need to be caught up first: a proof from a record that is behind still
   verifies, and the connector asks another coprocessor when it does not.
6. **Start the proof server.** Scale `<release>-solana-merkle-proof-server` back up.

## When it comes back

A mismatch again within 24 hours of a restore is a bug, not a fault: the same input produced the
same wrong record. Keep the proof server down, open an incident, and attach the mismatch log
lines, the dump's checkpoint and the indexer version. Do not restore again in a loop.

## Not covered yet

- The dump schedule and storage. Infra owns the `pg_dump` job, and its cadence must be shorter than
  the gap the indexer can catch up through the archive RPC in reasonable time.
- A data-only export that a partner could import without running another operator's SQL.
