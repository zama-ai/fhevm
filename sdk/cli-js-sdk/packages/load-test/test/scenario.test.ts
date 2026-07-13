import { describe, expect, it } from "vitest";

import { BUILTIN_SCENARIOS, createBuiltinScenario } from "../src/scenario/builtin";
import {
  plannedRequestCount,
  scenarioSchema,
  shapeDurationSec,
} from "../src/scenario/schema";

describe("builtin scenarios", () => {
  it("all parse against the schema", () => {
    for (const name of BUILTIN_SCENARIOS) {
      const scenario = createBuiltinScenario(name);
      expect(scenario.name.length).toBeGreaterThan(0);
      expect(scenario.flows.length).toBeGreaterThan(0);
    }
  });

  it("steady honors rate and duration overrides", () => {
    const scenario = createBuiltinScenario("open-steady", { rps: 25, durationSec: 60 });
    expect(scenario.name).toBe("open-steady-25");
    expect(scenario.shape).toEqual({ kind: "constant", rps: 25, durationSec: 60 });
    expect(plannedRequestCount(scenario.shape)).toBe(1500);
  });

  it("ramp enables saturation stop", () => {
    const scenario = createBuiltinScenario("open-ramp");
    expect(scenario.saturationStop.enabled).toBe(true);
    expect(scenario.shape.kind).toBe("segments");
  });

  it("closed builtins use the closed model", () => {
    const steady = createBuiltinScenario("closed-steady", {
      vus: 12,
      durationSec: 60,
      thinkTimeMs: 100,
      maxIterations: 500,
    });
    expect(steady.name).toBe("closed-steady-12vu");
    expect(steady.shape).toEqual({
      kind: "closed",
      vus: 12,
      durationSec: 60,
      thinkTimeMs: 100,
      maxIterations: 500,
    });
    expect(plannedRequestCount(steady.shape)).toBe(500);
    expect(shapeDurationSec(steady.shape)).toBe(60);

    const ramp = createBuiltinScenario("closed-ramp", { vus: 2, durationSec: 30 });
    expect(ramp.shape.kind).toBe("closed");
    if (ramp.shape.kind !== "closed") throw new Error("expected closed shape");
    expect(ramp.shape.stages?.map((stage) => stage.vus)).toEqual([2, 4, 6, 8, 10]);
    expect(shapeDurationSec(ramp.shape)).toBe(150);
  });

  it("advertised builtins distinguish open, closed, and drain models", () => {
    for (const name of BUILTIN_SCENARIOS) {
      const scenario = createBuiltinScenario(name);
      if (name.startsWith("open-") || name === "baseline") {
        expect(scenario.shape.kind === "constant" || scenario.shape.kind === "segments").toBe(true);
      } else if (name.startsWith("closed-")) {
        expect(scenario.shape.kind).toBe("closed");
      } else {
        expect(["drain", "smoke"]).toContain(name);
        expect(scenario.shape.kind).toBe("burst");
      }
    }
  });

  it("rejects unknown names with the available list", () => {
    expect(() => createBuiltinScenario("nope")).toThrow(/Built-ins:/);
    expect(() => createBuiltinScenario("steady")).toThrow(/Built-ins:/);
  });
});

describe("scenarioSchema", () => {
  it("applies defaults", () => {
    const scenario = scenarioSchema.parse({
      name: "custom",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: { kind: "constant", rps: 1, durationSec: 10 },
    });
    expect(scenario.requestTimeoutSec).toBe(600);
    expect(scenario.thresholds.maxVerifyFailures).toBe(0);
    expect(scenario.flows[0]?.handlesPerRequest).toBe(1);
    expect(scenario.saturationStop.enabled).toBe(false);
  });

  it("rejects empty flow mixes and bad shapes", () => {
    expect(() =>
      scenarioSchema.parse({ name: "x", flows: [], shape: { kind: "constant", rps: 1, durationSec: 1 } }),
    ).toThrow();
    expect(() =>
      scenarioSchema.parse({
        name: "x",
        flows: [{ flow: "input-proof", weight: 1 }],
        shape: { kind: "constant", rps: -1, durationSec: 1 },
      }),
    ).toThrow();
  });

  it("rejects duplicate flow entries that cannot map to distinct executors", () => {
    const result = scenarioSchema.safeParse({
      name: "duplicate",
      flows: [
        { flow: "public-decrypt", weight: 1, handlesPerRequest: 1 },
        { flow: "public-decrypt", weight: 1, handlesPerRequest: 2 },
      ],
      shape: { kind: "constant", rps: 1, durationSec: 1 },
    });
    expect(result.success).toBe(false);
    if (result.success) throw new Error("Expected duplicate flow validation to fail");
    expect(result.error.issues[0]?.message).toContain(
      'Duplicate flow "public-decrypt"',
    );
  });

  it("rejects scenario names that can escape artifact roots", () => {
    expect(() => scenarioSchema.parse({
      name: "../escape",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: { kind: "burst", count: 1 },
    })).toThrow(/safe slug/);
  });

  it("computes planned counts and durations per shape", () => {
    expect(plannedRequestCount({ kind: "burst", count: 42 })).toBe(42);
    expect(shapeDurationSec({ kind: "burst", count: 42 })).toBeUndefined();
    expect(
      scenarioSchema.parse({
        name: "closed",
        flows: [{ flow: "user-decrypt", weight: 1 }],
        shape: { kind: "closed", vus: 2, durationSec: 10 },
      }).shape,
    ).toMatchObject({ kind: "closed", thinkTimeMs: 0 });
    expect(plannedRequestCount({ kind: "closed", vus: 2, durationSec: 10, thinkTimeMs: 0 })).toBeUndefined();
    expect(
      shapeDurationSec({
        kind: "segments",
        segments: [
          { fromRps: 1, toRps: 1, durationSec: 30 },
          { fromRps: 2, toRps: 2, durationSec: 30 },
        ],
      }),
    ).toBe(60);
  });
});
