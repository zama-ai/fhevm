import { describe, expect, test } from "bun:test";

import { compatPolicyForState, requiresLegacyRelayerUrl, validateBundleCompatibility } from "./compat/compat";

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
      coprocessor: {
        version: 1,
        kind: "coprocessor-consensus",
        origin: "default",
        hostChains: [{ key: "host", chainId: "12345", rpcPort: 8545 }],
        topology: { count: 1, threshold: 1 },
        instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
      },
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
});
