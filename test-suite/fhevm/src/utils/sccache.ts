/**
 * Shared sccache build wiring for the source-built Rust images.
 *
 * The per-image coprocessor and kms-connector Dockerfiles activate the sccache
 * compiler cache ONLY when SCCACHE_BUCKET is set at build time; otherwise they run a plain cargo
 * build. So every helper here is gated on SCCACHE_BUCKET being present in the environment. With it
 * unset (local dev, forks) nothing is emitted: the generated compose documents and the resulting
 * `docker compose build` invocation are byte-for-byte identical to before.
 *
 * The names below mirror the publishing CI (zama-ai/ci-templates common-docker.yml @v1.0.11): the
 * S3 bucket/region/prefix arrive as build args, and the AWS credentials arrive as BuildKit secrets
 * sourced from the AWS_ACCESS_KEY_S3_USER / AWS_SECRET_KEY_S3_USER environment variables. Sharing
 * the same names keeps the solana-e2e builds on the same S3 cache as main's per-image builds.
 */

/** BuildKit secret ids the Rust Dockerfiles mount for sccache S3 authentication. */
export const SCCACHE_ACCESS_KEY_SECRET_ID = "sccache_aws_access_key_id";
export const SCCACHE_SECRET_KEY_SECRET_ID = "sccache_aws_secret_access_key";

/** Environment variables that carry the S3-cache AWS credentials (mirrors main's publishing CI). */
export const SCCACHE_ACCESS_KEY_ENV = "AWS_ACCESS_KEY_S3_USER";
export const SCCACHE_SECRET_KEY_ENV = "AWS_SECRET_KEY_S3_USER";

/** Build args forwarded to the Rust builder stage, in the order the Dockerfile declares them. */
const SCCACHE_BUILD_ARG_ENV = ["SCCACHE_BUCKET", "SCCACHE_REGION", "SCCACHE_S3_PREFIX"] as const;

/** True when sccache should be wired into the source builds. */
export const sccacheEnabled = () => !!process.env.SCCACHE_BUCKET;

/** Build args forwarded to the Rust builder stage (empty map when disabled). */
export const sccacheBuildArgs = (): Record<string, string> => {
  if (!sccacheEnabled()) {
    return {};
  }
  const args: Record<string, string> = {};
  for (const key of SCCACHE_BUILD_ARG_ENV) {
    const value = process.env[key];
    if (value !== undefined && value !== "") {
      args[key] = value;
    }
  }
  return args;
};

/** BuildKit secret ids a service's `build.secrets` should reference (empty when disabled). */
export const sccacheBuildSecretIds = (): string[] =>
  sccacheEnabled() ? [SCCACHE_ACCESS_KEY_SECRET_ID, SCCACHE_SECRET_KEY_SECRET_ID] : [];

/** Top-level compose `secrets:` mapping BuildKit ids to their env sources (empty when disabled). */
export const sccacheComposeSecrets = (): Record<string, { environment: string }> =>
  sccacheEnabled()
    ? {
        [SCCACHE_ACCESS_KEY_SECRET_ID]: { environment: SCCACHE_ACCESS_KEY_ENV },
        [SCCACHE_SECRET_KEY_SECRET_ID]: { environment: SCCACHE_SECRET_KEY_ENV },
      }
    : {};

/** Env vars the CLI forwards to `docker compose` so environment-sourced secrets resolve (empty when disabled). */
export const sccacheComposeEnv = (): Record<string, string> => {
  if (!sccacheEnabled()) {
    return {};
  }
  const env: Record<string, string> = {};
  for (const key of [...SCCACHE_BUILD_ARG_ENV, SCCACHE_ACCESS_KEY_ENV, SCCACHE_SECRET_KEY_ENV]) {
    const value = process.env[key];
    if (value !== undefined) {
      env[key] = value;
    }
  }
  return env;
};
