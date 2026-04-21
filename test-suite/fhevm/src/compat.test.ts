import { describe, expect, test } from "bun:test";

import {
  LEGACY_RELAYER_IMAGE_REPOSITORY,
  LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  MODERN_RELAYER_IMAGE_REPOSITORY,
  MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  compatPolicyForState,
  requiresLegacyKmsCoreConfig,
  requiresLegacyRelayerUrl,
  validateBundleCompatibility,
} from "./compat/compat";
import { testDefaultScenario } from "./test-fixtures";

describe("compat", () => {
  test("flags relayer v1 vs test-suite v2 incompatibility", () => {
    const issues = validateBundleCompatibility({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          RELAYER_VERSION: "v0.9.0",
          TEST_SUITE_VERSION: "v0.11.0",
        } as Record<string, string>,
        sources: [],
      },
    });
    expect(issues).toHaveLength(1);
  });

  test("treats prerelease relayer versions as older than the final release", () => {
    const issues = validateBundleCompatibility({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          RELAYER_VERSION: "v0.10.0-rc.1",
          TEST_SUITE_VERSION: "v0.11.0",
        } as Record<string, string>,
        sources: [],
      },
    });
    expect(issues).toHaveLength(1);
  });

  test("accepts latest-supported relayer prerelease paired with test-suite v0.11.0", () => {
    const issues = validateBundleCompatibility({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          RELAYER_VERSION: "v0.11.0-rc.1",
          TEST_SUITE_VERSION: "v0.11.0",
        } as Record<string, string>,
        sources: [],
      },
    });
    expect(issues).toHaveLength(0);
  });

  test("builds legacy shim policy for old connector listener", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          CONNECTOR_GW_LISTENER_VERSION: "v0.10.0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.connectorEnv.KMS_CONNECTOR_CHAIN_ID).toBe("KMS_CONNECTOR_GATEWAY_CHAIN_ID");
  });

  test("detects legacy relayer URL behavior", () => {
    expect(
      requiresLegacyRelayerUrl({
        versions: {
          target: "latest-supported",
          lockName: "latest-supported.json",
          env: { TEST_SUITE_VERSION: "v0.10.9" } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(true);
  });

  test("treats kms-core v0.13.10 prereleases as modern config schema", () => {
    expect(
      requiresLegacyKmsCoreConfig({
        versions: {
          target: "sha",
          lockName: "sha.json",
          env: { CORE_VERSION: "v0.13.10-rc.3" } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(false);
  });

  test("renders legacy pauser flags for old contract tags", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          HOST_VERSION: "v0.11.0",
          GATEWAY_VERSION: "v0.11.0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.HOST_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-pauser-set-address");
    expect(policy.composeEnv.GATEWAY_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-pauser-set-address");
  });

  test("keeps legacy pauser flags for v0.12 contract tags", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "v0.12.0.json",
        env: {
          HOST_VERSION: "v0.12.0",
          GATEWAY_VERSION: "v0.12.0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.HOST_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-pauser-set-address");
    expect(policy.composeEnv.GATEWAY_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-pauser-set-address");
  });

  test("renders modern pauser flags for unparsed mainline versions", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-main",
        lockName: "latest-main.json",
        env: {
          HOST_VERSION: "c5bb50b",
          GATEWAY_VERSION: "c5bb50b",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.HOST_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-proxy-address");
    expect(policy.composeEnv.GATEWAY_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-proxy-address");
  });

  test("routes semver relayer images to the legacy console registry", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          RELAYER_VERSION: "v0.11.0-rc.2",
          RELAYER_MIGRATE_VERSION: "v0.11.0-rc.1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.RELAYER_IMAGE_REPOSITORY).toBe(LEGACY_RELAYER_IMAGE_REPOSITORY);
    expect(policy.composeEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY).toBe(LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY);
  });

  test("routes sha-style relayer images to the fhevm registry", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-main",
        lockName: "latest-main.json",
        env: {
          RELAYER_VERSION: "b799892",
          RELAYER_MIGRATE_VERSION: "65cf86e",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.RELAYER_IMAGE_REPOSITORY).toBe(MODERN_RELAYER_IMAGE_REPOSITORY);
    expect(policy.composeEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY).toBe(MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY);
  });
});
