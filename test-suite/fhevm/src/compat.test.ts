import { describe, expect, test } from "bun:test";

import { compatPolicyForState, requiresLegacyRelayerUrl, validateBundleCompatibility } from "./compat/compat";
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
});
