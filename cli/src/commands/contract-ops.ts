import { getDotFhevmPaths, type DotFhevmPaths } from "../config/dotfhevm";
import { findExistingEnvFiles } from "../config/env-loader";
import { getServiceByName, type ServiceDefinition } from "../config/service-map";
import { buildVersionEnvVars, resolveAllVersions } from "../config/versions";
import { startAndWaitForServices } from "../docker/services";
import { ExitCode, FhevmCliError } from "../errors";

export type ContractTarget = "gateway" | "host";
export type ContractAction = "pause" | "unpause";

export interface ContractActionDeps {
  findExistingEnvFiles: typeof findExistingEnvFiles;
  resolveAllVersions: typeof resolveAllVersions;
  buildVersionEnvVars: typeof buildVersionEnvVars;
  startAndWaitForServices: typeof startAndWaitForServices;
}

const DEFAULT_DEPS: ContractActionDeps = {
  findExistingEnvFiles,
  resolveAllVersions,
  buildVersionEnvVars,
  startAndWaitForServices,
};

function isContractTarget(value: string): value is ContractTarget {
  return value === "gateway" || value === "host";
}

export function resolveContractActionService(target: string, action: ContractAction): ServiceDefinition {
  if (!isContractTarget(target)) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: action,
      message: `invalid target: '${target}'. Expected: gateway or host`,
    });
  }

  const serviceName = `${target}-sc-${action}`;
  const service = getServiceByName(serviceName);
  if (!service) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: action,
      message: `missing service definition: ${serviceName}`,
    });
  }

  return service;
}

export async function runContractAction(
  target: string,
  action: ContractAction,
  paths: DotFhevmPaths = getDotFhevmPaths(),
  deps: ContractActionDeps = DEFAULT_DEPS,
): Promise<ServiceDefinition> {
  const service = resolveContractActionService(target, action);
  const envFileByName = deps.findExistingEnvFiles(paths.env);

  if (!envFileByName.has(service.envFile)) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: action,
      message: `missing env file for: ${service.envFile}; run 'fhevm-cli up' first`,
    });
  }

  const versions = await deps.resolveAllVersions(paths.versionCache);
  const envVars = deps.buildVersionEnvVars(versions);

  await deps.startAndWaitForServices([service], {
    envFileByName,
    envVars,
  });

  return service;
}
