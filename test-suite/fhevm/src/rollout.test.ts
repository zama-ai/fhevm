import path from "node:path";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { afterEach, describe, expect, test } from "bun:test";

import { PreflightError } from "./errors";
import { generateRolloutLocks, readCompatTest, rollout, validateCompatSteps } from "./commands/rollout";
import { readJson, remove, writeJson } from "./utils/fs";
import type { VersionBundle } from "./types";
import type { CompatTestDefinition } from "./commands/rollout";

const tempDirs: string[] = [];
const versionKeys = [
  "GATEWAY_VERSION",
  "HOST_VERSION",
  "COPROCESSOR_DB_MIGRATION_VERSION",
  "COPROCESSOR_HOST_LISTENER_VERSION",
  "COPROCESSOR_GW_LISTENER_VERSION",
  "COPROCESSOR_TX_SENDER_VERSION",
  "COPROCESSOR_TFHE_WORKER_VERSION",
  "COPROCESSOR_ZKPROOF_WORKER_VERSION",
  "COPROCESSOR_SNS_WORKER_VERSION",
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
  "CORE_VERSION",
  "RELAYER_VERSION",
  "RELAYER_MIGRATE_VERSION",
  "TEST_SUITE_VERSION",
];

const envMap = (suffix: string) => Object.fromEntries(versionKeys.map((key) => [key, `${suffix}-${key.toLowerCase()}`]));
const compatTest = (): CompatTestDefinition => ({
  name: "v0.11-to-v0.12-upgrade",
  from: envMap("from"),
  to: envMap("to"),
  steps: [
    ["RELAYER"],
    ["GATEWAY_CONTRACTS", "HOST_CONTRACTS"],
    ["KMS_CORE", "KMS_CONNECTOR"],
    ["COPROCESSOR"],
    ["RELAYER_SDK"],
  ],
  units: {
    RELAYER: ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"],
    GATEWAY_CONTRACTS: ["GATEWAY_VERSION"],
    HOST_CONTRACTS: ["HOST_VERSION"],
    KMS_CORE: ["CORE_VERSION"],
    KMS_CONNECTOR: [
      "CONNECTOR_DB_MIGRATION_VERSION",
      "CONNECTOR_GW_LISTENER_VERSION",
      "CONNECTOR_KMS_WORKER_VERSION",
      "CONNECTOR_TX_SENDER_VERSION",
    ],
    COPROCESSOR: [
      "COPROCESSOR_DB_MIGRATION_VERSION",
      "COPROCESSOR_HOST_LISTENER_VERSION",
      "COPROCESSOR_GW_LISTENER_VERSION",
      "COPROCESSOR_TX_SENDER_VERSION",
      "COPROCESSOR_TFHE_WORKER_VERSION",
      "COPROCESSOR_ZKPROOF_WORKER_VERSION",
      "COPROCESSOR_SNS_WORKER_VERSION",
    ],
    RELAYER_SDK: ["TEST_SUITE_VERSION"],
  },
  execution: {
    scenario: "two-of-two",
    testProfile: "standard",
  },
});

afterEach(async () => {
  await Promise.all(tempDirs.splice(0).map((dir) => remove(dir)));
});

describe("rollout", () => {
  test("requires each rollout unit exactly once across steps", () => {
    const { units } = compatTest();
    expect(() => validateCompatSteps([["RELAYER"], ["COPROCESSOR"]], units)).toThrow(PreflightError);
    expect(() => validateCompatSteps([["RELAYER"], ["KMS_CORE", "KMS_CORE"]], units)).toThrow(PreflightError);
    expect(() => validateCompatSteps([["RELAYER"], ["BOGUS" as never]], units)).toThrow(PreflightError);
  });

  test("rejects compat-tests that reference unknown or duplicated version keys", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const testDef = compatTest();
    await writeJson(file, { ...testDef, units: { ...testDef.units, KMS_CORE: ["CORE_VERSION", "BOGUS"] } });
    await expect(readCompatTest(file)).rejects.toThrow("references unknown version key BOGUS");
    await writeJson(file, { ...testDef, units: { ...testDef.units, RELAYER_SDK: ["RELAYER_VERSION"] } });
    await expect(readCompatTest(file)).rejects.toThrow("assigned to multiple units");
  });

  test("rejects compat-tests that leave required version keys out of all units", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const testDef = compatTest();
    const { RELAYER_SDK: _relayerSdk, ...units } = testDef.units;
    await writeJson(file, { ...testDef, units });
    await expect(readCompatTest(file)).rejects.toThrow("do not cover required version keys: TEST_SUITE_VERSION");
  });

  test("generates cumulative mixed-version locks from ordered unit steps", () => {
    const locks = generateRolloutLocks(compatTest());
    expect(locks.map((entry) => entry.lockName)).toEqual([
      "00-baseline.lock.json",
      "01-relayer.lock.json",
      "02-gateway-contracts_host-contracts.lock.json",
      "03-kms-core_kms-connector.lock.json",
      "04-coprocessor.lock.json",
      "05-relayer-sdk.lock.json",
    ]);
    expect(locks[1].env.RELAYER_VERSION).toBe("to-relayer_version");
    expect(locks[2].env.GATEWAY_VERSION).toBe("to-gateway_version");
    expect(locks[2].env.HOST_VERSION).toBe("to-host_version");
    expect(locks[3].env.CORE_VERSION).toBe("to-core_version");
    expect(locks[3].env.CONNECTOR_KMS_WORKER_VERSION).toBe("to-connector_kms_worker_version");
    expect(locks[3].env.COPROCESSOR_GW_LISTENER_VERSION).toBe("from-coprocessor_gw_listener_version");
    expect(locks[5].env.TEST_SUITE_VERSION).toBe("to-test_suite_version");
  });

  test("writes lock files and matrix metadata from a compat-test file", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const outDir = path.join(root, "out");
    await writeJson(file, compatTest());
    await rollout({
      compatTest: file,
      out: outDir,
    });
    const matrix = await readJson<{ include: Array<{ step: string; lockFile: string; name: string }> }>(path.join(outDir, "matrix.json"));
    expect(matrix.include.map((entry) => entry.lockFile)).toEqual([
      "00-baseline.lock.json",
      "01-relayer.lock.json",
      "02-gateway-contracts_host-contracts.lock.json",
      "03-kms-core_kms-connector.lock.json",
      "04-coprocessor.lock.json",
      "05-relayer-sdk.lock.json",
    ]);
    const mixed = await readJson<VersionBundle>(path.join(outDir, "03-kms-core_kms-connector.lock.json"));
    expect(mixed.env.RELAYER_VERSION).toBe("to-relayer_version");
    expect(mixed.env.GATEWAY_VERSION).toBe("to-gateway_version");
    expect(mixed.env.CORE_VERSION).toBe("to-core_version");
    expect(mixed.env.COPROCESSOR_GW_LISTENER_VERSION).toBe("from-coprocessor_gw_listener_version");
  });
});
