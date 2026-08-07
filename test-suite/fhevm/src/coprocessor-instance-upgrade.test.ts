import path from "node:path";
import { describe, expect, test } from "bun:test";

import { coprocessorInstanceUpgradeTargets } from "./flow/repair";
import { assertCoprocessorUpgradeThreshold, upgradeCoprocessorInstance } from "./flow/up-flow";
import { presetBundle } from "./resolve/target";
import { testDefaultScenario } from "./test-fixtures";
import { withTempStateDir } from "./test-state";
import type { State } from "./types";
import { writeJson } from "./utils/fs";

const TARGET_TAG = "target-copro";

const threeInstanceState = (versions: State["versions"]): State => ({
  target: "latest-main",
  lockPath: "/tmp/baseline.json",
  requiresGitHub: true,
  versions,
  overrides: [],
  scenario: testDefaultScenario({
    topology: { count: 3, threshold: 2 },
    instances: [0, 1, 2].map((index) => ({ index, source: { mode: "inherit" as const }, env: {}, args: {} })),
  }),
  completedSteps: ["base", "coprocessor"],
  updatedAt: "2026-07-14T00:00:00.000Z",
});

const targetLock = async (stateDir: string, versions: State["versions"]) => {
  const lockFile = path.join(stateDir, "coprocessor.json");
  await writeJson(lockFile, {
    ...versions,
    lockName: "coprocessor.json",
    env: {
      ...versions.env,
      COPROCESSOR_DB_MIGRATION_VERSION: TARGET_TAG,
      COPROCESSOR_HOST_LISTENER_VERSION: TARGET_TAG,
      COPROCESSOR_GW_LISTENER_VERSION: TARGET_TAG,
      COPROCESSOR_TX_SENDER_VERSION: TARGET_TAG,
      COPROCESSOR_TFHE_WORKER_VERSION: TARGET_TAG,
      COPROCESSOR_ZKPROOF_WORKER_VERSION: TARGET_TAG,
      COPROCESSOR_SNS_WORKER_VERSION: TARGET_TAG,
    },
  });
  return lockFile;
};

const instanceSources = (state: State) =>
  state.scenario.kind === "coprocessor-consensus" ? state.scenario.instances.map((instance) => instance.source) : [];

describe("upgradeCoprocessorInstance", () => {
  test("pins one operator to the target tag and leaves its peers on the bundle", async () => {
    await withTempStateDir(async (stateDir) => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      let persisted = threeInstanceState(versions);
      const lockFile = await targetLock(stateDir, versions);
      const recreated: string[] = [];

      await upgradeCoprocessorInstance(1, { lockFile }, {
        async loadState() {
          return persisted;
        },
        async projectContainers() {
          return ["coprocessor-tfhe-worker"];
        },
        async ensureRuntimeArtifacts() {},
        async assertThreshold() {},
        async postBootHealthGate() {},
        async saveState(next) {
          persisted = next;
        },
        async generateRuntime() {},
        async composeUp(_component, services = []) {
          recreated.push(...services);
        },
        // Extra host chains compose from their generated override alone, so they land here
        // rather than in composeUp. Both are recorded, since the assertion below covers the
        // whole set of services the instance upgrade recreates.
        async multiChainComposeUp(_component, services = []) {
          recreated.push(...services);
        },
        async waitForContainer() {},
      });

      expect(instanceSources(persisted)).toEqual([
        { mode: "inherit" },
        { mode: "registry", tag: TARGET_TAG },
        { mode: "inherit" },
      ]);
      // The bundle stays put while the fleet is mixed — only the pinned instance is ahead.
      expect(persisted.versions.env.COPROCESSOR_TFHE_WORKER_VERSION).toBe(
        versions.env.COPROCESSOR_TFHE_WORKER_VERSION,
      );
      // Its own migration runs first, and nothing belonging to another operator is touched.
      expect(recreated[0]).toBe("coprocessor1-db-migration");
      expect(recreated.every((service) => service.startsWith("coprocessor1-"))).toBe(true);
    });
  });

  test("lands the bundle and releases every pin once the last operator crosses", async () => {
    await withTempStateDir(async (stateDir) => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      let persisted = threeInstanceState(versions);
      const lockFile = await targetLock(stateDir, versions);

      const operations = {
        async loadState() {
          return persisted;
        },
        async projectContainers() {
          return ["coprocessor-tfhe-worker"];
        },
        async ensureRuntimeArtifacts() {},
        async assertThreshold() {},
        async postBootHealthGate() {},
        async saveState(next: State) {
          persisted = next;
        },
        async generateRuntime() {},
        async composeUp() {},
        async multiChainComposeUp() {},
        async waitForContainer() {},
      };

      for (const index of [0, 1, 2]) {
        await upgradeCoprocessorInstance(index, { lockFile }, operations);
      }

      expect(persisted.versions.env.COPROCESSOR_TFHE_WORKER_VERSION).toBe(TARGET_TAG);
      expect(persisted.versions.env.COPROCESSOR_DB_MIGRATION_VERSION).toBe(TARGET_TAG);
      // Back to inheriting from the bundle, so nothing stays pinned to a stale tag.
      expect(instanceSources(persisted)).toEqual([{ mode: "inherit" }, { mode: "inherit" }, { mode: "inherit" }]);
    });
  });

  // `source.mode: registry` carries a single tag for the instance's whole fleet, so a lock that
  // splits the coprocessor components across tags cannot be staged one operator at a time.
  test("refuses a lock whose coprocessor components do not share one tag", async () => {
    await withTempStateDir(async (stateDir) => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      const persisted = threeInstanceState(versions);
      const lockFile = path.join(stateDir, "split.json");
      await writeJson(lockFile, {
        ...versions,
        lockName: "split.json",
        env: {
          ...versions.env,
          COPROCESSOR_DB_MIGRATION_VERSION: TARGET_TAG,
          COPROCESSOR_HOST_LISTENER_VERSION: TARGET_TAG,
          COPROCESSOR_GW_LISTENER_VERSION: TARGET_TAG,
          COPROCESSOR_TX_SENDER_VERSION: TARGET_TAG,
          COPROCESSOR_TFHE_WORKER_VERSION: "other-tag",
          COPROCESSOR_ZKPROOF_WORKER_VERSION: TARGET_TAG,
          COPROCESSOR_SNS_WORKER_VERSION: TARGET_TAG,
        },
      });

      await expect(
        upgradeCoprocessorInstance(0, { lockFile }, {
          async loadState() {
            return persisted;
          },
          async projectContainers() {
            return ["coprocessor-tfhe-worker"];
          },
          async ensureRuntimeArtifacts() {},
          async assertThreshold() {},
          async postBootHealthGate() {},
          async saveState() {},
          async generateRuntime() {},
          async composeUp() {},
          async multiChainComposeUp() {},
          async waitForContainer() {},
        }),
      ).rejects.toThrow(/share one tag/);
    });
  });

  // A release that adds a coprocessor component leaves the running bundle with no key for it,
  // so the lock that crosses into that release is the first thing to name it. Landing a new
  // component has to read as the fleet moving to one tag, not as a fleet split across two.
  test("lands a component the running bundle does not have yet", async () => {
    await withTempStateDir(async (stateDir) => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      const baseline: State["versions"] = {
        ...versions,
        env: { ...versions.env, COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "", COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "" },
      };
      let persisted = threeInstanceState(baseline);
      const lockFile = path.join(stateDir, "coprocessor.json");
      await writeJson(lockFile, {
        ...baseline,
        lockName: "coprocessor.json",
        env: {
          ...baseline.env,
          COPROCESSOR_DB_MIGRATION_VERSION: TARGET_TAG,
          COPROCESSOR_HOST_LISTENER_VERSION: TARGET_TAG,
          COPROCESSOR_GW_LISTENER_VERSION: TARGET_TAG,
          COPROCESSOR_TX_SENDER_VERSION: TARGET_TAG,
          COPROCESSOR_TFHE_WORKER_VERSION: TARGET_TAG,
          COPROCESSOR_ZKPROOF_WORKER_VERSION: TARGET_TAG,
          COPROCESSOR_SNS_WORKER_VERSION: TARGET_TAG,
          COPROCESSOR_CONSENSUS_DETECTOR_VERSION: TARGET_TAG,
          COPROCESSOR_UPGRADE_CONTROLLER_VERSION: TARGET_TAG,
        },
      });

      const operations = {
        async loadState() {
          return persisted;
        },
        async projectContainers() {
          return ["coprocessor-tfhe-worker"];
        },
        async ensureRuntimeArtifacts() {},
        async assertThreshold() {},
        async postBootHealthGate() {},
        async saveState(next: State) {
          persisted = next;
        },
        async generateRuntime() {},
        async composeUp() {},
        async multiChainComposeUp() {},
        async waitForContainer() {},
      };

      for (const index of [0, 1, 2]) {
        await upgradeCoprocessorInstance(index, { lockFile }, operations);
      }

      expect(persisted.versions.env.COPROCESSOR_UPGRADE_CONTROLLER_VERSION).toBe(TARGET_TAG);
      expect(persisted.versions.env.COPROCESSOR_CONSENSUS_DETECTOR_VERSION).toBe(TARGET_TAG);
    });
  });

  // Recreating an operator removes it from the fleet for the width of the upgrade. With 3
  // operators at threshold 2 that leaves exactly the threshold, so an already-unhealthy peer
  // means consensus silently stops instead of the rollout stopping.
  test("refuses to take an operator down when the remaining fleet cannot reach consensus", async () => {
    const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
    const state = threeInstanceState(versions);
    const down = new Set(["coprocessor2-tfhe-worker"]);

    await expect(
      assertCoprocessorUpgradeThreshold(state, 1, (async (container: string) =>
        down.has(container) ? [] : [{ State: { Status: "running" } }]) as never),
    ).rejects.toThrow(/1 remaining operators are ready, but the consensus threshold is 2/);
  });

  test("allows the upgrade while every remaining operator is running", async () => {
    const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
    const state = threeInstanceState(versions);

    await expect(
      assertCoprocessorUpgradeThreshold(state, 1, (async () => [{ State: { Status: "running" } }]) as never),
    ).resolves.toBeUndefined();
  });

  test("rejects an out-of-range operator index", async () => {
    await withTempStateDir(async (stateDir) => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      const persisted = threeInstanceState(versions);
      const lockFile = await targetLock(stateDir, versions);

      await expect(
        upgradeCoprocessorInstance(3, { lockFile }, {
          async loadState() {
            return persisted;
          },
          async projectContainers() {
            return ["coprocessor-tfhe-worker"];
          },
          async ensureRuntimeArtifacts() {},
          async assertThreshold() {},
          async postBootHealthGate() {},
          async saveState() {},
          async generateRuntime() {},
          async composeUp() {},
          async multiChainComposeUp() {},
          async waitForContainer() {},
        }),
      ).rejects.toThrow(/between 0 and 2/);
    });
  });
});

// Extra host chains have no `coprocessor-<chain>-docker-compose.yml` template — they exist only
// as generated overrides under the runtime compose dir. Composing them the ordinary way fails
// with "no such file or directory" and takes the whole instance upgrade down with it, so they
// must be flagged for multiChainComposeUp, which composes the generated file alone.
describe("coprocessorInstanceUpgradeTargets", () => {
  test("routes extra host chains through the generated-override compose path", () => {
    const state = threeInstanceState(presetBundle("latest-main", "abcdef0", "baseline.json"));
    const multiChainState: State = {
      ...state,
      scenario: testDefaultScenario({
        topology: { count: 3, threshold: 2 },
        instances: [0, 1, 2].map((index) => ({ index, source: { mode: "inherit" as const }, env: {}, args: {} })),
        hostChains: [
          { key: "host", chainId: "12345", rpcPort: 8545 },
          { key: "chain-b", chainId: "67890", rpcPort: 8547 },
        ],
      }),
    };

    const targets = coprocessorInstanceUpgradeTargets(multiChainState, 0);
    const defaultChain = targets.components.find((component) => component.component === "coprocessor");
    const extraChain = targets.components.find((component) => component.component === "coprocessor-chain-b");

    expect(defaultChain?.multiChain).toBe(false);
    expect(extraChain).toBeDefined();
    expect(extraChain?.multiChain).toBe(true);
    expect(extraChain?.runtimeServices.length).toBeGreaterThan(0);
  });
});
