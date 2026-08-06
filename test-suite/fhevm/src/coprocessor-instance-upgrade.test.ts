import path from "node:path";
import { describe, expect, test } from "bun:test";

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
          async waitForContainer() {},
        }),
      ).rejects.toThrow(/share one tag/);
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
          async waitForContainer() {},
        }),
      ).rejects.toThrow(/between 0 and 2/);
    });
  });
});
