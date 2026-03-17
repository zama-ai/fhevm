import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import {
  bundleFromFile,
  cachedResolve,
  ensureLockSnapshot,
  previewBundle,
  resolveBundle,
  resolveCachePath,
} from "./cache";
import { COMPAT_MATRIX } from "./compat";
import { GitHubApiError } from "./errors";
import { LOCK_DIR } from "./layout";
import { GitHubClient } from "./services/GitHubClient";
import type { VersionBundle } from "./types";
import { writeJson } from "./utils";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const makeBundle = (overrides: Partial<VersionBundle> = {}): VersionBundle => ({
  target: "latest-main",
  lockName: "latest-main-abc1234.json",
  env: {
    GATEWAY_VERSION: "abc1234",
    HOST_VERSION: "abc1234",
  },
  sources: ["preset=latest-main"],
  ...overrides,
});

/** Write a JSON file to a temp directory and return the file path. */
const writeTempBundle = async (bundle: VersionBundle, filename = "bundle.json") => {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), "cache-test-"));
  const file = path.join(dir, filename);
  await writeJson(file, bundle);
  return { dir, file };
};

// A stub GitHubClient that returns a known preset bundle
const RESOLVED_BUNDLE: VersionBundle = makeBundle({
  sources: ["preset=latest-main", "repo-owned=abc1234"],
});

const TestGitHubClient = Layer.succeed(GitHubClient, {
  latestStableRelease: () => Effect.succeed("v0.11.0"),
  mainCommits: () =>
    Effect.succeed([
      "abc1234000000000000000000000000000000dead",
      COMPAT_MATRIX.anchors.SIMPLE_ACL_MIN_SHA,
    ]),
  packageTags: () => Effect.succeed(new Set(["abc1234", "v0.11.0"])),
  gitopsFile: () => Effect.succeed(""),
});

// ---------------------------------------------------------------------------
// Tests: resolveCachePath (pure)
// ---------------------------------------------------------------------------

describe("resolveCachePath", () => {
  test("returns path with truncated short sha suffix", () => {
    const result = resolveCachePath("sha", "ABCDEF1234567890");
    expect(result).toBe(path.join(LOCK_DIR, ".cache-sha-abcdef1.json"));
  });

  test("keeps the full 40-char sha in the cache key", () => {
    const sha = "ABCDEF1234567890ABCDEF1234567890ABCDEF12";
    const result = resolveCachePath("sha", sha);
    expect(result).toBe(path.join(LOCK_DIR, `.cache-sha-${sha.toLowerCase()}.json`));
  });
});

// ---------------------------------------------------------------------------
// Tests: bundleFromFile (Effect-based)
// ---------------------------------------------------------------------------

describe("bundleFromFile", () => {
  test("reads and returns bundle from disk", async () => {
    const bundle = makeBundle();
    const { dir, file } = await writeTempBundle(bundle);
    const result = await Effect.runPromise(bundleFromFile("latest-main", file));
    expect(result.target).toBe("latest-main");
    expect(result.env.GATEWAY_VERSION).toBe("abc1234");
    await fs.rm(dir, { recursive: true });
  });

  test("sets target from argument when bundle target matches", async () => {
    const bundle = makeBundle({ target: "latest-main" });
    const { dir, file } = await writeTempBundle(bundle);
    const result = await Effect.runPromise(bundleFromFile("latest-main", file));
    expect(result.target).toBe("latest-main");
    await fs.rm(dir, { recursive: true });
  });

  test("fails with GitHubApiError on target mismatch", async () => {
    const bundle = makeBundle({ target: "devnet" });
    const { dir, file } = await writeTempBundle(bundle);
    const result = await Effect.runPromise(
      bundleFromFile("latest-main", file).pipe(Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(GitHubApiError);
      expect(result.left.message).toContain("Lock file target mismatch");
    }
    await fs.rm(dir, { recursive: true });
  });

  test("fails with GitHubApiError when file does not exist", async () => {
    const result = await Effect.runPromise(
      bundleFromFile("latest-main", "/tmp/nonexistent-cache-test-file.json").pipe(Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(GitHubApiError);
      expect(result.left.message).toContain("Failed to read lock file");
    }
  });
});

// ---------------------------------------------------------------------------
// Tests: cachedResolve (Effect-based)
// ---------------------------------------------------------------------------

describe("cachedResolve", () => {
  test("returns bundle from lockFile when provided", async () => {
    const bundle = makeBundle();
    const { dir, file } = await writeTempBundle(bundle);
    const program = cachedResolve({ target: "latest-main", lockFile: file, reset: false });
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.target).toBe("latest-main");
    expect(result.env.GATEWAY_VERSION).toBe("abc1234");
    await fs.rm(dir, { recursive: true });
  });

  test("returns bundle from cache when available and not reset", async () => {
    const bundle = makeBundle();
    const cachePath = resolveCachePath("latest-main");
    // Ensure the cache directory exists and write the cache file
    await fs.mkdir(path.dirname(cachePath), { recursive: true });
    await writeJson(cachePath, bundle);

    const program = cachedResolve({ target: "latest-main", reset: false });
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.target).toBe("latest-main");
    expect(result.env.GATEWAY_VERSION).toBe("abc1234");

    // Clean up
    await fs.rm(cachePath, { force: true });
  });

  test("skips cache when reset is true", async () => {
    // Write a cache file with a distinguishable version
    const cachedBundle = makeBundle({ env: { GATEWAY_VERSION: "cached-version", HOST_VERSION: "cached-version" } });
    const cachePath = resolveCachePath("latest-main");
    await fs.mkdir(path.dirname(cachePath), { recursive: true });
    await writeJson(cachePath, cachedBundle);

    const program = cachedResolve({ target: "latest-main", reset: true });
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    // Should get fresh-resolved version, not "cached-version"
    expect(result.env.GATEWAY_VERSION).not.toBe("cached-version");

    // Clean up
    await fs.rm(cachePath, { force: true });
  });

  test("fetches from GitHub when no cache exists", async () => {
    // Make sure cache file does not exist
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = cachedResolve({ target: "latest-release", reset: false });
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.target).toBe("latest-release");
    expect(result.env.GATEWAY_VERSION).toBe("v0.11.0");

    // Clean up the cache it wrote
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });

  test("skips cache when target does not match cached bundle", async () => {
    // Write a cache file with a different target
    const cachedBundle = makeBundle({ target: "devnet" });
    const cachePath = resolveCachePath("latest-release");
    await fs.mkdir(path.dirname(cachePath), { recursive: true });
    await writeJson(cachePath, cachedBundle);

    const program = cachedResolve({ target: "latest-release", reset: false });
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    // Should get fresh-resolved version, not the cached devnet bundle
    expect(result.target).toBe("latest-release");

    // Clean up
    await fs.rm(cachePath, { force: true });
  });
});

// ---------------------------------------------------------------------------
// Tests: resolveBundle (Effect-based)
// ---------------------------------------------------------------------------

describe("resolveBundle", () => {
  test("returns resolved bundle and lock path", async () => {
    // Ensure no cache interference
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = resolveBundle(
      { target: "latest-release", reset: false },
      {},
    );
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.bundle.target).toBe("latest-release");
    expect(result.lockPath).toContain("latest-release");
    expect(result.lockPath).toEndWith(".json");

    // Clean up
    await fs.rm(result.lockPath, { force: true }).catch(() => {});
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });

  test("applies env overrides to the resolved bundle", async () => {
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = resolveBundle(
      { target: "latest-release", reset: false },
      { GATEWAY_VERSION: "custom-v1" },
    );
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.bundle.env.GATEWAY_VERSION).toBe("custom-v1");
    expect(result.bundle.sources).toContain("env=GATEWAY_VERSION");

    // Clean up
    await fs.rm(result.lockPath, { force: true }).catch(() => {});
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });

  test("writes lock file to disk", async () => {
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = resolveBundle(
      { target: "latest-release", reset: false },
      {},
    );
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));

    // Verify the lock file was written
    const stat = await fs.stat(result.lockPath).catch(() => null);
    expect(stat).not.toBeNull();

    // Clean up
    await fs.rm(result.lockPath, { force: true }).catch(() => {});
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });

  test("restores a missing lock snapshot from persisted bundle state", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "lock-restore-test-"));
    const lockPath = path.join(dir, "restored-lock.json");
    const bundle = makeBundle({ target: "latest-release", lockName: "restored-lock.json" });

    await Effect.runPromise(ensureLockSnapshot(lockPath, bundle));

    expect(await fs.readFile(lockPath, "utf8")).toContain('"lockName": "restored-lock.json"');

    await fs.rm(dir, { recursive: true, force: true }).catch(() => {});
  });
});

// ---------------------------------------------------------------------------
// Tests: previewBundle (Effect-based)
// ---------------------------------------------------------------------------

describe("previewBundle", () => {
  test("returns resolved bundle with env overrides applied", async () => {
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = previewBundle(
      { target: "latest-release", reset: false },
      { GATEWAY_VERSION: "preview-v1" },
    );
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.env.GATEWAY_VERSION).toBe("preview-v1");
    expect(result.sources).toContain("env=GATEWAY_VERSION");

    // Clean up cache
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });

  test("does not write a lock file", async () => {
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = previewBundle(
      { target: "latest-release", reset: true },
      {},
    );
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));

    // Ensure the lock file does not exist — remove any leftover from prior tests first
    const lockPath = path.join(LOCK_DIR, result.lockName);
    await fs.rm(lockPath, { force: true }).catch(() => {});

    // Run again after cleanup to verify previewBundle does NOT create the lock
    const result2 = await Effect.runPromise(
      previewBundle({ target: "latest-release", reset: true }, {}).pipe(
        Effect.provide(TestGitHubClient),
      ),
    );
    const lockPath2 = path.join(LOCK_DIR, result2.lockName);
    const stat = await fs.stat(lockPath2).catch(() => null);
    expect(stat).toBeNull();

    // Clean up cache
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });

  test("returns unmodified bundle when env has no matching overrides", async () => {
    const cachePath = resolveCachePath("latest-release");
    await fs.rm(cachePath, { force: true }).catch(() => {});

    const program = previewBundle(
      { target: "latest-release", reset: false },
      { UNRELATED_KEY: "value" },
    );
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClient)));
    expect(result.env.GATEWAY_VERSION).toBe("v0.11.0");

    // Clean up cache
    await fs.rm(cachePath, { force: true }).catch(() => {});
  });
});
