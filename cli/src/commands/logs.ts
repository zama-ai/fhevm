import { defineCommand } from "citty";

import {
  LOCAL_COMPONENT_MAP,
  SERVICE_MAP,
  getComposeFilesForServices,
  getServiceByName,
  type ServiceDefinition,
} from "../config/service-map";
import { buildVersionEnvVars, resolveAllVersions } from "../config/versions";
import { composeLogs } from "../docker/compose";
import { DOCKER_PROJECT, type ComposeLogsOptions } from "../docker/types";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";

export interface LogsCommandArgs {
  service?: string;
  follow: boolean;
  json: boolean;
  tail?: string;
}

const DEFAULT_TAIL = 100;

export function resolveLogServices(name?: string): ServiceDefinition[] {
  if (!name) {
    return [...SERVICE_MAP];
  }

  const exact = getServiceByName(name);
  if (exact) {
    return [exact];
  }

  const mapped = LOCAL_COMPONENT_MAP[name];
  if (mapped) {
    const services = mapped
      .map((serviceName) => getServiceByName(serviceName))
      .filter((service): service is ServiceDefinition => Boolean(service));
    if (services.length > 0) {
      return services;
    }
  }

  const validNames = [...new Set([...Object.keys(LOCAL_COMPONENT_MAP), ...SERVICE_MAP.map((service) => service.name)])]
    .sort()
    .join(", ");

  throw new FhevmCliError({
    exitCode: ExitCode.CONFIG,
    step: "logs",
    message: `unknown service: '${name}'. Valid names: ${validNames}`,
  });
}

export function parseTail(value: string | undefined, follow: boolean): number | undefined {
  if (value === undefined || value === "") {
    return follow ? undefined : DEFAULT_TAIL;
  }

  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 0) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "logs",
      message: `invalid integer for --tail: ${value}`,
    });
  }

  return parsed;
}

export function buildLogsComposeOptions(args: LogsCommandArgs): ComposeLogsOptions {
  const services = resolveLogServices(args.service);
  const follow = args.follow;
  const tail = parseTail(args.tail, follow);

  return {
    project: DOCKER_PROJECT,
    files: getComposeFilesForServices(services),
    services: args.service ? services.map((service) => service.name) : undefined,
    follow,
    tail,
    noColor: args.json,
    format: args.json ? "json" : undefined,
  };
}

export async function runLogsCommand(args: LogsCommandArgs): Promise<void> {
  const options = buildLogsComposeOptions(args);
  const versions = await resolveAllVersions();
  options.envVars = { ...options.envVars, ...buildVersionEnvVars(versions) };
  await composeLogs(options);
}

export function toLogsError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "logs",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "logs",
    description: "Show service logs",
  },
  args: {
    service: { type: "positional", required: false, description: "Optional service name" },
    follow: { type: "boolean", alias: "f", required: false, description: "Follow logs" },
    json: { type: "boolean", required: false, description: "JSON output" },
    tail: { type: "string", required: false, description: "Number of lines to show" },
  },
  async run({ args }) {
    try {
      await runLogsCommand({
        service: args.service,
        follow: args.follow ?? false,
        json: args.json ?? false,
        tail: args.tail,
      });
    } catch (error) {
      exitWithError(toLogsError(error));
    }
  },
});
