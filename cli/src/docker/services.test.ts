import { describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";
import { getServiceByName, type ServiceDefinition } from "../config/service-map";

import {
  __internal,
  buildComposeOptions,
  discoverMinioIp,
  getProjectStatus,
  restartServices,
  startAndWaitForServiceBatches,
  startAndWaitForServices,
} from "./services";

function getService(name: string): ServiceDefinition {
  const service = getServiceByName(name);
  if (!service) {
    throw new Error(`missing fixture service: ${name}`);
  }
  return service;
}

describe("docker services orchestration", () => {
  test("stops one-shot log-sentinel services after readiness", async () => {
    const service = getService("host-sc-deploy");
    const stopCalls: string[][] = [];

    __internal.setDockerOpsForTests({
      composeUp: async () => {},
      composeStop: async (services) => {
        stopCalls.push([...services]);
      },
      waitForAllReady: async () => [{ service: service.name, ready: true, elapsedMs: 0 }],
    });

    try {
      const results = await startAndWaitForServices([service], { envFileByName: new Map() });
      expect(results).toEqual([{ service: service.name, ready: true, elapsedMs: 0 }]);
      expect(stopCalls).toEqual([["host-sc-deploy"]]);
    } finally {
      __internal.resetDockerOpsForTests();
    }
  });

  test("builds compose options with deduped files", () => {
    const options = buildComposeOptions(
      [getService("coprocessor-db-migration"), getService("coprocessor-tfhe-worker")],
      {
        envFileByName: new Map([["coprocessor", ".fhevm/env/coprocessor.env"]]),
      },
    );

    expect(options.project).toBe("fhevm");
    expect(options.files).toHaveLength(1);
    expect(options.files[0]).toContain("coprocessor-docker-compose.yml");
    expect(options.envFile).toBe(".fhevm/env/coprocessor.env");
    expect(options.services).toEqual(["coprocessor-db-migration", "coprocessor-tfhe-worker"]);
  });

  test("omits compose env-file for mixed service env files", () => {
    const options = buildComposeOptions(
      [getService("host-node"), getService("gateway-node")],
      {
        envFileByName: new Map([
          ["host-node", ".fhevm/env/host-node.env"],
          ["gateway-node", ".fhevm/env/gateway-node.env"],
        ]),
      },
    );

    expect(options.envFile).toBeUndefined();
    expect(options.files).toHaveLength(2);
  });

  test("startAndWaitForServices returns docker error on compose startup failure", async () => {
    const fixture = getService("coprocessor-tfhe-worker");
    const brokenService: ServiceDefinition = {
      ...fixture,
      composeFile: "/definitely/missing/compose.yml",
      containerName: "fhevm-cli-non-existent-container",
    };

    await expect(
      startAndWaitForServices([brokenService], {
        envFileByName: new Map([["coprocessor", ".fhevm/env/coprocessor.env"]]),
        wait: { timeoutMs: 20, pollIntervalMs: 10 },
      }),
    ).rejects.toMatchObject({
      exitCode: ExitCode.DOCKER,
    });
  });

  test("starts multi-instance service batches with one compose up per batch", async () => {
    const instance0 = [getService("coprocessor-db-migration"), getService("coprocessor-tfhe-worker")];
    const instance1 = [
      {
        ...getService("coprocessor-db-migration"),
        name: "coprocessor-2-db-migration",
        containerName: "coprocessor-2-db-migration",
        composeFile: ".fhevm/compose/coprocessor-2.yml",
        envFile: "coprocessor-2",
      },
      {
        ...getService("coprocessor-tfhe-worker"),
        name: "coprocessor-2-tfhe-worker",
        containerName: "coprocessor-2-tfhe-worker",
        composeFile: ".fhevm/compose/coprocessor-2.yml",
        envFile: "coprocessor-2",
      },
    ];

    const composeCalls: string[][] = [];
    __internal.setDockerOpsForTests({
      composeUp: async (options) => {
        composeCalls.push([...(options.services ?? [])]);
      },
      waitForAllReady: async (services) => services.map((service) => ({ service: service.name, ready: true, elapsedMs: 0 })),
      composeStop: async () => {},
    });

    try {
      const result = await startAndWaitForServiceBatches([instance0, instance1], {
        envFileByName: new Map([
          ["coprocessor", ".fhevm/env/coprocessor.env"],
          ["coprocessor-2", ".fhevm/env/coprocessor-2.env"],
        ]),
      });

      expect(composeCalls).toEqual([
        ["coprocessor-db-migration", "coprocessor-tfhe-worker"],
        ["coprocessor-2-db-migration", "coprocessor-2-tfhe-worker"],
      ]);
      expect(result).toHaveLength(4);
    } finally {
      __internal.resetDockerOpsForTests();
    }
  });

  test("restartServices is a no-op for empty input", async () => {
    const result = await restartServices([], { envFileByName: new Map() });
    expect(result).toEqual([]);
  });

  test("project status includes all known services", async () => {
    const status = await getProjectStatus();
    expect(status.size).toBeGreaterThan(30);
    expect(status.has("minio")).toBe(true);
    expect(status.get("minio")?.service).toBe("minio");
  });

  test("project status includes additional generated services when provided", async () => {
    const dynamic: ServiceDefinition = {
      ...getService("coprocessor-tfhe-worker"),
      name: "coprocessor-2-tfhe-worker",
      containerName: "coprocessor-2-tfhe-worker",
      composeFile: ".fhevm/compose/coprocessor-2.yml",
      envFile: "coprocessor-2",
    };
    const status = await getProjectStatus({ services: [dynamic] });

    expect(status.has("coprocessor-2-tfhe-worker")).toBe(true);
  });

  test("discoverMinioIp throws when container does not exist", async () => {
    await expect(discoverMinioIp("fhevm-cli-non-existent-container")).rejects.toMatchObject({
      exitCode: ExitCode.DOCKER,
    });
  });
});
