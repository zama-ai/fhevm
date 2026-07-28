import path from "node:path";
import { writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";

import { validateBundleCompatibility } from "./compat/compat";
import {
  explainGitHubCliError,
  isRateLimitGitHubCliError,
  rateLimitRetryDelayMs,
  retryDelayMs,
  shouldRetryGitHubCliError,
  shouldStopPackageTagScan,
} from "./resolve/github";
import {
  MAX_FALLBACK_COMMIT_DEPTH,
  SIMPLE_ACL_MIN_SHA,
  SHA_RUNTIME_COMPAT_MIN_SHA,
  applyVersionEnvOverrides,
  findPublishedAncestorIndex,
  presetBundle,
  resolveMissingRepoTagFallbacks,
  shaRuntimeCompatFloor,
  simpleAclFloor,
} from "./resolve/target";
import {
  previewBundle,
  resolveBundle,
  targetUsesCache,
} from "./resolve/bundle-store";
import { withTempStateDir } from "./test-state";

describe("resolve", () => {
  test("locates the ACL and runtime compat floors", () => {
    const commits = ["head", SHA_RUNTIME_COMPAT_MIN_SHA, SIMPLE_ACL_MIN_SHA, "tail"];
    expect(simpleAclFloor(commits)).toBe(2);
    expect(shaRuntimeCompatFloor(commits)).toBe(1);
  });

  test("applies env overrides to a bundle", () => {
    const bundle = presetBundle("latest-main", "abcdef0", "latest-main-abcdef0.json");
    const next = applyVersionEnvOverrides(bundle, {
      HOST_VERSION: "override-host",
      UNUSED: "ignored",
    });
    expect(next.env.HOST_VERSION).toBe("override-host");
    expect(next.sources.at(-1)).toBe("env=HOST_VERSION");
  });

  test("resolves relayer images as repo-owned for latest-main presets", () => {
    const bundle = presetBundle("latest-main", "abcdef0", "latest-main-abcdef0.json");
    expect(bundle.env.LISTENER_CORE_VERSION).toBe("abcdef0");
    expect(bundle.env.RELAYER_VERSION).toBe("abcdef0");
    expect(bundle.env.RELAYER_MIGRATE_VERSION).toBe("abcdef0");
    expect(bundle.env.CORE_VERSION).not.toBe("abcdef0");
  });

  test("finds the nearest ancestor with a published image tag", () => {
    const commits = [
      "d77a0417aa5d928063181454756ebb73cdbadc24",
      "2cde6c960c24405015ab03959a2d9053cde31f23",
      "8e2f724d4000000000000000000000000000000",
    ];
    expect(findPublishedAncestorIndex(commits, new Set(["2cde6c9", "8e2f724"]))).toBe(1);
    expect(findPublishedAncestorIndex(commits, new Set(["fffffff"]))).toBe(-1);
    expect(findPublishedAncestorIndex([], new Set(["2cde6c9"]))).toBe(-1);
  });

  test("falls back to the newest published ancestor tag for missing images", () => {
    const { overrides, sources } = resolveMissingRepoTagFallbacks({
      requestedTag: "d77a041",
      missingKeys: ["CONNECTOR_GW_LISTENER_VERSION"],
      commitShas: ["d77a0417aa5d928063181454756ebb73cdbadc24", "2cde6c960c24405015ab03959a2d9053cde31f23"],
      packageTagsMap: { CONNECTOR_GW_LISTENER_VERSION: new Set(["2cde6c9"]) },
    });
    expect(overrides).toEqual({ CONNECTOR_GW_LISTENER_VERSION: "2cde6c9" });
    expect(sources).toEqual(["CONNECTOR_GW_LISTENER_VERSION=2cde6c9 (fallback: d77a041 unpublished)"]);
  });

  test("fails resolution when the newest published image lags beyond the fallback limit", () => {
    const commits = Array.from(
      { length: MAX_FALLBACK_COMMIT_DEPTH + 2 },
      (_, index) => index.toString(16).padEnd(40, "f"),
    );
    expect(() =>
      resolveMissingRepoTagFallbacks({
        requestedTag: commits[0].slice(0, 7),
        missingKeys: ["CONNECTOR_GW_LISTENER_VERSION"],
        commitShas: commits,
        packageTagsMap: { CONNECTOR_GW_LISTENER_VERSION: new Set([commits.at(-1)!.slice(0, 7)]) },
      }),
    ).toThrow("beyond the 50-commit fallback limit");
  });

  test("keeps the unverified pin when the package has no visible published tags", () => {
    const { overrides, sources } = resolveMissingRepoTagFallbacks({
      requestedTag: "d77a041",
      missingKeys: ["CONNECTOR_GW_LISTENER_VERSION"],
      commitShas: ["d77a0417aa5d928063181454756ebb73cdbadc24"],
      packageTagsMap: { CONNECTOR_GW_LISTENER_VERSION: new Set<string>() },
    });
    expect(overrides).toEqual({});
    expect(sources).toEqual([
      "CONNECTOR_GW_LISTENER_VERSION=d77a041 (unverified: package has no visible published tags)",
    ]);
  });

  test("fails resolution when a published package has no tag anywhere on the ancestry", () => {
    expect(() =>
      resolveMissingRepoTagFallbacks({
        requestedTag: "d77a041",
        missingKeys: ["CONNECTOR_GW_LISTENER_VERSION"],
        commitShas: ["d77a0417aa5d928063181454756ebb73cdbadc24"],
        packageTagsMap: { CONNECTOR_GW_LISTENER_VERSION: new Set(["0000000"]) },
      }),
    ).toThrow("nor for any of the 1 commits behind it");
  });

  test("applies per-component fallback overrides to a sha preset bundle", () => {
    const bundle = presetBundle("sha", "d77a041", "sha-d77a041.json", [], {
      CONNECTOR_GW_LISTENER_VERSION: "2cde6c9",
    });
    expect(bundle.env.CONNECTOR_GW_LISTENER_VERSION).toBe("2cde6c9");
    expect(bundle.env.CONNECTOR_TX_SENDER_VERSION).toBe("d77a041");
    expect(bundle.env.COPROCESSOR_HOST_LISTENER_VERSION).toBe("d77a041");
  });

  test("only caches immutable sha targets", () => {
    expect(targetUsesCache("latest-supported")).toBe(false);
    expect(targetUsesCache("latest-main")).toBe(false);
    expect(targetUsesCache("devnet")).toBe(false);
    expect(targetUsesCache("testnet")).toBe(false);
    expect(targetUsesCache("mainnet")).toBe(false);
    expect(targetUsesCache("sha")).toBe(true);
  });

  test("rejects lock names that escape the lock directory", async () => {
    await withTempStateDir(async (stateDir) => {
      const lockFile = path.join(stateDir, "malicious-lock.json");
      await writeFile(
        lockFile,
        JSON.stringify({
          target: "latest-main",
          lockName: "../../escape.json",
          sources: ["test"],
          env: presetBundle("latest-main", "abcdef0", "latest-main.json").env,
        }),
      );
      await expect(
        resolveBundle(
          {
            target: "latest-main",
            requestedTarget: undefined,
            sha: undefined,
            lockFile,
            reset: false,
          },
          process.env,
        ),
      ).rejects.toThrow("invalid lockName");
    });
  });

  test("rejects incompatible env-overridden bundles during preview", () => {
    const issues = validateBundleCompatibility({
      versions: applyVersionEnvOverrides(presetBundle("latest-main", "abcdef0", "latest-main.json"), {
        RELAYER_VERSION: "v0.9.0",
        TEST_SUITE_VERSION: "v0.11.0",
      }),
    });
    expect(issues.map((issue) => issue.message).join("\n")).toContain("Relayer only serves /v1 API");
  });

  test("accepts explicit sha-like repo overrides without main-floor validation", () => {
    expect(
      applyVersionEnvOverrides(presetBundle("latest-main", "abcdef0", "latest-main.json"), {
        GATEWAY_VERSION: "deadbee",
      }).env.GATEWAY_VERSION,
    ).toBe("deadbee");

    const sha = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
    expect(
      applyVersionEnvOverrides(presetBundle("latest-main", "abcdef0", "latest-main.json"), {
        GATEWAY_VERSION: sha,
      }).env.GATEWAY_VERSION,
    ).toBe(sha);
  });

  test("retries transient GitHub 5xx responses", () => {
    expect(shouldRetryGitHubCliError("gh: HTTP 503")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 502 Bad Gateway")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 504 Gateway Timeout")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 429 Too Many Requests")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: API rate limit exceeded for user ID 12345 (HTTP 403)")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: You have exceeded a secondary rate limit. Please wait a few minutes before you try again. (HTTP 403)")).toBe(true);
    expect(
      shouldRetryGitHubCliError(
        "gh: No server is currently available to service your request. Sorry about that. Please try resubmitting your request and contact us if the problem persists. (HTTP 503)",
      ),
    ).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 404")).toBe(false);
  });

  test("classifies GitHub rate-limit responses separately", () => {
    expect(isRateLimitGitHubCliError("gh: HTTP 429 Too Many Requests")).toBe(true);
    expect(isRateLimitGitHubCliError("gh: API rate limit exceeded for user ID 12345 (HTTP 403)")).toBe(true);
    expect(isRateLimitGitHubCliError("gh: You have exceeded a secondary rate limit. Please wait a few minutes before you try again. (HTTP 403)")).toBe(true);
    expect(isRateLimitGitHubCliError("gh: HTTP 503")).toBe(false);
  });

  test("uses a longer retry delay budget for rate limits", () => {
    const random = Math.random;
    Math.random = () => 0;
    try {
      expect(retryDelayMs(1, false)).toBe(1_000);
      expect(retryDelayMs(1, true)).toBe(60_000);
      expect(retryDelayMs(3, true)).toBe(240_000);
    } finally {
      Math.random = random;
    }
  });

  test("prefers Retry-After for rate-limit cooldowns when available", () => {
    expect(rateLimitRetryDelayMs({ "retry-after": "90" }, 1, 0)).toBe(90_000);
  });

  test("falls back to X-RateLimit-Reset for rate-limit cooldowns", () => {
    expect(rateLimitRetryDelayMs({ "x-ratelimit-remaining": "0", "x-ratelimit-reset": "180" }, 1, 30_000)).toBe(150_000);
  });

  test("does not use X-RateLimit-Reset when the primary bucket still has room", () => {
    const random = Math.random;
    Math.random = () => 0;
    try {
      expect(rateLimitRetryDelayMs({ "x-ratelimit-remaining": "42", "x-ratelimit-reset": "180" }, 1, 30_000)).toBe(60_000);
    } finally {
      Math.random = random;
    }
  });

  test("rewrites missing package scope errors into actionable guidance", () => {
    expect(
      explainGitHubCliError(
        "gh: You need at least read:packages scope to get a package's versions. (HTTP 403)",
      ),
    ).toContain("gh auth refresh -s read:packages");
  });

  test("stops package tag scans when the requested tag is found", () => {
    expect(
      shouldStopPackageTagScan(
        new Set(["target22"]),
        new Array(100).fill({ metadata: { container: { tags: ["target22"] } } }),
        "target22",
      ),
    ).toBe(true);
    expect(
      shouldStopPackageTagScan(
        new Set(["other111"]),
        new Array(100).fill({ metadata: { container: { tags: ["other111"] } } }),
        "target22",
      ),
    ).toBe(false);
    expect(
      shouldStopPackageTagScan(
        new Set(["other111"]),
        [{ metadata: { container: { tags: ["other111"] } } }],
        "target22",
      ),
    ).toBe(true);
  });

});
