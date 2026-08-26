import { describe, expect, test } from "bun:test";

import {
  LEGACY_RELAYER_IMAGE_REPOSITORY,
  LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  MODERN_RELAYER_IMAGE_REPOSITORY,
  MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  assertSupportedBundleScenario,
  bootstrapUsesHostKmsGeneration,
  canonicalProtocolConfigSeedingUsesEnv,
  compatArgPolicyForPinnedTag,
  compatPolicyForState,
  coprocessorUsesHostKmsGeneration,
  kmsConnectorUsesHostKmsGeneration,
  requiresGatewayKmsGenerationAddress,
  requiresLegacyGatewayKmsGenerationAddress,
  requiresLegacyHostChainSeedShim,
  requiresLegacyKmsBootstrapBudget,
  requiresLegacyKmsCoreConfig,
  requiresLegacyRelayerUrl,
  requiresModernHostAddressArtifacts,
  supportsCanonicalProtocolConfigSeeding,
  supportsConsensusDetector,
  supportsHostListenerConsumer,
  supportsUpgradeController,
  validateBundleCompatibility,
} from "./compat/compat";
import { testDefaultScenario } from "./test-fixtures";
import type { LocalOverride } from "./types";

describe("compat", () => {
  test("requires a full local coprocessor build for multi-node consensus topologies", () => {
    const scenario = testDefaultScenario({ topology: { count: 3, threshold: 3 } });
    const versions = {
      target: "latest-main" as const,
      lockName: "latest-main.json",
      env: {} as Record<string, string>,
      sources: [],
    };
    expect(() => assertSupportedBundleScenario({ versions, overrides: [], scenario })).toThrow(
      "require a full local coprocessor build",
    );
    expect(() =>
      assertSupportedBundleScenario({ versions, overrides: [{ group: "coprocessor" }], scenario }),
    ).not.toThrow();
    expect(() =>
      assertSupportedBundleScenario({
        versions,
        overrides: [{ group: "coprocessor", services: ["coprocessor-host-listener"] }],
        scenario,
      }),
    ).toThrow("require a full local coprocessor build");
  });

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

  test("drops signer flags for legacy sns-worker images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          COPROCESSOR_SNS_WORKER_VERSION: "v0.11.0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["sns-worker"]).toContain("--signer-type");
    expect(policy.coprocessorDropFlags["sns-worker"]).toContain("--private-key");
  });

  test("keeps signer flags for non-semver sns-worker images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-main",
        lockName: "latest-main.json",
        env: {
          COPROCESSOR_SNS_WORKER_VERSION: "80f2357",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["sns-worker"] ?? []).not.toContain("--signer-type");
    expect(policy.coprocessorDropFlags["sns-worker"] ?? []).not.toContain("--private-key");
  });

  test("splits the unified bucket flag for legacy sns-worker images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          COPROCESSOR_SNS_WORKER_VERSION: "v0.14.0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["sns-worker"]).toContain("--bucket-name");
    expect(policy.coprocessorArgs["sns-worker"]).toContainEqual(["--bucket-name-ct128", { env: "BUCKET_NAME" }]);
    expect(policy.coprocessorArgs["sns-worker"]).toContainEqual(["--bucket-name-ct64", { env: "BUCKET_NAME" }]);
  });

  test("keeps the unified bucket flag for sns-worker images from v0.15.0 onward", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-main",
        lockName: "latest-main.json",
        env: {
          COPROCESSOR_SNS_WORKER_VERSION: "v0.15.0-0",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["sns-worker"] ?? []).not.toContain("--bucket-name");
    expect(policy.coprocessorArgs["sns-worker"] ?? []).toHaveLength(0);
  });

  test("shims a registry-pinned fleet from its tag, ignoring the resolved bundle", () => {
    // Blue-green's BCS fleet pins the previous release while the bundle points at
    // HEAD, so the tag is the only signal for which flag contract it speaks.
    const policy = compatArgPolicyForPinnedTag("v0.14.0-7");
    expect(policy.coprocessorDropFlags["sns-worker"]).toContain("--bucket-name");
    expect(policy.coprocessorArgs["sns-worker"]).toContainEqual(["--bucket-name-ct128", { env: "BUCKET_NAME" }]);
    expect(policy.coprocessorArgs["sns-worker"]).toContainEqual(["--bucket-name-ct64", { env: "BUCKET_NAME" }]);
    // v0.14 already carries the signer flags, so the 0.14.0 shim must not fire.
    expect(policy.coprocessorDropFlags["sns-worker"]).not.toContain("--signer-type");
  });

  test("leaves a registry-pinned fleet unshimmed once it reaches the current contract", () => {
    const policy = compatArgPolicyForPinnedTag("v0.15.0");
    expect(policy.coprocessorArgs).toEqual({});
    expect(policy.coprocessorDropFlags).toEqual({});
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

  test("keeps kms-generation-address for v0.13 prerelease host listener images", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        env: {
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.13.0-1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.coprocessorDropFlags["host-listener"] ?? []).not.toContain("--kms-generation-address");
    expect(policy.coprocessorDropFlags["host-listener-poller"] ?? []).not.toContain("--kms-generation-address");
  });

  test("requires legacy host chain seed shim for v0.12 coprocessor images", () => {
    expect(
      requiresLegacyHostChainSeedShim({
        versions: {
          target: "latest-supported",
          lockName: "v0.12.5.json",
          env: {
            COPROCESSOR_DB_MIGRATION_VERSION: "v0.12.5",
            COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.12.5",
          } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(true);
  });

  test("does not require legacy host chain seed shim for v0.11 coprocessor images", () => {
    // v0.11 images predate the remove_tenants migration, so host_chains does
    // not exist and the harness must not try to seed it.
    expect(
      requiresLegacyHostChainSeedShim({
        versions: {
          target: "latest-supported",
          lockName: "v0.11.json",
          env: {
            COPROCESSOR_DB_MIGRATION_VERSION: "v0.11.0",
            COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.11.0",
          } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(false);
  });

  test("requires legacy host chain seed shim for v0.13.0 coprocessor images", () => {
    // initialize_db.sh gained declarative seeding after v0.13.0 was cut;
    // all v0.13.0-N Docker builds still need the harness shim.
    expect(
      requiresLegacyHostChainSeedShim({
        versions: {
          target: "latest-supported",
          lockName: "v0.13.0-6.json",
          env: {
            COPROCESSOR_DB_MIGRATION_VERSION: "v0.13.0-6",
            COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.13.0-6",
          } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(true);
  });

  test("does not require legacy host chain seed shim for v0.13.1+ coprocessor images", () => {
    expect(
      requiresLegacyHostChainSeedShim({
        versions: {
          target: "latest-supported",
          lockName: "v0.13.1.json",
          env: {
            COPROCESSOR_DB_MIGRATION_VERSION: "v0.13.1",
            COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.13.1",
          } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(false);
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

  test("keeps host-listener consumer disabled for legacy host-listener bundles", () => {
    expect(
      supportsHostListenerConsumer({
        versions: {
          target: "latest-supported",
          lockName: "latest-supported.json",
          env: { COPROCESSOR_HOST_LISTENER_VERSION: "v0.11.0" } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(false);
  });

  test("gates consensus-detector and upgrade-controller on published image families", () => {
    const stateFor = (env: Record<string, string>) => ({
      versions: {
        target: "latest-main" as const,
        lockName: "latest-main.json",
        env,
        sources: [],
      },
    });
    // v0.13.x tags are never published for these images (keys are pinned to
    // host-listener's tag), so the whole family must stay unsupported.
    expect(supportsConsensusDetector(stateFor({ COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "v0.13.0" }))).toBe(false);
    expect(supportsUpgradeController(stateFor({ COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "v0.13.0" }))).toBe(false);
    expect(supportsConsensusDetector(stateFor({ COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "v0.11.0" }))).toBe(false);
    expect(supportsConsensusDetector(stateFor({}))).toBe(false);
    expect(supportsUpgradeController(stateFor({}))).toBe(false);
    expect(supportsConsensusDetector(stateFor({ COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "v0.14.0-rc.1" }))).toBe(true);
    expect(supportsConsensusDetector(stateFor({ COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "v0.14.0" }))).toBe(true);
    expect(supportsUpgradeController(stateFor({ COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "v0.14.0" }))).toBe(true);
    // Unparsed main sha tags are published by CI and count as modern.
    expect(supportsConsensusDetector(stateFor({ COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "02f6cc0" }))).toBe(true);
    expect(supportsUpgradeController(stateFor({ COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "02f6cc0" }))).toBe(true);
  });

  test("enables host-listener consumer for v0.13 prereleases and newer bundles", () => {
    expect(
      supportsHostListenerConsumer({
        versions: {
          target: "latest-main",
          lockName: "latest-main.json",
          env: { COPROCESSOR_HOST_LISTENER_VERSION: "v0.13.0-rc.1" } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(true);
    expect(
      supportsHostListenerConsumer({
        versions: {
          target: "latest-main",
          lockName: "latest-main.json",
          env: { COPROCESSOR_HOST_LISTENER_VERSION: "02f6cc0" } as Record<string, string>,
          sources: [],
        },
      }),
    ).toBe(true);
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

  test("keeps legacy pauser flags before host contracts v0.12.1", () => {
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

  // task:addHostPausers took useInternalProxyAddress from v0.12.1; the gateway task
  // kept useInternalPauserSetAddress until v0.13.0, so the two flags diverge here.
  test("uses the proxy pauser flag from host contracts v0.12.1", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-supported",
        lockName: "v0.12.1.json",
        env: {
          HOST_VERSION: "v0.12.1",
          GATEWAY_VERSION: "v0.12.1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.HOST_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-proxy-address");
    expect(policy.composeEnv.GATEWAY_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-pauser-set-address");
  });

  test("grants the larger bootstrap budget only to pre-v0.13.20 KMS cores", () => {
    expect(requiresLegacyKmsBootstrapBudget("v0.13.10")).toBe(true);
    expect(requiresLegacyKmsBootstrapBudget("v0.13.20")).toBe(false);
    expect(requiresLegacyKmsBootstrapBudget("v0.13.20-0")).toBe(false);
    // The checked-in node-by-node runbook must stay on the standard budget.
    expect(requiresLegacyKmsBootstrapBudget("v0.13.21")).toBe(false);
    // Unparsed (sha-tagged) cores follow the modern budget, as elsewhere in compat.
    expect(requiresLegacyKmsBootstrapBudget("abc1234")).toBe(false);
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

  test("treats v0.13 prerelease contract bundles as modern host address artifacts", () => {
    const state = {
      versions: {
        target: "mainnet" as const,
        lockName: "v0.13.0.json",
        env: {
          GATEWAY_VERSION: "v0.13.0-1",
          HOST_VERSION: "v0.13.0-1",
          COPROCESSOR_GW_LISTENER_VERSION: "v0.13.0-1",
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.13.0-1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    };

    expect(requiresLegacyGatewayKmsGenerationAddress(state)).toBe(false);
    expect(requiresModernHostAddressArtifacts(state)).toBe(true);
    expect(requiresGatewayKmsGenerationAddress(state)).toBe(false);

    const policy = compatPolicyForState(state);
    expect(policy.coprocessorArgs["gw-listener"]).toBeUndefined();
    expect(policy.coprocessorDropFlags["host-listener"]?.sort()).toEqual([
      "--confidential-bridge-address",
      "--protocol-config-address",
    ]);
    expect(policy.coprocessorDropFlags["host-listener-poller"]?.sort()).toEqual([
      "--confidential-bridge-address",
      "--protocol-config-address",
      "--seed-start-block",
    ]);
    expect(policy.composeEnv.HOST_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-proxy-address");
    expect(policy.composeEnv.GATEWAY_ADD_PAUSERS_INTERNAL_FLAG).toBe("--use-internal-proxy-address");
  });

  test("drops --seed-start-block only for host-listener bundles that predate the 0.13.2 backport", () => {
    const stateFor = (version: string) => ({
      versions: {
        target: "mainnet" as const,
        lockName: "compat.json",
        env: { COPROCESSOR_HOST_LISTENER_VERSION: version } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    const dropsSeedStartBlock = (version: string) =>
      (compatPolicyForState(stateFor(version)).coprocessorDropFlags["host-listener-poller"] ?? []).includes(
        "--seed-start-block",
      );

    expect(dropsSeedStartBlock("v0.11.0")).toBe(true);
    expect(dropsSeedStartBlock("v0.13.0-2")).toBe(true);
    expect(dropsSeedStartBlock("v0.13.1")).toBe(true);
    expect(dropsSeedStartBlock("v0.13.2-0")).toBe(false);
    expect(dropsSeedStartBlock("v0.13.3")).toBe(false);
    expect(dropsSeedStartBlock("v0.14.0-4")).toBe(false);
    expect(dropsSeedStartBlock("13a37bc")).toBe(false);
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

  test("routes KMSGeneration consumption by consumer version during RFC013 rollout", () => {
    const base = {
      versions: {
        target: "sha" as const,
        lockName: "sha.json",
        env: {
          HOST_VERSION: "13a37bc",
          CONNECTOR_GW_LISTENER_VERSION: "v0.12.2",
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.12.2",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    };
    expect(kmsConnectorUsesHostKmsGeneration(base)).toBe(false);
    expect(coprocessorUsesHostKmsGeneration(base)).toBe(false);
    expect(bootstrapUsesHostKmsGeneration(base)).toBe(false);

    const kmsUpgraded = {
      ...base,
      versions: {
        ...base.versions,
        env: {
          ...base.versions.env,
          CONNECTOR_GW_LISTENER_VERSION: "13a37bc",
        },
      },
    };
    expect(kmsConnectorUsesHostKmsGeneration(kmsUpgraded)).toBe(true);
    expect(coprocessorUsesHostKmsGeneration(kmsUpgraded)).toBe(false);
    expect(bootstrapUsesHostKmsGeneration(kmsUpgraded)).toBe(true);

    const coprocessorUpgraded = {
      ...kmsUpgraded,
      versions: {
        ...kmsUpgraded.versions,
        env: {
          ...kmsUpgraded.versions.env,
          COPROCESSOR_HOST_LISTENER_VERSION: "13a37bc",
        },
      },
    };
    expect(kmsConnectorUsesHostKmsGeneration(coprocessorUpgraded)).toBe(true);
    expect(coprocessorUsesHostKmsGeneration(coprocessorUpgraded)).toBe(true);
  });

  test("routes v0.13 prerelease consumers to host KMSGeneration", () => {
    const state = {
      versions: {
        target: "sha" as const,
        lockName: "sha.json",
        env: {
          HOST_VERSION: "v0.13.0-1",
          CONNECTOR_GW_LISTENER_VERSION: "v0.13.0-1",
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.13.0-1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    };
    expect(kmsConnectorUsesHostKmsGeneration(state)).toBe(true);
    expect(coprocessorUsesHostKmsGeneration(state)).toBe(true);
    expect(bootstrapUsesHostKmsGeneration(state)).toBe(true);
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

  test("routes v0.13 relayer images to the fhevm registry", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "mainnet",
        lockName: "v0.13.0.json",
        env: {
          RELAYER_VERSION: "v0.13.0-2",
          RELAYER_MIGRATE_VERSION: "v0.13.0-2",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.RELAYER_IMAGE_REPOSITORY).toBe(MODERN_RELAYER_IMAGE_REPOSITORY);
    expect(policy.composeEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY).toBe(MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY);
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

  test("routes v0.13 prerelease relayer images to the fhevm registry", () => {
    const policy = compatPolicyForState({
      versions: {
        target: "latest-main",
        lockName: "v0.13.0-1.json",
        env: {
          RELAYER_VERSION: "v0.13.0-1",
          RELAYER_MIGRATE_VERSION: "v0.13.0-1",
        } as Record<string, string>,
        sources: [],
      },
      overrides: [],
      scenario: testDefaultScenario(),
    });
    expect(policy.composeEnv.RELAYER_IMAGE_REPOSITORY).toBe(MODERN_RELAYER_IMAGE_REPOSITORY);
    expect(policy.composeEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY).toBe(MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY);
  });

  test("keeps canonical ProtocolConfig seeding on flags until the CANONICAL_* env build", () => {
    const stateFor = (hostVersion: string, overrides: LocalOverride[] = []) => ({
      versions: {
        target: "latest-main" as const,
        lockName: "latest-main.json",
        env: { HOST_VERSION: hostVersion } as Record<string, string>,
        sources: [],
      },
      overrides,
      scenario: testDefaultScenario(),
    });

    // Predates the seeding task entirely, so the harness keeps "fresh" seeding.
    expect(supportsCanonicalProtocolConfigSeeding(stateFor("v0.13.0"))).toBe(false);

    // Ships the task, but it takes its input as command-line flags.
    for (const version of ["v0.13.1", "v0.13.3", "v0.14.0-0", "v0.14.0-8"]) {
      expect(supportsCanonicalProtocolConfigSeeding(stateFor(version))).toBe(true);
      expect(canonicalProtocolConfigSeedingUsesEnv(stateFor(version))).toBe(false);
    }

    // A non-numeric suffix is a real prerelease, so it stays below the floor.
    expect(canonicalProtocolConfigSeedingUsesEnv(stateFor("v0.14.0-rc1"))).toBe(false);

    // The final tag and newer families read the CANONICAL_* env variables.
    for (const version of ["v0.14.0", "v0.14.1", "v0.15.0"]) {
      expect(canonicalProtocolConfigSeedingUsesEnv(stateFor(version))).toBe(true);
    }

    // Builds at or above the build floor read the CANONICAL_* env variables.
    for (const version of ["v0.14.0-9", "v0.14.0-10"]) {
      expect(canonicalProtocolConfigSeedingUsesEnv(stateFor(version))).toBe(true);
    }

    // Sha refs and local host-contracts overrides track the workspace, which is env mode.
    expect(canonicalProtocolConfigSeedingUsesEnv(stateFor("65cf86e"))).toBe(true);
    expect(canonicalProtocolConfigSeedingUsesEnv(stateFor("v0.14.0-8", [{ group: "host-contracts" }]))).toBe(true);
  });
});
