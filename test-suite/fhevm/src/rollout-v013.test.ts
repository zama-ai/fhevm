import { expect, test } from "bun:test";

import {
  normalizeLocalOneNodeMpcThreshold,
  resolveRolloutTestMode,
  rolloutPhaseTestProfiles,
} from "../rollouts/v0.12-to-v0.13/run";
import { phaseVersions, scenario } from "../rollouts/v0.12-to-v0.13/versions";

test("starts v0.12 without new v0.13 host chains", () => {
  expect(scenario).toBe("two-of-three");
});

test("normalizes only the local one-node ProtocolConfig mpc migration threshold", () => {
  const env = normalizeLocalOneNodeMpcThreshold({
    MIGRATION_KMS_NODES: JSON.stringify([{ txSenderAddress: "0x1" }]),
    MIGRATION_KMS_THRESHOLDS: JSON.stringify({ publicDecryption: "1", userDecryption: "1", kmsGen: "1", mpc: "0" }),
  });

  expect(JSON.parse(env.MIGRATION_KMS_THRESHOLDS).mpc).toBe("1");
});

test("keeps production-like mpc migration thresholds unchanged", () => {
  const env = {
    MIGRATION_KMS_NODES: JSON.stringify(Array.from({ length: 13 }, (_, index) => ({ txSenderAddress: String(index) }))),
    MIGRATION_KMS_THRESHOLDS: JSON.stringify({ publicDecryption: "7", userDecryption: "9", kmsGen: "7", mpc: "4" }),
  };

  expect(normalizeLocalOneNodeMpcThreshold(env)).toBe(env);
});

test("splits listener-core from coprocessor rollout image changes", () => {
  expect(phaseVersions.listenerCore.LISTENER_CORE_VERSION).toBe(phaseVersions.coprocessor.LISTENER_CORE_VERSION);
  expect(phaseVersions.listenerCore.COPROCESSOR_DB_MIGRATION_VERSION).toBe(
    phaseVersions.kms.COPROCESSOR_DB_MIGRATION_VERSION,
  );
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).not.toBe(
    phaseVersions.kms.COPROCESSOR_DB_MIGRATION_VERSION,
  );
});

test("uses rollout-standard at every phase by default", () => {
  expect(resolveRolloutTestMode(undefined)).toBe("rollout-standard");
  expect(rolloutPhaseTestProfiles("contracts", "rollout-standard")).toEqual(["rollout-standard"]);
  expect(rolloutPhaseTestProfiles("final", "rollout-standard")).toEqual(["rollout-standard"]);
});

test("adds sequential heavy checkpoint coverage to the rollout runbook", () => {
  expect(rolloutPhaseTestProfiles("contracts", "rollout-heavy")).toEqual([
    "rollout-standard",
    "operators",
    "random-subset",
    "hcu-block-cap",
  ]);
  expect(rolloutPhaseTestProfiles("final", "rollout-heavy")).toEqual([
    "rollout-standard",
    "operators",
    "random-subset",
    "negative-acl",
    "public-decryption",
    "hcu-block-cap",
    "coprocessor-db-state-revert",
    "rollout-standard",
    "ciphertext-drift-auto-recovery",
    "rollout-standard",
  ]);
});

test("rejects unsupported rollout test modes", () => {
  expect(() => resolveRolloutTestMode("standard")).toThrow("Unsupported ROLLOUT_TEST_PROFILE=standard");
});
