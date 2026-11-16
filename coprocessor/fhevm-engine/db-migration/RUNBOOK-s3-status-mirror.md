# S3 migration mirror-lock correction on deployed v0.14 databases

The forward migration `20261113120000_skip_s3_status_only_digest_mirror.sql`
stops publication-status-only updates from firing the digest branch mirror.
This covers both the NULL-to-0 startup backfill and the 0-to-1 S3 migration.
Their row locks remain, but they no longer acquire a mirror advisory stripe or
fan out writes to branch contexts.

v0.14's listener still contains branch-state producer resolution and reorg
cleanup. Keep the other UPDATE conditions, INSERT/DELETE mirrors, and PBS
context trigger. No v0.14 service consumes the branch digest's
`s3_format_version`; its value may now lag the authoritative legacy row.
v0.15 retires the direct branch-state consumers. Dropping the whole mechanism
is a separate schema-retirement change.

## Rollout

1. Ship this as an additional migration in a v0.14 patch release. Do not modify
   an existing migration or repair its checksum. Carry the identical migration
   forward into v0.15/main so its version/checksum stays consistent on upgrade.
2. Keep concurrent S3 migration disabled while applying the correction if the
   database is under load. Existing v0.14 service binaries can remain running:
   this change adds no binary/schema ordering requirement.
3. Run the normal database migration Job with the new migration image.
   `CREATE OR REPLACE TRIGGER` takes a brief table lock; the 3-second lock
   timeout bounds its wait. On timeout the transaction rolls back, retaining
   the original trigger. Retry the Job when contention subsides.
4. Check the installed definition (read-only):

   ```sql
   SELECT pg_get_triggerdef(oid)
   FROM pg_trigger
   WHERE tgrelid = 'public.ciphertext_digest'::regclass
     AND tgname = 'mirror_ciphertext_digest_branchless_upd';
   ```

   Its `WHEN` condition must not mention `s3_format_version`; the other
   conditions and the INSERT/DELETE triggers remain. Confirm successful
   migration recording in `_sqlx_migrations` before resuming migration work.
5. Resume one migration process at low concurrency first, then observe
   progress, database wait events, deadlock reports, and normal SNS backlog.

This removes one source of cross-handle blocking, not all possible deadlocks.
Normal uploads still invoke the mirror when digests change and retain their
existing transaction scope. Keep the PostgreSQL deadlock DETAIL/CONTEXT report
to confirm whether any remaining cycle involves another path.

## Validation

The existing SNS worker Rust test suite runs
`src/tests/s3_status_digest_mirror.rs` using the shared database test harness.
It applies all migrations to a disposable database and checks that S3 status
updates complete while another session holds the mirror stripe. It also checks
the preserved digest/transaction/INSERT/DELETE mirrors. These tests run in the
existing `coprocessor-cargo-test/sns-worker` job without a separate SQL test job.

Run the focused tests with:

```sh
cd coprocessor/fhevm-engine
cargo test -p sns-worker --lib tests::s3_status_digest_mirror
```
