export type ServiceGroup =
  | "infra"
  | "coprocessor"
  | "kms-connector"
  | "contracts"
  | "core"
  | "relayer"
  | "test-suite";

export type HealthCheckType =
  | "docker-compose-healthcheck"
  | "rpc"
  | "http"
  | "log-sentinel"
  | "docker-state"
  | "exit-code";

export const ENV_FILE_NAMES = [
  "coprocessor",
  "kms-connector",
  "kms-core",
  "database",
  "minio",
  "gateway-sc",
  "host-sc",
  "relayer",
  "gateway-node",
  "host-node",
  "test-suite",
  "gateway-mocked-payment",
] as const;

export type EnvFileName = (typeof ENV_FILE_NAMES)[number];

export interface ServiceDefinition {
  name: string;
  group: ServiceGroup;
  composeFile: string;
  envFile: string;
  versionVar?: string;
  containerName: string;
  isOneShot: boolean;
  isBuildable: boolean;
  healthCheck: HealthCheckType;
  healthEndpoint?: string;
  ports?: number[];
}

export interface CoprocessorServiceTemplate {
  suffix: string;
  versionVar: string;
  isOneShot: boolean;
  isBuildable: boolean;
  healthCheck: HealthCheckType;
}

export const COPROCESSOR_TEMPLATES: readonly CoprocessorServiceTemplate[] = [
  {
    suffix: "db-migration",
    versionVar: "COPROCESSOR_DB_MIGRATION_VERSION",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    suffix: "host-listener",
    versionVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    suffix: "host-listener-poller",
    versionVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    suffix: "gw-listener",
    versionVar: "COPROCESSOR_GW_LISTENER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    suffix: "tfhe-worker",
    versionVar: "COPROCESSOR_TFHE_WORKER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    suffix: "zkproof-worker",
    versionVar: "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    suffix: "sns-worker",
    versionVar: "COPROCESSOR_SNS_WORKER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    suffix: "transaction-sender",
    versionVar: "COPROCESSOR_TX_SENDER_VERSION",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
] as const;

const COMPOSE_ROOT = "test-suite/fhevm/docker-compose";

export const SERVICE_MAP: readonly ServiceDefinition[] = [
  {
    name: "db",
    group: "infra",
    composeFile: `${COMPOSE_ROOT}/database-docker-compose.yml`,
    envFile: "database",
    containerName: "coprocessor-and-kms-db",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "docker-compose-healthcheck",
    ports: [5432],
  },
  {
    name: "minio",
    group: "infra",
    composeFile: `${COMPOSE_ROOT}/minio-docker-compose.yml`,
    envFile: "minio",
    containerName: "fhevm-minio",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "docker-compose-healthcheck",
    ports: [9000, 9001],
  },
  {
    name: "minio-setup",
    group: "infra",
    composeFile: `${COMPOSE_ROOT}/minio-docker-compose.yml`,
    envFile: "minio",
    containerName: "fhevm-minio-setup",
    isOneShot: true,
    isBuildable: false,
    healthCheck: "exit-code",
  },
  {
    name: "host-node",
    group: "infra",
    composeFile: `${COMPOSE_ROOT}/host-node-docker-compose.yml`,
    envFile: "host-node",
    containerName: "host-node",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "rpc",
    healthEndpoint: "http://localhost:8545",
    ports: [8545],
  },
  {
    name: "gateway-node",
    group: "infra",
    composeFile: `${COMPOSE_ROOT}/gateway-node-docker-compose.yml`,
    envFile: "gateway-node",
    containerName: "gateway-node",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "rpc",
    healthEndpoint: "http://localhost:8546",
    ports: [8546],
  },
  {
    name: "kms-core",
    group: "core",
    composeFile: `${COMPOSE_ROOT}/core-docker-compose.yml`,
    envFile: "kms-core",
    versionVar: "CORE_VERSION",
    containerName: "kms-core",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "log-sentinel",
    ports: [50051],
  },
  {
    name: "coprocessor-db-migration",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_DB_MIGRATION_VERSION",
    containerName: "coprocessor-db-migration",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "coprocessor-host-listener",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    containerName: "coprocessor-host-listener",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "coprocessor-host-listener-poller",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    containerName: "coprocessor-host-listener-poller",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "coprocessor-gw-listener",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_GW_LISTENER_VERSION",
    containerName: "coprocessor-gw-listener",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
    ports: [8080],
  },
  {
    name: "coprocessor-tfhe-worker",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_TFHE_WORKER_VERSION",
    containerName: "coprocessor-tfhe-worker",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "coprocessor-zkproof-worker",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    containerName: "coprocessor-zkproof-worker",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "coprocessor-sns-worker",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_SNS_WORKER_VERSION",
    containerName: "coprocessor-sns-worker",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "coprocessor-transaction-sender",
    group: "coprocessor",
    composeFile: `${COMPOSE_ROOT}/coprocessor-docker-compose.yml`,
    envFile: "coprocessor",
    versionVar: "COPROCESSOR_TX_SENDER_VERSION",
    containerName: "coprocessor-transaction-sender",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "kms-connector-db-migration",
    group: "kms-connector",
    composeFile: `${COMPOSE_ROOT}/kms-connector-docker-compose.yml`,
    envFile: "kms-connector",
    versionVar: "CONNECTOR_DB_MIGRATION_VERSION",
    containerName: "kms-connector-db-migration",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "kms-connector-gw-listener",
    group: "kms-connector",
    composeFile: `${COMPOSE_ROOT}/kms-connector-docker-compose.yml`,
    envFile: "kms-connector",
    versionVar: "CONNECTOR_GW_LISTENER_VERSION",
    containerName: "kms-connector-gw-listener",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "kms-connector-kms-worker",
    group: "kms-connector",
    composeFile: `${COMPOSE_ROOT}/kms-connector-docker-compose.yml`,
    envFile: "kms-connector",
    versionVar: "CONNECTOR_KMS_WORKER_VERSION",
    containerName: "kms-connector-kms-worker",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "kms-connector-tx-sender",
    group: "kms-connector",
    composeFile: `${COMPOSE_ROOT}/kms-connector-docker-compose.yml`,
    envFile: "kms-connector",
    versionVar: "CONNECTOR_TX_SENDER_VERSION",
    containerName: "kms-connector-tx-sender",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
  {
    name: "gateway-sc-deploy",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-sc-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-deploy",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-sc-add-network",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-sc-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-add-network",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-sc-add-pausers",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-sc-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-add-pausers",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-sc-trigger-keygen",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-sc-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-trigger-keygen",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-sc-trigger-crsgen",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-sc-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-trigger-crsgen",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-sc-pause",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-pause-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-pause",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-sc-unpause",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-unpause-docker-compose.yml`,
    envFile: "gateway-sc",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-sc-unpause",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-deploy-mocked-zama-oft",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-mocked-payment-docker-compose.yml`,
    envFile: "gateway-mocked-payment",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-deploy-mocked-zama-oft",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "gateway-set-relayer-mocked-payment",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/gateway-mocked-payment-docker-compose.yml`,
    envFile: "gateway-mocked-payment",
    versionVar: "GATEWAY_VERSION",
    containerName: "gateway-set-relayer-mocked-payment",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "host-sc-deploy",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/host-sc-docker-compose.yml`,
    envFile: "host-sc",
    versionVar: "HOST_VERSION",
    containerName: "host-sc-deploy",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "log-sentinel",
  },
  {
    name: "host-sc-add-pausers",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/host-sc-docker-compose.yml`,
    envFile: "host-sc",
    versionVar: "HOST_VERSION",
    containerName: "host-sc-add-pausers",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "host-sc-pause",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/host-pause-docker-compose.yml`,
    envFile: "host-sc",
    versionVar: "HOST_VERSION",
    containerName: "host-sc-pause",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "host-sc-unpause",
    group: "contracts",
    composeFile: `${COMPOSE_ROOT}/host-unpause-docker-compose.yml`,
    envFile: "host-sc",
    versionVar: "HOST_VERSION",
    containerName: "host-sc-unpause",
    isOneShot: true,
    isBuildable: true,
    healthCheck: "exit-code",
  },
  {
    name: "relayer-db",
    group: "relayer",
    composeFile: `${COMPOSE_ROOT}/relayer-docker-compose.yml`,
    envFile: "relayer",
    containerName: "fhevm-relayer-db",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "docker-compose-healthcheck",
    ports: [5433],
  },
  {
    name: "relayer-db-migration",
    group: "relayer",
    composeFile: `${COMPOSE_ROOT}/relayer-docker-compose.yml`,
    envFile: "relayer",
    versionVar: "RELAYER_MIGRATE_VERSION",
    containerName: "relayer-db-migration",
    isOneShot: true,
    isBuildable: false,
    healthCheck: "exit-code",
    ports: [3001],
  },
  {
    name: "relayer",
    group: "relayer",
    composeFile: `${COMPOSE_ROOT}/relayer-docker-compose.yml`,
    envFile: "relayer",
    versionVar: "RELAYER_VERSION",
    containerName: "fhevm-relayer",
    isOneShot: false,
    isBuildable: false,
    healthCheck: "log-sentinel",
    ports: [3000],
  },
  {
    name: "test-suite-e2e-debug",
    group: "test-suite",
    composeFile: `${COMPOSE_ROOT}/test-suite-docker-compose.yml`,
    envFile: "test-suite",
    versionVar: "TEST_SUITE_VERSION",
    containerName: "fhevm-test-suite-e2e-debug",
    isOneShot: false,
    isBuildable: true,
    healthCheck: "docker-state",
  },
] as const;

export function getServiceByName(name: string): ServiceDefinition | undefined {
  return SERVICE_MAP.find((service) => service.name === name);
}

export function getServicesByGroup(group: ServiceGroup): ServiceDefinition[] {
  return SERVICE_MAP.filter((service) => service.group === group);
}

export function getServicesByEnvFile(envFile: EnvFileName): ServiceDefinition[] {
  return SERVICE_MAP.filter((service) => service.envFile === envFile);
}

export function getComposeFilesForServices(services: ServiceDefinition[]): string[] {
  return [...new Set(services.map((service) => service.composeFile))];
}

const COPROCESSOR_ALL = [
  "coprocessor-db-migration",
  "coprocessor-host-listener",
  "coprocessor-host-listener-poller",
  "coprocessor-gw-listener",
  "coprocessor-tfhe-worker",
  "coprocessor-zkproof-worker",
  "coprocessor-sns-worker",
  "coprocessor-transaction-sender",
] as const;

const KMS_CONNECTOR_ALL = [
  "kms-connector-db-migration",
  "kms-connector-gw-listener",
  "kms-connector-kms-worker",
  "kms-connector-tx-sender",
] as const;

export const LOCAL_COMPONENT_MAP: Readonly<Record<string, readonly string[]>> = {
  coprocessor: COPROCESSOR_ALL,
  "kms-connector": KMS_CONNECTOR_ALL,
  "tfhe-worker": ["coprocessor-tfhe-worker"],
  "sns-worker": ["coprocessor-sns-worker"],
  "zkproof-worker": ["coprocessor-zkproof-worker"],
  "host-listener": ["coprocessor-host-listener", "coprocessor-host-listener-poller"],
  "gw-listener": ["coprocessor-gw-listener"],
  "transaction-sender": ["coprocessor-transaction-sender"],
  "kms-gw-listener": ["kms-connector-gw-listener"],
  "kms-worker": ["kms-connector-kms-worker"],
  "kms-tx-sender": ["kms-connector-tx-sender"],
  relayer: ["relayer", "relayer-db", "relayer-db-migration"],
  "gateway-contracts": [
    "gateway-sc-deploy",
    "gateway-sc-add-network",
    "gateway-sc-add-pausers",
    "gateway-sc-trigger-keygen",
    "gateway-sc-trigger-crsgen",
    "gateway-sc-pause",
    "gateway-sc-unpause",
    "gateway-deploy-mocked-zama-oft",
    "gateway-set-relayer-mocked-payment",
  ],
  "host-contracts": ["host-sc-deploy", "host-sc-add-pausers", "host-sc-pause", "host-sc-unpause"],
};
