import { describe, expect, test } from "bun:test";

import {
  parseCoprocessorScenario,
  resolveScenarioFile,
  synthesizeOverrideScenario,
} from "./scenario";

describe("parseCoprocessorScenario", () => {
  test("parses a valid scenario and preserves per-instance overrides", () => {
    const scenario = parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
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
  });
});

describe("synthesizeOverrideScenario", () => {
  test("turns --override coprocessor into a one-instance local scenario", () => {
    expect(synthesizeOverrideScenario([{ group: "coprocessor" }])).toEqual({
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
          localServices: undefined,
        },
      ],
    });
  });
});
