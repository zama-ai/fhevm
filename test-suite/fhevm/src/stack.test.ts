import { describe, expect, test } from "bun:test";

import { DEFAULT_EXTRA_HOST_RPC_PORT } from "./layout";
import {
  assertVersionLockChanges,
  changedVersionKeys,
  previewStateFromBundle,
  removeRuntimeUpgradeOverrides,
  resolveUpgradePlan,
  restagePromotedGreen,
  startDeferredGreen,
} from "./flow/up-flow";
import { presetBundle } from "./resolve/target";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";
const defaultScenario: State["scenario"] = testDefaultScenario();
const blueGreenScenario: State["scenario"] = {
  version: 1,
  kind: "blue-green",
  origin: "default",
  hostChains: defaultScenario.hostChains,
  topology: { count: 2, threshold: 2 },
  bcs: { source: { mode: "registry", tag: "v0.14.0-10" }, env: {}, args: {} },
  gcs: { source: { mode: "local" }, env: {}, args: {}, stackVersion: "0.15.0", deferredStart: true },
  kms: defaultScenario.kms,
};
const thresholdBlueGreenScenario: State["scenario"] = {
  ...blueGreenScenario,
  kms: { mode: "threshold", parties: 4, threshold: 1, committeeSize: 4, fheParams: "Test" },
};

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

  test("upgrade plan supports listener-core local overrides", () => {
    const plan = resolveUpgradePlan({ overrides: [{ group: "listener-core" }], scenario: defaultScenario }, "listener-core");
    expect(plan.versionKeys).toEqual(["LISTENER_CORE_VERSION"]);
    expect(plan.migrationServices).toEqual([]);
    expect(plan.runtimeServices).toEqual(["listener-publisher-for-anvil"]);
  });

  test("runtime upgrade locks drop only overrides owned by the upgraded group", () => {
    expect(
      removeRuntimeUpgradeOverrides(
        [{ group: "relayer" }, { group: "test-suite" }],
        "relayer",
      ),
    ).toEqual([{ group: "test-suite" }]);
    expect(
      removeRuntimeUpgradeOverrides(
        [{ group: "kms-connector" }, { group: "listener-core" }, { group: "test-suite" }],
        "kms",
      ),
    ).toEqual([{ group: "listener-core" }, { group: "test-suite" }]);
    expect(
      removeRuntimeUpgradeOverrides(
        [{ group: "listener-core" }, { group: "test-suite" }],
        "listener-core",
      ),
    ).toEqual([{ group: "test-suite" }]);
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

  test("Blue-Green coprocessor release upgrade restarts Blue only", () => {
    const plan = resolveUpgradePlan(
      { overrides: [{ group: "test-suite" }], scenario: blueGreenScenario },
      "coprocessor",
      { lockFile: true },
    );
    expect(plan.runtimeServices).toContain("coprocessor-host-listener");
    expect(plan.runtimeServices).toContain("coprocessor1-host-listener");
    expect(plan.runtimeServices.some((service) => service.includes("gcs-"))).toBe(false);
    expect(plan.runtimeServices).not.toContain("coprocessor-consensus-detector");
    expect(plan.runtimeServices).not.toContain("coprocessor-upgrade-controller");
    expect(plan.runtimeServices).not.toContain("coprocessor1-consensus-detector");
    expect(plan.runtimeServices).not.toContain("coprocessor1-upgrade-controller");
    expect(plan.versionKeys).toContain("COPROCESSOR_CONSENSUS_DETECTOR_VERSION");
    expect(plan.versionKeys).toContain("COPROCESSOR_UPGRADE_CONTROLLER_VERSION");
  });

  test("Blue-Green connector upgrades require the operator-paired threshold rollout", () => {
    expect(() =>
      resolveUpgradePlan(
        { overrides: [{ group: "test-suite" }], scenario: thresholdBlueGreenScenario },
        "kms-connector",
        { lockFile: true },
      ),
    ).toThrow(/operator-paired rollout primitive/);
  });

  test("commits deferred Green state only after startup health succeeds", async () => {
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/lock.json",
      versions: presetBundle("latest-main", "abcdef0", "lock.json"),
      overrides: [],
      scenario: blueGreenScenario,
      completedSteps: ["base", "coprocessor"],
      updatedAt: "2026-07-14T00:00:00.000Z",
    };
    let saved: State | undefined;
    let buildPersistedState = true;
    const generatedDeferredStates: boolean[] = [];
    let removedContainers: string[] = [];
    const operations = {
      async loadState() { return state; },
      async generateRuntime(next: State) {
        if (next.scenario.kind === "blue-green") {
          generatedDeferredStates.push(next.scenario.gcs.deferredStart);
        }
      },
      async maybeBuild(_component: string, _state: State, options?: { persistState?: boolean }) {
        buildPersistedState = options?.persistState !== false;
      },
      async composeUp() {},
      async waitForContainer() {},
      async waitForCoprocessorServices() {},
      async multiChainComposeUp() {},
      async postBootHealthGate() { throw new Error("Green unhealthy"); },
      async removeContainers(containers: string[]) { removedContainers = containers; },
      async saveState(next: State) { saved = next; },
    };

    await expect(startDeferredGreen(operations)).rejects.toThrow("Green unhealthy");
    expect(buildPersistedState).toBe(false);
    expect(saved).toBeUndefined();
    expect(generatedDeferredStates).toEqual([false, true]);
    expect(removedContainers.length).toBeGreaterThan(0);

    await startDeferredGreen({ ...operations, async postBootHealthGate() {} });
    expect(saved?.scenario.kind).toBe("blue-green");
    if (saved?.scenario.kind === "blue-green") {
      expect(saved.scenario.gcs.deferredStart).toBe(false);
    }
  });

  test("re-homes a promoted local Green before staging the next Green version", async () => {
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/lock.json",
      versions: presetBundle("latest-main", "abcdef0", "lock.json"),
      overrides: [],
      scenario: {
        ...blueGreenScenario,
        gcs: {
          ...blueGreenScenario.gcs,
          deferredStart: false,
          env: { FORCE_LEGACY_SERVER_KEY: "true" },
        },
      },
      completedSteps: ["base", "coprocessor"],
      updatedAt: "2026-07-14T00:00:00.000Z",
    };
    const generated: string[] = [];
    const removed: string[][] = [];
    const saved: State[] = [];
    const started: string[][] = [];
    const operations = {
      async loadState() { return state; },
      async generateRuntime(next: State) {
        if (next.scenario.kind === "blue-green") {
          generated.push(`${next.scenario.gcs.stackVersion}:${next.scenario.gcs.deferredStart}`);
        }
      },
      async maybeBuild() {},
      async composeUp(_component: string, services: string[] = []) { started.push(services); },
      async waitForContainer() {},
      async waitForCoprocessorServices() {},
      async multiChainComposeUp() {},
      async postBootHealthGate() {},
      async removeContainers(containers: string[]) { removed.push(containers); },
      async saveState(next: State) { saved.push(next); },
    };

    await restagePromotedGreen({ stackVersion: "0.15.1" }, operations);

    expect(generated).toEqual(["0.15.0:true", "0.15.1:true"]);
    expect(started[0]?.some((service) => service.includes("-gcs-"))).toBe(false);
    expect(removed[0]).toContain("coprocessor-tfhe-worker");
    expect(removed[1]).toContain("coprocessor-gcs-tfhe-worker");
    expect(saved).toHaveLength(1);
    const next = saved[0]?.scenario;
    expect(next?.kind).toBe("blue-green");
    if (next?.kind === "blue-green") {
      expect(next.bcs.source).toEqual({
        mode: "registry",
        tag: "gcs-0.15.0",
        compatTag: "v0.15.0",
      });
      expect(next.bcs.env.FORCE_LEGACY_SERVER_KEY).toBe("true");
      expect(next.gcs.stackVersion).toBe("0.15.1");
      expect(next.gcs.deferredStart).toBe(true);
      expect(next.gcs.env.FORCE_LEGACY_SERVER_KEY).toBeUndefined();
    }
  });

  test("restores the promoted Green definition when Blue re-homing fails", async () => {
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/lock.json",
      versions: presetBundle("latest-main", "abcdef0", "lock.json"),
      overrides: [],
      scenario: {
        ...blueGreenScenario,
        gcs: { ...blueGreenScenario.gcs, deferredStart: false },
      },
      completedSteps: ["base", "coprocessor"],
      updatedAt: "2026-07-14T00:00:00.000Z",
    };
    const generatedDeferredStates: boolean[] = [];
    let saves = 0;
    const operations = {
      async loadState() { return state; },
      async generateRuntime(next: State) {
        if (next.scenario.kind === "blue-green") {
          generatedDeferredStates.push(next.scenario.gcs.deferredStart);
        }
      },
      async maybeBuild() {},
      async composeUp() {},
      async waitForContainer() {},
      async waitForCoprocessorServices() {},
      async multiChainComposeUp() {},
      async postBootHealthGate() { throw new Error("replacement Blue unhealthy"); },
      async removeContainers() {},
      async saveState() { saves += 1; },
    };

    await expect(restagePromotedGreen({ stackVersion: "0.15.1" }, operations)).rejects.toThrow(
      "replacement Blue unhealthy",
    );
    expect(generatedDeferredStates).toEqual([true, false]);
    expect(saves).toBe(0);
  });
});
