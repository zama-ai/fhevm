import path from "node:path";

import { expect, test } from "bun:test";

import { loadRolloutRunbook } from "./commands/rollout-run";
import {
  needsLocalOneNodeMpcNormalization,
  normalizeLocalOneNodeMpcThreshold,
  resolveRolloutTestMode,
  rolloutPhaseTestProfiles,
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

  expect(phaseVersions.contractsV013.GATEWAY_VERSION).toBe("v0.13.0-6");
  expect(phaseVersions.contractsV013.HOST_VERSION).toBe("v0.13.0-6");
});

test("keeps every runtime component on v0.11 while contracts reach v0.13", () => {
  expect(phaseVersions.contractsV013.RELAYER_VERSION).toBe(phaseVersions.baseline.RELAYER_VERSION);
  expect(phaseVersions.contractsV013.CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contractsV013.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.11.0");
  expect(phaseVersions.contractsV013.LISTENER_CORE_VERSION).toBe("v0.11.0");
});

test("jumps each runtime group straight from v0.11 to v0.13 in plan order", () => {
  // relayer
  expect(phaseVersions.relayer.RELAYER_VERSION).toBe("v0.13.0-6");
  expect(phaseVersions.relayer.CORE_VERSION).toBe("v0.13.0");
  // kms (core + connector)
  expect(phaseVersions.kms.CORE_VERSION).toBe("v0.13.20-0");
  expect(phaseVersions.kms.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.13.0-6");
  expect(phaseVersions.kms.LISTENER_CORE_VERSION).toBe("v0.11.0");
  // listener-core moves before the coprocessor image changes
  expect(phaseVersions.listenerCore.LISTENER_CORE_VERSION).toBe("v0.13.0-6");
  expect(phaseVersions.listenerCore.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.11.0");
  // coprocessor last
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.13.0-6");
  expect(phaseVersions.coprocessor.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.13.0-6");
});

test("pins the test harness to the target across every phase", () => {
  for (const phase of Object.values(phaseVersions)) {
    expect(phase.TEST_SUITE_VERSION).toBe("v0.13.0-6");
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

test("defaults to rollout-standard at every phase", () => {
  expect(resolveRolloutTestMode(undefined)).toBe("rollout-standard");
  expect(rolloutPhaseTestProfiles("contracts", "rollout-standard")).toEqual(["rollout-standard"]);
  expect(rolloutPhaseTestProfiles("final", "rollout-standard")).toEqual(["rollout-standard"]);
  expect(() => resolveRolloutTestMode("standard")).toThrow("Unsupported ROLLOUT_TEST_PROFILE=standard");
});

test("loads the checked-in v0.11 to v0.13 runbook", async () => {
  await expect(loadRolloutRunbook(path.join(CLI_DIR, "rollouts/v0.11-to-v0.13/run.ts"))).resolves.toBeFunction();
});
