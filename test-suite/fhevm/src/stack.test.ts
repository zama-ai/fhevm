import { describe, expect, test } from "bun:test";

import { resolveUpgradePlan, previewStateFromBundle } from "./stack";
import { presetBundle } from "./resolve";
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

  test("upgrade plan includes migrations for a full kms-connector override", () => {
    const plan = resolveUpgradePlan({ overrides: [{ group: "kms-connector" }], scenario: defaultScenario }, "kms-connector");
    expect(plan.migrationServices).toEqual(["kms-connector-db-migration"]);
    expect(plan.runtimeServices).toEqual([
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ]);
  });

  test("upgrade rejects schema-coupled partial runtime overrides without migrations", () => {
    expect(() =>
      resolveUpgradePlan(
        {
          overrides: [{ group: "kms-connector", services: ["kms-connector-gw-listener"] }],
          scenario: defaultScenario,
        },
        "kms-connector",
      ),
    ).toThrow("upgrade for kms-connector requires a full-group local override so DB migrations can run");
  });
});
