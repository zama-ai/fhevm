import { ExitCode, FhevmCliError } from "../errors";
import type { ServiceDefinition } from "../config/service-map";

import { getContainerLogs } from "./containers";
import { toLogLines } from "./logs";
import {
  checkComposeHealthcheck,
  checkDockerState,
  checkExitCode,
  checkHttp,
  checkLogSentinel,
  checkRpc,
  detectCrash,
} from "./strategies";
import {
  DEFAULT_POLL_INTERVAL_MS,
  DEFAULT_TIMEOUTS,
  WAIT_TIMEOUT_ENV_NAME,
  type ReadinessOptions,
  type ReadinessResult,
} from "./types";

const LOG_SENTINEL_PATTERNS: Readonly<Record<string, string>> = {
  "kms-core": "Starting KMS core on socket",
  "relayer": "All servers are ready",
  "host-sc-deploy": "Contract deployment done!",
};

function getWaitTimeoutOverrideMs(): number | undefined {
  const raw = Bun.env[WAIT_TIMEOUT_ENV_NAME];
  if (!raw) {
    return undefined;
  }

  const seconds = Number.parseInt(raw, 10);
  if (!Number.isFinite(seconds) || seconds <= 0) {
    return undefined;
  }

  return seconds * 1_000;
}

function getRpcEndpoint(service: ServiceDefinition): string | undefined {
  return service.healthEndpoint;
}

function getLogSentinelPattern(service: ServiceDefinition): string | undefined {
  return LOG_SENTINEL_PATTERNS[service.name];
}

function readinessMessage(service: ServiceDefinition, suffix: string): string {
  return `${service.name} ${suffix}`;
}

function assertNever(value: never): never {
  throw new Error(`unsupported health check strategy: ${String(value)}`);
}

async function checkStrategy(service: ServiceDefinition): Promise<boolean> {
  const containerName = service.containerName;

  switch (service.healthCheck) {
    case "docker-compose-healthcheck":
      return checkComposeHealthcheck(containerName);
    case "rpc": {
      const endpoint = getRpcEndpoint(service);
      if (!endpoint) {
        return false;
      }
      return checkRpc(endpoint, "eth_chainId");
    }
    case "http": {
      if (!service.healthEndpoint) {
        return false;
      }
      return checkHttp(service.healthEndpoint);
    }
    case "log-sentinel": {
      const pattern = getLogSentinelPattern(service);
      if (!pattern) {
        return false;
      }
      return checkLogSentinel(containerName, pattern);
    }
    case "docker-state":
      return checkDockerState(containerName);
    case "exit-code":
      return checkExitCode(containerName);
    default:
      return assertNever(service.healthCheck);
  }
}

function toReadinessError(
  service: ServiceDefinition,
  message: string,
  logLines: string[] = [],
): FhevmCliError {
  return new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    step: "readiness",
    service: service.name,
    message,
    logLines,
  });
}

export function getDefaultTimeout(service: ServiceDefinition): number {
  const override = getWaitTimeoutOverrideMs();
  if (typeof override === "number") {
    return override;
  }

  if (service.name === "gateway-sc-trigger-keygen" || service.name === "gateway-sc-trigger-crsgen") {
    return DEFAULT_TIMEOUTS.keyBootstrapMs;
  }

  if (service.name === "relayer") {
    return DEFAULT_TIMEOUTS.relayerMs;
  }

  if (service.isOneShot) {
    return DEFAULT_TIMEOUTS.oneShotMs;
  }

  return DEFAULT_TIMEOUTS.serviceMs;
}

export async function waitForReady(
  service: ServiceDefinition,
  options: Partial<ReadinessOptions> = {},
): Promise<ReadinessResult> {
  const timeoutMs = options.timeoutMs ?? getDefaultTimeout(service);
  const pollIntervalMs = options.pollIntervalMs ?? DEFAULT_POLL_INTERVAL_MS;
  const startedAt = Date.now();

  while (true) {
    const elapsedMs = Date.now() - startedAt;
    options.onPoll?.(service.name, elapsedMs);

    const crash = await detectCrash(service.containerName);
    if (crash) {
      const exitCode = crash.exitCode ?? 1;
      const isOneShotSuccess = service.isOneShot && exitCode === 0;
      // Long-running services may exit 0 briefly during init (e.g. PostgreSQL
      // stops and restarts during first-run setup). Don't treat that as a crash
      // — the normal timeout will fire if the container never comes back.
      const isGracefulRestart = !service.isOneShot && exitCode === 0;

      if (!isOneShotSuccess && !isGracefulRestart) {
        options.onCrash?.(service.name, exitCode);
        const logLines = toLogLines(await getContainerLogs(service.containerName, { tail: 20 }));

        throw toReadinessError(
          service,
          readinessMessage(service, `crashed (${crash.state}, exit ${exitCode})`),
          logLines,
        );
      }
    }

    if (await checkStrategy(service)) {
      return {
        service: service.name,
        ready: true,
        elapsedMs,
      };
    }

    if (elapsedMs >= timeoutMs) {
      const logLines = toLogLines(await getContainerLogs(service.containerName, { tail: 20 }));

      throw toReadinessError(
        service,
        readinessMessage(service, `did not become ready within ${timeoutMs}ms`),
        logLines,
      );
    }

    await Bun.sleep(pollIntervalMs);
  }
}

export async function waitForAllReady(
  services: ServiceDefinition[],
  options: Partial<ReadinessOptions> = {},
): Promise<ReadinessResult[]> {
  return Promise.all(services.map((service) => waitForReady(service, options)));
}

export const __internal = {
  getWaitTimeoutOverrideMs,
};
