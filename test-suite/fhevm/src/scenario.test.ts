import path from "node:path";
import { describe, expect, test } from "bun:test";

import { parseCoprocessorScenario, resolveScenarioReference, synthesizeOverrideScenario, effectiveOverrides } from "./scenario";

describe("scenario", () => {
  test("parses the bundled two-of-two scenario", async () => {
    const file = await resolveScenarioReference("two-of-two");
    expect(path.basename(file)).toBe("two-of-two.yaml");
  });

  test("rejects localServices unless source.mode=local", () => {
    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 1
  threshold: 1
instances:
  - index: 0
    source:
      mode: inherit
    localServices: [host-listener]
`),
    ).toThrow("localServices requires source.mode=local");
  });

  test("synthesizes a local coprocessor scenario from override shorthand", () => {
    const scenario = synthesizeOverrideScenario([
      { group: "coprocessor", services: ["coprocessor-host-listener"] },
    ]);
    expect(scenario?.instances[0]?.source.mode).toBe("local");
    expect(scenario?.instances[0]?.localServices).toEqual(["coprocessor-host-listener"]);
  });

  test("merges scenario-local coprocessor overrides into effective overrides", () => {
    const overrides = effectiveOverrides(
      [{ group: "kms-connector" }],
      {
        instances: [
          { index: 0, source: { mode: "local" }, env: {}, args: {}, localServices: ["coprocessor-host-listener"] },
        ],
      },
    );
    expect(overrides).toContainEqual({
      group: "coprocessor",
      services: ["coprocessor-host-listener"],
    });
  });
});
