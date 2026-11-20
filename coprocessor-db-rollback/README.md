# Coprocessor database rollback

Rolls the coprocessor database schema back to an earlier migration, so that an
older coprocessor version can be deployed again.

It is a separate tool from the coprocessor. The db-migration image applies the
migrations with `sqlx migrate run` and has no way to undo them; this image is
that image plus a copy of those migrations, a down file for each one, and
`scripts/migrate.sh`. It never applies a migration and contains no coprocessor
code. It has the same shape as `relayer-db-rollback` and
`kms-connector-db-rollback`, and connects to the database the way the
migration Job does (RDS IAM token or password).

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
   coprocessor moved ahead and this tool was not updated, it stops with
   `out of sync: the database has migrations this image does not know`.
3. **Every applied migration has the same content as the copy in the image.**
   A copy that differs from what was applied means its down file cannot be
   trusted.
4. **The target is not below the baseline** (see [The baseline](#the-baseline)).

If a check fails, nothing is changed. After a rollback the tool confirms that
the database is exactly at the requested version instead of trusting the exit
code of `sqlx`.

## The baseline

The coprocessor has 119 migrations. Only the 31 above `20260704130000` have a
real down file: the 30 migrations v0.15.x added and one v0.14.x migration that
sits among them (see below). Every migration at or below `20260704130000` is
the baseline. Its
down file always fails without changing anything, and `migrate.sh` refuses a
target below the baseline before reaching them.

`20260704130000` is the last migration that v0.14.x and v0.15.x share below
the v0.15.x range. It is the target for a rollback from v0.15.x to v0.14.x.
Rolling back from v0.14.x to v0.13.x needs down files for the 26 migrations
v0.14.x added (`20260514120000` to `20260704130000`) and a lower baseline; they
are not written yet.

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
| `DATABASE_URL` | for `info` and `rollback` | | Postgres connection string of the coprocessor database |
| `MAX_ATTEMPTS` | no | `20` | Connection attempts, 2 seconds apart |
| `BASELINE_VERSION` | no | `20260704130000` | Lowest target accepted |
| `DATABASE_IAM_AUTH_ENABLED` | no | | When true, `/prepare_database_url.sh` from the db-migration base image mints an RDS IAM token into `PGPASSWORD` and sets `PGSSLMODE`, as the migration Job does. Needs `DATABASE_IAM_REGION` and the pod's AWS identity. |

The image runs `migrate.sh info` by default. A rollback has to be requested
explicitly:

```sh
docker run --rm -e DATABASE_URL=postgres://user:password@host:5432/coprocessor \
  ghcr.io/zama-ai/fhevm/coprocessor-db-rollback:<tag> rollback 20260704130000
```

Output is one `key=value` line per event:

```text
event=rollback_plan action=rollback current=20261012150000 target=20260704130000 force=false revert=20261012150000,20261007120000,...
event=rollback_result action=rollback status=success previous=20261012150000 current=20260704130000 target=20260704130000 reverted=...
```

`status` is `success`, `noop` (already at the target) or `failed`. The exit code
is non-zero on `failed`.

## Rollback procedure

The target version is the highest migration of the coprocessor release being
rolled back **to** that sits below the migrations being removed:

| Coprocessor release rolled back to | Target version | Note |
| --- | --- | --- |
| v0.14.x | `20260704130000` | Also reverts `20260928120000_blocks_valid_pending_index`, a v0.14.x migration whose version is above the v0.15.x range. It only creates an index; the v0.14.x db-migration image applies it again on its next start. |
| v0.15.0 or v0.15.1 | none | The two have the same migrations (`20261012150000` is the last one). |

For any other release:

```sh
git ls-tree --name-only <tag> coprocessor/fhevm-engine/db-migration/migrations/ | tail -1
```

and compare it with the migrations the release being rolled back **from** added
(`comm -13` of the two listings): the target is the highest migration below
the first added one.

Use the image built from the release being rolled back **from** (or a newer
one): it is the one that knows every migration applied to the database.

1. Read the notes on the migrations that will be reverted (see
   [What each down does](#what-each-down-does)). Most lose data, and one
   refuses to run until the data is dealt with.
2. Run `info` against the environment. It must end with `in_sync=true`.
3. Stop every workload of the operator that uses the database: the workers,
   the host and gateway listeners, the consumers, the transaction senders and
   the upgrade controller. The running version expects the schema that is about
   to be removed, and an open session can hold a lock a down needs.
4. Take an RDS snapshot of the instance. The downs drop tables and columns, and
   the snapshot is the only way back to the data they held.
5. Run `rollback <target-version>` and check for `status=success`.
6. Deploy the previous coprocessor and db-migration versions. The db-migration
   image re-applies the migrations of the previous release that the rollback
   removed (`20260928120000` for v0.14.x) and refuses to start against a
   database that has migrations it does not know, so deploying it before step 5
   fails without causing damage.

Blue/Green stacks keep a copy of some tables in a `gcs-<epoch>` schema. No
migration above the baseline touches those schemas, so neither does any down:
they only change the `public` schema. A `gcs-<epoch>` schema that exists at
rollback time keeps its copies as they are. Rolling back the Blue/Green controller itself
is a separate procedure: it drops the green schema.

### Forcing a rollback

One down refuses to run because of data the restored constraint does not
accept. `--force` makes it delete that data and carry on:

| Migration | What `--force` deletes |
| --- | --- |
| `20260725120000_upgrade_state_per_chain_windows` | Every `upgrade_state` row of a stack role except the one with the lowest `host_chain_id`. v0.14.x keeps one row per stack role. |

`--force` does not relax the checks above, and no other down honors it.

### If a rollback fails half way

Each down file runs in its own transaction. If one fails, the migrations
reverted before it stay reverted and the tool reports where it stopped:

```text
event=rollback_result action=rollback status=failed previous=20261012150000 current=20260725120000 target=20260704130000 error="..."
```

`current` is a consistent schema version: the failed down changed nothing. From
there, either fix the cause and run the same `rollback` again (it continues
from `current`), or go forward again by running the db-migration image of the
version the database was on, which re-applies what was reverted.

## What each down does

A down restores the schema, never the data a migration removed. Columns that
are dropped and added back end up last in their table. Dropping a table or a
column loses its rows or values; the table below only calls out the cases that
need attention.

| Migration | Down | Data impact |
| --- | --- | --- |
| `20260708120000_add_consensus_latches_to_upgrade_state` | Drops `host_chain_id`, `host_consensus_reached`, `gw_consensus_reached` | Lost. |
| `20260716120000_block_manifest_publication` | Drops `block_manifest`, `block_range_commitment`, `block_manifest_state` | Archived manifests lost. |
| `20260716120100_block_manifest_verification` | Drops the six verification tables | Verification history and drifted handle findings lost. |
| `20260717120000_create_gateway_config_coprocessors` | Drops the table | Rebuilt from the gateway on the next v0.15 start. |
| `20260721120000_add_proposal_block_to_upgrade_state` | Drops `proposal_block` | Lost. |
| `20260725120000_upgrade_state_per_chain_windows` | Restores the primary key on `stack_role`, makes `host_chain_id` nullable, drops the attempt index | Refuses while a stack role has more than one row. The rows the migration deleted (no host chain) are not recreated. |
| `20260805120000_add_multi_output_columns` | Drops `group_id`, `output_index`, `output_count` from `computations` | Lost. |
| `20260807130000_kms_existing_key_id` | Drops `existing_key_id` | Lost. |
| `20260807140000_add_consensus_epoch_history` | Drops `consensus_epoch_history`, `blue_green_consensus_epoch` | Epoch history lost. |
| `20260813120000_computations_operand_boundary_mask` | Drops the constraint and `operand_boundary_mask` | Lost. |
| `20260814120000_pending_computations_dcid_acquisition_index` | Drops an index | None. |
| `20260817110000_add_consensus_epoch_block_window` | Drops the table | Epoch windows lost. |
| `20260817120000_handle_producer_block` | Drops the table | Lost. |
| `20260817130000_ciphertext_digest_s3_publication_verification` | Drops the two witness columns | Verification witnesses lost. |
| `20260818120000_add_synthetic_txn_hashes_to_upgrade_state` | Drops the constraint and `synthetic_txn_hashes` | Lost. |
| `20260819120000_add_consensus_version_to_versioning` | Drops `consensus_version` | Lost. |
| `20260828120000_computations_error_retry_count` | Drops `error_retry_count` | Lost. |
| `20260904120000_ciphertext_digest_s3_publication_pending` | Drops an index | None. |
| `20260905120000_dependence_chain_expired_lease_index` | Drops an index | None. |
| `20260905120100_computations_pending_dcid_index` | Drops an index | None. |
| `20260917120000_healing_work_notify` | Drops the two triggers and the function | None. |
| `20260918120000_healing_notify_unpinned` | Restores the triggers of `20260917120000` | None. |
| `20260925120000_drifted_handle_heal_attempts` | Drops `heal_attempts`, `heal_abandoned_at` | Lost. |
| `20260928120000_blocks_valid_pending_index` | Drops an index | None. Belongs to v0.14.x too; its db-migration image recreates it. |
| `20260930120000_drift_superseded` | Drops `superseded_at` and the function | Lost. |
| `20260930120100_epoch_upload_start` | Drops `upload_start_block` | Lost. |
| `20261001120000_ciphertext_digest_legacy_s3_witness` | Drops the trigger and the function | None by itself. |
| `20261002120100_synthetic_handle_digest` | Drops the table | Lost. |
| `20261005120000_add_gw_window_rewound_to_upgrade_state` | Drops `gw_window_rewound` | Lost. |
| `20261007120000_add_consensus_version_to_ciphertexts` | Drops `consensus_version` from `ciphertexts` | Lost. |
| `20261012150000_chain_recovery_containment` | Drops `chain_recovery_halts`, `listener_replay_state` | Lost. |
| at or below `20260704130000` | Always fails | None. The baseline is never reverted; a target below it is refused. |

## Keeping the tool in sync with the coprocessor

This is a manual step. The migrations are **copied** into `migrations/`; nothing
reads them from the coprocessor at build time, and nothing forces the
coprocessor to ship a down file. The price is that the tool falls behind every
time the coprocessor adds a migration, and says so only when it is run.

After each coprocessor release that adds migrations:

1. Copy each new file from
   `coprocessor/fhevm-engine/db-migration/migrations/<name>.sql` to
   `coprocessor-db-rollback/migrations/<name>.up.sql`, byte for byte. Only the
   file name changes.
2. Write `<name>.down.sql` next to it. It must undo the migration, fail loudly
   when it cannot do so without guessing, and contain no `BEGIN`/`COMMIT`
   (`sqlx` wraps it in a transaction).
3. Run `tests/roundtrip.sh`.
4. Add the migration to the tables in this file.

A release branch can carry a migration that `main` does not have:
`v0.14.8-1` has `20261113120000_skip_s3_status_only_digest_mirror`, which no
v0.15.x tag has. A database that ran v0.14.8-1 and was then upgraded to v0.15.x
has it applied, and this image reports it as unknown. Copy it in and write its
down before rolling such a database back.

Run `migrate.sh info` against each environment as part of the coprocessor
release, so that a tool that fell behind is found then and not during an
incident.

## Testing

`tests/roundtrip.sh` needs Docker and `sqlx-cli` 0.7.2, the version the
db-migration image pins. The pull-request workflow
`coprocessor-db-rollback-check` runs it on every change to this directory.
Against a throwaway Postgres it:

- checks the copies are identical to the coprocessor's migrations;
- applies the coprocessor's migrations one by one, reverts the ones above the
  baseline one by one, and compares the schema after each revert with the
  schema recorded before the migration;
- rolls back from the newest migration to the baseline in one command with data
  in the tables, then upgrades again;
- checks that each down that refuses on data stops the rollback and changes
  nothing, and that `--force` deletes exactly that data;
- checks that every guard refuses to run and leaves the database untouched.

```sh
coprocessor-db-rollback/tests/roundtrip.sh
```

## Image and deployment

The image is `ghcr.io/zama-ai/fhevm/coprocessor/db-migration:<DB_MIGRATION_TAG>`
plus this directory. `DB_MIGRATION_TAG` is set in the Dockerfile and must be a
db-migration image of the same release branch as the migrations copied here:
the base image is the one that applied them, so the tool runs the same
`sqlx` build, and it brings `psql`, the AWS CLI and `/prepare_database_url.sh`.

The image is not built automatically. It is built on demand, when a rollback is
being prepared, by running
`.github/workflows/coprocessor-db-rollback-docker-build.yml` by hand from the
branch or tag to build:

```sh
gh workflow run coprocessor-db-rollback-docker-build.yml --ref <branch-or-tag>
```

It publishes `ghcr.io/zama-ai/fhevm/coprocessor-db-rollback`, tagged with the
short SHA of the commit that was built, or with the `image-tag` input when one
is given (`-f image-tag=v0.15.1`). The branch or tag has to contain the
workflow file.

Clusters only pull from allowed registries, so the image reference depends on
the environment:

| Environment | Image | What it takes |
| --- | --- | --- |
| dev, testnet | `hub.zama.org/ghcr/zama-ai/fhevm/coprocessor-db-rollback:<tag>` | Nothing: pull-through proxy of `ghcr.io`. |
| testnet, mainnet | `hub.zama.org/internal/zama-ai/fhevm/coprocessor-db-rollback:<tag>` | The tag has to be imported first, in the gitops repository: `image-promotion/scripts/zpromo import ghcr zama-ai/fhevm/coprocessor-db-rollback <tag>`. Mainnet accepts no other source. |

Run it as a one-off Kubernetes Job in the `coproc` namespace of the operator,
one Job per coprocessor database. In gitops the simplest form is the
coprocessor chart's db-migration Job with `dbMigration.image` pointing at this
image and `dbMigration.command` set to `["/app/scripts/migrate.sh", "info"]`
or `["/app/scripts/migrate.sh", "rollback", "<target-version>"]`: the chart
then gives the Job the same `DATABASE_URL`, IAM settings, SSL root certificate
and service account as the migration Job of that operator. Keep the Job in
`info` mode between rollbacks, or disabled: a rollback Job must not run again
on a later sync.
