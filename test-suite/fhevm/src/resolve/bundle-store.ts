/**
 * Caches resolved bundles and persists the lock snapshot used by the current local stack state.
 */
import path from "node:path";

import { validateBundleCompatibility } from "../compat/compat";
import { LOCK_DIR } from "../layout";
import type { UpOptions, VersionBundle, VersionTarget } from "../types";
import { exists, readJson, writeJson } from "../utils/fs";
import { GitHubApiError } from "../errors";
import {
  PACKAGE_TO_REPOSITORY,
  applyVersionEnvOverrides,
  assertSupportedShaBundle,
  resolveTarget,
} from "./target";
import { mainCommits } from "./github";

const VERSION_KEYS = Object.keys(PACKAGE_TO_REPOSITORY);
const SAFE_LOCK_NAME = /^[A-Za-z0-9._-]+\.json$/;

/** Computes the cache file path for a resolved bundle target. */
const resolveCachePath = (target: string, sha?: string) => {
  const normalizedSha = sha?.toLowerCase();
  const suffix = normalizedSha
    ? normalizedSha.length === 40
      ? normalizedSha
      : normalizedSha.slice(0, 7)
    : undefined;
  return path.join(LOCK_DIR, `.cache-${target}${suffix ? `-${suffix}` : ""}.json`);
};

/** Validates that a lock bundle has the fields required by the CLI. */
const validateLockBundleShape = (bundle: unknown): VersionBundle => {
  if (!bundle || typeof bundle !== "object") {
    throw new GitHubApiError("Lock file must contain a JSON object bundle");
  }
  const candidate = bundle as Partial<VersionBundle>;
  if (typeof candidate.target !== "string") {
    throw new GitHubApiError("Lock file must include a string target");
  }
  if (typeof candidate.lockName !== "string" || !candidate.lockName.length) {
    throw new GitHubApiError("Lock file must include a non-empty lockName");
  }
  if (path.basename(candidate.lockName) !== candidate.lockName || !SAFE_LOCK_NAME.test(candidate.lockName)) {
    throw new GitHubApiError(`Lock file has an invalid lockName: ${candidate.lockName}`);
  }
  if (!Array.isArray(candidate.sources) || candidate.sources.some((source) => typeof source !== "string")) {
    throw new GitHubApiError("Lock file must include a string[] sources list");
  }
  if (!candidate.env || typeof candidate.env !== "object") {
    throw new GitHubApiError("Lock file must include an env object with every version key");
  }
  const missing = VERSION_KEYS.filter((key) => typeof candidate.env?.[key] !== "string" || !candidate.env[key]?.length);
  if (missing.length) {
    throw new GitHubApiError(`Lock file is missing required version keys: ${missing.join(", ")}`);
  }
  return candidate as VersionBundle;
};

const validateBundleCompat = async (bundle: VersionBundle) => {
  const incompatibilities = validateBundleCompatibility({ versions: bundle });
  if (incompatibilities.length) {
    throw new GitHubApiError(incompatibilities.map((item) => item.message).join("\n"));
  }
  return bundle;
};

const validateRuntimeCompat = async (bundle: VersionBundle) => {
  await validateBundleCompat(bundle);
  if (bundle.target === "sha") {
    try {
      assertSupportedShaBundle(bundle, await mainCommits(5000));
    } catch (error) {
      throw new GitHubApiError(error instanceof Error ? error.message : String(error));
    }
  }
  return bundle;
};

/** Writes a resolved bundle into the persistent lock directory. */
const writeLock = async (bundle: VersionBundle) => {
  const file = path.join(LOCK_DIR, bundle.lockName);
  try {
    await writeJson(file, bundle);
  } catch (error) {
    throw new GitHubApiError(`Failed to write lock file: ${error}`);
  }
  return file;
};

/** Restores a missing persisted lock snapshot from an in-memory bundle. */
export const ensureLockSnapshot = async (lockPath: string, bundle: VersionBundle) => {
  if (await exists(lockPath)) {
    return;
  }
  try {
    await writeJson(lockPath, bundle);
  } catch (error) {
    throw new GitHubApiError(`Failed to restore lock file: ${error}`);
  }
};

/** Loads and validates a version bundle from a lock file. */
const bundleFromFile = async (target: VersionTarget | undefined, lockFile: string) => {
  let raw: VersionBundle;
  try {
    raw = await readJson<VersionBundle>(path.resolve(lockFile));
  } catch (error) {
    throw new GitHubApiError(`Failed to read lock file: ${error}`);
  }
  const bundle = validateLockBundleShape(raw);
  if (target && bundle.target && bundle.target !== target) {
    throw new GitHubApiError(`Lock file target mismatch: bundle=${bundle.target}, requested=${target}`);
  }
  return validateRuntimeCompat({
    ...bundle,
    target: bundle.target ?? target ?? "latest-main",
  });
};

type CachedResolveOptions = Pick<UpOptions, "target" | "requestedTarget" | "sha" | "lockFile" | "reset">;

/** Emits periodic progress logs while a long resolve task runs. */
const withProgressLogs = async <T>(task: Promise<T>, label: string) => {
  const timer = setInterval(() => {
    console.log(`[resolve] ${label}`);
  }, 10_000);
  try {
    return await task;
  } finally {
    clearInterval(timer);
  }
};

/** Returns whether a target should reuse the on-disk resolve cache. */
export const targetUsesCache = (target: VersionTarget) => target === "sha";

/** Resolves a bundle through lock-file, cache, or live metadata lookup. */
const cachedResolve = async (options: CachedResolveOptions) => {
  if (options.lockFile) {
    console.log(`[resolve] reading lock file ${options.lockFile}`);
    return bundleFromFile(options.requestedTarget, options.lockFile);
  }

  const cachePath = resolveCachePath(options.target, options.sha);
  if (targetUsesCache(options.target) && !options.reset && await exists(cachePath)) {
    const cached = validateLockBundleShape(await readJson<VersionBundle>(cachePath));
    if (cached.target !== options.target) {
      throw new GitHubApiError(`Cached bundle target mismatch: cache=${cached.target}, requested=${options.target}`);
    }
    console.log(`[resolve] using cached ${options.target} bundle`);
    return await validateRuntimeCompat(cached);
  }

  console.log(`[resolve] resolving ${options.target} bundle`);
  if (options.target === "latest-main" || options.target === "sha") {
    console.log("[resolve] fetching main commits and published image tags");
  }
  const bundle = await withProgressLogs(resolveTarget(options.target, { sha: options.sha }), `still fetching ${options.target} metadata`);
  try {
    if (targetUsesCache(options.target)) {
      await writeJson(cachePath, bundle);
    }
  } catch (error) {
    throw new GitHubApiError(`Failed to write cache: ${error}`);
  }
  return bundle;
};

/** Resolves and persists the bundle used for a real stack boot. */
export const resolveBundle = async (
  options: CachedResolveOptions,
  env: Record<string, string | undefined>,
) => {
  const bundle = await cachedResolve(options);
  const resolved = await validateBundleCompat(applyVersionEnvOverrides(bundle, env));
  const lockPath = await writeLock(resolved);
  return { bundle: resolved, lockPath };
};

/** Resolves a bundle for preview output without mutating persisted state. */
export const previewBundle = async (
  options: CachedResolveOptions,
  env: Record<string, string | undefined>,
) => validateBundleCompat(applyVersionEnvOverrides(await cachedResolve(options), env));
