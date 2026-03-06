import { readdir } from "fs/promises";

import { defineCommand } from "citty";

import { buildConfig } from "../config/config-builder";
import { getDotFhevmPaths } from "../config/dotfhevm";
import { MAX_COPROCESSORS } from "../config/model";
import { SERVICE_MAP, type ServiceDefinition, type ServiceGroup } from "../config/service-map";
import { getProjectStatus } from "../docker/services";
import type { ContainerInfo } from "../docker/types";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";
import { buildTopology, generateAllInstanceServices } from "../multi-coproc";
import { bold, dim, green, red, yellow } from "../utils/output";

export interface ServiceStatusDescription {
  label: string;
  color: "green" | "yellow" | "red" | "dim";
}

export interface StatusCommandResult {
  statusByService: ReadonlyMap<string, ContainerInfo>;
  services: readonly ServiceDefinition[];
}

const GROUP_ORDER: readonly ServiceGroup[] = [
  "infra",
  "core",
  "coprocessor",
  "kms-connector",
  "contracts",
  "relayer",
  "test-suite",
];

const GROUP_LABEL: Record<ServiceGroup, string> = {
  infra: "Infrastructure",
  core: "Core",
  coprocessor: "Coprocessor",
  "kms-connector": "KMS Connector",
  contracts: "Contracts",
  relayer: "Relayer",
  "test-suite": "Test Suite",
};

function getColorizer(color: ServiceStatusDescription["color"]): (value: string) => string {
  switch (color) {
    case "green":
      return green;
    case "yellow":
      return yellow;
    case "red":
      return red;
    case "dim":
      return dim;
  }
}

function mergeServices(additionalServices: readonly ServiceDefinition[] = []): ServiceDefinition[] {
  const byName = new Map<string, ServiceDefinition>();
  for (const service of [...SERVICE_MAP, ...additionalServices]) {
    byName.set(service.name, service);
  }
  return [...byName.values()];
}

async function findMultiCoprocessorServices(): Promise<ServiceDefinition[]> {
  const paths = getDotFhevmPaths();
  let files: string[] = [];
  try {
    files = await readdir(paths.env);
  } catch {
    return [];
  }

  const instanceNumbers = files
    .map((file) => file.match(/^coprocessor-(\d+)\.env$/)?.[1])
    .filter((value): value is string => value !== undefined)
    .map((value) => Number.parseInt(value, 10))
    .filter((value) => Number.isFinite(value) && value >= 2);

  if (instanceNumbers.length === 0) {
    return [];
  }

  const maxInstances = Math.min(Math.max(...instanceNumbers), MAX_COPROCESSORS);
  const presentEnvFiles = new Set(
    files.filter((file) => file.startsWith("coprocessor-") && file.endsWith(".env")).map((file) => file.slice(0, -4)),
  );
  const config = buildConfig({ numCoprocessors: maxInstances });
  const topology = buildTopology(config, paths).filter(
    (instance) => !instance.usesBaseCompose && presentEnvFiles.has(instance.envFileName),
  );

  return generateAllInstanceServices(topology);
}

export function describeServiceStatus(
  container: ContainerInfo,
  service: ServiceDefinition,
): ServiceStatusDescription {
  if (container.state === "running") {
    if (container.health === "healthy") {
      return { label: "running (healthy)", color: "green" };
    }
    if (container.health === "unhealthy") {
      return { label: "running (unhealthy)", color: "red" };
    }
    if (container.health === "starting") {
      return { label: "running (starting)", color: "yellow" };
    }
    return { label: "running", color: "green" };
  }

  if (container.state === "paused") {
    return { label: "paused", color: "yellow" };
  }

  if (container.state === "exited" || container.state === "dead") {
    const exitCode = container.exitCode ?? 0;
    if (service.isOneShot && exitCode === 0) {
      return { label: "completed", color: "green" };
    }
    if (exitCode === 0) {
      return { label: "exited", color: "dim" };
    }
    return { label: `failed (exit ${exitCode})`, color: "red" };
  }

  if (container.state === "not-found") {
    return { label: "not running", color: "dim" };
  }

  return { label: container.state, color: "yellow" };
}

export function formatStatusLine(service: ServiceDefinition, container: ContainerInfo): string {
  const status = describeServiceStatus(container, service);
  const colorize = getColorizer(status.color);
  const parts = [`  ${service.name.padEnd(34)}`, colorize(status.label)];
  if (container.state === "running" && container.uptime) {
    parts.push(dim(`up ${container.uptime}`));
  }
  if (container.ports) {
    parts.push(dim(container.ports));
  }
  return parts.join("  ");
}

export function buildStatusOutput(statusByService: ReadonlyMap<string, ContainerInfo>): string[] {
  return buildStatusOutputForServices(statusByService, SERVICE_MAP);
}

export function buildStatusOutputForServices(
  statusByService: ReadonlyMap<string, ContainerInfo>,
  services: readonly ServiceDefinition[],
): string[] {
  const lines = [bold("fhEVM Stack Status")];

  for (const group of GROUP_ORDER) {
    const grouped = services.filter((service) => service.group === group);
    if (grouped.length === 0) {
      continue;
    }
    lines.push("", bold(`${GROUP_LABEL[group]}:`));
    for (const service of grouped) {
      const container = statusByService.get(service.name) ?? {
        name: service.containerName,
        service: service.name,
        state: "not-found",
        health: "none",
      };
      lines.push(formatStatusLine(service, container));
    }
  }

  return lines;
}

export function buildStatusJson(statusByService: ReadonlyMap<string, ContainerInfo>): Record<string, unknown> {
  return buildStatusJsonForServices(statusByService, SERVICE_MAP);
}

export function buildStatusJsonForServices(
  statusByService: ReadonlyMap<string, ContainerInfo>,
  services: readonly ServiceDefinition[],
): Record<string, unknown> {
  return Object.fromEntries(
    services.map((service) => {
      const container = statusByService.get(service.name) ?? {
        name: service.containerName,
        service: service.name,
        state: "not-found",
        health: "none",
      };
      return [
        service.name,
        {
          group: service.group,
          container: container.name,
          state: container.state,
          health: container.health ?? "none",
          exitCode: container.exitCode ?? null,
          uptime: container.uptime ?? null,
          ports: container.ports ?? null,
          isOneShot: service.isOneShot,
        },
      ];
    }),
  );
}

export async function runStatusCommand(): Promise<StatusCommandResult> {
  const additionalServices = await findMultiCoprocessorServices();
  const services = mergeServices(additionalServices);
  const statusByService = await getProjectStatus({ services: additionalServices });

  return {
    statusByService,
    services,
  };
}

export function toStatusError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "status",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "status",
    description: "Show stack status",
  },
  args: {
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    try {
      const result = await runStatusCommand();

      if (args.json) {
        console.log(JSON.stringify(buildStatusJsonForServices(result.statusByService, result.services), null, 2));
        return;
      }

      for (const line of buildStatusOutputForServices(result.statusByService, result.services)) {
        console.log(line);
      }
    } catch (error) {
      exitWithError(toStatusError(error));
    }
  },
});
