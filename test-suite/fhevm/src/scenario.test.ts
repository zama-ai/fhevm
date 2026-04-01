import path from "node:path";
import { describe, expect, test } from "bun:test";

import {
  assertScenarioOverrideCompatibility,
  parseCoprocessorScenario,
  resolveScenarioReference,
  synthesizeOverrideScenario,
  effectiveOverrides,
} from "./scenario/resolve";

describe("scenario", () => {
  test("parses the bundled two-of-two scenario", async () => {
    const file = await resolveScenarioReference("two-of-two");
    expect(path.basename(file)).toBe("two-of-two.yaml");
  });

  test("parses bundled scenarios by filename form", async () => {
    const file = await resolveScenarioReference("two-of-two.yaml");
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

  test("allows any valid first hostChains key", () => {
    expect(
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
hostChains:
  - key: chain-b
    chainId: "67890"
    rpcPort: 9651
  - key: host
    chainId: "12345"
    rpcPort: 8545
topology:
  count: 2
  threshold: 2
`),
    ).toMatchObject({
      hostChains: [
        { key: "chain-b", chainId: "67890", rpcPort: 9651 },
        { key: "host", chainId: "12345", rpcPort: 8545 },
      ],
    });
  });

  test("rejects empty hostChains arrays", () => {
    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
hostChains: []
topology:
  count: 1
  threshold: 1
`),
    ).toThrow("hostChains must not be empty");
  });

  test("rejects duplicate hostChains rpcPorts", () => {
    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
hostChains:
  - key: host
    chainId: "12345"
    rpcPort: 8545
  - key: chain-b
    chainId: "67890"
    rpcPort: 8545
topology:
  count: 1
  threshold: 1
`),
    ).toThrow('duplicate hostChains rpcPort "8545"');
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

  test("allows coprocessor overrides with topology-only scenarios", () => {
    expect(() =>
      assertScenarioOverrideCompatibility(
        {
          instances: [
            { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
            { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
          ],
        },
        [{ group: "coprocessor" }],
      ),
    ).not.toThrow();
  });

  test("rejects coprocessor overrides when the scenario explicitly owns coprocessor source", () => {
    expect(() =>
      assertScenarioOverrideCompatibility(
        {
          sourcePath: "/tmp/explicit-source.yaml",
          instances: [
            { index: 0, source: { mode: "registry", tag: "abcdef0" }, env: {}, args: {} },
          ],
        },
        [{ group: "coprocessor", services: ["coprocessor-host-listener"] }],
      ),
    ).toThrow("conflicts with scenario-defined coprocessor source");
  });
});
