import { composeEnvFilePath } from "../config/env-writer";
import {
  LOCAL_COMPONENT_MAP,
  SERVICE_MAP,
  getComposeFilesForServices,
  getServiceByName,
  type EnvFileName,
  type ServiceDefinition,
} from "../config/service-map";
import { ExitCode, FhevmCliError } from "../errors";

export const LOCAL_PORT_MAP: Readonly<Record<string, { port: number; envVarHint: string }>> = {
  postgres: { port: 5432, envVarHint: "DATABASE_URL" },
  relayerPostgres: { port: 5433, envVarHint: "DATABASE_URL" },
  hostRpc: { port: 8545, envVarHint: "HOST_CHAIN_RPC_URL" },
  gatewayRpc: { port: 8546, envVarHint: "GATEWAY_CHAIN_RPC_URL" },
  minioApi: { port: 9000, envVarHint: "AWS_ENDPOINT_URL" },
  kmsCore: { port: 50051, envVarHint: "KMS_ENDPOINT" },
  relayerHttp: { port: 3000, envVarHint: "RELAYER_URL" },
};

const RUN_HINTS: Readonly<Record<string, string>> = {
  "coprocessor-host-listener": "cargo run --bin host_listener",
  "coprocessor-host-listener-poller": "cargo run --bin host_listener_poller",
  "coprocessor-gw-listener": "cargo run --bin gw_listener",
  "coprocessor-tfhe-worker": "cargo run --bin tfhe_worker",
  "coprocessor-zkproof-worker": "cargo run --bin zkproof_worker",
  "coprocessor-sns-worker": "cargo run --bin sns_worker",
  "coprocessor-transaction-sender": "cargo run --bin transaction_sender",
  "kms-connector-gw-listener": "cargo run --bin kms_connector_gw_listener",
  "kms-connector-kms-worker": "cargo run --bin kms_connector_kms_worker",
  "kms-connector-tx-sender": "cargo run --bin kms_connector_tx_sender",
  "relayer": "./bin/server --config-file test-suite/fhevm/config/relayer/local.yaml",
};

function splitComponents(values: string[]): string[] {
  return values
    .flatMap((value) => value.split(","))
    .map((value) => value.trim())
    .filter(Boolean);
}

export function validateLocalComponents(components: string[]): void {
  const invalid = splitComponents(components).find((component) => !LOCAL_COMPONENT_MAP[component]);
  if (!invalid) {
    return;
  }

  const valid = Object.keys(LOCAL_COMPONENT_MAP).sort().join(", ");
  throw new FhevmCliError({
    exitCode: ExitCode.CONFIG,
    step: "local-mode",
    message: `unknown --local component '${invalid}'. Valid values: ${valid}`,
  });
}

export function resolveExcludedServices(localComponents: string[]): string[] {
  const parsed = splitComponents(localComponents);
  validateLocalComponents(parsed);

  const excluded = new Set<string>();
  for (const component of parsed) {
    for (const serviceName of LOCAL_COMPONENT_MAP[component] ?? []) {
      excluded.add(serviceName);
    }
  }

  return [...excluded];
}

export function filterDockerServices(localComponents: string[]): ServiceDefinition[] {
  const excluded = new Set(resolveExcludedServices(localComponents));
  return SERVICE_MAP.filter((service) => !excluded.has(service.name));
}

export function getActiveComposeFiles(localComponents: string[]): string[] {
  return getComposeFilesForServices(filterDockerServices(localComponents));
}

export function getLocalRunHints(localComponents: string[]): Map<string, string> {
  const hints = new Map<string, string>();

  for (const serviceName of resolveExcludedServices(localComponents)) {
    const hint = getRunHintForService(serviceName);
    if (!hint) {
      continue;
    }
    hints.set(serviceName, hint);
  }

  return hints;
}

export function getRunHintForService(serviceName: string): string | undefined {
  const hint = RUN_HINTS[serviceName];
  if (!hint) {
    return undefined;
  }

  const service = getServiceByName(serviceName);
  const envName = service?.envFile;
  if (!envName) {
    return hint;
  }

  return `source ${composeEnvFilePath(".fhevm/env", envName as EnvFileName)} && ${hint}`;
}
