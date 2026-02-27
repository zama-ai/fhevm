import { describe, expect, test } from "bun:test";

import { getServiceByName, type ServiceDefinition } from "../config/service-map";
import { getDotFhevmPaths } from "../config/dotfhevm";
import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig } from "../config/model";
import { ExitCode } from "../errors";
import type { ContainerInfo } from "../docker/types";
import { buildTopology, generateAllInstanceServices } from "../multi-coproc";

import {
  buildStatusJson,
  buildStatusJsonForServices,
  buildStatusOutput,
  buildStatusOutputForServices,
  describeServiceStatus,
  formatStatusLine,
  toStatusError,
} from "./status";

function mustService(name: string): ServiceDefinition {
  const service = getServiceByName(name);
  if (!service) {
    throw new Error(`missing fixture service: ${name}`);
  }
  return service;
}

function container(service: string, partial: Partial<ContainerInfo>): ContainerInfo {
  return {
    name: service,
    service,
    state: "running",
    health: "none",
    ...partial,
  };
}

describe("status command", () => {
  test("marks one-shot exited with zero as completed", () => {
    const service = mustService("gateway-sc-pause");
    const description = describeServiceStatus(
      container(service.name, { state: "exited", exitCode: 0 }),
      service,
    );

    expect(description).toEqual({ label: "completed", color: "green" });
  });

  test("marks failed exits with error color", () => {
    const service = mustService("coprocessor-tfhe-worker");
    const description = describeServiceStatus(
      container(service.name, { state: "exited", exitCode: 7 }),
      service,
    );

    expect(description).toEqual({ label: "failed (exit 7)", color: "red" });
  });

  test("marks missing services as not running", () => {
    const service = mustService("relayer");
    const description = describeServiceStatus(container(service.name, { state: "not-found" }), service);

    expect(description).toEqual({ label: "not running", color: "dim" });
  });

  test("formats service lines with ports", () => {
    const service = mustService("host-node");
    const line = formatStatusLine(
      service,
      container(service.name, {
        state: "running",
        health: "healthy",
        uptime: "3 minutes",
        ports: "0.0.0.0:8545->8545/tcp",
      }),
    );

    expect(line).toContain("host-node");
    expect(line).toContain("running (healthy)");
    expect(line).toContain("up 3 minutes");
    expect(line).toContain("8545");
  });

  test("builds grouped output sections", () => {
    const status = new Map<string, ContainerInfo>();
    status.set("minio", container("minio", { state: "running", health: "healthy" }));

    const lines = buildStatusOutput(status);
    expect(lines[0]).toContain("fhEVM Stack Status");
    expect(lines.join("\n")).toContain("Infrastructure:");
    expect(lines.join("\n")).toContain("Coprocessor:");
  });

  test("serializes json output for all services", () => {
    const status = new Map<string, ContainerInfo>();
    status.set("host-node", container("host-node", { state: "running", health: "healthy", uptime: "5 minutes" }));

    const json = buildStatusJson(status) as Record<string, Record<string, unknown>>;
    expect(json["host-node"]?.state).toBe("running");
    expect(json["host-node"]?.group).toBe("infra");
    expect(json["host-node"]?.uptime).toBe("5 minutes");
    expect(json["coprocessor-tfhe-worker"]?.state).toBe("not-found");
  });

  test("includes generated multi-coprocessor services when provided", () => {
    const keys = deriveAllKeys(DEFAULT_MNEMONIC, 2, 1);
    const config = createDefaultConfig(keys, {
      topology: {
        numKmsNodes: 1,
        numCoprocessors: 2,
        numCustodians: 3,
        numPausers: 2,
        numHostChains: 1,
      },
    });
    const topology = buildTopology(config, getDotFhevmPaths("/tmp/fhevm"));
    const services = generateAllInstanceServices(topology);
    const status = new Map<string, ContainerInfo>([
      [
        "coprocessor-2-gw-listener",
        container("coprocessor-2-gw-listener", { state: "running", health: "healthy" }),
      ],
    ]);

    const lines = buildStatusOutputForServices(status, services);
    const json = buildStatusJsonForServices(status, services) as Record<string, Record<string, unknown>>;

    expect(lines.join("\n")).toContain("coprocessor-2-gw-listener");
    expect(json["coprocessor-2-gw-listener"]?.state).toBe("running");
  });

  test("maps unknown errors to docker status errors", () => {
    const converted = toStatusError(new Error("boom"));

    expect(converted).toMatchObject({
      exitCode: ExitCode.DOCKER,
      step: "status",
      message: "boom",
    });
  });
});
