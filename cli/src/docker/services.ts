import { readdir } from "fs/promises";
import { join } from "path";

import {
  SERVICE_MAP,
  getComposeFilesForServices,
  getServiceByName,
  type ServiceDefinition,
} from "../config/service-map";
import { ExitCode, FhevmCliError } from "../errors";

import { composeDown, composePs, composeStart, composeStop, composeUp } from "./compose";
import { getContainerIp, listProjectContainers } from "./containers";
import { waitForAllReady } from "./readiness";
import {
  DOCKER_PROJECT,
  type ContainerInfo,
  type ReadinessOptions,
  type ReadinessResult,
  type ServiceStartOptions,
  type ServiceWaitOptions,
} from "./types";

interface DockerServiceOps {
  composeDown: typeof composeDown;
  composePs: typeof composePs;
  composeStart: typeof composeStart;
  composeStop: typeof composeStop;
  composeUp: typeof composeUp;
  listProjectContainers: typeof listProjectContainers;
  waitForAllReady: typeof waitForAllReady;
}

const DEFAULT_DOCKER_OPS: DockerServiceOps = {
  composeDown,
  composePs,
  composeStart,
  composeStop,
  composeUp,
  listProjectContainers,
  waitForAllReady,
};

let dockerOps: DockerServiceOps = DEFAULT_DOCKER_OPS;
const GENERATED_MULTI_COMPOSE_PATTERN = /^coprocessor-\d+\.yml$/;

function ensureServices(services: ServiceDefinition[]): void {
  if (services.length === 0) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "docker-services",
      message: "no services provided",
    });
  }
}

function resolveComposeEnvFile(
  services: ServiceDefinition[],
  envFileByName: ReadonlyMap<string, string>,
): string | undefined {
  const paths = new Set<string>();

  for (const service of services) {
    const path = envFileByName.get(service.envFile);
    if (path) {
      paths.add(path);
    }
  }

  if (paths.size === 1) {
    return [...paths][0];
  }

  return undefined;
}

export function buildComposeOptions(
  services: ServiceDefinition[],
  options: ServiceStartOptions,
): {
  project: string;
  files: string[];
  envFile?: string;
  envVars?: Record<string, string>;
  cwd?: string;
  services: string[];
  build?: boolean;
  noBuild?: boolean;
  noCache?: boolean;
} {
  ensureServices(services);

  return {
    project: DOCKER_PROJECT,
    files: getComposeFilesForServices(services),
    envFile: resolveComposeEnvFile(services, options.envFileByName),
    envVars: options.envVars,
    cwd: options.cwd,
    services: services.map((service) => service.name),
    build: options.build,
    noBuild: !options.build,
    noCache: options.noCache,
  };
}

function collectServices(extraServices: readonly ServiceDefinition[] = []): ServiceDefinition[] {
  const combined = [...SERVICE_MAP, ...extraServices];
  const byName = new Map<string, ServiceDefinition>();
  for (const service of combined) {
    byName.set(service.name, service);
  }
  return [...byName.values()];
}

async function getGeneratedComposeFiles(cwd?: string): Promise<string[]> {
  const composeDir = join(cwd ?? process.cwd(), ".fhevm", "compose");
  try {
    const files = await readdir(composeDir);
    return files
      .filter((file) => GENERATED_MULTI_COMPOSE_PATTERN.test(file))
      .map((file) => `.fhevm/compose/${file}`);
  } catch {
    return [];
  }
}

async function stopLogSentinelOneShots(
  services: ServiceDefinition[],
  options: ServiceStartOptions,
): Promise<void> {
  const targets = services
    .filter((service) => service.isOneShot && service.healthCheck === "log-sentinel")
    .map((service) => service.name);

  if (targets.length === 0) {
    return;
  }

  const composeOptions = buildComposeOptions(
    services.filter((service) => targets.includes(service.name)),
    options,
  );

  await dockerOps.composeStop(targets, composeOptions);
}

export async function startAndWaitForServices(
  services: ServiceDefinition[],
  options: ServiceStartOptions & { wait?: ServiceWaitOptions },
): Promise<ReadinessResult[]> {
  if (services.length === 0) {
    return [];
  }

  const composeOptions = buildComposeOptions(services, options);
  await dockerOps.composeUp(composeOptions);

  const readinessOptions: Partial<ReadinessOptions> = {
    timeoutMs: options.wait?.timeoutMs,
    pollIntervalMs: options.wait?.pollIntervalMs,
  };

  const results = await dockerOps.waitForAllReady(services, readinessOptions);
  await stopLogSentinelOneShots(services, options);
  return results;
}

export async function startAndWaitForServiceBatches(
  batches: ReadonlyArray<readonly ServiceDefinition[]>,
  options: ServiceStartOptions & { wait?: ServiceWaitOptions },
): Promise<ReadinessResult[]> {
  const nonEmptyBatches = batches.filter((batch) => batch.length > 0);
  if (nonEmptyBatches.length === 0) {
    return [];
  }

  await Promise.all(
    nonEmptyBatches.map((services) => {
      const composeOptions = buildComposeOptions([...services], options);
      return dockerOps.composeUp(composeOptions);
    }),
  );

  const allServices = nonEmptyBatches.flat();
  const results = await dockerOps.waitForAllReady(allServices, {
    timeoutMs: options.wait?.timeoutMs,
    pollIntervalMs: options.wait?.pollIntervalMs,
  });
  await stopLogSentinelOneShots(allServices, options);
  return results;
}

export async function stopAllServices(options: {
  volumes?: boolean;
  timeout?: number;
  cwd?: string;
} = {}): Promise<void> {
  const generatedComposeFiles = await getGeneratedComposeFiles(options.cwd);
  const files = [...new Set([...getComposeFilesForServices([...SERVICE_MAP]), ...generatedComposeFiles])];

  await dockerOps.composeDown({
    project: DOCKER_PROJECT,
    files,
    volumes: options.volumes,
    timeout: options.timeout,
    cwd: options.cwd,
  });
}

export async function stopServices(
  serviceNames: string[],
  options: { cwd?: string; definitions?: readonly ServiceDefinition[] } = {},
): Promise<void> {
  if (serviceNames.length === 0) {
    return;
  }

  const availableServices = collectServices(options.definitions ?? []);
  const services = serviceNames
    .map((name) => availableServices.find((service) => service.name === name) ?? getServiceByName(name))
    .filter((service): service is ServiceDefinition => service !== undefined);
  if (services.length !== serviceNames.length) {
    const unknown = serviceNames.filter((name) => !services.find((service) => service.name === name));
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "docker-services",
      message: `unknown service(s): ${unknown.join(", ")}`,
    });
  }

  const composeOptions = {
    project: DOCKER_PROJECT,
    files: getComposeFilesForServices(services),
    cwd: options.cwd,
  };
  await dockerOps.composeStop(serviceNames, composeOptions);
}

export async function restartServices(
  services: ServiceDefinition[],
  options: ServiceStartOptions & { wait?: ServiceWaitOptions },
): Promise<ReadinessResult[]> {
  if (services.length === 0) {
    return [];
  }

  const composeOptions = buildComposeOptions(services, options);
  const names = services.map((service) => service.name);
  await dockerOps.composeStop(names, composeOptions);
  await dockerOps.composeStart(names, composeOptions);

  const results = await dockerOps.waitForAllReady(services, {
    timeoutMs: options.wait?.timeoutMs,
    pollIntervalMs: options.wait?.pollIntervalMs,
  });

  await stopLogSentinelOneShots(services, options);
  return results;
}

export async function getProjectStatus(options: {
  cwd?: string;
  services?: readonly ServiceDefinition[];
} = {}): Promise<Map<string, ContainerInfo>> {
  const services = collectServices(options.services);
  const discovered = await dockerOps.listProjectContainers(DOCKER_PROJECT, { all: true });
  const byContainer = new Map(discovered.map((container) => [container.name, container]));
  const byService = new Map(discovered.map((container) => [container.service, container]));

  const status = new Map<string, ContainerInfo>();
  for (const service of services) {
    const found = byService.get(service.name) ?? byContainer.get(service.containerName);
    status.set(
      service.name,
      found ?? {
        name: service.containerName,
        service: service.name,
        state: "not-found",
        health: "none",
      },
    );
  }

  if (discovered.length === 0) {
    try {
      const composeStatus = await dockerOps.composePs({
        project: DOCKER_PROJECT,
        files: getComposeFilesForServices(services),
        cwd: options.cwd,
      });

      for (const container of composeStatus) {
        if (!status.has(container.service)) {
          status.set(container.service, container);
        }
      }
    } catch {
      // Ignore compose ps errors and rely on defaults.
    }
  }

  return status;
}

export async function discoverMinioIp(containerName = "fhevm-minio"): Promise<string> {
  const ip = await getContainerIp(containerName);
  if (ip) {
    return ip;
  }

  throw new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    step: "minio-ip-discovery",
    service: containerName,
    message: "could not discover MinIO container IP address",
    logHint: "fhevm-cli logs fhevm-minio",
  });
}

export const __internal = {
  resetDockerOpsForTests(): void {
    dockerOps = DEFAULT_DOCKER_OPS;
  },
  setDockerOpsForTests(overrides: Partial<DockerServiceOps>): void {
    dockerOps = { ...DEFAULT_DOCKER_OPS, ...overrides };
  },
};
