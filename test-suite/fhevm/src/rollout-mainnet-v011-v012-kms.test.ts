import { describe, expect, test } from "bun:test";
import path from "node:path";

import { relayerSdkV042DecryptionError } from "../rollouts/mainnet-v0.11-to-v0.12-kms/run";
import { from, scenario, to } from "../rollouts/mainnet-v0.11-to-v0.12-kms/versions";
import type { RolloutRunContext } from "./commands/rollout-run";
import { loadRolloutRunbook } from "./commands/rollout-run";
import { resolveKmsTopology } from "./scenario/resolve";

const RUNBOOK = path.resolve(import.meta.dir, "../rollouts/mainnet-v0.11-to-v0.12-kms/run.ts");

describe("mainnet v0.11 to v0.12 KMS rollout", () => {
  test("pins the incident versions and changes only the KMS core", () => {
    expect(scenario).toBe("four-party-threshold-kms");
    expect(from).toMatchObject({
      RELAYER_VERSION: "v0.11.1",
      RELAYER_MIGRATE_VERSION: "v0.11.0",
      GATEWAY_VERSION: "v0.12.1",
      HOST_VERSION: "v0.12.1",
      CORE_VERSION: "v0.13.3",
      CONNECTOR_KMS_WORKER_VERSION: "v0.12.0",
      COPROCESSOR_TFHE_WORKER_VERSION: "v0.11.0",
      RELAYER_SDK_VERSION: "0.4.2",
    });
    expect(Object.entries(to).filter(([key, value]) => from[key as keyof typeof from] !== value)).toEqual([
      ["CORE_VERSION", "v0.13.10"],
    ]);
  });

  test("reproduces the incident at the two-old two-new boundary", async () => {
    const calls: string[] = [];
    const context = {
      async writeVersionLock(name: string, options: { versions: Record<string, string> }) {
        calls.push(`lock:${name}:${options.versions.CORE_VERSION}`);
        return `/tmp/${name}.lock.json`;
      },
      async up(options: { overrides?: Array<{ group: string }>; scenario?: string }) {
        calls.push(`up:${options.scenario}:${options.overrides?.map((override) => override.group).join(",")}`);
      },
      async readState() {
        return { scenario: { kms: resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1 }) } };
      },
      async upgradeKmsNodes(nodeIds: readonly number[], options: { lockFile: string }) {
        calls.push(`upgrade:${nodeIds.join(",")}:${options.lockFile}`);
      },
      async test(profile: string) {
        calls.push(`test:${profile}`);
      },
      async expectTestFailure(profile: string, options: { errorIncludes: string; grep?: string }) {
        calls.push(`expected-failure:${profile}:${options.grep}:${options.errorIncludes}`);
      },
    } as unknown as RolloutRunContext;

    await (await loadRolloutRunbook(RUNBOOK))(context);

    expect(calls).toEqual([
      "lock:00-kms-core-baseline:v0.13.3",
      "lock:01-kms-core-target:v0.13.10",
      "up:four-party-threshold-kms:test-suite",
      "test:rollout-standard",
      "upgrade:1,2:/tmp/01-kms-core-target.lock.json",
      `expected-failure:user-decryption:test user decrypt ebool$:${relayerSdkV042DecryptionError}`,
      "upgrade:3,4:/tmp/01-kms-core-target.lock.json",
      "test:rollout-standard",
    ]);
  });
});
