import path from "node:path";
import { spawnSync } from "node:child_process";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { COPROCESSOR_DB_CONTAINER, DEFAULT_POSTGRES_USER, PROJECT } from "../../src/layout";
import { phaseVersions, scenario, versionSources } from "./versions";

export type RolloutEnv = Record<string, string>;

const logPhase = (label: string) => {
  console.log(`\n[rollout:s3-format] ${label}`);
};

const writePhaseVersionLock = (ctx: RolloutRunContext, name: string, versions: RolloutEnv) =>
  ctx.writeVersionLock(name, { versions, sources: versionSources });

const SNS_WORKER_SERVICE = "coprocessor-sns-worker";
const SNS_WORKER_LOCAL_OVERRIDE = { group: "coprocessor" as const, services: [SNS_WORKER_SERVICE] };
const SNS_PHASE_ALLOWED_KEYS = [
  "COPROCESSOR_SNS_WORKER_VERSION",
  "S3_MIGRATION_MODE",
  "CLEAN_OLD_S3_FORMAT_VERSION",
];
const POSTGRES_CONTAINER = process.env.POSTGRES_CONTAINER || COPROCESSOR_DB_CONTAINER;
const POSTGRES_USER = process.env.POSTGRES_USER || DEFAULT_POSTGRES_USER;
const MINIO_MC_IMAGE = process.env.MINIO_MC_IMAGE || "quay.io/minio/mc";
const MINIO_ALIAS = "localminio";
const MINIO_NETWORK = process.env.MINIO_NETWORK || `${PROJECT}_default`;
const CURRENT_CONTEXT_ID = "1";

/**
 * Prepares the image for the *previous* (old) sns-worker version.
 *
 * There are two ways to get the "old" binary for this rollout test:
 *
 * 1. **Published previous tag** (recommended for CI / "test migration from 0.13"):
 *    Set OLD_SNS_IMAGE_TAG to a real released 0.13 image tag, e.g.:
 *      OLD_SNS_IMAGE_TAG=v0.13.0-6
 *    The script will simply `docker pull` the published ghcr image.
 *    This lets you validate that data produced by a real 0.13 deployment can be migrated
 *    by the new code with concurrent migration.
 *
 * 2. **Build from previous commit** (for local development of the migration feature itself):
 *    When OLD_SNS_IMAGE_TAG is the dev tag ("pre-s3-format" or similar), or when the
 *    published image isn't available, we checkout an old commit (via PREVIOUS_SNS_COMMIT
 *    env or default) and build the binary ourselves, then tag it locally.
 *
 * Why both "previous tag" and "previous commit"?
 * - The *tag* (OLD_SNS_IMAGE_TAG) is the Docker image tag that ends up in
 *   COPROCESSOR_SNS_WORKER_VERSION for the baseline phase. It controls what image
 *   the compose stack starts for the "old" worker.
 * - The *commit* is only used as a source of the old *code* when you want to simulate
 *   the behavior of the previous version locally (before any 0.13 release contained the
 *   change, or for a custom build).
 *
 * In the GitHub workflow you can invoke this rollout with a real 0.13 tag as the old version.
 */
const DEFAULT_OLD_SNS_IMAGE_TAG = "pre-s3-format";
const DEFAULT_PREVIOUS_SNS_COMMIT = "8f52aa7cf"; // only used for local dev builds of old code

async function prepareOldSnsImage() {
  const oldTag = process.env.OLD_SNS_IMAGE_TAG || DEFAULT_OLD_SNS_IMAGE_TAG;
  const fullImageRef = `ghcr.io/zama-ai/fhevm/coprocessor/sns-worker:${oldTag}`;

  console.log(`[rollout] === Prepare old sns-worker image (previous version) ===`);
  console.log(`[rollout] OLD_SNS_IMAGE_TAG = ${oldTag}`);
  console.log(`[rollout] Full image ref    = ${fullImageRef}`);

  // If this looks like a released 0.13 (or any published) tag, just pull it.
  // This is the path used when testing "migration from real 0.13 latest".
  const isPublishedTag = /^v?0\.13|latest/i.test(oldTag) || oldTag !== DEFAULT_OLD_SNS_IMAGE_TAG;

  if (isPublishedTag) {
    console.log(`[rollout] Treating ${oldTag} as a published previous version tag (e.g. 0.13 release). Pulling...`);
    const pull = spawnSync("docker", ["pull", fullImageRef], { stdio: "inherit" });
    if (pull.status !== 0) {
      throw new Error(`Failed to docker pull previous version image: ${fullImageRef}`);
    }
    console.log(`[rollout] Successfully pulled previous version: ${fullImageRef}`);
    return;
  }

  // Dev path: build from a previous commit and tag it with the dev OLD_SNS_IMAGE_TAG.
  const previousCommit = process.env.PREVIOUS_SNS_COMMIT || DEFAULT_PREVIOUS_SNS_COMMIT;

  // Fast path if the dev tag is already present locally
  const inspect = spawnSync("docker", ["image", "inspect", fullImageRef], { stdio: "ignore" });
  if (inspect.status === 0) {
    console.log(`[rollout] Dev old image already present locally: ${fullImageRef}`);
    return;
  }

  console.log(`[rollout] Building dev old sns-worker from commit ${previousCommit} ...`);
  const worktreeDir = `/tmp/fhevm-sns-previous-${previousCommit.replace(/[^a-z0-9]/gi, "")}`;

  spawnSync("git", ["worktree", "remove", "--force", worktreeDir], { stdio: "ignore" });

  const addResult = spawnSync("git", ["worktree", "add", "--detach", worktreeDir, previousCommit], { encoding: "utf8" });
  if (addResult.status !== 0) {
    throw new Error(
      `Failed to checkout previous commit ${previousCommit} to build old sns-worker.\n` +
      `${addResult.stderr || addResult.stdout}\n` +
      `Alternatively set OLD_SNS_IMAGE_TAG to a published pre-S3-format 0.13 tag (e.g. v0.13.0-6) so we just pull it.`
    );
  }

  try {
    const buildContext = path.join(worktreeDir, "coprocessor/fhevm-engine");

    const buildResult = spawnSync(
      "docker",
      [
        "build",
        "-f",
        path.join(buildContext, "Dockerfile.workspace"),
        "-t",
        fullImageRef,
        "--target",
        "sns-worker",
        "--progress=plain",
        buildContext,
      ],
      {
        stdio: "inherit",
        cwd: buildContext,
        env: { ...process.env, DOCKER_BUILDKIT: "1" },
      }
    );

    if (buildResult.status !== 0) {
      throw new Error("docker build of previous (dev) sns-worker version failed.");
    }

    console.log(`[rollout] Previous dev version built and tagged: ${fullImageRef}`);
  } finally {
    spawnSync("git", ["worktree", "remove", "--force", worktreeDir], { stdio: "ignore" });
  }
}

/**
 * Runs a shell command and returns {code, stdout, stderr}.
 * Used for psql checks and S3 verification (no extra node deps).
 */
type ExpectedS3Object = { bucket: "ct64" | "ct128"; key: string; source: string };

function run(cmd: string[], opts: { cwd?: string; env?: Record<string, string> } = {}) {
  const res = spawnSync(cmd[0], cmd.slice(1), {
    encoding: "utf8",
    cwd: opts.cwd,
    env: { ...process.env, ...(opts.env ?? {}) },
  });
  return { code: res.status ?? 1, stdout: res.stdout ?? "", stderr: res.stderr ?? "" };
}

function runChecked(label: string, cmd: string[], opts: { cwd?: string; env?: Record<string, string> } = {}) {
  const res = run(cmd, opts);
  if (res.code !== 0) {
    throw new Error(`${label} failed (${res.code}): ${cmd.join(" ")}\n${res.stderr || res.stdout}`);
  }
  return res;
}

const driftDatabaseName = (index: number) => (index === 0 ? "coprocessor" : `coprocessor_${index}`);

async function coprocessorDatabases(ctx: RolloutRunContext) {
  const state = await ctx.readState();
  return Array.from({ length: state.scenario.topology.count }, (_, index) => driftDatabaseName(index));
}

const psql = (db: string, args: string[], label: string) =>
  runChecked(label, [
    "docker",
    "exec",
    "-i",
    POSTGRES_CONTAINER,
    "psql",
    "-U",
    POSTGRES_USER,
    "-d",
    db,
    ...args,
  ]);

const psqlExec = (db: string, sql: string, label: string) =>
  psql(db, ["-v", "ON_ERROR_STOP=1", "-c", sql], label);

const psqlRows = (db: string, sql: string, label: string) =>
  psql(db, ["-v", "ON_ERROR_STOP=1", "-tA", "-F", "\t", "-c", sql], label)
    .stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);

function psqlCount(db: string, sql: string, label: string) {
  const res = psql(db, ["-v", "ON_ERROR_STOP=1", "-tAc", sql], label);
  const trimmed = res.stdout.trim();
  const value = Number(trimmed);
  if (!Number.isInteger(value)) {
    throw new Error(`${label} returned a non-integer count: ${JSON.stringify(trimmed)}`);
  }
  return value;
}

function formatCounts(counts: Record<string, number>) {
  return Object.entries(counts)
    .map(([db, count]) => `${db}=${count}`)
    .join(", ");
}

const s3VerifyLimitClause = () => {
  const raw = process.env.S3_VERIFY_LIMIT_PER_DB;
  if (!raw) {
    return "";
  }
  const limit = Number(raw);
  if (!Number.isInteger(limit) || limit < 1) {
    throw new Error(`S3_VERIFY_LIMIT_PER_DB must be a positive integer, got ${JSON.stringify(raw)}`);
  }
  return `LIMIT ${limit}`;
};

function s3MetadataHasAttestation(stdout: string) {
  try {
    return JSON.stringify(JSON.parse(stdout)).toLowerCase().includes("ct-attestation");
  } catch {
    return stdout.toLowerCase().includes("ct-attestation");
  }
}

/**
 * Normalize any ciphertext_digest rows created by the *old* binary (they land with s3_format_version=NULL
 * because the old code path never wrote the column). The migration only looks for =0.
 * MUST be called after baseline tests (legacy CTs written) and BEFORE starting the new sns-worker with Concurrent.
 */
async function normalizeLegacyNullsToZero(dbs: string[]) {
  console.log("[rollout] normalizing legacy NULL s3_format_version -> 0 so concurrent migration will pick them up");

  for (const db of dbs) {
    const sql = `
      UPDATE ciphertext_digest
      SET s3_format_version = 0
      WHERE s3_format_version IS NULL
        AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL);
    `;
    const res = psqlExec(db, sql, `normalize legacy rows in ${db}`);
    console.log(`[rollout]   ${db}: ${res.stdout.trim() || "updated"}`);
  }
}

/**
 * Wait for the concurrent migration to reach zero old-format backlog in every coprocessor DB.
 * Concurrent mode intentionally keeps running and sleeping after the backlog reaches zero.
 */
async function waitForConcurrentMigration(dbs: string[], timeoutMs = Number(process.env.S3_MIGRATION_WAIT_MS ?? 300_000)) {
  console.log("[rollout] waiting for concurrent S3 migration to finish...");
  const start = Date.now();
  let lastReport = 0;

  while (Date.now() - start < timeoutMs) {
    const counts = Object.fromEntries(
      dbs.map((db) => [
        db,
        psqlCount(
          db,
          `SELECT COUNT(*)::bigint
           FROM ciphertext_digest
           WHERE (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
             AND (s3_format_version = 0 OR s3_format_version IS NULL);`,
          `count legacy S3 rows in ${db}`,
        ),
      ]),
    );
    const remaining = Object.values(counts).reduce((sum, count) => sum + count, 0);
    if (remaining === 0) {
      console.log("[rollout] no more legacy rows in coprocessor DBs");
      return;
    }

    if (Date.now() - lastReport > 15_000) {
      console.log(`[rollout] migration backlog: ${formatCounts(counts)}`);
      lastReport = Date.now();
    }

    await new Promise((r) => setTimeout(r, 3000));
  }

  const counts = Object.fromEntries(
    dbs.map((db) => [
      db,
      psqlCount(
        db,
        `SELECT COUNT(*)::bigint
         FROM ciphertext_digest
         WHERE (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
           AND (s3_format_version = 0 OR s3_format_version IS NULL);`,
        `count legacy S3 rows in ${db} after timeout`,
      ),
    ]),
  );
  throw new Error(`S3 migration wait timed out with backlog: ${formatCounts(counts)}`);
}

/**
 * Verification: DB side.
 * After everything, every row that has a ct or ct128 must be on v1.
 */
async function assertDbAllS3FormatV1(dbs: string[]) {
  console.log("[rollout] verifying DB: all ciphertext_digest rows with data are s3_format_version=1");

  let totalLegacy = 0;

  for (const db of dbs) {
    const bad = psqlCount(
      db,
      `
      SELECT COUNT(*)::bigint
      FROM ciphertext_digest
      WHERE (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
        AND (s3_format_version IS DISTINCT FROM 1);
    `,
      `verify DB S3 format in ${db}`,
    );
    totalLegacy += bad;
    console.log(`[rollout]   ${db}: ${bad} rows not on v1`);
  }

  if (totalLegacy > 0) {
    throw new Error(`DB verification failed: ${totalLegacy} legacy (non-v1) rows found`);
  }
  console.log("[rollout] DB verification passed (all relevant rows are v1)");
}

/**
 * Verification: S3 side (minio).
 * After migration, every sampled DB row must have current-format S3 objects under handle/<context>.
 * CT128 rows must also keep the digest-key compatibility object.
 *
 * We use the same minio mc image as the stack setup service via a throwaway container.
 */
async function expectedS3Objects(dbs: string[]) {
  const expected = new Map<string, ExpectedS3Object>();
  const limit = s3VerifyLimitClause();
  for (const db of dbs) {
    const rows = psqlRows(
      db,
      `SELECT encode(handle, 'hex'),
              (ciphertext IS NOT NULL)::int,
              COALESCE(encode(ciphertext128, 'hex'), '')
       FROM ciphertext_digest
       WHERE (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
       ORDER BY handle
       ${limit};`,
      `list expected S3 objects in ${db}`,
    );
    for (const row of rows) {
      const [handle, hasCt64, ct128Digest] = row.split("\t");
      if (!handle) {
        continue;
      }
      const currentKey = `${handle}/${CURRENT_CONTEXT_ID}`;
      if (hasCt64 === "1") {
        expected.set(`ct64/${currentKey}`, { bucket: "ct64", key: currentKey, source: `${db}:${handle}:ct64` });
      }
      if (ct128Digest) {
        expected.set(`ct128/${currentKey}`, { bucket: "ct128", key: currentKey, source: `${db}:${handle}:ct128-handle` });
        expected.set(`ct128/${ct128Digest}`, { bucket: "ct128", key: ct128Digest, source: `${db}:${handle}:ct128-digest` });
      }
    }
  }
  return [...expected.values()];
}

async function assertS3Attestations(dbs: string[]) {
  console.log("[rollout] verifying S3: current-format objects carry ct-attestation metadata");

  const expected = await expectedS3Objects(dbs);
  if (expected.length === 0) {
    throw new Error("S3 verification failed: no ciphertext rows found in DBs after rollout tests");
  }

  const access = process.env.AWS_ACCESS_KEY_ID || "fhevm-access-key";
  const secret = process.env.AWS_SECRET_ACCESS_KEY || "fhevm-access-secret-key";

  let checked = 0;
  const missingObjects: string[] = [];
  const missingMetadata: string[] = [];

  for (const item of expected) {
    const stat = run([
      "docker",
      "run",
      "--rm",
      "--network",
      MINIO_NETWORK,
      "-e",
      `MC_HOST_${MINIO_ALIAS}=http://${access}:${secret}@minio:9000`,
      MINIO_MC_IMAGE,
      "stat",
      "--json",
      `${MINIO_ALIAS}/${item.bucket}/${item.key}`,
    ]);

    if (stat.code !== 0) {
      missingObjects.push(`${item.bucket}/${item.key} (${item.source}): ${stat.stderr || stat.stdout}`);
      continue;
    }

    if (!s3MetadataHasAttestation(stat.stdout)) {
      missingMetadata.push(`${item.bucket}/${item.key} (${item.source})`);
      continue;
    }

    checked++;
  }

  if (missingObjects.length || missingMetadata.length) {
    const details = [
      ...missingObjects.slice(0, 10).map((item) => `missing object: ${item}`),
      ...missingMetadata.slice(0, 10).map((item) => `missing ct-attestation: ${item}`),
    ].join("\n");
    throw new Error(
      `S3 verification failed: ${missingObjects.length} missing objects, ${missingMetadata.length} objects without ct-attestation.\n${details}`,
    );
  }
  console.log(`[rollout] S3 verification passed (${checked} current-format objects checked with ct-attestation)`);
}

const testPhase = async (ctx: RolloutRunContext, phase: string) => {
  console.log(`[rollout] ${phase} tests (rollout-standard)`);
  await ctx.test("rollout-standard", { parallel: false });
};

export default async function run(ctx: RolloutRunContext) {
  // Allow overriding the "old" sns-worker version/tag from the environment.
  // This is how you run the scenario using a published old-format 0.13 image as the previous version:
  //   OLD_SNS_IMAGE_TAG=v0.13.0-6 ./fhevm-cli rollout run rollouts/sns-worker-s3-format-migration/run.ts
  const oldSnsVersion = process.env.OLD_SNS_IMAGE_TAG || phaseVersions.baseline.COPROCESSOR_SNS_WORKER_VERSION;

  // Build a baseline versions object that uses the (possibly overridden) old tag.
  const baselineVersions = {
    ...phaseVersions.baseline,
    COPROCESSOR_SNS_WORKER_VERSION: oldSnsVersion,
  };

  const baselineLock = await writePhaseVersionLock(ctx, "00-baseline", baselineVersions);
  const snsLock = await writePhaseVersionLock(ctx, "01-sns", phaseVersions.sns);

  // ========================================================================
  // Explicit "prepare old / previous version" step (deploy at previous version).
  // - If OLD_SNS_IMAGE_TAG is a 0.13 release tag → docker pull the published image.
  // - Otherwise (local dev) → build from PREVIOUS_SNS_COMMIT and tag it.
  // ========================================================================
  await prepareOldSnsImage();

  logPhase("00 baseline: boot with OLD sns-worker (pre S3 format v1)");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  const dbs = await coprocessorDatabases(ctx);

  // DB migration has already run (column exists, old rows backfilled to 0).
  await testPhase(ctx, "baseline");

  // Critical (from review): normalize any NULLs created by the old binary
  // BEFORE we start the new worker with Concurrent migration.
  await normalizeLegacyNullsToZero(dbs);

  logPhase("01 sns: upgrade only the sns-worker to NEW code + concurrent migration");
  // Apply the SNS phase config first, including the env-only migration mode change.
  // Then add a coprocessor override scoped to sns-worker so the upgrade builds only
  // the current local SNS binary and leaves the rest of the coprocessor group on
  // the locked published images.
  await ctx.applyVersionLock("01-sns", {
    lockFile: snsLock,
    allowedVersionKeys: SNS_PHASE_ALLOWED_KEYS,
    overrides: [SNS_WORKER_LOCAL_OVERRIDE],
  });
  await ctx.upgradeRuntimeGroup("coprocessor");

  // The new worker is now running with S3_MIGRATION_MODE=concurrent (from the phase env + glue in env.ts).
  // Migration runs in the background while normal uploads continue.
  await testPhase(ctx, "sns-after-upgrade");

  // Give the background migration a chance to finish on the legacy rows.
  await waitForConcurrentMigration(dbs);

  // Final assertions: everything (legacy + newly created) must be on the new format on both DB and S3.
  await assertDbAllS3FormatV1(dbs);
  await assertS3Attestations(dbs);

  console.log("\n[rollout:s3-format] SUCCESS: legacy data migrated, new data written in v1 format, all checks passed.");
}
