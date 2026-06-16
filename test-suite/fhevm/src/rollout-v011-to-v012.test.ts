import path from "node:path";

import { expect, test } from "bun:test";

import { loadRolloutRunbook } from "./commands/rollout-run";
import { loadCoprocessorScenario } from "./scenario/resolve";
import { phaseVersions, scenario } from "../rollouts/v0.11-to-v0.12/versions";

const CLI_DIR = path.resolve(import.meta.dir, "..");

test("runs the first staggered hop on a single-coprocessor (threshold-1) topology", async () => {
  expect(scenario).toBe("single");
  const resolved = await loadCoprocessorScenario(scenario);
  expect(resolved.topology.count).toBe(1);
  expect(resolved.topology.threshold).toBe(1);
});

test("starts from the v0.11 mainnet baseline", () => {
  expect(phaseVersions.baseline.GATEWAY_VERSION).toBe("v0.11.0");
  expect(phaseVersions.baseline.HOST_VERSION).toBe("v0.11.0");
  expect(phaseVersions.baseline.CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.baseline.RELAYER_VERSION).toBe("v0.11.1");
});

test("upgrades contracts to v0.12 first, runtime still v0.11", () => {
  expect(phaseVersions.contracts.GATEWAY_VERSION).toBe("v0.12.5");
  expect(phaseVersions.contracts.HOST_VERSION).toBe("v0.12.5");
  // nothing else moves with the contracts
  expect(phaseVersions.contracts.CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contracts.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.11.0");
  expect(phaseVersions.contracts.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.11.0");
});

test("moves kms (core + connector) then listener then coprocessor to v0.12", () => {
  expect(phaseVersions.kms.CORE_VERSION).toBe("v0.13.10");
  expect(phaseVersions.kms.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.12.5");
  expect(phaseVersions.kms.LISTENER_CORE_VERSION).toBe("v0.11.0");
  expect(phaseVersions.listenerCore.LISTENER_CORE_VERSION).toBe("v0.12.5");
  expect(phaseVersions.listenerCore.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.11.0");
  expect(phaseVersions.coprocessor.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.12.5");
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.12.5");
});

test("keeps the relayer on the shared v0.11.x line across every phase", () => {
  for (const phase of Object.values(phaseVersions)) {
    expect(phase.RELAYER_VERSION).toBe("v0.11.1");
    expect(phase.TEST_SUITE_VERSION).toBe("v0.12.5");
    expect(phase.RELAYER_SDK_VERSION).toBe("0.4.2");
  }
});

test("loads the checked-in v0.11 to v0.12 runbook", async () => {
  await expect(loadRolloutRunbook(path.join(CLI_DIR, "rollouts/v0.11-to-v0.12/run.ts"))).resolves.toBeFunction();
});
