import { describe, expect, test } from "bun:test";

import {
  SIMPLE_ACL_MIN_SHA,
  SHA_RUNTIME_COMPAT_MIN_SHA,
  applyVersionEnvOverrides,
  missingRepoPackages,
  presetBundle,
  shaRuntimeCompatFloor,
  simpleAclFloor,
} from "./resolve/target";
import { targetUsesCache } from "./resolve/bundle-store";

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

  test("does not reuse the cache for latest-supported", () => {
    expect(targetUsesCache("latest-supported")).toBe(false);
    expect(targetUsesCache("latest-main")).toBe(true);
  });
});
