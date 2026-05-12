import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { receiptJsonlPath, receiptMarkdownPath } from "./commands/rollout-receipt";
import { type RolloutRunContext, loadRolloutRunbook, runRolloutRunbook } from "./commands/rollout-run";
import { withTempStateDir } from "./test-state";
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
});
