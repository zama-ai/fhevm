# KMS Connector database rollback

Rolls the KMS Connector database schema back to an earlier migration, so that
an older connector version can be deployed again.

It is a separate tool from the connector. The connector applies its migrations
with the `db-migration` image (`sqlx migrate run`) and has no way to undo them;
this image carries `sqlx-cli`, a copy of those migrations, a down file for each
one, and `scripts/migrate.sh`. It never applies a migration and contains no
connector code.

It is the same tool as [`relayer-db-rollback`](../relayer-db-rollback), with the
connector's migrations. It is meant to be used a few times a year, by an
operator, following the procedure below.

## All or nothing

The tool either can roll everything back or refuses to run. Before touching the
database it checks that:

1. **Every migration in the image has a down file.** `sqlx migrate revert`
   silently skips a migration without one and still reports success, so the
   whole directory is validated, not only the range being reverted. The image
   build and a pull-request check enforce the same rule.
2. **Every migration applied to the database exists in the image.** If the
   connector moved ahead and this tool was not updated, it stops with
   `out of sync: the database has migrations this image does not know`.
3. **Every applied migration has the same content as the copy in the image.**
   A copy that differs from what was applied means its down file cannot be
   trusted.

If one of these checks fails, nothing is changed. After a rollback the tool
confirms that the database is exactly at the requested version instead of
trusting the exit code of `sqlx`.

These checks are about the migration files. A down can still refuse to run
because of the rows it finds, after the downs above it have run: see
[Before a rollback](#before-a-rollback).

## Usage

```sh
migrate.sh check                      # validate the migration files, no database needed
migrate.sh info                       # run every check, print the database state (read-only)
migrate.sh rollback <target-version>  # revert every migration above <target-version>
migrate.sh rollback <target-version> --force
                                      # same, deleting the data that blocks a down
```

| Variable | Required | Default | Purpose |
| --- | --- | --- | --- |
| `DATABASE_URL` | for `info` and `rollback` | | Postgres connection string of the connector database |
| `MAX_ATTEMPTS` | no | `20` | Connection attempts, 2 seconds apart |

`PGUSER`, `PGPASSWORD` and `PGSSLMODE` are honored, so the tool connects the
same way the connector's `db-migration` job does:
`DATABASE_URL=postgresql://<endpoint>/connector` with the credentials in
`PGUSER` / `PGPASSWORD`.

The image runs `migrate.sh info` by default. A rollback has to be requested
explicitly:

```sh
docker run --rm -e DATABASE_URL=postgres://user:password@host:5432/connector \
  ghcr.io/zama-ai/fhevm/kms-connector/db-rollback:<tag> rollback 20260701000000
```

Output is one `key=value` line per event:

```text
event=rollback_plan action=rollback current=20260828000000 target=20260701000000 force=false revert=20260828000000,20260805100000,20260804000000,20260713000000,20260708000000
event=rollback_result action=rollback status=success previous=20260828000000 current=20260701000000 target=20260701000000 force=false reverted=...
```

`status` is `success`, `noop` (already at the target) or `failed`. The exit code
is non-zero on `failed`. The `Skipped <version>/revert` lines `sqlx` prints for
the migrations at or below the target are not skipped work.

## Rollback procedure

Each KMS party has its own connector database: the procedure is run once per
database.

The target version is the last migration of the connector release being rolled
back **to**:

| Connector release | Last migration (target version) |
| --- | --- |
| v0.12.x | `20260226161117` |
| v0.13.x | `20260407130028` |
| v0.14.x | `20260701000000` |
| v0.15.x | `20260828000000` |

For any other release:

```sh
git ls-tree --name-only <tag> kms-connector/connector-db/migrations/ | tail -1
```

Use the image built from the release being rolled back **from** (or a newer
one): it is the one that knows every migration applied to the database.

1. Read the notes on the migrations that will be reverted (see
   [What each down does](#what-each-down-does)). Some lose data, and some refuse
   to run until the data is dealt with.
2. Run `info` against the database. It must end with `in_sync=true`.
3. Stop the `gw-listener` of this database, and `endpoint` where deployed, so
   that no new request comes in. Let the `kms-worker` and the `tx-sender`
   finish what is in flight, then check that nothing is left
   (see [Before a rollback](#before-a-rollback)).
4. Stop the `kms-worker` and the `tx-sender`. The running version expects the
   schema that is about to be removed.
5. Run `rollback <target-version>` and check for `status=success`.
6. Deploy the previous connector version, including its `db-migration` job.

Do not sync the connector application between steps 5 and 6 while it still
points at the newer version: its `db-migration` job is a Helm
pre-install/pre-upgrade hook and would apply again everything that was just
reverted.

The previous version's `db-migration` job refuses to run against a database that
has migrations it does not know, so deploying it before step 5 fails without
causing damage.

### Before a rollback

The connector keeps finished requests and responses: nothing removes them. The
downs keep them too, converting what they have to, and refuse to run only on
work that is still in flight (`pending` or `under_process`) and that the
previous version would process wrongly.

A down that refuses does so after the downs above it have run, and those are
not undone. So check for in-flight work before starting, with the connector
stopped as in step 3:

```sql
SELECT 'public_decryption_requests', status, count(*) FROM public_decryption_requests
WHERE status IN ('pending', 'under_process') GROUP BY 2
UNION ALL
SELECT 'user_decryption_requests', status, count(*) FROM user_decryption_requests
WHERE status IN ('pending', 'under_process') GROUP BY 2
UNION ALL
SELECT 'keygen_requests', status, count(*) FROM keygen_requests
WHERE status IN ('pending', 'under_process') GROUP BY 2
UNION ALL
SELECT 'prep_keygen_requests', status, count(*) FROM prep_keygen_requests
WHERE status IN ('pending', 'under_process') GROUP BY 2;
```

No row means none of the in-flight conditions below applies.

### Forcing a rollback

Some downs refuse to run because of rows the previous version cannot handle.
`--force` makes them deal with those rows and carry on:

| Migration | Refuses while | What `--force` does |
| --- | --- | --- |
| `20260828000000_http_decryption_source_and_errors` | An HTTP-originated decryption request is in flight | Deletes those requests |
| `20260805100000_rfc029_compressed_key_migration` | An in-flight keygen request targets an existing key | Marks those requests `failed` |
| `20260804000000_add_compressed_key_set` | A keygen response holds a `CompressedKeySet` digest | Deletes those responses |
| `20260713000000_handle_only_decryption` | A decryption request stored with handles only (`sns_ct_materials` is NULL) is in flight | Deletes those requests |
| `20260421092426_unified_user_decryption` | A unified (RFC016) user decryption request is in flight | Deletes those requests |
| `20260407130028_key_generation_ethereum_migration` | `prep_keygen_requests` has rows, finished or not | Deletes them |
| `20250826111757_keygen_crs` | `keygen_requests` or `crsgen_requests` has rows, finished or not | Deletes them |

A request deleted this way is never answered by this party, and cannot be
recovered. `--force` is passed to every down of the rollback: to accept the
deletion of one migration only, roll back to just below it with `--force`, then
continue without it.

`--force` does not relax the three checks above.

### If a rollback fails half way

Each down file runs in its own transaction. If one fails, the migrations
reverted before it stay reverted and the tool reports where it stopped:

```text
event=rollback_result action=rollback status=failed previous=20260828000000 current=20260713000000 target=20260701000000 force=false error="..."
```

`current` is a consistent schema version: the failed down changed nothing. From
there, either fix the cause and run the same `rollback` again (it continues
from `current`), or go forward again by running the `db-migration` job of the
version the database was on, which re-applies what was reverted.

The downs that ran before the failure may have dropped tables and rows that
going forward again does not bring back (for instance the ProtocolConfig event
tables when a rollback to v0.13.x stops half way). This is why the in-flight
check comes before the rollback.

## What each down does

| Migration | Down | Data impact |
| --- | --- | --- |
| `20250604084502_gw_events` | Always fails | None. The baseline is never reverted; target `0` is refused. |
| `20250604141515_decryption_response` | Drops the decryption response tables | Responses are lost. |
| `20250604143523_kms_worker_notify` | Drops the kms-worker notify triggers | None. |
| `20250606125742_tx_sender_notify` | Drops the tx-sender notify triggers | None. |
| `20250826111757_keygen_crs` | Restores the previous key generation tables | Refuses while `keygen_requests` or `crsgen_requests` has rows. Prep keygen requests and every keygen response are lost. |
| `20251007112229_add_otlp_context` | Drops `otlp_context` | Tracing contexts are lost. |
| `20251025081948_prss_init` | Drops `prss_init` | Its rows are lost. |
| `20251026091602_refresh_keygen_reshare` | Drops `key_reshare_same_set` | Its rows are lost. |
| `20251027122128_add_already_sent` | Drops `already_sent` | The flags are lost. |
| `20251104142103_add_last_block_polled` | Drops `last_block_polled` | Polling positions are lost. |
| `20251113084850_add_decryption_error_counter` | Drops `error_counter` | The counters are lost. |
| `20251208161022_implem_garbage_collection` | Restores the `under_process` flag and the request autoremoval triggers | `pending` becomes unlocked, every other status becomes locked; `updated_at` is lost. |
| `20260122130522_add_tx_hash` | Drops `tx_hash` | Transaction hashes are lost. |
| `20260203091107_timestamp_to_timestamptz` | Timestamps back to `TIMESTAMP` with a `NOW()` default | None. |
| `20260226161117_add_kms_context` | Drops `kms_context` | The validation cache is lost. |
| `20260407130028_key_generation_ethereum_migration` | Restores `prep_keygen_requests.epoch_id`, drops `extra_data` | Refuses while `prep_keygen_requests` has rows. The key generation polling cursors are reset to NULL: check where the previous version will start polling from. |
| `20260421092426_unified_user_decryption` | Drops the RFC016 columns | Refuses while a unified user decryption request is in flight. Finished ones are kept, without those columns. |
| `20260526120000_add_protocol_config_events` | Drops the ProtocolConfig event tables and the per-chain polling cursor | The per-chain position is copied back to the per-event cursor. Stored context and epoch events are lost; `kms_context` keeps one row per context, invalid if any of its rows was. |
| `20260527090000_add_protocol_config_responses` | Drops the ProtocolConfig response tables | Responses not yet submitted are never submitted. |
| `20260701000000_add_abort_key_crs_gen` | Drops the abort tables, removes `aborted` from the status enum | Abort requests are lost; an `aborted` request becomes `failed`. |
| `20260708000000_add_context_epoch_destruction` | Merges `kms_context` and `kms_epoch` back into one cache, drops the destruction tables | Destruction events are lost. The cache keeps the live epochs of each known context; the previous version asks the chain for the rest. |
| `20260713000000_handle_only_decryption` | Drops `ct_handles`, `sns_ct_materials` mandatory again | Refuses while a request stored with handles only is in flight. Finished ones are kept with an empty material list. |
| `20260804000000_add_compressed_key_set` | Removes `CompressedKeySet` from `key_type` | Refuses while a keygen response holds such a digest. |
| `20260805100000_rfc029_compressed_key_migration` | Drops `existing_key_id` | Refuses while an in-flight keygen targets an existing key. |
| `20260828000000_http_decryption_source_and_errors` | Drops `source` and the error columns, restores the previous triggers | Refuses while an HTTP-originated request is in flight. HTTP-originated responses and error responses are deleted; finished HTTP-originated requests are kept. |
| `20261103000000_pad_decryption` | Drops the pad decryption tables | Pad decryption requests and responses are lost. |
| `20261105000000_rfc037_signed_confirmations` | Drops the destruction response tables and the confirmation columns | Signed confirmations are lost. |

A down restores the schema, never the data a migration removed. Columns that
are dropped and added back end up last in their table.

Crossing a release boundary, in practice:

- **v0.15.x to v0.14.x** reverts `20260708000000` to `20260828000000`. It goes
  through without `--force` once no decryption request is in flight. The
  HTTP-originated responses and the error responses are deleted.
- **v0.14.x to v0.13.x** reverts `20260421092426` to `20260701000000`. It goes
  through without `--force` once no unified user decryption request is in
  flight. The polling position is carried over to the per-event cursor the
  previous version reads.

## Keeping the tool in sync with the connector

This is a manual step. The migrations are **copied** into `migrations/`; nothing
reads them from the connector at build time, and nothing forces the connector
to ship a down file. The price is that the tool falls behind every time the
connector adds a migration, and says so only when it is run.

After each connector release that adds migrations:

1. Copy each new file from `kms-connector/connector-db/migrations/<name>.sql` to
   `kms-connector-db-rollback/migrations/<name>.up.sql`, byte for byte. Only
   the file name changes.
2. Write `<name>.down.sql` next to it. It must undo the migration, fail loudly
   when it cannot do so without guessing, and contain no `BEGIN`/`COMMIT`
   (`sqlx` wraps it in a transaction).
3. Add its scenario to `tests/roundtrip.sh` if it can refuse or converts data,
   and run the script.
4. Add the migration to the tables in this file.

Run `migrate.sh info` against each database as part of the connector release,
so that a tool that fell behind is found then and not during an incident.

## Testing

`tests/roundtrip.sh` needs Docker and `sqlx-cli` 0.8.6. Against a throwaway
Postgres it:

- checks the copies are identical to the connector's migrations;
- applies the connector's migrations one by one, reverts them one by one, and
  compares the schema after each revert with the schema recorded before the
  migration, then upgrades again;
- rolls back to v0.14.x and to v0.13.x with rows shaped like the ones each
  connector version writes, finished ones included, checks what is kept, what
  is converted and what is deleted, then upgrades again;
- checks that every down that can refuse does refuse, stops the rollback and
  changes nothing, and deletes only what it says when forced;
- rolls back to the baseline in steps with data in the tables, then upgrades
  again;
- checks that every guard refuses to run and leaves the database untouched.

```sh
kms-connector-db-rollback/tests/roundtrip.sh
PG_IMAGE=postgres:15-alpine kms-connector-db-rollback/tests/roundtrip.sh
```

## Image and deployment

The image is not built automatically. It is built on demand, when a rollback is
being prepared, by running
`.github/workflows/qa-kms-connector-db-rollback-docker-build.yml` by hand from the
branch or tag to build:

```sh
gh workflow run qa-kms-connector-db-rollback-docker-build.yml --ref <branch-or-tag>
```

It publishes `ghcr.io/zama-ai/fhevm/kms-connector/db-rollback`, tagged with the
short SHA of the commit that was built, or with the `image-tag` input when one
is given (`-f image-tag=v0.15.0`). The branch or tag has to contain the workflow
file.

Run it as a one-off Kubernetes Job in the namespace of the connector, one per
connector database, connecting like that connector's `db-migration` job and
with `args: ["rollback", "<target-version>"]`. Run the Job for the rollback and
disable it afterwards: it must not run again on a later sync.
