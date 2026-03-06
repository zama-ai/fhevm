import { defineCommand } from "citty";

import { getDotFhevmPaths, type DotFhevmPaths } from "../config/dotfhevm";
import { findExistingEnvFiles } from "../config/env-loader";
import { LOCAL_COMPONENT_MAP, SERVICE_MAP, getServiceByName, type ServiceDefinition } from "../config/service-map";
import { getRunHintForService } from "../docker/local";
import { stopServices } from "../docker/services";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";
import { dim, green } from "../utils/output";

export interface RestartCommandResult {
  services: ServiceDefinition[];
  hints: ReadonlyMap<string, string>;
}

export function resolveRestartServices(name: string): ServiceDefinition[] {
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
    step: "restart",
    message: `unknown service: '${name}'. Valid names: ${validNames}`,
  });
}

export function buildRestartHints(services: ServiceDefinition[]): ReadonlyMap<string, string> {
  const hints = new Map<string, string>();

  for (const service of services) {
    const hint = getRunHintForService(service.name);
    if (hint) {
      hints.set(service.name, hint);
    }
  }

  return hints;
}

export async function runRestartCommand(
  name: string,
  paths: DotFhevmPaths = getDotFhevmPaths(),
): Promise<RestartCommandResult> {
  const services = resolveRestartServices(name);
  const envFileByName = findExistingEnvFiles(paths.env);

  const missingEnvFiles = [...new Set(services.map((service) => service.envFile))].filter(
    (envFileName) => !envFileByName.has(envFileName),
  );

  if (missingEnvFiles.length > 0) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "restart",
      message: `missing env file(s) for: ${missingEnvFiles.join(", ")}; run 'fhevm-cli up' first`,
    });
  }

  await stopServices(services.map((service) => service.name));

  return {
    services,
    hints: buildRestartHints(services),
  };
}

export function toRestartError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "restart",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "restart",
    description: "Stop a service container and print local run command",
  },
  args: {
    service: { type: "positional", required: true, description: "Service name" },
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    const service = args.service;
    const json = args.json ?? false;

    try {
      const result = await runRestartCommand(service);
      if (json) {
        console.log(
          JSON.stringify({
            ok: true,
            command: "restart",
            services: result.services.map((definition) => definition.name),
            hints: Object.fromEntries(result.hints),
          }),
        );
        return;
      }

      for (const definition of result.services) {
        console.log(green(`Stopped ${definition.name}`));
      }

      for (const hint of result.hints.values()) {
        console.log(dim(`Run locally: ${hint}`));
      }
    } catch (error) {
      exitWithError(toRestartError(error), { json });
    }
  },
});
