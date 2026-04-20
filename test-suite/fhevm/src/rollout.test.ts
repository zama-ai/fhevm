import path from "node:path";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { afterEach, describe, expect, test } from "bun:test";

import { PreflightError } from "./errors";
import { generateRolloutLocks, readCompatTest, renderRolloutStep, rollout, rolloutMatrix, rolloutStep, validateCompatSteps } from "./commands/rollout";
import { REPO_ROOT } from "./layout";
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
];

const envMap = (suffix: string) => Object.fromEntries(versionKeys.map((key) => [key, `${suffix}-${key.toLowerCase()}`]));
const compatTest = (): CompatTestDefinition => ({
  name: "v0.12-to-main-upgrade",
  from: envMap("from"),
  to: envMap("to"),
  harness: {
    testSuiteImageTag: "to-test-suite",
    relayerSdkVersion: "0.5.0-alpha.1",
  },
  profiles: {
    baseline: "rollout-baseline",
    final: "standard",
  },
  steps: [
    { name: "relayer", units: ["RELAYER"] },
    { name: "contracts", units: ["GATEWAY_CONTRACTS", "HOST_CONTRACTS"] },
    { name: "kms", units: ["KMS_CORE", "KMS_CONNECTOR"] },
    {
      name: "coprocessor",
      substeps: [
        { name: "db-migration", units: ["COPROCESSOR_DB_MIGRATION"] },
        { name: "host-listener", units: ["COPROCESSOR_HOST_LISTENER"] },
        { name: "gw-listener", units: ["COPROCESSOR_GW_LISTENER"] },
        { name: "tx-sender", units: ["COPROCESSOR_TX_SENDER"] },
        { name: "tfhe-worker", units: ["COPROCESSOR_TFHE_WORKER"] },
        { name: "zkproof-worker", units: ["COPROCESSOR_ZKPROOF_WORKER"] },
        { name: "sns-worker", units: ["COPROCESSOR_SNS_WORKER"] },
      ],
    },
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
    COPROCESSOR_DB_MIGRATION: ["COPROCESSOR_DB_MIGRATION_VERSION"],
    COPROCESSOR_HOST_LISTENER: ["COPROCESSOR_HOST_LISTENER_VERSION"],
    COPROCESSOR_GW_LISTENER: ["COPROCESSOR_GW_LISTENER_VERSION"],
    COPROCESSOR_TX_SENDER: ["COPROCESSOR_TX_SENDER_VERSION"],
    COPROCESSOR_TFHE_WORKER: ["COPROCESSOR_TFHE_WORKER_VERSION"],
    COPROCESSOR_ZKPROOF_WORKER: ["COPROCESSOR_ZKPROOF_WORKER_VERSION"],
    COPROCESSOR_SNS_WORKER: ["COPROCESSOR_SNS_WORKER_VERSION"],
  },
  execution: {
    scenario: "two-of-two",
  },
});

afterEach(async () => {
  await Promise.all(tempDirs.splice(0).map((dir) => remove(dir)));
});

describe("rollout", () => {
  test("requires each rollout unit exactly once across steps and substeps", () => {
    const { units } = compatTest();
    expect(() => validateCompatSteps([{ name: "relayer", units: ["RELAYER"] }], units)).toThrow(PreflightError);
    expect(() => validateCompatSteps([{ name: "relayer", units: ["RELAYER", "RELAYER"] }], units)).toThrow(PreflightError);
    expect(() => validateCompatSteps([{ name: "relayer", units: ["BOGUS" as never] }], units)).toThrow(PreflightError);
    expect(() =>
      validateCompatSteps([{ name: "coprocessor", units: ["RELAYER"], substeps: [{ name: "db", units: ["COPROCESSOR_DB_MIGRATION"] }] }], units),
    ).toThrow(PreflightError);
  });

  test("rejects compat-tests that reference unknown or duplicated version keys", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const testDef = compatTest();
    await writeJson(file, { ...testDef, units: { ...testDef.units, KMS_CORE: ["CORE_VERSION", "BOGUS"] } });
    await expect(readCompatTest(file)).rejects.toThrow("references unknown version key BOGUS");
    await writeJson(file, { ...testDef, units: { ...testDef.units, COPROCESSOR_SNS_WORKER: ["RELAYER_VERSION"] } });
    await expect(readCompatTest(file)).rejects.toThrow("assigned to multiple units");
  });

  test("injects pinned harness env into from/to env maps", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const testDef = compatTest();
    await writeJson(file, testDef);
    const loaded = await readCompatTest(file);
    expect(loaded.from.TEST_SUITE_VERSION).toBe("to-test-suite");
    expect(loaded.to.TEST_SUITE_VERSION).toBe("to-test-suite");
    expect(loaded.harness?.relayerSdkVersion).toBe("0.5.0-alpha.1");
  });

  test("derives the from relayer-sdk version for the checked-in v0.12 rollout", async () => {
    const loaded = await readCompatTest(path.join(REPO_ROOT, "test-suite/fhevm/compat-tests/v0.12-to-main.json"));
    expect(loaded.harness?.testSuiteImageTag).toBe("186c343");
    expect(loaded.harness?.relayerSdkVersion).toBe("0.5.0-alpha.1");
  });

  test("rejects changing version keys left outside rollout units", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const testDef = compatTest();
    const { RELAYER: _relayer, ...units } = testDef.units;
    await writeJson(file, { ...testDef, units, steps: testDef.steps.filter((step) => step.name !== "relayer") });
    await expect(readCompatTest(file)).rejects.toThrow("do not cover changing version keys: RELAYER_MIGRATE_VERSION, RELAYER_VERSION");
  });

  test("generates cumulative mixed-version locks from ordered unit steps and substeps", () => {
    const locks = generateRolloutLocks(compatTest());
    expect(locks.map((entry) => entry.lockName)).toEqual([
      "00-baseline.lock.json",
      "01-relayer.lock.json",
      "02-contracts.lock.json",
      "03-kms.lock.json",
      "04-coprocessor-db-migration.lock.json",
      "05-coprocessor-host-listener.lock.json",
      "06-coprocessor-gw-listener.lock.json",
      "07-coprocessor-tx-sender.lock.json",
      "08-coprocessor-tfhe-worker.lock.json",
      "09-coprocessor-zkproof-worker.lock.json",
      "10-coprocessor-sns-worker.lock.json",
    ]);
    expect(locks[0].env.TEST_SUITE_VERSION).toBe("to-test-suite");
    expect(locks[0].env.RELAYER_SDK_VERSION).toBe("0.5.0-alpha.1");
    expect(locks[1].env.RELAYER_VERSION).toBe("to-relayer_version");
    expect(locks[2].env.GATEWAY_VERSION).toBe("to-gateway_version");
    expect(locks[3].env.CORE_VERSION).toBe("to-core_version");
    expect(locks[4].env.COPROCESSOR_DB_MIGRATION_VERSION).toBe("to-coprocessor_db_migration_version");
    expect(locks[5].env.COPROCESSOR_HOST_LISTENER_VERSION).toBe("to-coprocessor_host_listener_version");
    expect(locks[5].env.COPROCESSOR_GW_LISTENER_VERSION).toBe("from-coprocessor_gw_listener_version");
    expect(locks[10].env.COPROCESSOR_SNS_WORKER_VERSION).toBe("to-coprocessor_sns_worker_version");
    expect(locks[2].sources).toContain("compat-from:GATEWAY_VERSION=from-gateway_version");
    expect(locks[2].sources).toContain("compat-from:HOST_VERSION=from-host_version");
  });

  test("renders one rollout step on demand", () => {
    const bundle = renderRolloutStep(compatTest(), 5);
    expect(bundle.lockName).toBe("05-coprocessor-host-listener.lock.json");
    expect(bundle.env.RELAYER_VERSION).toBe("to-relayer_version");
    expect(bundle.env.COPROCESSOR_DB_MIGRATION_VERSION).toBe("to-coprocessor_db_migration_version");
    expect(bundle.env.COPROCESSOR_HOST_LISTENER_VERSION).toBe("to-coprocessor_host_listener_version");
    expect(bundle.env.COPROCESSOR_GW_LISTENER_VERSION).toBe("from-coprocessor_gw_listener_version");
  });

  test("prints matrix metadata with selected test profiles", () => {
    expect(rolloutMatrix(compatTest())).toEqual({
      include: [
        { step: "baseline", stepIndex: 0, name: "00-baseline", overrides: "", testProfile: "rollout-baseline" },
        { step: "relayer", stepIndex: 1, name: "01-relayer", overrides: "relayer", testProfile: "rollout-baseline" },
        { step: "contracts", stepIndex: 2, name: "02-contracts", overrides: "relayer,gateway-contracts,host-contracts", testProfile: "rollout-baseline" },
        { step: "kms", stepIndex: 3, name: "03-kms", overrides: "relayer,gateway-contracts,host-contracts,kms-connector", testProfile: "rollout-baseline" },
        { step: "coprocessor-db-migration", stepIndex: 4, name: "04-coprocessor-db-migration", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "rollout-baseline" },
        { step: "coprocessor-host-listener", stepIndex: 5, name: "05-coprocessor-host-listener", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "rollout-baseline" },
        { step: "coprocessor-gw-listener", stepIndex: 6, name: "06-coprocessor-gw-listener", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "rollout-baseline" },
        { step: "coprocessor-tx-sender", stepIndex: 7, name: "07-coprocessor-tx-sender", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "rollout-baseline" },
        { step: "coprocessor-tfhe-worker", stepIndex: 8, name: "08-coprocessor-tfhe-worker", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "rollout-baseline" },
        { step: "coprocessor-zkproof-worker", stepIndex: 9, name: "09-coprocessor-zkproof-worker", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "rollout-baseline" },
        { step: "coprocessor-sns-worker", stepIndex: 10, name: "10-coprocessor-sns-worker", overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor", testProfile: "standard" },
      ],
    });
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
    const matrix = await readJson<{ include: Array<{ step: string; stepIndex: number; name: string; overrides: string; testProfile: string }> }>(
      path.join(outDir, "matrix.json"),
    );
    expect(matrix.include.at(-1)).toEqual({
      step: "coprocessor-sns-worker",
      stepIndex: 10,
      name: "10-coprocessor-sns-worker",
      overrides: "relayer,gateway-contracts,host-contracts,kms-connector,coprocessor",
      testProfile: "standard",
    });
    const mixed = await readJson<VersionBundle>(path.join(outDir, "05-coprocessor-host-listener.lock.json"));
    expect(mixed.env.RELAYER_VERSION).toBe("to-relayer_version");
    expect(mixed.env.HOST_VERSION).toBe("to-host_version");
    expect(mixed.env.COPROCESSOR_DB_MIGRATION_VERSION).toBe("to-coprocessor_db_migration_version");
    expect(mixed.env.COPROCESSOR_HOST_LISTENER_VERSION).toBe("to-coprocessor_host_listener_version");
    expect(mixed.env.COPROCESSOR_GW_LISTENER_VERSION).toBe("from-coprocessor_gw_listener_version");
  });

  test("writes one ephemeral lock file for a specific step", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const file = path.join(root, "compat.json");
    const outFile = path.join(root, "tmp", "step.lock.json");
    await writeJson(file, compatTest());
    await rolloutStep({ compatTest: file, out: outFile, step: 3 });
    const mixed = await readJson<VersionBundle>(outFile);
    expect(mixed.lockName).toBe("step.lock.json");
    expect(mixed.env.CORE_VERSION).toBe("to-core_version");
    expect(mixed.env.COPROCESSOR_DB_MIGRATION_VERSION).toBe("from-coprocessor_db_migration_version");
  });
});
