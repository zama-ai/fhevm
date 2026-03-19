import { describe, expect, test } from "bun:test";
import path from "node:path";
import { Effect } from "effect";

import {
  listScenarioSummaries,
  parseCoprocessorScenario,
  resolveScenarioReference,
  resolveScenarioFile,
  synthesizeOverrideScenario,
} from "./scenario";

describe("parseCoprocessorScenario", () => {
  test("parses a valid scenario and preserves per-instance overrides", () => {
    const scenario = parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
name: Two Of Two
description: Smallest multi-coprocessor consensus setup.
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: registry
      tag: abc1234
    env:
      OTEL_SERVICE_NAME: coprocessor-1
    args:
      tfhe-worker:
        - --coprocessor-fhe-threads=4
`);
    const resolved = resolveScenarioFile("/tmp/matrix.yml", scenario);
    expect(resolved.instances).toEqual([
      {
        index: 0,
        source: { mode: "inherit" },
        env: {},
        args: {},
      },
      {
        index: 1,
        source: { mode: "registry", tag: "abc1234" },
        env: { OTEL_SERVICE_NAME: "coprocessor-1" },
        args: { "tfhe-worker": ["--coprocessor-fhe-threads=4"] },
      },
    ]);
    expect(resolved.name).toBe("Two Of Two");
    expect(resolved.description).toBe("Smallest multi-coprocessor consensus setup.");
  });

  test("parses localServices for local instances", () => {
    const scenario = parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: local
    localServices:
      - host-listener
`);
    const resolved = resolveScenarioFile("/tmp/matrix.yml", scenario);
    expect(resolved.instances[1]).toEqual({
      index: 1,
      source: { mode: "local" },
      env: {},
      args: {},
      localServices: [
        "coprocessor-host-listener",
        "coprocessor-host-listener-poller",
      ],
    });
  });

  test("rejects duplicate indices and invalid arg targets", () => {
    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 1
instances:
  - index: 0
  - index: 0
`),
    ).toThrow("duplicate instance index 0");

    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 1
  threshold: 1
instances:
  - index: 0
    args:
      not-a-service:
        - --foo
`),
    ).toThrow('unknown arg target "not-a-service"');
  });

  test("rejects malformed arg/env values and oversized topologies", () => {
    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 1
  threshold: 1
instances:
  - index: 0
    args:
      tfhe-worker: --foo
`),
    ).toThrow("instances[0].args.tfhe-worker must be an array");

    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 1
  threshold: 1
instances:
  - index: 0
    env:
      BROKEN:
        nested: true
`),
    ).toThrow("instances[0].env.BROKEN must be a string, number, or boolean");

    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 6
  threshold: 1
`),
    ).toThrow("topology.count must be <= 5");

    expect(() =>
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 1
  threshold: 1
instances:
  - index: 0
    localServices:
      - host-listener
`),
    ).toThrow("instances[0].localServices requires source.mode=local");
  });
});

describe("synthesizeOverrideScenario", () => {
  test("turns --override coprocessor into a one-instance local scenario", () => {
    expect(synthesizeOverrideScenario([{ group: "coprocessor" }])).toEqual({
      version: 1,
      kind: "coprocessor-consensus",
      origin: "override-shorthand",
      name: "Override Shorthand",
      description: "Single local coprocessor instance synthesized from --override coprocessor.",
      topology: { count: 1, threshold: 1 },
      instances: [
        {
          index: 0,
          source: { mode: "local" },
          env: {},
          args: {},
          localServices: undefined,
        },
      ],
    });
  });
});

describe("scenario references", () => {
  test("resolves bundled scenario names by filename stem", async () => {
    const resolved = await Effect.runPromise(resolveScenarioReference("two-of-two"));
    expect(resolved).toBe(path.join(process.cwd(), "scenarios", "two-of-two.yaml"));
  });

  test("rejects --scenario list with a helpful message", async () => {
    const result = await Effect.runPromise(resolveScenarioReference("list").pipe(Effect.either));
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("fhevm-cli scenario list");
    }
  });

  test("lists bundled scenarios with metadata", async () => {
    const scenarios = await Effect.runPromise(listScenarioSummaries());
    expect(scenarios).toContainEqual({
      key: "two-of-two",
      filePath: path.join(process.cwd(), "scenarios", "two-of-two.yaml"),
      name: "Two Of Two",
      description: "Smallest multi-coprocessor consensus setup for drift and quorum testing.",
    });
  });
});
