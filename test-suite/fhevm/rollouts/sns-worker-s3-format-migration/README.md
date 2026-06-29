# SNS-worker S3 format v0 → v1 rollout E2E test

This rollout exercises the exact scenario requested for the new S3 upload format + migration feature:

1. Start with an *old* (pre-feature) `sns-worker` binary.
2. Run e2e tests so that legacy ciphertexts are produced (flat S3 keys, no `ct-attestation` metadata, `s3_format_version = 0` or `NULL` in `ciphertext_digest`).
3. Switch *only* the sns-worker to the new binary with `S3_MIGRATION_MODE=concurrent`.
4. Run tests again (new uploads go through the v1 path with attestation + `/<context>` keys, currently `/1`).
5. Verify that migration ran and that **everything** (legacy + new) is on the new format both in the DB and on S3 (minio).

## Prereqs / image for the "old" binary

By default, the baseline phase pins `COPROCESSOR_SNS_WORKER_VERSION=pre-s3-format`.

You must have a docker image tagged `ghcr.io/zama-ai/fhevm/coprocessor/sns-worker:pre-s3-format` that was built from a commit *before* the S3 metadata / format work (roughly before `145b9465e` "feat(coprocessor): sns-worker, push metadata on S3").

The `run.ts` contains a best-effort `prepareOldSnsImage()` helper that does a `git worktree` + `docker build ... --target sns-worker` from a configurable `PREVIOUS_SNS_COMMIT`. Update the constant and run the rollout; it will build the old image if the tag is missing.

In CI you will usually publish (or cache) a pre-feature image under that tag before running this rollout.

## Running

```bash
cd test-suite/fhevm
./fhevm-cli rollout run rollouts/sns-worker-s3-format-migration/run.ts
# or with the heavy profile
ROLLOUT_TEST_PROFILE=rollout-heavy ./fhevm-cli rollout run rollouts/sns-worker-s3-format-migration/run.ts
```

### Using a real 0.13 release as the "old" version (default in CI)

`v0.13.0-6` contains the old S3 format behavior (pre new upload format and migration feature).

The GitHub workflow (`test-suite-stateful-rollout.yml`) automatically defaults `OLD_SNS_IMAGE_TAG=v0.13.0-6` (a pre-S3-format 0.13 release) when executing this specific runbook. This tag is used only for `COPROCESSOR_SNS_WORKER_VERSION`; the rest of the baseline stack follows the standard v0.13 pairing (`v0.13.0` components, KMS core `v0.13.20`, test-suite `v0.13.0`). This tests migration from a real released 0.13 SNS worker without carrying the whole stack on the prerelease image set.

You can override with a different published old-format 0.13 tag:

```bash
OLD_SNS_IMAGE_TAG=v0.13.0-6 ./fhevm-cli rollout run rollouts/sns-worker-s3-format-migration/run.ts
```

In GitHub Actions (via `workflow_dispatch`), use the `old_sns_image_tag` input.

The script will `docker pull` the corresponding published image for the baseline phase (no git commit build needed).

It uses the `two-of-three` scenario by default (like the v0.12→v0.13 rollout) so you get realistic multi-copro + drift DB names.

## What the runbook does (high level)

- `prepareOldSnsImage()` (if needed)
- `ctx.up(baselineLock)` (old SNS)
- `ctx.test("rollout-standard")` — populates legacy CTs
- `normalizeLegacyNullsToZero()` — critical step: old binaries + the column migration leave `NULL`; the concurrent migration only scans `=0`. We do the UPDATE before starting the new worker.
- `ctx.applyVersionLock("01-sns", ...)` — applies `S3_MIGRATION_MODE=concurrent` and adds a local override scoped to `coprocessor-sns-worker`.
- `ctx.upgradeRuntimeGroup("coprocessor")` — rebuilds/restarts only the local SNS-worker service for each coprocessor instance.
- Second `ctx.test(...)`
- `waitForConcurrentMigration()` (row count across all coprocessor DBs)
- `assertDbAllS3FormatV1()` + `assertS3Attestations()` (checks DB v1 state, current `hex/1` objects, CT128 digest-key compatibility objects, and `ct-attestation` metadata)

## Glue that had to be added

- `coprocessor/fhevm-engine/sns-worker/src/bin/utils/daemon_cli.rs`: the `--s3-migration` (and clean) arg now also reads `S3_MIGRATION_MODE` / `CLEAN_OLD_S3_FORMAT_VERSION` via clap `env=`. This lets us control the mode from the generated `coprocessor.env` without ever putting a new `--flag` on the command line in `coprocessor-docker-compose.yml` (old images must continue to start).
- `test-suite/fhevm/src/generate/env.ts`: the two extra keys are written into the coprocessor env maps (base + all `coprocessor*` + chain-specific copies) when present in the version bundle. They also naturally land in `versionsEnv`.

No change was needed to the compose template command list.

## Files

- `versions.ts` — phase version pins + extra `S3_MIGRATION_MODE`
- `run.ts` — the executable rollout (contains the verify helpers)
- This README

See the parent `rollouts/v0.12-to-v0.13/` for the pattern this was modeled on.
