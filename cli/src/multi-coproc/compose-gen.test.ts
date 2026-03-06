import { describe, expect, test } from "bun:test";
import { mkdtemp, readFile, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { getDotFhevmPaths } from "../config/dotfhevm";
import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig } from "../config/model";
import { COPROCESSOR_TEMPLATES } from "../config/service-map";

import { COPROCESSOR_SERVICE_TEMPLATES, generateComposeYaml, writeComposeFile } from "./compose-gen";
import { buildTopology } from "./topology";

function buildConfig(coprocessors = 2) {
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

describe("multi-coproc compose generation", () => {
  test("keeps compose and service-map coprocessor suffixes in sync", () => {
    expect(COPROCESSOR_SERVICE_TEMPLATES.map((template) => template.suffix)).toEqual(
      COPROCESSOR_TEMPLATES.map((template) => template.suffix),
    );
  });

  test("generates instance-specific yaml without container_name", () => {
    const config = buildConfig(2);
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));
    const yaml = generateComposeYaml(topology[1]!);

    expect(yaml).toContain("coprocessor-2-db-migration");
    expect(yaml).toContain("coprocessor-2-gw-listener");
    expect(yaml).toContain("../env/coprocessor-2.env");
    expect(yaml).toContain("8180:8080");
    expect(yaml).toContain("depends_on:");
    expect(yaml).toContain("coprocessor-2-db-migration:");
    expect(yaml).toContain("external: true");
    expect(yaml).not.toContain("container_name:");
    expect(yaml).not.toContain("cache_to:");
  });

  test("keeps cache_to only on instance 0", () => {
    const config = buildConfig(2);
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));

    expect(generateComposeYaml(topology[0]!)).toContain("cache_to:");
    expect(generateComposeYaml(topology[1]!)).not.toContain("cache_to:");
  });

  test("writes generated compose file for non-base instances", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-compose-"));

    try {
      const config = buildConfig(2);
      const topology = buildTopology(config, getDotFhevmPaths(root));
      const composePath = await writeComposeFile(topology[1]!);
      const content = await readFile(composePath, "utf8");

      expect(composePath).toBe(join(root, ".fhevm", "compose", "coprocessor-2.yml"));
      expect(content).toContain("coprocessor-2-tfhe-worker");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });
});
