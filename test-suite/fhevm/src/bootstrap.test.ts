import { describe, expect, test } from "bun:test";

import {
  BOOTSTRAP_SUSPENDED_GROUPS,
  GATEWAY_CONTRACT_UPGRADES,
  HOST_CONTRACT_UPGRADES,
  bootstrapBootOverrides,
  bootstrapBootVersions,
  bootstrapPhaseEnv,
  contractUpgradeCommand,
} from "./flow/bootstrap";

describe("blue-green bootstrap", () => {
  const bundle = {
    target: "latest-main" as const,
    lockName: "latest-main.json",
    env: {
      CORE_VERSION: "v0.15.0-0",
      GATEWAY_VERSION: "main",
      HOST_VERSION: "main",
      LISTENER_CORE_VERSION: "main",
      CONNECTOR_DB_MIGRATION_VERSION: "main",
      CONNECTOR_GW_LISTENER_VERSION: "main",
      CONNECTOR_KMS_WORKER_VERSION: "main",
      CONNECTOR_TX_SENDER_VERSION: "main",
      CONNECTOR_ENDPOINT_VERSION: "main",
      CONNECTOR_PROXY_VERSION: "main",
      RELAYER_VERSION: "main",
      COPROCESSOR_TFHE_WORKER_VERSION: "main",
      TEST_SUITE_VERSION: "main",
    },
    sources: ["preset=latest-main"],
  };

  test("pins contracts, KMS core and connector and listener-core to the release", () => {
    const boot = bootstrapBootVersions(bundle, { tag: "v0.14.2-0", coreVersion: "v0.14.1-0" });
    expect(boot.env).toEqual({
      CORE_VERSION: "v0.14.1-0",
      GATEWAY_VERSION: "v0.14.2-0",
      HOST_VERSION: "v0.14.2-0",
      LISTENER_CORE_VERSION: "v0.14.2-0",
      CONNECTOR_DB_MIGRATION_VERSION: "v0.14.2-0",
      CONNECTOR_GW_LISTENER_VERSION: "v0.14.2-0",
      CONNECTOR_KMS_WORKER_VERSION: "v0.14.2-0",
      CONNECTOR_TX_SENDER_VERSION: "v0.14.2-0",
      RELAYER_VERSION: "main",
      COPROCESSOR_TFHE_WORKER_VERSION: "main",
      TEST_SUITE_VERSION: "main",
    });
    expect(boot.sources).toEqual(["preset=latest-main", "bootstrap=v0.14.2-0"]);
    expect(bundle.env.CONNECTOR_PROXY_VERSION).toBe("main");
  });

  test("suspends only the overrides of the pinned components", () => {
    const overrides = [
      { group: "coprocessor" as const },
      { group: "kms-connector" as const },
      { group: "gateway-contracts" as const },
      { group: "host-contracts" as const },
      { group: "listener-core" as const },
      { group: "relayer" as const },
      { group: "test-suite" as const },
    ];
    expect(bootstrapBootOverrides(overrides).map((override) => override.group)).toEqual([
      "coprocessor",
      "relayer",
      "test-suite",
    ]);
    expect(BOOTSTRAP_SUSPENDED_GROUPS).toEqual(["gateway-contracts", "host-contracts", "kms-connector", "listener-core"]);
  });

  test("advances one deployment unit per phase and restores optional connector services", () => {
    const boot = bootstrapBootVersions(bundle, { tag: "v0.14.2-0", coreVersion: "v0.14.2-0" }).env;
    const contracts = bootstrapPhaseEnv(boot, bundle.env, ["GATEWAY_VERSION", "HOST_VERSION"]);
    expect(contracts).toEqual({ ...boot, GATEWAY_VERSION: "main", HOST_VERSION: "main" });
    const connector = bootstrapPhaseEnv(contracts, bundle.env, [
      "CONNECTOR_KMS_WORKER_VERSION",
      "CONNECTOR_ENDPOINT_VERSION",
      "CONNECTOR_PROXY_VERSION",
    ]);
    expect(connector.CONNECTOR_KMS_WORKER_VERSION).toBe("main");
    expect(connector.CONNECTOR_ENDPOINT_VERSION).toBe("main");
    expect(connector.CONNECTOR_TX_SENDER_VERSION).toBe("v0.14.2-0");
    expect(bootstrapPhaseEnv(connector, { ...bundle.env, CONNECTOR_PROXY_VERSION: undefined as never }, ["CONNECTOR_PROXY_VERSION"]))
      .not.toHaveProperty("CONNECTOR_PROXY_VERSION");
  });

  test("upgrades every contract whose reinitializer moved since 0.14 through its task", () => {
    expect(GATEWAY_CONTRACT_UPGRADES.map(([, contract]) => contract)).toEqual([
      "Decryption",
      "CiphertextCommits",
      "InputVerification",
      "GatewayConfig",
    ]);
    expect(HOST_CONTRACT_UPGRADES.map(([, contract]) => contract)).toEqual(["KMSGeneration", "FHEVMExecutor", "ProtocolConfig"]);
    for (const [task, contract] of [...GATEWAY_CONTRACT_UPGRADES, ...HOST_CONTRACT_UPGRADES]) {
      expect(task).toBe(`task:upgrade${contract}`);
    }
    expect(contractUpgradeCommand("task:upgradeDecryption", "Decryption")).toBe(
      "npx hardhat task:upgradeDecryption --current-implementation previous-contracts/Decryption.sol:Decryption " +
        "--new-implementation contracts/Decryption.sol:Decryption --verify-contract false --use-internal-proxy-address true",
    );
  });
});
