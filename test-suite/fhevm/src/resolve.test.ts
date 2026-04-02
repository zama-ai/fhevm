import path from "node:path";
import { writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";

import { validateBundleCompatibility } from "./compat/compat";
import { shouldRetryGitHubCliError } from "./resolve/github";
import {
  SIMPLE_ACL_MIN_SHA,
  SHA_RUNTIME_COMPAT_MIN_SHA,
  applyVersionEnvOverrides,
  assertSupportedShaBundle,
  missingRepoPackages,
  presetBundle,
  shaRuntimeCompatFloor,
  simpleAclFloor,
} from "./resolve/target";
import {
  assertSupportedRepoOverrideFloors,
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
    expect(bundle.env.RELAYER_VERSION).toBe("abcdef0");
    expect(bundle.env.RELAYER_MIGRATE_VERSION).toBe("abcdef0");
    expect(bundle.env.CORE_VERSION).not.toBe("abcdef0");
  });

  test("reports missing repo packages for a tag", () => {
    const missing = missingRepoPackages(
      {
        GATEWAY_VERSION: new Set(["1234567"]),
      } as Record<string, Set<string>>,
      "1234567",
    );
    expect(missing).toContain("fhevm/host-contracts");
  });

  test("rejects locked sha bundles that predate the compat floor", () => {
    const commits = [
      "head0000000000000000000000000000000000000",
      SHA_RUNTIME_COMPAT_MIN_SHA,
      SIMPLE_ACL_MIN_SHA,
      "0000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ];
    expect(() => assertSupportedShaBundle(presetBundle("sha", "0000000", "sha-0000000.json"), commits)).toThrow(
      "unsupported",
    );
  });

  test("accepts locked sha bundles at the compat floor", () => {
    const commits = [
      "head0000000000000000000000000000000000000",
      SHA_RUNTIME_COMPAT_MIN_SHA,
      SIMPLE_ACL_MIN_SHA,
    ];
    expect(() =>
      assertSupportedShaBundle(
        presetBundle("sha", SHA_RUNTIME_COMPAT_MIN_SHA.slice(0, 7), "sha-floor.json"),
        commits,
      ),
    ).not.toThrow();
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

  test("rejects unsupported sha-like repo overrides", () => {
    expect(() =>
      assertSupportedRepoOverrideFloors(
        applyVersionEnvOverrides(presetBundle("latest-main", "abcdef0", "latest-main.json"), {
          GATEWAY_VERSION: "deadbee",
        }),
        {
          GATEWAY_VERSION: "deadbee",
        },
        [
          "abcdef0aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          SHA_RUNTIME_COMPAT_MIN_SHA,
          SIMPLE_ACL_MIN_SHA,
        ],
      ),
    ).toThrow("sha target deadbee is unsupported");
  });

  test("retries transient GitHub 5xx responses", () => {
    expect(shouldRetryGitHubCliError("gh: HTTP 503")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 502 Bad Gateway")).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 504 Gateway Timeout")).toBe(true);
    expect(
      shouldRetryGitHubCliError(
        "gh: No server is currently available to service your request. Sorry about that. Please try resubmitting your request and contact us if the problem persists. (HTTP 503)",
      ),
    ).toBe(true);
    expect(shouldRetryGitHubCliError("gh: HTTP 404")).toBe(false);
  });

});
