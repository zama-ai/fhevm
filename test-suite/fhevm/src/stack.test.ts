import { describe, expect, test } from "bun:test";

import { DEFAULT_EXTRA_HOST_RPC_PORT } from "./layout";
import { assertVersionLockChanges, changedVersionKeys, previewStateFromBundle, resolveUpgradePlan } from "./flow/up-flow";
import { presetBundle } from "./resolve/target";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";
const defaultScenario: State["scenario"] = testDefaultScenario();

describe("stack", () => {
  test("dry-run preview state uses the resolved lock target", () => {
    const bundle = {
      ...presetBundle("latest-main", "abcdef0", "devnet.json"),
      target: "devnet" as const,
      lockName: "devnet.json",
    };
    const state = previewStateFromBundle({ overrides: [], lockFile: "/tmp/devnet-lock.json" }, bundle, defaultScenario);
    expect(state.target).toBe("devnet");
    expect(state.requiresGitHub).toBe(false);
  });

  test("rejects multi-chain scenarios on network targets during preview", () => {
    const bundle = {
      ...presetBundle("latest-main", "abcdef0", "testnet.json"),
      target: "testnet" as const,
      lockName: "testnet.json",
    };
    const multiChainScenario: State["scenario"] = {
      ...defaultScenario,
      hostChains: [
        defaultScenario.hostChains[0],
        { key: "chain-b", chainId: "67890", rpcPort: DEFAULT_EXTRA_HOST_RPC_PORT },
      ],
    };
    expect(() => previewStateFromBundle({ overrides: [], lockFile: "/tmp/testnet-lock.json" }, bundle, multiChainScenario)).toThrow(
      "--target testnet does not currently support multi-chain scenarios",
    );
  });

  test("rejects multi-chain scenarios on legacy coprocessor bundles during preview", () => {
    const bundle = {
      ...presetBundle("latest-main", "abcdef0", "latest-supported.json"),
      target: "latest-supported" as const,
      env: {
        ...presetBundle("latest-main", "abcdef0", "latest-supported.json").env,
        HOST_VERSION: "v0.11.0",
        GATEWAY_VERSION: "v0.11.0",
        COPROCESSOR_HOST_LISTENER_VERSION: "v0.11.0",
        COPROCESSOR_HOST_LISTENER_POLLER_VERSION: "v0.11.0",
      },
    };
    const multiChainScenario: State["scenario"] = {
      ...defaultScenario,
      hostChains: [
        defaultScenario.hostChains[0],
        { key: "chain-b", chainId: "67890", rpcPort: DEFAULT_EXTRA_HOST_RPC_PORT },
      ],
    };
    expect(() => previewStateFromBundle({ overrides: [], lockFile: "/tmp/latest-supported-lock.json" }, bundle, multiChainScenario)).toThrow(
      "Multi-chain scenarios require coprocessor runtime >= v0.12.0",
    );
  });

  test("upgrade plan restarts runtime services for a full kms-connector override", () => {
    const plan = resolveUpgradePlan({ overrides: [{ group: "kms-connector" }], scenario: defaultScenario }, "kms-connector");
    expect(plan.migrationServices).toEqual(["kms-connector-db-migration"]);
    expect(plan.runtimeServices).toEqual([
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ]);
  });

  test("upgrade plan supports schema-coupled partial runtime overrides when runtime services exist", () => {
    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "kms-connector", services: ["kms-connector-gw-listener"] }],
        scenario: defaultScenario,
      },
      "kms-connector",
    );
    expect(plan.migrationServices).toEqual([]);
    expect(plan.runtimeServices).toEqual(["kms-connector-gw-listener"]);
  });

  test("upgrade plan supports relayer version locks without a local override", () => {
    const plan = resolveUpgradePlan({ overrides: [], scenario: defaultScenario }, "relayer", { lockFile: true });
    expect(plan.versionKeys).toEqual(["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"]);
    expect(plan.migrationServices).toEqual(["relayer-db-migration"]);
    expect(plan.runtimeServices).toEqual(["relayer"]);
  });

  test("upgrade plan links kms-core and connector for version-lock upgrades", () => {
    const plan = resolveUpgradePlan({ overrides: [], scenario: defaultScenario }, "kms", { lockFile: true });
    expect(plan.versionKeys).toEqual([
      "CORE_VERSION",
      "CONNECTOR_DB_MIGRATION_VERSION",
      "CONNECTOR_GW_LISTENER_VERSION",
      "CONNECTOR_KMS_WORKER_VERSION",
      "CONNECTOR_TX_SENDER_VERSION",
    ]);
    expect(plan.migrationServices).toEqual(["kms-connector-db-migration"]);
    expect(plan.runtimeServices).toEqual([
      "kms-core",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ]);
  });

  test("upgrade plan supports listener-core version-lock upgrades", () => {
    const plan = resolveUpgradePlan({ overrides: [], scenario: defaultScenario }, "listener-core", { lockFile: true });
    expect(plan.versionKeys).toEqual(["LISTENER_CORE_VERSION"]);
    expect(plan.migrationServices).toEqual([]);
    expect(plan.runtimeServices).toEqual(["listener-redis", "listener-publisher-for-anvil"]);
  });

  test("version lock checks reject unrelated version keys", () => {
    const base = presetBundle("latest-main", "abcdef0", "base.json");
    const next = {
      ...base,
      env: {
        ...base.env,
        RELAYER_VERSION: "next-relayer",
        HOST_VERSION: "next-host",
      },
    };
    const changed = changedVersionKeys(base, next);
    expect(changed).toEqual(["HOST_VERSION", "RELAYER_VERSION"]);
    expect(() => assertVersionLockChanges("relayer", ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"], changed)).toThrow(
      "HOST_VERSION",
    );
  });

  test("upgrade treats inherited multi-instance coprocessor build overrides as an active local runtime path", () => {
    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "coprocessor" }],
        scenario: {
          ...defaultScenario,
          topology: { count: 2, threshold: 2 },
          instances: [
            { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
            { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
          ],
        },
      },
      "coprocessor",
    );
    expect(plan.runtimeServices).toContain("coprocessor-host-listener");
    expect(plan.runtimeServices).toContain("coprocessor1-host-listener");
  });

  test("coprocessor release upgrade restarts inherited registry services", () => {
    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "test-suite" }],
        scenario: {
          ...defaultScenario,
          topology: { count: 2, threshold: 2 },
          instances: [],
        },
      },
      "coprocessor",
      { lockFile: true },
    );
    expect(plan.migrationServices).toContain("coprocessor-db-migration");
    expect(plan.runtimeServices).toContain("coprocessor-host-listener");
    expect(plan.runtimeServices).toContain("coprocessor1-host-listener");
  });
});
