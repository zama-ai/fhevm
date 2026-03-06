import { describe, expect, test } from "bun:test";
import { mkdtemp, readFile, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { getDotFhevmPaths } from "../config/dotfhevm";
import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig } from "../config/model";

import { generateComposeYaml, writeComposeFile } from "./compose-gen";
import { generateAllCoprocessorEnvFiles } from "./env-gen";
import { generateAllInstanceServices } from "./services";
import { buildTopology } from "./topology";

function buildConfig(coprocessors: number) {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, coprocessors, 1);
  return createDefaultConfig(keys, {
    topology: {
      numKmsNodes: 1,
      numCoprocessors: coprocessors,
      numCustodians: 3,
      numPausers: 2,
      numHostChains: 1,
    },
  });
}

describe("multi-coproc integration", () => {
  test("builds topology and artifacts for 3 coprocessors", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-multi-"));

    try {
      const config = buildConfig(3);
      const paths = getDotFhevmPaths(root);
      const topology = buildTopology(config, paths);

      expect(topology).toHaveLength(3);
      expect(topology.map((instance) => instance.databaseName)).toEqual([
        "coprocessor",
        "coprocessor_2",
        "coprocessor_3",
      ]);

      const envFiles = await generateAllCoprocessorEnvFiles(config, topology);
      expect(envFiles.get("coprocessor-2")).toBe(join(root, "test-suite", "fhevm", "env", "staging", "coprocessor-2.env"));

      await writeComposeFile(topology[1]!);
      await writeComposeFile(topology[2]!);

      const compose2 = await readFile(join(root, ".fhevm", "compose", "coprocessor-2.yml"), "utf8");
      expect(compose2).toContain("coprocessor-2-gw-listener");
      expect(compose2).toContain("8180:8080");

      const compose3 = generateComposeYaml(topology[2]!);
      expect(compose3).toContain("coprocessor-3-db-migration");
      expect(compose3).toContain("../env/coprocessor-3.env");

      const services = generateAllInstanceServices(topology);
      expect(services).toHaveLength(24);
      expect(services.some((service) => service.name === "coprocessor-3-transaction-sender")).toBe(true);
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });
});
