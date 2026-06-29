import path from "node:path";

import { expect, test } from "bun:test";

import { loadRolloutRunbook } from "./commands/rollout-run";
import { loadCoprocessorScenario } from "./scenario/resolve";
import { needsLocalOneNodeMpcNormalization } from "../rollouts/v0.13.0-testnet/run";
import { phaseVersions, scenario } from "../rollouts/v0.13.0-testnet/versions";

const CLI_DIR = path.resolve(import.meta.dir, "..");

test("models testnet on a single-coprocessor, two-host-chain topology (Polygon stand-in)", async () => {
  expect(scenario).toBe("multi-chain");
  const resolved = await loadCoprocessorScenario(scenario);
  expect(resolved.topology.count).toBe(1);
  expect(resolved.topology.threshold).toBe(1);
  expect(resolved.hostChains?.length).toBe(2);
});

test("pins the EXACT current testnet baseline (heterogeneous from-tags)", () => {
  expect(phaseVersions.baseline.GATEWAY_VERSION).toBe("v0.12.1");
  expect(phaseVersions.baseline.HOST_VERSION).toBe("v0.12.1");
  expect(phaseVersions.baseline.CORE_VERSION).toBe("v0.13.10"); // KMS already promoted on testnet
  expect(phaseVersions.baseline.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.12.0");
  expect(phaseVersions.baseline.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.12.3");
  expect(phaseVersions.baseline.COPROCESSOR_SNS_WORKER_VERSION).toBe("v0.12.0");
  expect(phaseVersions.baseline.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.12.2");
  expect(phaseVersions.baseline.RELAYER_VERSION).toBe("v0.11.1");
});

test("contracts go to v0.13.0 first; runtime stays at the testnet baseline", () => {
  expect(phaseVersions.contracts.GATEWAY_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contracts.HOST_VERSION).toBe("v0.13.0");
  expect(phaseVersions.contracts.CORE_VERSION).toBe("v0.13.10");
  expect(phaseVersions.contracts.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.12.0");
  expect(phaseVersions.contracts.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.12.3");
});

test("kms then coprocessor then relayer reach the v0.13.0 target (KMS core v0.13.20)", () => {
  expect(phaseVersions.kms.CORE_VERSION).toBe("v0.13.20");
  expect(phaseVersions.kms.CONNECTOR_KMS_WORKER_VERSION).toBe("v0.13.0");
  expect(phaseVersions.kms.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.12.3"); // not yet
  expect(phaseVersions.coprocessor.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.13.0");
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).toBe("v0.13.0");
  expect(phaseVersions.coprocessor.RELAYER_VERSION).toBe("v0.11.1"); // relayer last
  expect(phaseVersions.relayer.RELAYER_VERSION).toBe("v0.13.0");
});

test("pins the target test harness across every phase", () => {
  for (const phase of Object.values(phaseVersions)) {
    expect(phase.TEST_SUITE_VERSION).toBe("v0.13.0");
    expect(phase.RELAYER_SDK_VERSION).toBe("0.4.2");
  }
});

test("reuses the one-node ProtocolConfig mpc-threshold normalization", () => {
  const env = {
    MIGRATION_KMS_NODES: JSON.stringify([{ txSenderAddress: "0x1" }]),
    MIGRATION_KMS_THRESHOLDS: JSON.stringify({ mpc: "0" }),
  };
  expect(needsLocalOneNodeMpcNormalization(env)).toBe(true);
});

test("loads the checked-in v0.13.0-testnet runbook", async () => {
  await expect(loadRolloutRunbook(path.join(CLI_DIR, "rollouts/v0.13.0-testnet/run.ts"))).resolves.toBeFunction();
});
