import { describe, expect, test } from "bun:test";
import path from "node:path";
import YAML from "yaml";

import { COMPONENT_BY_STEP, isEvmHost } from "./layout";
import { STEP_NAMES } from "./types";

const scenario = async (name: string) => {
  const file = path.join(import.meta.dir, "..", "scenarios", `${name}.yaml`);
  return YAML.parse(await Bun.file(file).text()) as {
    hostChains: { key: string; type?: string }[];
  };
};

describe("which hosts carry the EVM contract pipeline", () => {
  // What this gates is the Solidity machinery — compose services, contract artifacts, seeding —
  // not how a node is launched. Both are one boolean today only because every EVM host runs in
  // compose and the sole non-EVM host does not; keep them distinguishable in the reading.
  test("EVM hosts do, including when the scenario omits the type", () => {
    expect(isEvmHost({ type: "evm" })).toBe(true);
    expect(isEvmHost({})).toBe(true);
    expect(isEvmHost({ type: "solana" })).toBe(false);
  });
});

describe("the host-process pipeline step", () => {
  // Ordering is load-bearing, not cosmetic: the Solana bring-up reads live gateway addresses and
  // the KMS/coprocessor signer set, so it cannot precede the steps that deploy and register them.
  test("runs after the stack is live and before the test-suite", () => {
    expect(STEP_NAMES.indexOf("host-process")).toBeGreaterThan(STEP_NAMES.indexOf("relayer"));
    expect(STEP_NAMES.indexOf("host-process")).toBeLessThan(STEP_NAMES.indexOf("test-suite"));
  });

  test("declares no compose components, because fhevm-cli spawns these nodes itself", () => {
    expect(COMPONENT_BY_STEP["host-process"]).toEqual([]);
  });

  test("every step has a component mapping", () => {
    for (const step of STEP_NAMES) {
      expect(COMPONENT_BY_STEP[step]).toBeDefined();
    }
  });
});

describe("the shipped Solana scenarios", () => {
  // Pins the #1879 retirement: the step picks its work by chain kind, so a `type: solana` host in
  // these scenarios is what makes `up` provision the validator at all. If that entry goes away,
  // clean-e2e.sh's dropped step 3 silently stops happening.
  test.each(["solana", "solana-threshold-kms"])("%s declares a Solana host", async (name) => {
    const solana = (await scenario(name)).hostChains.find((chain) => chain.type === "solana");
    expect(solana).toBeDefined();
    expect(isEvmHost({ type: "solana" })).toBe(false);
  });

  test("solana keeps an EVM host alongside it", async () => {
    const evm = (await scenario("solana")).hostChains.find((chain) => chain.type === undefined);
    expect(evm).toBeDefined();
    expect(isEvmHost(evm as { type?: undefined })).toBe(true);
  });
});
