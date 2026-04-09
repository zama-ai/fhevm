import path from "node:path";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { afterEach, describe, expect, test } from "bun:test";

import { PreflightError } from "./errors";
import { generateRolloutLocks, parseRolloutOrder, rollout } from "./commands/rollout";
import { readJson, remove, writeJson } from "./utils/fs";
import type { VersionBundle } from "./types";

const tempDirs: string[] = [];
const repoKeys = {
  GATEWAY_VERSION: "",
  HOST_VERSION: "",
  COPROCESSOR_DB_MIGRATION_VERSION: "",
  COPROCESSOR_HOST_LISTENER_VERSION: "",
  COPROCESSOR_GW_LISTENER_VERSION: "",
  COPROCESSOR_TX_SENDER_VERSION: "",
  COPROCESSOR_TFHE_WORKER_VERSION: "",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "",
  COPROCESSOR_SNS_WORKER_VERSION: "",
  CONNECTOR_DB_MIGRATION_VERSION: "",
  CONNECTOR_GW_LISTENER_VERSION: "",
  CONNECTOR_KMS_WORKER_VERSION: "",
  CONNECTOR_TX_SENDER_VERSION: "",
  CORE_VERSION: "",
  RELAYER_VERSION: "",
  RELAYER_MIGRATE_VERSION: "",
  TEST_SUITE_VERSION: "",
};

const bundle = (suffix: string): VersionBundle => ({
  target: "latest-supported",
  lockName: `${suffix}.json`,
  sources: [suffix],
  env: Object.fromEntries(Object.keys(repoKeys).map((key) => [key, `${suffix}-${key.toLowerCase()}`])),
});

afterEach(async () => {
  await Promise.all(tempDirs.splice(0).map((dir) => remove(dir)));
});

describe("rollout", () => {
  test("requires each rollout group exactly once", () => {
    expect(() => parseRolloutOrder("relayer,contracts")).toThrow(PreflightError);
    expect(() => parseRolloutOrder("relayer,contracts,kms-plane,coprocessor,coprocessor")).toThrow(PreflightError);
    expect(() => parseRolloutOrder("relayer,contracts,kms-plane,coprocessor,test-suite,bogus")).toThrow(PreflightError);
  });

  test("generates cumulative mixed-version locks in the requested order", () => {
    const from = bundle("from");
    const to = bundle("to");
    const locks = generateRolloutLocks(from, to, parseRolloutOrder("relayer,contracts,kms-plane,coprocessor,test-suite"));
    expect(locks.map((entry) => entry.lockName)).toEqual([
      "00-baseline.lock.json",
      "01-relayer.lock.json",
      "02-contracts.lock.json",
      "03-kms-plane.lock.json",
      "04-coprocessor.lock.json",
      "05-test-suite.lock.json",
    ]);
    expect(locks[0].env.RELAYER_VERSION).toBe(from.env.RELAYER_VERSION);
    expect(locks[1].env.RELAYER_VERSION).toBe(to.env.RELAYER_VERSION);
    expect(locks[1].env.HOST_VERSION).toBe(from.env.HOST_VERSION);
    expect(locks[2].env.HOST_VERSION).toBe(to.env.HOST_VERSION);
    expect(locks[3].env.CORE_VERSION).toBe(to.env.CORE_VERSION);
    expect(locks[4].env.COPROCESSOR_TFHE_WORKER_VERSION).toBe(to.env.COPROCESSOR_TFHE_WORKER_VERSION);
    expect(locks[5].env.TEST_SUITE_VERSION).toBe(to.env.TEST_SUITE_VERSION);
  });

  test("writes lock files and matrix metadata", async () => {
    const root = await mkdtemp(path.join(tmpdir(), "fhevm-rollout-"));
    tempDirs.push(root);
    const fromFile = path.join(root, "from.json");
    const toFile = path.join(root, "to.json");
    const outDir = path.join(root, "out");
    await writeJson(fromFile, bundle("from"));
    await writeJson(toFile, bundle("to"));
    await rollout({
      from: fromFile,
      to: toFile,
      order: "relayer,contracts,kms-plane,coprocessor,test-suite",
      out: outDir,
    });
    const matrix = await readJson<{ include: Array<{ group: string; lockFile: string; name: string }> }>(path.join(outDir, "matrix.json"));
    expect(matrix.include.map((entry) => entry.lockFile)).toEqual([
      "00-baseline.lock.json",
      "01-relayer.lock.json",
      "02-contracts.lock.json",
      "03-kms-plane.lock.json",
      "04-coprocessor.lock.json",
      "05-test-suite.lock.json",
    ]);
    const mixed = await readJson<VersionBundle>(path.join(outDir, "03-kms-plane.lock.json"));
    expect(mixed.env.RELAYER_VERSION).toBe("to-relayer_version");
    expect(mixed.env.GATEWAY_VERSION).toBe("to-gateway_version");
    expect(mixed.env.CORE_VERSION).toBe("to-core_version");
    expect(mixed.env.COPROCESSOR_GW_LISTENER_VERSION).toBe("from-coprocessor_gw_listener_version");
  });
});
