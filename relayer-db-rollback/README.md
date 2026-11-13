# Relayer database rollback

Rolls the relayer database schema back to an earlier migration, so that an
older relayer version can be deployed again.

It is a separate tool from the relayer. The relayer applies its migrations with
`relayer-migrate` and has no way to undo them; this image carries `sqlx-cli`, a
copy of those migrations, a down file for each one, and `scripts/migrate.sh`. It
never applies a migration and contains no relayer code.

It is meant to be used a few times a year, by an operator, following the
procedure below.

## All or nothing

The tool either can roll everything back or refuses to run. Before touching the
database it checks that:

1. **Every migration in the image has a down file.** `sqlx migrate revert`
   silently skips a migration without one and still reports success, so the
   whole directory is validated, not only the range being reverted. The image
   build and a pull-request check enforce the same rule.
2. **Every migration applied to the database exists in the image.** If the
   relayer moved ahead and this tool was not updated, it stops with
   `out of sync: the database has migrations this image does not know`.
3. **Every applied migration has the same content as the copy in the image.**
   A copy that differs from what was applied means its down file cannot be
   trusted.

If a check fails, nothing is changed. After a rollback the tool confirms that
the database is exactly at the requested version instead of trusting the exit
code of `sqlx`.

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
| `DATABASE_URL` | for `info` and `rollback` | | Postgres connection string of the relayer database |
| `MAX_ATTEMPTS` | no | `20` | Connection attempts, 2 seconds apart |

The image runs `migrate.sh info` by default. A rollback has to be requested
explicitly:

```sh
docker run --rm -e DATABASE_URL=postgres://user:password@host:5432/relayer \
  ghcr.io/zama-ai/fhevm/relayer-db-rollback:<tag> rollback 20260519152353
```

Output is one `key=value` line per event:

```text
event=rollback_plan action=rollback current=20260904120000 target=20260519152353 revert=20260904120000,20260902120000,20260825130000,20260825120000,20260824120000
event=rollback_result action=rollback status=success previous=20260904120000 current=20260519152353 target=20260519152353 reverted=...
```

`status` is `success`, `noop` (already at the target) or `failed`. The exit code
is non-zero on `failed`.

## Rollback procedure

The target version is the last migration of the relayer release being rolled
back **to**:

| Relayer release | Last migration (target version) |
| --- | --- |
| v0.13.x | `20260416120000` |
| v0.14.x | `20260519152353` |

For any other release:

```sh
git ls-tree --name-only <tag> relayer/relayer-migrate/migrations/ | tail -1
```

Use the image built from the release being rolled back **from** (or a newer
one): it is the one that knows every migration applied to the database.

1. Read the notes on the migrations that will be reverted (see
   [What each down does](#what-each-down-does)). Some lose data, and some refuse
   to run until the data is dealt with.
2. Run `info` against the environment. It must end with `in_sync=true`.
3. Stop the relayer. The running version expects the schema that is about to be
   removed.
4. If `20260824120000_create_gateway_chain_cursor` is going to be reverted,
   record the gateway position, which the rollback discards:
   `SELECT last_block_number FROM gateway_chain_cursor;`
5. Run `rollback <target-version>` and check for `status=success`.
6. Deploy the previous relayer and `relayer-migrate` versions. If a position was
   recorded in step 4, set it as `listener_pool.last_block_number` in the
   previous relayer's configuration for its first start: it takes precedence
   over `gateway_block_number_store`, which was not updated since the upgrade.
   Without it the previous version re-reads every block since the upgrade or,
   if that table is empty, starts at the chain head and misses the events
   emitted while it was down.

`relayer-migrate` refuses to start against a database that has migrations it
does not know, so deploying the previous version before step 5 fails without
causing damage.

### Forcing a rollback

Some downs refuse to run because of data they cannot convert. `--force` makes
them delete that data and carry on:

| Migration | What `--force` deletes |
| --- | --- |
| `20260519152353_drop_legacy_user_decrypt_req_type` | Every `user_decrypt_req` row using `unified`, and the `user_decrypt_share` rows collected for them. |

This is how a rollback from v0.14.x to v0.13.x is exercised in a test
environment. The deleted requests cannot be recovered, so do not use it on a
database whose requests matter.

`--force` does not relax the three checks above, and no other down honors it:
`20260113011557` still refuses to run on duplicated `int_job_id` values.

### If a rollback fails half way

Each down file runs in its own transaction. If one fails, the migrations
reverted before it stay reverted and the tool reports where it stopped:

```text
event=rollback_result action=rollback status=failed previous=20260904120000 current=20260519152353 target=20251109145104 error="..."
```

`current` is a consistent schema version: the failed down changed nothing. From
there, either fix the cause and run the same `rollback` again (it continues
from `current`), or go forward again by running `relayer-migrate` of the
version the database was on, which re-applies what was reverted.

Going forward again is not always possible once the rollback went below
`20260204120000`: see the note on upgrading again under
[What each down does](#what-each-down-does). The downs that can refuse to run
are `20260519152353` and `20260113011557`; check their conditions before
starting a rollback that crosses them, since the downs above them will already
have run.

## What each down does

| Migration | Down | Data impact |
| --- | --- | --- |
| `20251109145104_create_tables` | Always fails | None. The baseline is never reverted; target `0` is refused. |
| `20260113011557_fix_int_job_id_unique_constraint` | Restores the `UNIQUE (int_job_id)` constraints on the decrypt tables | Refuses to run if two rows of a table share an `int_job_id`. |
| `20260113101031_replace_int_request_id_with_int_job_id` | Restores `input_proof_req.int_request_id` | In-flight input proof requests are failed (clients resubmit). Rows get new random `int_request_id` values. `int_job_id` is lost. |
| `20260116031357_fix_input_proof_index_include_completed` | Restores the previous partial unique index | None. |
| `20260130120000_fix_updated_at_trigger_conditional` | Restores the previous trigger function | None. |
| `20260204120000_add_user_decrypt_req_type` | Drops `req_type`, its index and its enum | `req_type` values are lost. |
| `20260416120000_add_resolved_threshold` | Drops `resolved_threshold` | `resolved_threshold` values are lost. |
| `20260519152353_drop_legacy_user_decrypt_req_type` | Restores the enum with `legacy`, without `unified` | Refuses to run while any row uses `unified`, the value introduced for new requests. Those rows have to be deleted, or cleared by the relayer's expiry worker before the relayer is stopped. With `--force` the down deletes them itself. |
| `20260824120000_create_gateway_chain_cursor` | Drops `gateway_chain_cursor` | The cursor position is lost and `gateway_block_number_store` is stale. Carry the position over by hand (steps 4 and 6 of the procedure). |
| `20260825120000_add_owner_epoch` | Drops `owner_epoch` from four tables | `owner_epoch` values are lost. |
| `20260825130000_add_dispatcher_epoch_sequence` | Drops `dispatcher_epoch_seq` and resets every `owner_epoch` to 0 | Stamped epochs are lost, and the rows that carried one get a new `updated_at`. The sequence restarts at 1 if the migration is applied again. |
| `20260902120000_add_sweep_claim_indexes` | Drops three indexes | None. |
| `20260904120000_add_active_status_indexes` | Drops three indexes | None. |

A down restores the schema, never the data a migration removed. Columns that
are dropped and added back end up last in their table.

Upgrading again after rolling back below `20260204120000` is subject to the
relayer's own rule: `20260519152353` refuses to run while `user_decrypt_req`
rows carry the `legacy` value that `20260204120000` stamps on existing rows.

## Keeping the tool in sync with the relayer

This is a manual step. The migrations are **copied** into `migrations/`; nothing
reads them from the relayer at build time, and nothing forces the relayer to
ship a down file. The price is that the tool falls behind every time the relayer
adds a migration, and says so only when it is run.

After each relayer release that adds migrations:

1. Copy each new file from `relayer/relayer-migrate/migrations/<name>.sql` to
   `relayer-db-rollback/migrations/<name>.up.sql`, byte for byte. Only the file
   name changes.
2. Write `<name>.down.sql` next to it. It must undo the migration, fail loudly
   when it cannot do so without guessing, and contain no `BEGIN`/`COMMIT`
   (`sqlx` wraps it in a transaction).
3. Run `tests/roundtrip.sh`.
4. Add the migration to the tables in this file.

Run `migrate.sh info` against each environment as part of the relayer release,
so that a tool that fell behind is found then and not during an incident.

## Testing

`tests/roundtrip.sh` needs Docker and `sqlx-cli` 0.8.6. Against a throwaway
Postgres it:

- checks the copies are identical to the relayer's migrations;
- applies the relayer's migrations one by one, reverts them one by one, and
  compares the schema after each revert with the schema recorded before the
  migration;
- rolls back from the newest migration to the baseline in one command with data
  in the tables, then upgrades again;
- rolls back to a release with the data a running relayer leaves behind, checks
  the request rows are unchanged, then upgrades again;
- checks that a down that cannot revert stops the rollback and changes nothing;
- checks that every guard refuses to run and leaves the database untouched.

```sh
relayer-db-rollback/tests/roundtrip.sh
```

## Image and deployment

The image is not built automatically. It is built on demand, when a rollback is
being prepared, by running
`.github/workflows/relayer-db-rollback-docker-build.yml` by hand from the branch
or tag to build:

```sh
gh workflow run relayer-db-rollback-docker-build.yml --ref <branch-or-tag>
```

It publishes `ghcr.io/zama-ai/fhevm/relayer-db-rollback`, tagged with the short
SHA of the commit that was built, or with the `image-tag` input when one is
given (`-f image-tag=v0.15.0`). The branch or tag has to contain the workflow
file.

Clusters only pull from allowed registries, so the image reference depends on
the environment:

| Environment | Image | What it takes |
| --- | --- | --- |
| dev, testnet | `hub.zama.org/ghcr/zama-ai/fhevm/relayer-db-rollback:<tag>` | Nothing: pull-through proxy of `ghcr.io`. |
| testnet, mainnet | `hub.zama.org/internal/zama-ai/fhevm/relayer-db-rollback:<tag>` | The tag has to be imported first, in the gitops repository: `image-promotion/scripts/zpromo import ghcr zama-ai/fhevm/relayer-db-rollback <tag>`. Mainnet accepts no other source. |

Run it as a one-off Kubernetes Job in the relayer namespace, with `DATABASE_URL`
built from the relayer database credentials and
`args: ["rollback", "<target-version>"]`. Add the Job for the rollback and
remove it afterwards: it must not run again on a later sync.
