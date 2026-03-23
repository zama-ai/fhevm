import { describe, expect, test } from "bun:test";

import { previewStateFromBundle, resolveUpgradePlan } from "./flow/up-flow";
import { presetBundle } from "./resolve/target";
import type { State } from "./types";

const defaultScenario: State["scenario"] = {
  version: 1,
  kind: "coprocessor-consensus",
  origin: "default",
  topology: { count: 1, threshold: 1 },
  instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
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

  test("upgrade plan restarts runtime services for a full kms-connector override", () => {
    const plan = resolveUpgradePlan({ overrides: [{ group: "kms-connector" }], scenario: defaultScenario }, "kms-connector");
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
    expect(plan.runtimeServices).toEqual(["kms-connector-gw-listener"]);
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
});
