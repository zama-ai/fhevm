import path from "node:path";
import { describe, expect, test } from "bun:test";

import {
  assertScenarioOverrideCompatibility,
  loadBlueGreenScenario,
  parseBlueGreenScenario,
  parseCoprocessorScenario,
  resolveBlueGreenScenario,
  resolveScenarioReference,
  synthesizeOverrideScenario,
  effectiveOverrides,
} from "./scenario/resolve";
import { resolveScenarioForOptions } from "./stack-spec/stack-spec";

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

  test("rejects duplicate hostChains chainIds", () => {
    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
hostChains:
  - key: host
    chainId: "12345"
    rpcPort: 8545
  - key: chain-b
    chainId: "12345"
    rpcPort: 8547
topology:
  count: 1
  threshold: 1
`),
    ).toThrow('duplicate hostChains chainId "12345"');
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

  describe("blue-green", () => {
    test("loads the bundled blue-green.yaml", async () => {
      const scenario = await loadBlueGreenScenario("blue-green");
      expect(scenario.kind).toBe("blue-green");
      expect(scenario.name).toBe("Blue-Green Upgrade");
      expect(scenario.bcs.source).toEqual({ mode: "registry", tag: "v0.14.0-1" });
      expect(scenario.gcs.source).toEqual({ mode: "local" });
      expect(scenario.gcs.stackVersion).toBe("0.15.0");
      expect(scenario.hostChains).toHaveLength(1);
      // Default topology = single-operator dev flow.
      expect(scenario.topology).toEqual({ count: 1, threshold: 1 });
    });

    test("parses multi-operator topology", () => {
      const parsed = parseBlueGreenScenario(`
version: 1
kind: blue-green
topology:
  count: 2
  threshold: 2
gcs:
  stackVersion: "0.15.0"
`);
      expect(parsed.topology).toEqual({ count: 2, threshold: 2 });
    });

    test("rejects topology.threshold > count", () => {
      expect(() =>
        parseBlueGreenScenario(`
version: 1
kind: blue-green
topology:
  count: 2
  threshold: 3
gcs:
  stackVersion: "0.15.0"
`),
      ).toThrow("threshold must be between 1 and count");
    });

    test("applies default sources when bcs/gcs source blocks are omitted", () => {
      const parsed = parseBlueGreenScenario(`
version: 1
kind: blue-green
gcs:
  stackVersion: "1.2.3"
`);
      expect(parsed.bcs).toBeUndefined();
      expect(parsed.gcs.source).toBeUndefined();
      expect(parsed.gcs.stackVersion).toBe("1.2.3");
    });

    test("rejects invalid gcs.stackVersion", () => {
      expect(() =>
        parseBlueGreenScenario(`
version: 1
kind: blue-green
gcs:
  stackVersion: "not-a-version"
`),
      ).toThrow("gcs.stackVersion must be a semver-like string");
    });

    test("rejects missing gcs block", () => {
      expect(() =>
        parseBlueGreenScenario(`
version: 1
kind: blue-green
bcs:
  source: { mode: inherit }
`),
      ).toThrow("gcs block is required");
    });

    test("rejects wrong kind", () => {
      expect(() =>
        parseBlueGreenScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 1
  threshold: 1
`),
      ).toThrow("expected kind blue-green");
    });

    test("rejects unknown source.mode", () => {
      expect(() =>
        parseBlueGreenScenario(`
version: 1
kind: blue-green
gcs:
  source: { mode: "sha" }
  stackVersion: "0.15.0"
`),
      ).toThrow("source.mode must be inherit, local, or registry");
    });

    test("registry source requires a tag", () => {
      expect(() =>
        parseBlueGreenScenario(`
version: 1
kind: blue-green
bcs:
  source: { mode: registry }
gcs:
  stackVersion: "0.15.0"
`),
      ).toThrow("tag is required for registry mode");
    });

    test("up-flow loads a blue-green scenario via the kind dispatch", async () => {
      const resolved = await resolveScenarioForOptions({
        scenarioPath: "blue-green",
        overrides: [],
      });
      expect(resolved.kind).toBe("blue-green");
      // Narrow — hitting this branch is what makes downstream typing safe.
      if (resolved.kind === "blue-green") {
        expect(resolved.gcs.stackVersion).toBe("0.15.0");
      }
    });

    test("--bcs-tag overrides bcs.source to registry mode with the given tag", async () => {
      const resolved = await resolveScenarioForOptions({
        scenarioPath: "blue-green",
        overrides: [],
        bcsTag: "v0.14.1-1",
      });
      expect(resolved.kind).toBe("blue-green");
      if (resolved.kind === "blue-green") {
        expect(resolved.bcs.source).toEqual({ mode: "registry", tag: "v0.14.1-1" });
      }
    });

    test("--bcs-tag older than v0.14 is rejected", async () => {
      await expect(
        resolveScenarioForOptions({
          scenarioPath: "blue-green",
          overrides: [],
          bcsTag: "v0.13.1-1",
        }),
      ).rejects.toThrow("predates v0.14");
    });

    test("--override coprocessor is rejected for blue-green scenarios", async () => {
      await expect(
        resolveScenarioForOptions({
          scenarioPath: "blue-green",
          overrides: [{ group: "coprocessor" }],
        }),
      ).rejects.toThrow("not supported with blue-green");
    });

    test("rejects non-local gcs.source at resolve", () => {
      expect(() =>
        resolveBlueGreenScenario(
          "/tmp/bg-registry-gcs.yaml",
          parseBlueGreenScenario(`
version: 1
kind: blue-green
gcs:
  source: { mode: registry, tag: v0.15.0 }
  stackVersion: "0.15.0"
`),
        ),
      ).toThrow("must be local");
    });

    test("rejects more than one host chain", () => {
      expect(() =>
        resolveBlueGreenScenario(
          "/tmp/bg-two-chains.yaml",
          parseBlueGreenScenario(`
version: 1
kind: blue-green
hostChains:
  - key: host
    chainId: "12345"
    rpcPort: 8545
  - key: chain-b
    chainId: "67890"
    rpcPort: 8547
gcs:
  stackVersion: "0.15.0"
`),
        ),
      ).toThrow("exactly one host chain");
    });

    test("--bcs-tag with a 7-char short SHA passes through unchanged", async () => {
      const resolved = await resolveScenarioForOptions({
        scenarioPath: "blue-green",
        overrides: [],
        bcsTag: "1a3646e",
      });
      if (resolved.kind === "blue-green") {
        expect(resolved.bcs.source).toEqual({ mode: "registry", tag: "1a3646e" });
      }
    });

    test("--bcs-tag with a full 40-char SHA auto-shortens to 7 chars", async () => {
      const resolved = await resolveScenarioForOptions({
        scenarioPath: "blue-green",
        overrides: [],
        bcsTag: "1a3646e87b1234567890abcdef1234567890abcd",
      });
      if (resolved.kind === "blue-green") {
        expect(resolved.bcs.source).toEqual({ mode: "registry", tag: "1a3646e" });
      }
    });

    test("--bcs-tag mixed-case SHA is normalized to lower-case short form", async () => {
      const resolved = await resolveScenarioForOptions({
        scenarioPath: "blue-green",
        overrides: [],
        bcsTag: "1A3646E87b1234567890AbCdEf1234567890abcd",
      });
      if (resolved.kind === "blue-green") {
        expect(resolved.bcs.source).toEqual({ mode: "registry", tag: "1a3646e" });
      }
    });

    test("--bcs-tag on a non-blue-green scenario is rejected", async () => {
      await expect(
        resolveScenarioForOptions({
          scenarioPath: "two-of-three",
          overrides: [],
          bcsTag: "v0.13.0",
        }),
      ).rejects.toThrow("--bcs-tag only applies to blue-green scenarios");
    });
  });
});
