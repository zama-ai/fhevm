import { describe, expect, it } from "vitest";

import { binomial } from "../src/pool/combinations";
import { createBuiltinScenario } from "../src/scenario/builtin";
import { BUILTIN_SUITES, createBuiltinSuite } from "../src/suite/builtin";
import {
  computeFlowNeeds,
  minHandleCountForCombos,
  requiredDelegationValidUntil,
} from "../src/suite/requirements";
import { suiteSchema } from "../src/suite/schema";

describe("builtin suites", () => {
  it("all parse and reference resolvable scenarios", () => {
    for (const name of BUILTIN_SUITES) {
      const suite = createBuiltinSuite(name);
      expect(suite.entries.length).toBeGreaterThan(0);
      for (const entry of suite.entries) {
        // Throws on unknown built-ins.
        const scenario = createBuiltinScenario(entry.scenario, entry.params);
        expect(scenario.flows.length).toBeGreaterThan(0);
      }
    }
  });

  it("rejects unknown suite names with the available list", () => {
    expect(() => createBuiltinSuite("nope")).toThrow(/Built-ins:/);
  });

  it("applies schema defaults", () => {
    const suite = suiteSchema.parse({
      name: "custom",
      entries: [{ scenario: "open-steady" }],
    });
    expect(suite.pauseSec).toBe(30);
    expect(suite.entries[0]?.params).toEqual({});
  });

  it("rejects suite labels that can escape output and baseline roots", () => {
    expect(() => suiteSchema.parse({
      name: "custom",
      entries: [{ scenario: "open-steady", label: "../../escape" }],
    })).toThrow(/safe slug/);
  });
});

describe("computeFlowNeeds", () => {
  it("sums exact scheduler allocations across scenarios", () => {
    const steady = createBuiltinScenario("open-steady", { rps: 10, durationSec: 100 });
    const needs = computeFlowNeeds([steady, steady]);
    const inputProof = needs.find((need) => need.flow === "input-proof");
    expect(inputProof?.requests).toBe(2000);
  });

  it("tracks public-decrypt needs per combination size", () => {
    const mixed = createBuiltinScenario("open-mixed", { rps: 10, durationSec: 100 });
    const needs = computeFlowNeeds([mixed]);
    const publicDecrypt = needs.find((need) => need.flow === "public-decrypt");
    expect(publicDecrypt).toBeDefined();
    expect([...(publicDecrypt?.byHandleCount.keys() ?? [])]).toEqual([2]);
  });

  it("allows unlimited closed reusable decrypt scenarios", () => {
    const closed = createBuiltinScenario("closed-steady", { vus: 4, durationSec: 60 });
    const needs = computeFlowNeeds([closed]);
    expect(needs).toEqual([
      {
        flow: "user-decrypt",
        requests: 1,
        byHandleCount: new Map(),
      },
    ]);
  });

  it("rejects unlimited closed single-use scenarios", () => {
    const closed = createBuiltinScenario("closed-steady", {
      flow: "input-proof",
      vus: 4,
      durationSec: 60,
    });
    expect(() => computeFlowNeeds([closed])).toThrow(/single-use pools require a finite request cap/);
  });
});

describe("requiredDelegationValidUntil", () => {
  it("covers every run, drain/request window, pause, and safety margin", () => {
    const first = createBuiltinScenario("open-steady", { rps: 1, durationSec: 10 });
    const second = createBuiltinScenario("open-steady", { rps: 1, durationSec: 20 });
    const expectedSeconds =
      10 + Math.max(first.drainTimeoutSec, first.requestTimeoutSec) +
      20 + Math.max(second.drainTimeoutSec, second.requestTimeoutSec) +
      30 + 120;
    expect(requiredDelegationValidUntil([first, second], {
      pauseSec: 30,
      nowSeconds: 1_000n,
    })).toBe(1_000n + BigInt(expectedSeconds));
  });
});

describe("minHandleCountForCombos", () => {
  it("returns current size when the space already suffices", () => {
    expect(
      minHandleCountForCombos(10, new Map([[2, { needed: 45, consumedRanks: 0n }]])),
    ).toBe(10);
  });

  it("grows the pool past consumed ranks", () => {
    // C(10,2)=45; 40 consumed, 20 needed -> need C(n,2) >= 60 -> n=12 (C(12,2)=66).
    const n = minHandleCountForCombos(10, new Map([[2, { needed: 20, consumedRanks: 40n }]]));
    expect(n).toBe(12);
    expect(binomial(n, 2) - 40n).toBeGreaterThanOrEqual(20n);
  });

  it("satisfies multiple combination sizes simultaneously", () => {
    const n = minHandleCountForCombos(
      0,
      new Map([
        [1, { needed: 10, consumedRanks: 0n }],
        [3, { needed: 200, consumedRanks: 0n }],
      ]),
    );
    expect(binomial(n, 1)).toBeGreaterThanOrEqual(10n);
    expect(binomial(n, 3)).toBeGreaterThanOrEqual(200n);
    // Minimality: one less fails at least one constraint.
    expect(
      binomial(n - 1, 1) < 10n || binomial(n - 1, 3) < 200n,
    ).toBe(true);
  });
});
