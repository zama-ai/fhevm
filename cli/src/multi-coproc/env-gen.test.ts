import { describe, expect, test } from "bun:test";
import { mkdtemp, readFile, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { getDotFhevmPaths } from "../config/dotfhevm";
import { generateCoprocessorEnv } from "../config/env-mapping";
import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig, type FhevmConfig } from "../config/model";

import { generateAllCoprocessorEnvFiles, generateCoprocessorInstanceEnv } from "./env-gen";
import { buildTopology } from "./topology";

function buildConfig(overrides: Partial<FhevmConfig> = {}): FhevmConfig {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, 3, 1);
  return createDefaultConfig(keys, {
    topology: {
      numKmsNodes: 1,
      numCoprocessors: 3,
      numCustodians: 3,
      numPausers: 2,
      numHostChains: 1,
    },
    ...overrides,
  });
}

describe("multi-coproc env generation", () => {
  test("keeps instance 0 env aligned with base coprocessor env", () => {
    const config = buildConfig();
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));
    const env = generateCoprocessorInstanceEnv(config, topology[0]!);

    expect(env).toEqual(generateCoprocessorEnv(config));
  });

  test("overrides database and tx-sender key per instance", () => {
    const config = buildConfig();
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));
    const env0 = generateCoprocessorInstanceEnv(config, topology[0]!);
    const env1 = generateCoprocessorInstanceEnv(config, topology[1]!);

    expect(env0.DATABASE_URL).toContain("/coprocessor");
    expect(env1.DATABASE_URL).toContain("/coprocessor_2");
    expect(env1.TX_SENDER_PRIVATE_KEY).toBe(config.keys.coprocessors[1]?.txSender.privateKey);
    expect(env0.TX_SENDER_PRIVATE_KEY).not.toBe(env1.TX_SENDER_PRIVATE_KEY);

    const differingKeys = Object.keys(env0).filter((key) => env0[key] !== env1[key]);
    expect(differingKeys.sort()).toEqual(["DATABASE_URL", "TX_SENDER_PRIVATE_KEY"]);
  });

  test("propagates discovered minio endpoint to all instances", () => {
    const config = buildConfig({ runtime: { minioIp: "10.42.0.12" } });
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));

    for (const instance of topology) {
      const env = generateCoprocessorInstanceEnv(config, instance);
      expect(env.AWS_ENDPOINT_URL).toBe("http://10.42.0.12:9000");
    }
  });

  test("writes all instance env files", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-env-"));

    try {
      const config = buildConfig();
      const topology = buildTopology(config, getDotFhevmPaths(root));
      const outputs = await generateAllCoprocessorEnvFiles(config, topology);

      expect(outputs.size).toBe(3);
      const envDir = join(root, "test-suite", "fhevm", "env", "staging");
      expect(outputs.get("coprocessor")).toBe(join(envDir, "coprocessor.env"));
      expect(outputs.get("coprocessor-2")).toBe(join(envDir, "coprocessor-2.env"));

      const secondEnv = await readFile(join(envDir, "coprocessor-2.env"), "utf8");
      expect(secondEnv).toContain("DATABASE_URL=postgresql://postgres:postgres@coprocessor-and-kms-db:5432/coprocessor_2");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });
});
