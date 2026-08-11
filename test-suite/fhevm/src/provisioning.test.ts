import { describe, expect, test } from "bun:test";
import path from "node:path";
import YAML from "yaml";

import { COMPONENT_BY_STEP, runsInCompose } from "./layout";
import { STEP_NAMES } from "./types";

const scenario = async (name: string) => {
  const file = path.join(import.meta.dir, "..", "scenarios", `${name}.yaml`);
  return YAML.parse(await Bun.file(file).text()) as {
    hostChains: { key: string; type?: string }[];
  };
};

describe("which nodes compose runs", () => {
  // fhevm-cli owns every node's lifecycle; this only says how it runs one. A Solana host is always
  // a native solana-test-validator, so the answer follows from the chain kind and is derived rather
  // than configured.
  test("every host runs in compose except Solana", () => {
    expect(runsInCompose({ type: "evm" })).toBe(true);
    expect(runsInCompose({})).toBe(true);
    expect(runsInCompose({ type: "solana" })).toBe(false);
  });
});

describe("the host-process pipeline step", () => {
  // Ordering is load-bearing, not cosmetic: the Solana bring-up reads live gateway addresses and
  // the KMS/coprocessor signer set, so it cannot precede the steps that deploy and register them.
  test("runs after the stack is live and before the test-suite", () => {
    expect(STEP_NAMES.indexOf("host-process")).toBeGreaterThan(STEP_NAMES.indexOf("relayer"));
    expect(STEP_NAMES.indexOf("host-process")).toBeLessThan(STEP_NAMES.indexOf("test-suite"));
  });

  test("declares no compose components, because it runs nodes outside compose", () => {
    expect(COMPONENT_BY_STEP["host-process"]).toEqual([]);
  });

  test("every step has a component mapping", () => {
    for (const step of STEP_NAMES) {
      expect(COMPONENT_BY_STEP[step]).toBeDefined();
    }
  });
});

describe("the shipped Solana scenarios", () => {
  // Pins the #1879 retirement: the step picks its work by finding host chains compose does not run,
  // so a Solana host in these scenarios is what makes `up` provision the validator at all. If the
  // `type: solana` entry goes away, clean-e2e.sh's dropped step 3 silently stops happening.
  test.each(["solana", "solana-threshold-kms"])("%s declares a Solana host outside compose", async (name) => {
    const solana = (await scenario(name)).hostChains.find((chain) => chain.type === "solana");
    expect(solana).toBeDefined();
    expect(runsInCompose({ type: "solana" })).toBe(false);
  });

  test("solana keeps its EVM host in compose", async () => {
    const evm = (await scenario("solana")).hostChains.find((chain) => chain.type === undefined);
    expect(evm).toBeDefined();
    expect(runsInCompose(evm as { type?: undefined })).toBe(true);
  });
});
