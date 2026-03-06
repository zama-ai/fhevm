import { describe, expect, test } from "bun:test";

import { getDotFhevmPaths } from "../config/dotfhevm";
import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig, type FhevmConfig } from "../config/model";

import { buildTopology, gwListenerPort } from "./topology";

function buildConfig(numCoprocessors: number): FhevmConfig {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, numCoprocessors, 1);
  return createDefaultConfig(keys, {
    topology: {
      numKmsNodes: 1,
      numCoprocessors,
      numCustodians: 3,
      numPausers: 2,
      numHostChains: 1,
    },
  });
}

describe("multi-coproc topology", () => {
  test("builds single-instance topology with base compose", () => {
    const config = buildConfig(1);
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));

    expect(topology).toHaveLength(1);
    expect(topology[0]).toMatchObject({
      index: 0,
      displayIndex: 1,
      servicePrefix: "coprocessor",
      envFileName: "coprocessor",
      composeFile: "test-suite/fhevm/docker-compose/coprocessor-docker-compose.yml",
      databaseName: "coprocessor",
      gwListenerPort: 8080,
      usesBaseCompose: true,
    });
  });

  test("builds three-instance topology with deterministic names", () => {
    const config = buildConfig(3);
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));

    expect(topology.map((instance) => instance.servicePrefix)).toEqual([
      "coprocessor",
      "coprocessor-2",
      "coprocessor-3",
    ]);
    expect(topology.map((instance) => instance.databaseName)).toEqual([
      "coprocessor",
      "coprocessor_2",
      "coprocessor_3",
    ]);
    expect(topology.map((instance) => instance.gwListenerPort)).toEqual([8080, 8180, 8280]);
    expect(topology[1]?.composeFile).toBe("/tmp/fhevm/.fhevm/compose/coprocessor-2.yml");
  });

  test("supports max five coprocessors and expected ports", () => {
    const config = buildConfig(5);
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));

    expect(topology).toHaveLength(5);
    expect(topology.map((instance) => instance.gwListenerPort)).toEqual([8080, 8180, 8280, 8380, 8480]);
    expect(topology[4]?.txSenderPrivateKey).toBe(config.keys.coprocessors[4]?.txSender.privateKey);
  });

  test("computes gw-listener ports with fixed formula", () => {
    expect(gwListenerPort(0)).toBe(8080);
    expect(gwListenerPort(1)).toBe(8180);
    expect(gwListenerPort(4)).toBe(8480);
  });
});
