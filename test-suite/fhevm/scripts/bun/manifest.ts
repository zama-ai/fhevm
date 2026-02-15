export type ServiceState = "running" | "complete";

export interface ServiceCheck {
  service: string;
  state: ServiceState;
}

export interface DeploymentStep {
  name: string;
  component?: string;
  description: string;
  buildable: boolean;
  serviceChecks: ServiceCheck[];
}

export interface VersionEntry {
  envVar: string;
  defaultValue: string;
  group: string;
  displayName: string;
  appendBuildTag: boolean;
  groupOverrideEnv?: string;
}

export interface TestTypeConfig {
  logMessage?: string;
  grep?: string;
  parallel?: boolean;
  debugShell?: boolean;
}

export const PROJECT = "fhevm";
export const DEFAULT_OTEL_EXPORTER_OTLP_ENDPOINT = "http://jaeger:4317";

export const DEFAULT_STACK_VERSION = "v0.11.0-1";
export const DEFAULT_CORE_VERSION = "v0.13.0-rc.2";
export const DEFAULT_RELAYER_VERSION = "v0.9.0-rc.1";

export const STACK_VERSION_OVERRIDE_ENV = "FHEVM_STACK_VERSION";
export const CORE_VERSION_OVERRIDE_ENV = "FHEVM_CORE_VERSION";
export const RELAYER_VERSION_OVERRIDE_ENV = "FHEVM_RELAYER_VERSION";

export const DEPLOYMENT_STEPS: DeploymentStep[] = [
  {
    name: "minio",
    component: "minio",
    description: "MinIO Services",
    buildable: false,
    serviceChecks: [
      { service: "fhevm-minio", state: "running" },
      { service: "fhevm-minio-setup", state: "complete" },
    ],
  },
  {
    name: "core",
    component: "core",
    description: "Core Services",
    buildable: false,
    serviceChecks: [{ service: "kms-core", state: "running" }],
  },
  {
    name: "kms-signer",
    description: "KMS signer setup",
    buildable: false,
    serviceChecks: [],
  },
  {
    name: "database",
    component: "database",
    description: "Database service",
    buildable: false,
    serviceChecks: [{ service: "coprocessor-and-kms-db", state: "running" }],
  },
  {
    name: "host-node",
    component: "host-node",
    description: "Host node service",
    buildable: true,
    serviceChecks: [{ service: "host-node", state: "running" }],
  },
  {
    name: "gateway-node",
    component: "gateway-node",
    description: "Gateway node service",
    buildable: true,
    serviceChecks: [{ service: "gateway-node", state: "running" }],
  },
  {
    name: "coprocessor",
    component: "coprocessor",
    description: "Coprocessor Services",
    buildable: true,
    serviceChecks: [
      { service: "coprocessor-and-kms-db", state: "running" },
      { service: "coprocessor-db-migration", state: "complete" },
      { service: "coprocessor-host-listener", state: "running" },
      { service: "coprocessor-host-listener-poller", state: "running" },
      { service: "coprocessor-gw-listener", state: "running" },
      { service: "coprocessor-tfhe-worker", state: "running" },
      { service: "coprocessor-zkproof-worker", state: "running" },
      { service: "coprocessor-sns-worker", state: "running" },
      { service: "coprocessor-transaction-sender", state: "running" },
    ],
  },
  {
    name: "kms-connector",
    component: "kms-connector",
    description: "KMS Connector Services",
    buildable: true,
    serviceChecks: [
      { service: "coprocessor-and-kms-db", state: "running" },
      { service: "kms-connector-db-migration", state: "complete" },
      { service: "kms-connector-gw-listener", state: "running" },
      { service: "kms-connector-kms-worker", state: "running" },
      { service: "kms-connector-tx-sender", state: "running" },
    ],
  },
  {
    name: "gateway-mocked-payment",
    component: "gateway-mocked-payment",
    description: "Gateway mocked payment",
    buildable: true,
    serviceChecks: [
      { service: "gateway-deploy-mocked-zama-oft", state: "complete" },
      { service: "gateway-set-relayer-mocked-payment", state: "complete" },
    ],
  },
  {
    name: "gateway-sc",
    component: "gateway-sc",
    description: "Gateway contracts",
    buildable: true,
    serviceChecks: [
      { service: "gateway-sc-deploy", state: "complete" },
      { service: "gateway-sc-add-network", state: "complete" },
      { service: "gateway-sc-trigger-keygen", state: "complete" },
      { service: "gateway-sc-trigger-crsgen", state: "complete" },
      { service: "gateway-sc-add-pausers", state: "complete" },
    ],
  },
  {
    name: "host-sc",
    component: "host-sc",
    description: "Host contracts",
    buildable: true,
    serviceChecks: [
      { service: "host-sc-deploy", state: "complete" },
      { service: "host-sc-add-pausers", state: "complete" },
    ],
  },
  {
    name: "relayer",
    component: "relayer",
    description: "Relayer Services",
    buildable: true,
    serviceChecks: [{ service: "fhevm-relayer", state: "running" }],
  },
  {
    name: "test-suite",
    component: "test-suite",
    description: "Test Suite E2E Tests",
    buildable: true,
    serviceChecks: [{ service: "fhevm-test-suite-e2e-debug", state: "running" }],
  },
];

export const VERSION_ENTRIES: VersionEntry[] = [
  {
    envVar: "GATEWAY_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Contracts",
    displayName: "gateway-contracts",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "HOST_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Contracts",
    displayName: "host-contracts",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_DB_MIGRATION_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/db-migration",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_GW_LISTENER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/gw-listener",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/host-listener",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/poller",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_TX_SENDER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/tx-sender",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_TFHE_WORKER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/tfhe-worker",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_SNS_WORKER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/sns-worker",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Coprocessor Services",
    displayName: "coprocessor/zkproof-worker",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "CONNECTOR_DB_MIGRATION_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM KMS Connector Services",
    displayName: "kms-connector/db-migration",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "CONNECTOR_GW_LISTENER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM KMS Connector Services",
    displayName: "kms-connector/gw-listener",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "CONNECTOR_KMS_WORKER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM KMS Connector Services",
    displayName: "kms-connector/kms-worker",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "CONNECTOR_TX_SENDER_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM KMS Connector Services",
    displayName: "kms-connector/tx-sender",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "TEST_SUITE_VERSION",
    defaultValue: DEFAULT_STACK_VERSION,
    group: "FHEVM Test Suite",
    displayName: "test-suite/e2e",
    appendBuildTag: true,
    groupOverrideEnv: STACK_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "CORE_VERSION",
    defaultValue: DEFAULT_CORE_VERSION,
    group: "External Dependencies",
    displayName: "kms-core-service",
    appendBuildTag: false,
    groupOverrideEnv: CORE_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "RELAYER_VERSION",
    defaultValue: DEFAULT_RELAYER_VERSION,
    group: "External Dependencies",
    displayName: "fhevm-relayer",
    appendBuildTag: false,
    groupOverrideEnv: RELAYER_VERSION_OVERRIDE_ENV,
  },
  {
    envVar: "RELAYER_MIGRATE_VERSION",
    defaultValue: DEFAULT_RELAYER_VERSION,
    group: "External Dependencies",
    displayName: "fhevm-relayer-migrate",
    appendBuildTag: false,
    groupOverrideEnv: RELAYER_VERSION_OVERRIDE_ENV,
  },
];

export const LOCAL_CACHE_SERVICES: string[] = [
  "gateway-deploy-mocked-zama-oft",
  "gateway-sc-add-network",
  "gateway-sc-add-pausers",
  "gateway-sc-deploy",
  "gateway-sc-pause",
  "gateway-sc-trigger-crsgen",
  "gateway-sc-trigger-keygen",
  "gateway-sc-unpause",
  "gateway-set-relayer-mocked-payment",
  "host-sc-add-pausers",
  "host-sc-deploy",
  "host-sc-pause",
  "host-sc-unpause",
  "kms-connector-db-migration",
  "test-suite-e2e-debug",
];

export const TELEMETRY_REQUIRED_JAEGER_SERVICES: string[] = [
  "host-listener",
  "host-listener-poller",
  "tfhe-worker",
  "txn-sender",
  "sns-executor",
  "zkproof-worker",
];

export const UPGRADE_SERVICES: string[] = [
  "minio",
  "core",
  "gateway-node",
  "gateway-sc",
  "gateway-mocked-payment",
  "host-node",
  "host-sc",
  "kms-connector",
  "coprocessor",
  "relayer",
  "test-suite",
];

export const TEST_TYPE_CONFIG: Record<string, TestTypeConfig> = {
  "input-proof": {
    logMessage: "[TEST] INPUT PROOF (uint64)",
    grep: "test user input uint64",
  },
  "input-proof-compute-decrypt": {
    logMessage: "[TEST] INPUT PROOF (uint64)",
    grep: "test add 42 to uint64 input and decrypt",
  },
  "user-decryption": {
    logMessage: "[TEST] USER DECRYPTION",
    grep: "test user decrypt",
  },
  "delegated-user-decryption": {
    logMessage: "[TEST] DELEGATED USER DECRYPTION",
    grep: "test delegated user decrypt",
  },
  "public-decryption": {
    logMessage: "[TEST] PUBLIC DECRYPTION",
    grep: "test async decrypt (uint.*|ebytes.* trivial|ebytes64 non-trivial|ebytes256 non-trivial with snapshot|addresses|several addresses)",
  },
  erc20: {
    logMessage: "[TEST] ERC20",
    grep: "should transfer tokens between two users.",
  },
  "public-decrypt-http-ebool": {
    logMessage: "[TEST] PUBLIC DECRYPTION OVER HTTP FOR EBOOL",
    grep: "test HTTPPublicDecrypt ebool",
  },
  "public-decrypt-http-mixed": {
    logMessage: "[TEST] PUBLIC DECRYPTION OVER HTTP FOR MIXED",
    grep: "test HTTPPublicDecrypt mixed",
  },
  operators: {
    logMessage: "[TEST] OPERATORS",
    parallel: true,
    grep: "test operator|FHEVM manual operations",
  },
  random: {
    logMessage: "[TEST] RANDOM OPERATORS",
    grep: "generate and decrypt|generating rand in reverting sub-call|upper bound and decrypt",
  },
  "random-subset": {
    logMessage: "[TEST] RANDOM OPERATORS (SUBSET)",
    grep: "64 bits generate and decrypt|generating rand in reverting sub-call|64 bits generate with upper bound and decrypt",
  },
  "paused-host-contracts": {
    logMessage: "[TEST] PAUSED HOST CONTRACTS",
    grep: "test paused host.*",
  },
  "paused-gateway-contracts": {
    logMessage: "[TEST] PAUSED GATEWAY CONTRACTS",
    grep: "test paused gateway.*",
  },
  debug: {
    debugShell: true,
  },
};

export const COLORS = {
  blue: "\x1b[0;34m",
  lightBlue: "\x1b[1;34m",
  green: "\x1b[0;32m",
  red: "\x1b[0;31m",
  yellow: "\x1b[0;33m",
  purple: "\x1b[0;35m",
  cyan: "\x1b[0;36m",
  bold: "\x1b[1m",
  reset: "\x1b[0m",
} as const;
