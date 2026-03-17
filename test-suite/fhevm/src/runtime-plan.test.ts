import { describe, expect, test } from "bun:test";
import { Effect } from "effect";

import {
  resolveScenarioForOptions,
  runtimePlanFromResolved,
  topologyFromScenario,
} from "./runtime-plan";

describe("runtime-plan", () => {
  test("topologyFromScenario derives topology from the resolved scenario", () => {
    expect(
      topologyFromScenario(
        {
          version: 1,
          kind: "coprocessor-consensus",
          origin: "file",
          topology: { count: 2, threshold: 2 },
          instances: [
            {
              index: 0,
              source: { mode: "inherit" },
              env: {},
              args: {},
            },
            {
              index: 1,
              source: { mode: "local" },
              env: { OTEL_SERVICE_NAME: "coprocessor-1" },
              args: { "*": ["--verbose"] },
            },
          ],
        },
      ),
    ).toEqual({
      count: 2,
      threshold: 2,
    });
  });

  test("resolveScenarioForOptions maps override shorthand to a scenario", async () => {
    const scenario = await Effect.runPromise(
      resolveScenarioForOptions({
        overrides: [{ group: "coprocessor" }],
        scenarioPath: undefined,
      }),
    );
    expect(scenario?.origin).toBe("override-shorthand");
    expect(scenario?.instances[0]?.source).toEqual({ mode: "local" });
  });

  test("runtimePlanFromResolved centralizes the final coprocessor plan", () => {
    const plan = runtimePlanFromResolved({
      requiresGitHub: false,
      target: "latest-release",
      versions: {
        target: "latest-release",
        lockName: "test.json",
        env: {},
        sources: [],
      },
      overrides: [{ group: "coprocessor" }],
      scenario: {
        version: 1,
        kind: "coprocessor-consensus",
        origin: "override-shorthand",
        topology: { count: 1, threshold: 1 },
        instances: [
          {
            index: 0,
            source: { mode: "local" },
            env: {},
            args: {},
          },
        ],
      },
    });
    expect(plan.requiresGitHub).toBe(false);
    expect(plan.coprocessor.instances[0]?.source).toEqual({ mode: "local" });
    expect(plan.topology.count).toBe(1);
  });
});
