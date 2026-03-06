import { describe, expect, test } from "bun:test";

import { getDotFhevmPaths } from "../config/dotfhevm";
import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig } from "../config/model";
import { DOCKER_PROJECT } from "../docker/types";

import {
  generateAllInstanceServices,
  generateInstanceServices,
  getAllCoprocessorServiceNames,
} from "./services";
import { buildTopology } from "./topology";

function buildTopologyFixture(coprocessors: number) {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, coprocessors, 1);
  const config = createDefaultConfig(keys, {
    topology: {
      numKmsNodes: 1,
      numCoprocessors: coprocessors,
      numCustodians: 3,
      numPausers: 2,
      numHostChains: 1,
    },
  });

  return buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));
}

describe("multi-coproc service definitions", () => {
  test("returns base static services for instance 0", () => {
    const topology = buildTopologyFixture(2);
    const services = generateInstanceServices(topology[0]!);

    expect(services).toHaveLength(8);
    expect(services[0]?.name).toBe("coprocessor-db-migration");
  });

  test("generates instance-specific services for instance 1", () => {
    const topology = buildTopologyFixture(2);
    const services = generateInstanceServices(topology[1]!);

    expect(services).toHaveLength(8);
    expect(services[0]?.name).toBe("coprocessor-2-db-migration");
    expect(services.every((service) => service.composeFile.endsWith("coprocessor-2.yml"))).toBe(true);

    const gwListener = services.find((service) => service.name === "coprocessor-2-gw-listener");
    expect(gwListener?.healthEndpoint).toBe("http://localhost:8180/liveness");
    expect(gwListener?.ports).toEqual([8180]);
    expect(gwListener?.containerName).toBe(`${DOCKER_PROJECT}-coprocessor-2-gw-listener-1`);
  });

  test("builds unique service names across instances", () => {
    const topology = buildTopologyFixture(3);
    const names = getAllCoprocessorServiceNames(topology);

    expect(names).toHaveLength(24);
    expect(new Set(names).size).toBe(24);
    expect(names).toContain("coprocessor-3-transaction-sender");
  });

  test("flattens all service definitions for all instances", () => {
    const topology = buildTopologyFixture(3);
    const services = generateAllInstanceServices(topology);

    expect(services).toHaveLength(24);
    expect(services.some((service) => service.name === "coprocessor-gw-listener")).toBe(true);
    expect(services.some((service) => service.name === "coprocessor-3-gw-listener")).toBe(true);
  });
});
