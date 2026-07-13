import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { createBuiltinScenario } from "../src/scenario/builtin";
import { loadScenario } from "../src/scenario/load";
import { plannedRequestCount } from "../src/scenario/schema";
import { suiteSchema } from "../src/suite/schema";

let dir: string;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-scenario-overrides-"));
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

const writeScenario = async (value: unknown): Promise<string> => {
  const path = join(dir, "scenario.json");
  await writeFile(path, JSON.stringify(value));
  return path;
};

describe("shared scenario overrides", () => {
  it("preserves the canonical mixed workload before applying overrides", () => {
    const mixed = createBuiltinScenario("open-mixed");
    expect(mixed.flows).toEqual([
      { flow: "input-proof", weight: 6, handlesPerRequest: 1 },
      { flow: "user-decrypt", weight: 3, handlesPerRequest: 1 },
      { flow: "public-decrypt", weight: 1, handlesPerRequest: 2 },
    ]);
    expect(mixed.shape).toEqual({ kind: "constant", rps: 10, durationSec: 600 });
  });

  it("applies standalone overrides to custom JSON before planning", async () => {
    const path = await writeScenario({
      name: "custom-open",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: { kind: "constant", rps: 1, durationSec: 10 },
    });
    const resolved = await loadScenario(path, {
      rps: 4,
      durationSec: 5,
      flow: "public-decrypt",
    });
    expect(resolved.flows[0]?.flow).toBe("public-decrypt");
    expect(resolved.shape).toEqual({ kind: "constant", rps: 4, durationSec: 5 });
    expect(plannedRequestCount(resolved.shape)).toBe(20);
  });

  it("uses the same override schema for suite entries and custom JSON", async () => {
    const path = await writeScenario({
      name: "custom-closed",
      flows: [{ flow: "user-decrypt", weight: 1 }],
      shape: { kind: "closed", vus: 2, durationSec: 30 },
    });
    const suite = suiteSchema.parse({
      name: "custom-suite",
      entries: [{
        scenario: path,
        params: {
          vus: 6,
          durationSec: 20,
          thinkTimeMs: 100,
          maxIterations: 50,
          flow: "delegated-user-decrypt",
        },
      }],
    });
    const entry = suite.entries[0]!;
    const resolved = await loadScenario(entry.scenario, entry.params);
    expect(resolved.flows[0]?.flow).toBe("delegated-user-decrypt");
    expect(resolved.shape).toEqual({
      kind: "closed",
      vus: 6,
      durationSec: 20,
      thinkTimeMs: 100,
      maxIterations: 50,
    });
  });

  it("scales segmented rates and stages from their first configured level", async () => {
    const openPath = await writeScenario({
      name: "custom-ramp",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: {
        kind: "segments",
        segments: [
          { fromRps: 2, toRps: 2, durationSec: 10 },
          { fromRps: 4, toRps: 4, durationSec: 10 },
        ],
      },
    });
    const open = await loadScenario(openPath, { rps: 5, durationSec: 3 });
    expect(open.shape).toEqual({
      kind: "segments",
      segments: [
        { fromRps: 5, toRps: 5, durationSec: 3 },
        { fromRps: 10, toRps: 10, durationSec: 3 },
      ],
    });

    const closed = createBuiltinScenario("closed-ramp", { vus: 3, durationSec: 7 });
    expect(closed.shape.kind).toBe("closed");
    if (closed.shape.kind !== "closed") throw new Error("Expected closed shape.");
    expect(closed.shape.stages?.map((stage) => stage.vus)).toEqual([3, 6, 9, 12, 15, 18]);
    expect(closed.shape.stages?.every((stage) => stage.durationSec === 7)).toBe(true);
  });

  it("rejects model-mismatched and ambiguous flow overrides", async () => {
    const path = await writeScenario({
      name: "custom-open",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: { kind: "constant", rps: 1, durationSec: 10 },
    });
    await expect(loadScenario(path, { vus: 2 }))
      .rejects.toThrow(/constant shape; unsupported override\(s\): vus/);
    expect(() => createBuiltinScenario("drain", { durationSec: 10 }))
      .toThrow(/burst shape; unsupported override\(s\): durationSec/);
    expect(() => createBuiltinScenario("baseline", { flow: "input-proof" }))
      .toThrow(/--flow can only override a single-flow scenario/);
  });

  it("rejects unknown suite override keys instead of dropping them", () => {
    expect(() => suiteSchema.parse({
      name: "bad-suite",
      entries: [{ scenario: "open-steady", params: { requests: 10 } }],
    })).toThrow();
  });
});
