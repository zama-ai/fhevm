import { describe, expect, test } from "bun:test";
import path from "node:path";

import type { RolloutRunContext } from "./commands/rollout-run";
import { loadRolloutRunbook } from "./commands/rollout-run";
import { resolveKmsTopology } from "./scenario/resolve";
import { from, scenario, to } from "../rollouts/kms-core-progressive/versions";

const RUNBOOK = path.resolve(import.meta.dir, "../rollouts/kms-core-progressive/run.ts");

describe("progressive KMS core rollout", () => {
  test("keeps SDK and connector versions unchanged", () => {
    expect(scenario).toBe("four-party-threshold-kms");
    expect("RELAYER_SDK_VERSION" in from).toBe(false);
    const baseline = from as Record<string, string>;
    expect(Object.entries(to).filter(([key, value]) => baseline[key] !== value)).toEqual([
      ["CORE_VERSION", "v0.13.20"],
    ]);
  });

  test("upgrades every core and requires each upgraded node in a healthy quorum", async () => {
    const calls: string[] = [];
    const context = {
      async writeVersionLock(name: string, options: { versions: Record<string, string> }) {
        calls.push(`lock:${name}:${options.versions.CORE_VERSION}`);
        return `/tmp/${name}.lock.json`;
      },
      async up(options: { lockFile: string; scenario?: string }) {
        calls.push(`up:${options.scenario}:${options.lockFile}`);
      },
      async readState() {
        calls.push("state");
        return { scenario: { kms: resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1 }) } };
      },
      async upgradeKmsNodes(nodeIds: readonly number[], options: { lockFile: string }) {
        calls.push(`upgrade:${nodeIds.join(",")}:${options.lockFile}`);
      },
      async test(profile: string, options?: { grep?: string }) {
        calls.push(`test:${profile}${options?.grep ? `:${options.grep}` : ""}`);
      },
      async withRequiredKmsNode(nodeId: number, task: () => Promise<void>) {
        calls.push(`require:${nodeId}`);
        await task();
      },
    } as unknown as RolloutRunContext;

    await (await loadRolloutRunbook(RUNBOOK))(context);

    expect(calls).toEqual([
      "lock:00-kms-core-baseline:v0.13.10",
      "lock:01-kms-core-target:v0.13.20",
      "up:four-party-threshold-kms:/tmp/00-kms-core-baseline.lock.json",
      "test:rollout-standard",
      "state",
      "upgrade:1:/tmp/01-kms-core-target.lock.json",
      "require:1",
      "test:user-decryption:test user decrypt ebool$",
      "test:rollout-standard",
      "upgrade:2:/tmp/01-kms-core-target.lock.json",
      "require:2",
      "test:user-decryption:test user decrypt ebool$",
      "test:rollout-standard",
      "upgrade:3:/tmp/01-kms-core-target.lock.json",
      "require:3",
      "test:user-decryption:test user decrypt ebool$",
      "test:rollout-standard",
      "upgrade:4:/tmp/01-kms-core-target.lock.json",
      "require:4",
      "test:user-decryption:test user decrypt ebool$",
      "test:rollout-standard",
    ]);
  });
});
