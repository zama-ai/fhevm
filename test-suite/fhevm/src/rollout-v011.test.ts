import path from "node:path";

import { expect, test } from "bun:test";

import { loadRolloutRunbook } from "./commands/rollout-run";
import {
  needsLocalOneNodeMpcNormalization,
  normalizeLocalOneNodeMpcThreshold,
  PER_HOP_TEST_PROFILES,
  rolloutPhaseProfiles,
} from "../rollouts/v0.11-to-v0.13/run";
import { phaseVersions, scenario } from "../rollouts/v0.11-to-v0.13/versions";

const CLI_DIR = path.resolve(import.meta.dir, "..");

test("starts from the v0.11 baseline contracts", () => {
  expect(scenario).toBe("two-of-three");
  expect(phaseVersions.baseline.GATEWAY_VERSION).toBe("v0.11.0");
  expect(phaseVersions.baseline.HOST_VERSION).toBe("v0.11.0");
  expect(phaseVersions.baseline.CORE_VERSION).toBe("v0.13.0");
});

test("hops host and gateway contracts through v0.12 before v0.13", () => {
  expect(phaseVersions.contractsV012.GATEWAY_VERSION).toBe("v0.12.5");
  expect(phaseVersions.contractsV012.HOST_VERSION).toBe("v0.12.5");
  // The v0.12 contract waypoint touches nothing else.
  expect(phaseVersions.contractsV012.CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contractsV012.RELAYER_VERSION).toBe(phaseVersions.baseline.RELAYER_VERSION);
  expect(phaseVersions.contractsV012.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.11.0");

  expect(phaseVersions.contractsV013.GATEWAY_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contractsV013.HOST_VERSION).toBe("v0.13.0");
});

test("keeps every runtime component on v0.11 while contracts reach v0.13", () => {
  expect(phaseVersions.contractsV013.RELAYER_VERSION).toBe(phaseVersions.baseline.RELAYER_VERSION);
  expect(phaseVersions.contractsV013.CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contractsV013.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.11.0");
  expect(phaseVersions.contractsV013.LISTENER_CORE_VERSION).toBe("v0.11.0");
});

test("jumps each runtime group straight from v0.11 to v0.13 in plan order", () => {
  // relayer
  expect(phaseVersions.relayer.RELAYER_VERSION).toBe("v0.13.0");
  expect(phaseVersions.relayer.CORE_VERSION).toBe("v0.13.0");
  // kms (core + connector)
  expect(phaseVersions.kms.CORE_VERSION).toBe("v0.13.20");
  expect(phaseVersions.kms.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.13.0");
  expect(phaseVersions.kms.LISTENER_CORE_VERSION).toBe("v0.11.0");
  // listener-core moves before the coprocessor image changes
  expect(phaseVersions.listenerCore.LISTENER_CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.listenerCore.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.11.0");
  // coprocessor last
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.13.0");
  expect(phaseVersions.coprocessor.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.13.0");
});

test("pins the test harness to the target across every phase", () => {
  for (const phase of Object.values(phaseVersions)) {
    expect(phase.TEST_SUITE_VERSION).toBe("v0.13.0");
    expect(phase.RELAYER_SDK_VERSION).toBe("0.4.2");
  }
});

test("reuses the one-node ProtocolConfig mpc threshold normalization", () => {
  const env = {
    MIGRATION_KMS_NODES: JSON.stringify([{ txSenderAddress: "0x1" }]),
    MIGRATION_KMS_THRESHOLDS: JSON.stringify({ publicDecryption: "1", userDecryption: "1", kmsGen: "1", mpc: "0" }),
  };
  expect(needsLocalOneNodeMpcNormalization(env)).toBe(true);
  expect(JSON.parse(normalizeLocalOneNodeMpcThreshold(env).MIGRATION_KMS_THRESHOLDS).mpc).toBe("1");
});

test("gates every hop with the standard suite minus stateful profiles, full standard at the final gate", () => {
  // The per-hop suite drops the slow/stateful and topology-bound profiles...
  for (const profile of ["coprocessor-db-state-revert", "ciphertext-drift-auto-recovery", "multi-chain-isolation"] as const) {
    expect(PER_HOP_TEST_PROFILES).not.toContain(profile);
  }
  // ...but still covers the robustness profiles the thin rollout-standard skipped.
  for (const profile of ["negative-acl", "random-subset", "hcu-block-cap", "paused-host-contracts"] as const) {
    expect(PER_HOP_TEST_PROFILES).toContain(profile);
  }
  for (const phase of ["baseline", "contracts-v012", "contracts-v013", "relayer", "kms", "listener-core"] as const) {
    expect(rolloutPhaseProfiles(phase)).toEqual([...PER_HOP_TEST_PROFILES]);
  }
  // The final gate runs the complete standard aggregate (incl. the stateful
  // profiles, which self-skip when their preconditions are unmet).
  expect(rolloutPhaseProfiles("final")).toEqual(["standard"]);
});

test("loads the checked-in v0.11 to v0.13 runbook", async () => {
  await expect(loadRolloutRunbook(path.join(CLI_DIR, "rollouts/v0.11-to-v0.13/run.ts"))).resolves.toBeFunction();
});
