import path from "node:path";

import { LOCK_DIR } from "../layout";
import type { UpOptions, VersionBundle, VersionTarget } from "../types";
import { exists, readJson, writeJson } from "../utils";
import { GitHubApiError } from "./errors";
import {
  PACKAGE_TO_REPOSITORY,
  REPO_KEYS,
  REPO_TAG,
  SHA_RUNTIME_COMPAT_MIN_SHA,
  applyVersionEnvOverrides,
  resolveTarget,
  shaRuntimeCompatFloor,
} from "./resolve";
import { mainCommits } from "./github";

const VERSION_KEYS = Object.keys(PACKAGE_TO_REPOSITORY);

const resolveCachePath = (target: string, sha?: string) => {
  const normalizedSha = sha?.toLowerCase();
  const suffix = normalizedSha
    ? normalizedSha.length === 40
      ? normalizedSha
      : normalizedSha.slice(0, 7)
    : undefined;
  return path.join(LOCK_DIR, `.cache-${target}${suffix ? `-${suffix}` : ""}.json`);
};

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

const validateLockedRuntimeCompat = async (bundle: VersionBundle) => {
  const taggedKeys = [...REPO_KEYS].filter((key) => REPO_TAG.test(bundle.env[key] ?? ""));
  if (!taggedKeys.length) {
    return bundle;
  }
  let compatFloor: number;
  const commits = await mainCommits(5000);
  try {
    compatFloor = shaRuntimeCompatFloor(commits);
  } catch (error) {
    throw new GitHubApiError(error instanceof Error ? error.message : String(error));
  }
  const unsupported = taggedKeys
    .map((key) => {
      const tag = bundle.env[key];
      const index = commits.findIndex((sha) => sha.startsWith(tag));
      return { key, tag, index };
    })
    .filter(({ index }) => index < 0 || index > compatFloor);
  if (!unsupported.length) {
    return bundle;
  }
  const missing = unsupported.filter(({ index }) => index < 0);
  if (missing.length) {
    throw new GitHubApiError(
      `Lock file contains unsupported repo-owned shas: ${missing.map(({ key, tag }) => `${key}=${tag}`).join(", ")}; only main commits at or after ${SHA_RUNTIME_COMPAT_MIN_SHA.slice(0, 7)} are supported`,
    );
  }
  throw new GitHubApiError(
    `Lock file contains repo-owned shas that predate the modern gw-listener drift-address cutover (${SHA_RUNTIME_COMPAT_MIN_SHA.slice(0, 7)}): ${unsupported.map(({ key, tag }) => `${key}=${tag}`).join(", ")}; regenerate the lock file from latest-main or a newer sha`,
  );
};

const writeLock = async (bundle: VersionBundle) => {
  const file = path.join(LOCK_DIR, bundle.lockName);
  try {
    await writeJson(file, bundle);
  } catch (error) {
    throw new GitHubApiError(`Failed to write lock file: ${error}`);
  }
  return file;
};

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
  return validateLockedRuntimeCompat({
    ...bundle,
    target: bundle.target ?? target ?? "latest-main",
  });
};

type CachedResolveOptions = Pick<UpOptions, "target" | "requestedTarget" | "sha" | "lockFile" | "reset">;

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

const cachedResolve = async (options: CachedResolveOptions) => {
  if (options.lockFile) {
    console.log(`[resolve] reading lock file ${options.lockFile}`);
    return bundleFromFile(options.requestedTarget, options.lockFile);
  }

  const cachePath = resolveCachePath(options.target, options.sha);
  if (!options.reset) {
    try {
      const cached = await readJson<VersionBundle>(cachePath);
      if (cached.target === options.target) {
        console.log(`[resolve] using cached ${options.target} bundle`);
        const validated = validateLockBundleShape(cached);
        return await validateLockedRuntimeCompat(validated);
      }
    } catch {
      // cache miss
    }
  }

  console.log(`[resolve] resolving ${options.target} bundle`);
  if (options.target === "latest-main" || options.target === "sha") {
    console.log("[resolve] fetching main commits and published image tags");
  }
  const bundle = await withProgressLogs(resolveTarget(options.target, { sha: options.sha }), `still fetching ${options.target} metadata`);
  try {
    await writeJson(cachePath, bundle);
  } catch (error) {
    throw new GitHubApiError(`Failed to write cache: ${error}`);
  }
  return bundle;
};

export const resolveBundle = async (
  options: CachedResolveOptions,
  env: Record<string, string | undefined>,
) => {
  const bundle = await cachedResolve(options);
  const resolved = applyVersionEnvOverrides(bundle, env);
  const lockPath = await writeLock(resolved);
  return { bundle: resolved, lockPath };
};

export const previewBundle = async (
  options: CachedResolveOptions,
  env: Record<string, string | undefined>,
) => applyVersionEnvOverrides(await cachedResolve(options), env);
