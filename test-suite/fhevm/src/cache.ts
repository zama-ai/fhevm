import path from "node:path";
import { Effect } from "effect";

import { GitHubApiError } from "./errors";
import { LOCK_DIR } from "./layout";
import {
  applyVersionEnvOverrides,
  PACKAGE_TO_REPOSITORY,
  REPO_KEYS,
  REPO_TAG,
  resolveTarget,
  SHA_RUNTIME_COMPAT_MIN_SHA,
  shaRuntimeCompatFloor,
} from "./resolve";
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

const VERSION_KEYS = Object.keys(PACKAGE_TO_REPOSITORY);

const validateLockBundleShape = (
  bundle: unknown,
): Effect.Effect<VersionBundle, GitHubApiError> => {
  if (!bundle || typeof bundle !== "object") {
    return Effect.fail(new GitHubApiError({ message: "Lock file must contain a JSON object bundle" }));
  }
  const candidate = bundle as Partial<VersionBundle>;
  if (typeof candidate.target !== "string") {
    return Effect.fail(new GitHubApiError({ message: "Lock file must include a string target" }));
  }
  if (typeof candidate.lockName !== "string" || !candidate.lockName.length) {
    return Effect.fail(new GitHubApiError({ message: "Lock file must include a non-empty lockName" }));
  }
  if (!Array.isArray(candidate.sources) || candidate.sources.some((source) => typeof source !== "string")) {
    return Effect.fail(new GitHubApiError({ message: "Lock file must include a string[] sources list" }));
  }
  if (!candidate.env || typeof candidate.env !== "object") {
    return Effect.fail(new GitHubApiError({ message: "Lock file must include an env object with every version key" }));
  }
  const missing = VERSION_KEYS.filter((key) => typeof candidate.env?.[key] !== "string" || !candidate.env[key]?.length);
  if (missing.length) {
    return Effect.fail(
      new GitHubApiError({
        message: `Lock file is missing required version keys: ${missing.join(", ")}`,
      }),
    );
  }
  return Effect.succeed(candidate as VersionBundle);
};

const lockedRepoSha = (bundle: VersionBundle) => {
  const repoOwnedVersions = [...REPO_KEYS].map((key) => bundle.env[key]);
  const first = repoOwnedVersions[0];
  return first && repoOwnedVersions.every((value) => value === first) && REPO_TAG.test(first)
    ? first
    : undefined;
};

const validateLockedRuntimeCompat = (
  bundle: VersionBundle,
): Effect.Effect<VersionBundle, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    const tag = lockedRepoSha(bundle);
    if (!tag) {
      return bundle;
    }
    const gh = yield* GitHubClient;
    const commits = yield* gh.mainCommits(5000);
    let compatFloor: number;
    try {
      compatFloor = shaRuntimeCompatFloor(commits);
    } catch (error) {
      return yield* Effect.fail(
        new GitHubApiError({ message: error instanceof Error ? error.message : String(error) }),
      );
    }
    const index = commits.findIndex((sha) => sha.startsWith(tag));
    if (index < 0) {
      return yield* Effect.fail(
        new GitHubApiError({
          message: `Lock file repo-owned sha ${tag} is unsupported; only main commits at or after ${SHA_RUNTIME_COMPAT_MIN_SHA.slice(0, 7)} are supported`,
        }),
      );
    }
    if (index > compatFloor) {
      return yield* Effect.fail(
        new GitHubApiError({
          message: `Lock file repo-owned sha ${tag} predates the modern gw-listener drift-address cutover (${SHA_RUNTIME_COMPAT_MIN_SHA.slice(0, 7)}) and is unsupported by the current CLI; regenerate the lock file from latest-main or a newer sha`,
        }),
      );
    }
    return bundle;
  });

const writeLock = (bundle: VersionBundle): Effect.Effect<string, GitHubApiError> =>
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
  target: VersionTarget | undefined,
  lockFile: string,
): Effect.Effect<VersionBundle, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    const raw = yield* Effect.tryPromise({
      try: () => readJson<VersionBundle>(path.resolve(lockFile)),
      catch: (error) => new GitHubApiError({ message: `Failed to read lock file: ${error}` }),
    });
    const bundle = yield* validateLockBundleShape(raw);
    if (target && bundle.target && bundle.target !== target) {
      return yield* Effect.fail(
        new GitHubApiError({
          message: `Lock file target mismatch: bundle=${bundle.target}, requested=${target}`,
        }),
      );
    }
    return yield* validateLockedRuntimeCompat({
      ...bundle,
      target: bundle.target ?? target ?? "latest-main",
    });
  });

type CachedResolveOptions = Pick<UpOptions, "target" | "requestedTarget" | "sha" | "lockFile" | "reset">;

const withProgressLogs = <A, E, R>(
  task: Effect.Effect<A, E, R>,
  label: string,
) =>
  Effect.acquireUseRelease(
    Effect.sync(() =>
      setInterval(() => {
        console.log(`[resolve] ${label}`);
      }, 10_000),
    ),
    () => task,
    (timer) => Effect.sync(() => clearInterval(timer)),
  );

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
      yield* Effect.log(`[resolve] reading lock file ${options.lockFile}`);
      return yield* bundleFromFile(options.requestedTarget, options.lockFile);
    }

    const cachePath = resolveCachePath(options.target, options.sha);

    if (!options.reset) {
      const cached = yield* Effect.tryPromise({
        try: () => readJson<VersionBundle>(cachePath),
        catch: () => new GitHubApiError({ message: "cache-miss" }),
      }).pipe(Effect.option);
      if (cached._tag === "Some" && cached.value.target === options.target) {
        yield* Effect.log(`[resolve] using cached ${options.target} bundle`);
        return cached.value;
      }
    }

    yield* Effect.log(`[resolve] resolving ${options.target} bundle`);
    if (options.target === "latest-main" || options.target === "sha") {
      yield* Effect.log("[resolve] fetching main commits and published image tags");
    }
    const bundle = yield* withProgressLogs(
      resolveTarget(options.target, { sha: options.sha }),
      `still fetching ${options.target} metadata`,
    );

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
