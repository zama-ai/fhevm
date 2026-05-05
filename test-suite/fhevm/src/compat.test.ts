import { describe, expect, test } from "bun:test";

import {
  LEGACY_RELAYER_IMAGE_REPOSITORY,
  LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  MODERN_RELAYER_IMAGE_REPOSITORY,
  MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  compatPolicyForState,
  requiresGatewayKmsGenerationAddress,
  requiresLegacyGatewayKmsGenerationAddress,
  requiresLegacyKmsCoreConfig,
  requiresLegacyRelayerUrl,
  requiresModernHostAddressArtifacts,
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

  test("adds kms-generation-address for v0.12 gateway listener images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          COPROCESSOR_GW_LISTENER_VERSION: "v0.12.1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorArgs["gw-listener"]).toContainEqual([
      "--kms-generation-address",
      { env: "KMS_GENERATION_ADDRESS" },
    ]);
  });

  test("drops kms-generation-address for old host listener images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.11.0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["host-listener"]).toContain("--kms-generation-address");
    expect(policy.coprocessorDropFlags["host-listener-poller"]).toContain("--kms-generation-address");
  });

  test("drops kms-generation-address for v0.12 host listener images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.12.1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["host-listener"]).toContain("--kms-generation-address");
    expect(policy.coprocessorDropFlags["host-listener-poller"]).toContain("--kms-generation-address");
  });

  test("treats sha-style gateway bundles as modern kms-generation sourcing", () => {
    expect(
      requiresLegacyGatewayKmsGenerationAddress({
        versions: {
          target: "latest-main",
          lockName: "latest-main.json",
          env: { GATEWAY_VERSION: "abcdef0" } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(false);
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

  test("does not require ProtocolConfig or KMSGeneration host addresses on pre-v0.13 bundles", () => {
    const state = {
      versions: {
        target: "latest-supported" as const,
        lockName: "latest-supported.json",
        env: { GATEWAY_VERSION: "v0.11.0", HOST_VERSION: "v0.11.0" } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    };
    expect(requiresModernHostAddressArtifacts(state)).toBe(false);
    expect(requiresGatewayKmsGenerationAddress(state)).toBe(true);
  });

  test("requires gateway KMSGeneration on v0.12 gateway bundles", () => {
    const state = {
      versions: {
        target: "latest-supported" as const,
        lockName: "v0.12.0.json",
        env: { GATEWAY_VERSION: "v0.12.0", HOST_VERSION: "v0.12.0" } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    };
    expect(requiresModernHostAddressArtifacts(state)).toBe(false);
    expect(requiresGatewayKmsGenerationAddress(state)).toBe(true);
  });

  test("requires ProtocolConfig and KMSGeneration host addresses on v0.13+ bundles", () => {
    const state = {
      versions: {
        target: "sha" as const,
        lockName: "sha.json",
        env: { HOST_VERSION: "13a37bc" } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    };
    expect(requiresModernHostAddressArtifacts(state)).toBe(true);
    expect(requiresGatewayKmsGenerationAddress(state)).toBe(false);
  });

  test("requires modern host addresses when host-contracts is locally overridden", () => {
    const state = {
      versions: {
        target: "latest-supported" as const,
        lockName: "latest-supported.json",
        env: { HOST_VERSION: "v0.11.0" } as Record<string, string>,
        sources: [],
      },
      overrides: [{ group: "host-contracts" as const }],
      scenario: testDefaultScenario(),
    };
    expect(requiresModernHostAddressArtifacts(state)).toBe(true);
    expect(requiresGatewayKmsGenerationAddress(state)).toBe(false);
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
