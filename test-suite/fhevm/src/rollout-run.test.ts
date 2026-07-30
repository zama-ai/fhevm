import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { receiptJsonlPath, receiptMarkdownPath, requireDockerSnapshot } from "./commands/rollout-receipt";
import {
  type RolloutRunContext,
  loadRolloutRunbook,
  matchesExpectedTestFailure,
  runRolloutRunbook,
} from "./commands/rollout-run";
import { CommandError, PreflightError } from "./errors";
import { withTempStateDir } from "./test-state";
import { saveState } from "./state/state";
import { presetBundle } from "./resolve/target";
import { testDefaultScenario } from "./test-fixtures";
import type { State, VersionBundle } from "./types";
import { readJson, remove, writeJson } from "./utils/fs";

const tempDirs: string[] = [];
const CLI_DIR = path.resolve(import.meta.dir, "..");

afterEach(async () => {
  await Promise.all(tempDirs.splice(0).map((dir) => remove(dir)));
});

const fakeContext = () => {
  const calls: string[] = [];
  const ctx: RolloutRunContext = {
    async applyVersionLock(label, options) {
      calls.push(`apply-version-lock:${label}:${options.lockFile}`);
    },
    async expectTestFailure(profile, options) {
      calls.push(`expected-test-failure:${profile}:${options.errorIncludes}`);
    },
    async readState() {
      calls.push("state");
      return {} as State;
    },
    async refreshDiscovery() {
      calls.push("refresh-discovery");
    },
    async runGatewayContractTask(command) {
      calls.push(`gateway:${command}`);
    },
    async runHostContractTask(command) {
      calls.push(`host:${command}`);
    },
    async runHostContractTaskOnChain(chainKey, command) {
      calls.push(`host[${chainKey}]:${command}`);
    },
    async snapshotContracts(surface) {
      calls.push(`snapshot:${surface}`);
    },
    stateDir() {
      return "/tmp/fhevm";
    },
    async test(profile = "rollout-standard") {
      calls.push(`test:${profile}`);
    },
    async up(options) {
      calls.push(`up:${options.lockFile}`);
    },
    async upgradeKmsNodes(nodeIds, options) {
      calls.push(`upgrade-kms-nodes:${nodeIds.join(",")}:${options.lockFile}`);
    },
    async withRequiredKmsNode(nodeId, task) {
      calls.push(`require-kms-node:${nodeId}`);
      await task();
    },
    async upgradeRuntimeGroup(group, options = {}) {
      calls.push(`upgrade:${group}:${options.lockFile ?? ""}`);
    },
    async writeVersionLock(name, options) {
      calls.push(`lock:${name}:${options.versions.RELAYER_VERSION ?? ""}`);
      return `/tmp/${name}.lock.json`;
    },
  };
  return { calls, ctx };
};

describe("rollout runbook", () => {
  test("matches only command failures containing the expected test error", () => {
    const expected = "expected test failure";
    expect(matchesExpectedTestFailure(new CommandError(["test"], 1, expected), expected)).toBe(true);
    expect(matchesExpectedTestFailure(new CommandError(["test"], 1, "connection refused"), expected)).toBe(false);
    expect(matchesExpectedTestFailure(new PreflightError(expected), expected)).toBe(false);
  });

  test("loads a default exported runbook", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-run-"));
    tempDirs.push(root);
    const script = path.join(root, "runbook.ts");
    await writeFile(script, "export default async (ctx) => { await ctx.test('rollout-standard'); };\n");

    const { calls, ctx } = fakeContext();
    await (
      await loadRolloutRunbook(script)
    )(ctx);
    expect(calls).toEqual(["test:rollout-standard"]);
  });

  test("loads the checked-in v0.12 to v0.13 runbook", async () => {
    await expect(loadRolloutRunbook(path.join(CLI_DIR, "rollouts/v0.12-to-v0.13/run.ts"))).resolves.toBeFunction();
  });

  test("executes runbook helpers in code order", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-run-"));
    tempDirs.push(root);
    const script = path.join(root, "runbook.ts");
    await writeFile(
      script,
      [
        "export const run = async (ctx) => {",
        "  await ctx.up({ lockFile: '00-baseline.lock.json' });",
        "  await ctx.upgradeRuntimeGroup('relayer', { lockFile: '01-relayer.lock.json' });",
        "  await ctx.writeVersionLock('02-contracts', { versions: { RELAYER_VERSION: 'next-relayer' } });",
        "  await ctx.applyVersionLock('contracts', { lockFile: '02-contracts.lock.json', allowedVersionKeys: ['GATEWAY_VERSION', 'HOST_VERSION'] });",
        "  await ctx.runGatewayContractTask('npx hardhat task:exportKmsMigrationState');",
        "  await ctx.refreshDiscovery();",
        "  await ctx.test();",
        "};",
        "",
      ].join("\n"),
    );

    const { calls, ctx } = fakeContext();
    await runRolloutRunbook(script, ctx);
    expect(calls).toEqual([
      "up:00-baseline.lock.json",
      "upgrade:relayer:01-relayer.lock.json",
      "lock:02-contracts:next-relayer",
      "apply-version-lock:contracts:02-contracts.lock.json",
      "gateway:npx hardhat task:exportKmsMigrationState",
      "refresh-discovery",
      "test:rollout-standard",
    ]);
  });

  test("writes runbook lock artifacts", async () => {
    await withTempStateDir(async (stateDir) => {
      const { createRolloutContext } = await import("./commands/rollout-run");
      const file = await createRolloutContext().writeVersionLock("01-relayer", {
        versions: { RELAYER_VERSION: "next-relayer" },
        sources: ["test"],
      });
      const lock = await readJson<VersionBundle>(file);
      expect(file).toBe(path.join(stateDir, "rollout", "01-relayer.lock.json"));
      expect(lock.target).toBe("latest-main");
      expect(lock.lockName).toBe("01-relayer.lock.json");
      expect(lock.env.RELAYER_VERSION).toBe("next-relayer");
      expect(lock.sources).toEqual(["test"]);
    });
  });

  test("writes a rollout receipt with applied lock deltas", async () => {
    await withTempStateDir(async (stateDir) => {
      const first = path.join(stateDir, "rollout", "00-baseline.lock.json");
      const second = path.join(stateDir, "rollout", "01-relayer.lock.json");
      await writeJson(first, {
        target: "sha",
        lockName: "00-baseline.lock.json",
        env: { RELAYER_VERSION: "old" },
        sources: ["test"],
      });
      await writeJson(second, {
        target: "sha",
        lockName: "01-relayer.lock.json",
        env: { RELAYER_VERSION: "new" },
        sources: ["test"],
      });

      const { createRolloutReceipt } = await import("./commands/rollout-receipt");
      const receipt = createRolloutReceipt();
      await receipt.start("rollout.ts");
      await receipt.record("up", "boot stack", { lockFile: first });
      await receipt.record("upgrade-runtime", "relayer", { lockFile: second });

      const entries = (await Bun.file(receiptJsonlPath()).text())
        .trim()
        .split("\n")
        .map((line) => JSON.parse(line));
      expect(entries[0].versionChanges).toEqual([{ key: "RELAYER_VERSION", to: "old" }]);
      expect(entries[1].versionChanges).toEqual([{ key: "RELAYER_VERSION", from: "old", to: "new" }]);
      expect(await Bun.file(receiptMarkdownPath()).text()).toContain("RELAYER_VERSION");
    });
  });

  test("requires audit-critical Docker snapshots to contain inspected containers", () => {
    expect(() => requireDockerSnapshot({ containers: [], error: "docker inspect failed" })).toThrow(
      "Required Docker snapshot failed: docker inspect failed",
    );
    expect(() => requireDockerSnapshot({ containers: [] })).toThrow("contained no project containers");
    expect(() =>
      requireDockerSnapshot({
        containers: [{ image: "image", imageId: "sha256:id", name: "kms-core", state: "running" }],
      }),
    ).not.toThrow();
  });

  test("records a failed required Docker snapshot before rejecting it", async () => {
    await withTempStateDir(async () => {
      const { createRolloutReceipt } = await import("./commands/rollout-receipt");
      const receipt = createRolloutReceipt({
        async inspectContainers() {
          return { containers: [], error: "daemon unavailable" };
        },
      });
      await receipt.start("rollout.ts");
      await expect(receipt.record("upgrade-kms-node", "KMS node 2", { docker: true })).rejects.toThrow(
        "Required Docker snapshot failed: daemon unavailable",
      );

      const entry = JSON.parse((await Bun.file(receiptJsonlPath()).text()).trim());
      expect(entry.dockerInspectError).toBe("daemon unavailable");
      expect(await Bun.file(receiptMarkdownPath()).text()).toContain("Docker inspect failed: `daemon unavailable`");
    });
  });

  test("records Docker evidence when a KMS node changes but readiness fails", async () => {
    await withTempStateDir(async (stateDir) => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      await saveState({
        target: "latest-main",
        lockPath: "/tmp/baseline.json",
        requiresGitHub: true,
        versions,
        overrides: [],
        scenario: testDefaultScenario({
          kms: { mode: "threshold", parties: 4, threshold: 1, committeeSize: 4, fheParams: "Test" },
        }),
        completedSteps: ["base"],
        updatedAt: "2026-07-14T00:00:00.000Z",
      });
      const lockFile = path.join(stateDir, "target.json");
      await writeJson(lockFile, {
        ...versions,
        lockName: "target.json",
        env: { ...versions.env, CORE_VERSION: "target-core" },
      });
      const { createRolloutContext } = await import("./commands/rollout-run");
      const { createRolloutReceipt } = await import("./commands/rollout-receipt");
      const receipt = createRolloutReceipt({
        async inspectContainers() {
          return {
            containers: [
              {
                image: "ghcr.io/zama-ai/kms/core-service:target-core",
                imageId: "sha256:target",
                name: "kms-core-2",
                state: "running",
              },
            ],
          };
        },
      });
      await receipt.start("rollout.ts");
      const context = createRolloutContext(receipt, {
        async upgradeThresholdKmsNode() {
          throw new Error("readiness failed");
        },
      });

      await expect(context.upgradeKmsNodes([2], { lockFile })).rejects.toThrow("readiness failed");

      const entry = JSON.parse((await Bun.file(receiptJsonlPath()).text()).trim());
      expect(entry.kind).toBe("upgrade-kms-node-failed");
      expect(entry.details).toMatchObject({ error: "readiness failed", nodeId: 2 });
      expect(entry.containers[0]).toMatchObject({ imageId: "sha256:target", name: "kms-core-2" });
    });
  });

  test("requires the selected KMS node in the live quorum and restores the stopped node", async () => {
    await withTempStateDir(async () => {
      const versions = presetBundle("latest-main", "abcdef0", "baseline.json");
      await saveState({
        target: "latest-main",
        lockPath: "/tmp/baseline.json",
        requiresGitHub: true,
        versions,
        overrides: [],
        scenario: testDefaultScenario({
          kms: { mode: "threshold", parties: 4, threshold: 1, committeeSize: 4, fheParams: "Test" },
        }),
        completedSteps: ["base"],
        updatedAt: "2026-07-14T00:00:00.000Z",
      });
      const { createRolloutContext } = await import("./commands/rollout-run");
      const { createRolloutReceipt } = await import("./commands/rollout-receipt");
      const receipt = createRolloutReceipt({
        async inspectContainers() {
          return {
            containers: [{ image: "core", imageId: "sha256:core", name: "kms-core", state: "running" }],
          };
        },
      });
      await receipt.start("rollout.ts");
      const calls: string[] = [];
      const context = createRolloutContext(receipt, {
        async setRunning(containers, action) {
          calls.push(`${action}:${containers.join(",")}`);
        },
        async waitForPartiesRunning(parties) {
          calls.push(`running:${parties.join(",")}`);
        },
        async waitForPartiesStopped(parties) {
          calls.push(`stopped:${parties.join(",")}`);
        },
      });

      let rejected = false;
      try {
        await context.withRequiredKmsNode(4, async () => {
          calls.push("test");
          throw undefined;
        });
      } catch (error) {
        rejected = true;
        expect(error).toBeUndefined();
      }

      expect(rejected).toBe(true);
      expect(calls).toEqual([
        "stop:kms-core-3,kms-connector-3-gw-listener,kms-connector-3-kms-worker,kms-connector-3-tx-sender",
        "stopped:3",
        "test",
        "start:kms-core-3,kms-connector-3-gw-listener,kms-connector-3-kms-worker,kms-connector-3-tx-sender",
        "running:3",
      ]);
      const entries = (await Bun.file(receiptJsonlPath()).text())
        .trim()
        .split("\n")
        .map((line) => JSON.parse(line));
      expect(entries.map((entry) => entry.kind)).toEqual(["require-kms-node", "restore-kms-nodes"]);
      expect(entries[0].details).toEqual({ nodeId: 4, running: [4, 1, 2], stopped: [3] });
    });
  });
});
