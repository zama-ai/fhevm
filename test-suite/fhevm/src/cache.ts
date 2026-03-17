import path from "node:path";
import { Effect } from "effect";

import { GitHubApiError } from "./errors";
import { LOCK_DIR } from "./layout";
import { applyVersionEnvOverrides, resolveTarget } from "./resolve";
import { GitHubClient } from "./services/GitHubClient";
import type { UpOptions, VersionBundle, VersionTarget } from "./types";
import { exists, readJson, writeJson } from "./utils";

// ---------------------------------------------------------------------------
// Pure helpers
// ---------------------------------------------------------------------------

export const resolveCachePath = (target: string, sha?: string) => {
  const normalizedSha = sha?.toLowerCase();
  const suffix = normalizedSha
    ? normalizedSha.length === 40
      ? normalizedSha
      : normalizedSha.slice(0, 7)
    : undefined;
  return path.join(LOCK_DIR, `.cache-${target}${suffix ? `-${suffix}` : ""}.json`);
};

// ---------------------------------------------------------------------------
// Effect-based functions
// ---------------------------------------------------------------------------

export const writeLock = (bundle: VersionBundle): Effect.Effect<string, GitHubApiError> =>
  Effect.gen(function* () {
    const file = path.join(LOCK_DIR, bundle.lockName);
    yield* Effect.tryPromise({
      try: () => writeJson(file, bundle),
      catch: (error) => new GitHubApiError({ message: `Failed to write lock file: ${error}` }),
    });
    return file;
  });

export const ensureLockSnapshot = (
  lockPath: string,
  bundle: VersionBundle,
): Effect.Effect<void, GitHubApiError> =>
  Effect.gen(function* () {
    if (yield* Effect.promise(() => exists(lockPath))) {
      return;
    }
    yield* Effect.tryPromise({
      try: () => writeJson(lockPath, bundle),
      catch: (error) => new GitHubApiError({ message: `Failed to restore lock file: ${error}` }),
    });
  });

/**
 * Read a VersionBundle from a lock file on disk.
 * Fails with GitHubApiError if the file cannot be read or the target mismatches.
 */
export const bundleFromFile = (
  target: VersionTarget,
  lockFile: string,
): Effect.Effect<VersionBundle, GitHubApiError> =>
  Effect.gen(function* () {
    const bundle = yield* Effect.tryPromise({
      try: () => readJson<VersionBundle>(path.resolve(lockFile)),
      catch: (error) => new GitHubApiError({ message: `Failed to read lock file: ${error}` }),
    });
    if (bundle.target && bundle.target !== target) {
      return yield* Effect.fail(
        new GitHubApiError({
          message: `Lock file target mismatch: bundle=${bundle.target}, requested=${target}`,
        }),
      );
    }
    return { ...bundle, target };
  });

type CachedResolveOptions = Pick<UpOptions, "target" | "sha" | "lockFile" | "reset">;

/**
 * Resolve a VersionBundle, using a lock file or cache when available.
 * Falls through to GitHub resolution via `resolveTarget` when no cached
 * version is found (or `reset` is true).
 */
export const cachedResolve = (
  options: CachedResolveOptions,
): Effect.Effect<VersionBundle, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    if (options.lockFile) {
      return yield* bundleFromFile(options.target, options.lockFile);
    }

    const cachePath = resolveCachePath(options.target, options.sha);

    if (!options.reset) {
      const cached = yield* Effect.tryPromise({
        try: () => readJson<VersionBundle>(cachePath),
        catch: () => new GitHubApiError({ message: "cache-miss" }),
      }).pipe(Effect.option);
      if (cached._tag === "Some" && cached.value.target === options.target) {
        return cached.value;
      }
    }

    const bundle = yield* resolveTarget(options.target, { sha: options.sha });

    yield* Effect.tryPromise({
      try: () => writeJson(cachePath, bundle),
      catch: (error) => new GitHubApiError({ message: `Failed to write cache: ${error}` }),
    });

    return bundle;
  });

/**
 * Resolve a bundle (with caching), apply env overrides, and write a lock file.
 * Returns both the resolved bundle and the path of the written lock file.
 */
export const resolveBundle = (
  options: CachedResolveOptions,
  env: Record<string, string | undefined>,
): Effect.Effect<{ bundle: VersionBundle; lockPath: string }, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    const bundle = yield* cachedResolve(options);
    const resolved = applyVersionEnvOverrides(bundle, env);
    const lockPath = yield* writeLock(resolved);
    return { bundle: resolved, lockPath };
  });

/**
 * Resolve a bundle (with caching) and apply env overrides, but do NOT write
 * a lock file. Used for preview/dry-run scenarios.
 */
export const previewBundle = (
  options: CachedResolveOptions,
  env: Record<string, string | undefined>,
): Effect.Effect<VersionBundle, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    const bundle = yield* cachedResolve(options);
    return applyVersionEnvOverrides(bundle, env);
  });
