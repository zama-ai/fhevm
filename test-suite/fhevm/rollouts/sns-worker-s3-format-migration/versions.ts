/**
 * Rollout test for SNS-worker S3 format v0 -> v1 + concurrent migration.
 *
 * See run.ts for the full flow:
 *   1. Baseline with *old* pre-feature sns-worker (produces legacy CTs: flat S3 keys, no ct-attestation meta, s3_format_version=0/NULL).
 *   2. Run tests to populate legacy ciphertexts.
 *   3. Upgrade *only* the sns-worker (per-service override) to new code + S3_MIGRATION_MODE=concurrent.
 *   4. Run tests again (new uploads use v1 + /context keys).
 *   5. Assert DB + S3 are fully on new format (including migration of legacy rows).
 *
 * Usage (typical):
 *   cd test-suite/fhevm
 *   # Ensure an "old" image is available locally as the tag below (or the run.ts can build it via worktree).
 *   ./fhevm-cli rollout run rollouts/sns-worker-s3-format-migration/run.ts
 *
 * "from" = previous version (the sns-worker binary *before* the new S3 format upload + migration feature).
 *
 * The actual old image is controlled at runtime by the OLD_SNS_IMAGE_TAG environment variable:
 *   - OLD_SNS_IMAGE_TAG=v0.13.0-6    → use a real published pre-S3-format 0.13 release as the "old" worker (pull, no build)
 *   - (default) "pre-s3-format"      → local dev: build from an old commit
 *
 * The script (run.ts) has an explicit "prepare old / previous version" step (i.e. deploy at the previous version)
 * that either pulls a published tag or builds from PREVIOUS_SNS_COMMIT.
 */

type Env = Record<string, string>;

export const scenario = "two-of-three";

// The previous ("old") sns-worker version is controlled at runtime via the
// OLD_SNS_IMAGE_TAG environment variable.
//
// In the GitHub stateful-rollout workflow, this scenario automatically uses
// a pre-S3-format 0.13 release (e.g. v0.13.0-6) as OLD because it contains the old S3 format
// behavior. This allows testing real migration from released 0.13 deployments.
//
// For local dev of the feature itself, leave unset (builds from old commit using "pre-s3-format" tag).
const fromTag = process.env.OLD_SNS_IMAGE_TAG || "pre-s3-format";
const targetTag = "fhevm-local";   // or a post-feature published tag / sha; per-service override will build current source anyway
const releasedComponentTag = "v0.13.0-6";

const relayerSdkVersion = "0.4.2";

export const from = {
  RELAYER_VERSION: "v0.11.0",
  RELAYER_MIGRATE_VERSION: "v0.11.0",
  GATEWAY_VERSION: releasedComponentTag,
  HOST_VERSION: releasedComponentTag,
  CORE_VERSION: "v0.13.10",
  CONNECTOR_DB_MIGRATION_VERSION: releasedComponentTag,
  CONNECTOR_GW_LISTENER_VERSION: releasedComponentTag,
  CONNECTOR_KMS_WORKER_VERSION: releasedComponentTag,
  CONNECTOR_TX_SENDER_VERSION: releasedComponentTag,
  COPROCESSOR_DB_MIGRATION_VERSION: releasedComponentTag,
  COPROCESSOR_HOST_LISTENER_VERSION: releasedComponentTag,
  COPROCESSOR_GW_LISTENER_VERSION: releasedComponentTag,
  COPROCESSOR_TX_SENDER_VERSION: releasedComponentTag,
  COPROCESSOR_TFHE_WORKER_VERSION: releasedComponentTag,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: releasedComponentTag,
  COPROCESSOR_SNS_WORKER_VERSION: fromTag,   // previous version (old sns-worker binary) - deployed explicitly by run.ts
  LISTENER_CORE_VERSION: releasedComponentTag,
  TEST_SUITE_VERSION: targetTag,
  RELAYER_SDK_VERSION: relayerSdkVersion,

  // Extra (non-VERSION) keys that flow through versionsEnv and are available
  // for env substitution + the coprocessor render glue we added.
  S3_MIGRATION_MODE: "no",
  CLEAN_OLD_S3_FORMAT_VERSION: "false",
} satisfies Env;

export const to = {
  ...from,
  // Most components stay "new enough"; the interesting one is SNS + migration mode.
  COPROCESSOR_SNS_WORKER_VERSION: targetTag,
  // For the migration phase we override this via the lock + env glue.
  S3_MIGRATION_MODE: "concurrent",
  // Optional: set to "true" in a cleanup phase if you want the (future) cleanup path.
  CLEAN_OLD_S3_FORMAT_VERSION: "false",
} satisfies Env;

type EnvKey = keyof typeof from;

const withTargetValues = (...keys: EnvKey[]): Env => ({
  ...from,
  ...Object.fromEntries(keys.map((key) => [key, to[key]])),
});

export const phaseVersions = {
  // Baseline: everything on "from" (old SNS). DB migration runs first (adds the column + backfills pre-existing).
  baseline: from,

  // sns phase: new SNS binary + concurrent migration mode.
  // We only actually upgrade the sns-worker service (see run.ts + per-service override).
  sns: withTargetValues(
    "COPROCESSOR_SNS_WORKER_VERSION",
    "S3_MIGRATION_MODE",
    "CLEAN_OLD_S3_FORMAT_VERSION",
  ),
};

export const versionSources = [
  `rollout=sns-worker-s3-format-migration`,
  `target=${targetTag}`,
  `pre-s3-sns=${fromTag}`,
];
