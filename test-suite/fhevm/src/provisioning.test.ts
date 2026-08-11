import { describe, expect, test } from "bun:test";
import path from "node:path";
import YAML from "yaml";

import { COMPONENT_BY_STEP, isCliManagedHostProcess, resolveNodeProvisioning, runsInCompose } from "./layout";
import { HOST_CHAIN_NODE_PROVISIONING, STEP_NAMES } from "./types";

const scenario = async (name: string) => {
  const file = path.join(import.meta.dir, "..", "scenarios", `${name}.yaml`);
  return YAML.parse(await Bun.file(file).text()) as {
    hostChains: { key: string; type?: string; nodeProvisioning?: string }[];
  };
};

describe("host node provisioning", () => {
  test("defaults by chain kind: solana runs as a host process, evm in compose", () => {
    expect(resolveNodeProvisioning({ type: "solana" })).toBe("host-process");
    expect(resolveNodeProvisioning({ type: "evm" })).toBe("container");
    expect(resolveNodeProvisioning({})).toBe("container");
  });

  test("an explicit nodeProvisioning beats the per-kind default", () => {
    expect(resolveNodeProvisioning({ type: "solana", nodeProvisioning: "external" })).toBe("external");
    expect(resolveNodeProvisioning({ type: "evm", nodeProvisioning: "host-process" })).toBe("host-process");
  });

  // The reason `runsInCompose` exists rather than `!isExternalNode`: a host-process node is owned
  // by fhevm-cli but has no compose service, so "not external" is not the same question as "does
  // compose run it". Getting this wrong generates a compose service for the validator.
  test("only container nodes run in compose", () => {
    expect(runsInCompose({ nodeProvisioning: "container" })).toBe(true);
    expect(runsInCompose({ nodeProvisioning: "host-process" })).toBe(false);
    expect(runsInCompose({ nodeProvisioning: "external" })).toBe(false);
    expect(runsInCompose({ type: "solana" })).toBe(false);
  });

  test("only host-process nodes are fhevm-cli-managed processes", () => {
    expect(isCliManagedHostProcess({ nodeProvisioning: "host-process" })).toBe(true);
    expect(isCliManagedHostProcess({ nodeProvisioning: "container" })).toBe(false);
    expect(isCliManagedHostProcess({ nodeProvisioning: "external" })).toBe(false);
  });

  test("compose ownership and cli-process ownership are mutually exclusive", () => {
    for (const nodeProvisioning of HOST_CHAIN_NODE_PROVISIONING) {
      expect(runsInCompose({ nodeProvisioning }) && isCliManagedHostProcess({ nodeProvisioning })).toBe(false);
    }
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
  // Pins the #1879 retirement itself: these declare host-process, so `fhevm-cli up` owns the
  // validator and program deploy. If either reverts to `external`, clean-e2e.sh's dropped step 3
  // silently stops happening and the suite runs against no Solana node at all.
  test.each(["solana", "solana-threshold-kms"])("%s declares its Solana host as host-process", async (name) => {
    const chains = (await scenario(name)).hostChains;
    const solana = chains.find((chain) => chain.type === "solana");
    expect(solana).toBeDefined();
    expect(solana?.nodeProvisioning).toBe("host-process");
  });

  test("solana keeps its EVM host in compose", async () => {
    const evm = (await scenario("solana")).hostChains.find((chain) => chain.type === undefined);
    expect(evm).toBeDefined();
    expect(runsInCompose(evm as { nodeProvisioning?: undefined })).toBe(true);
  });
});
