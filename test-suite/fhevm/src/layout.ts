/**
 * Defines CLI filesystem layout, compose and template locations, override groups, and named test profile metadata.
 */
import { existsSync } from "node:fs";
import path from "node:path";

import type { HostChainScenario, OverrideGroup, StepName } from "./types";

const CLI_DIR = path.resolve(import.meta.dir, "..");
export const REPO_ROOT = path.resolve(CLI_DIR, "../..");
export const DEFAULT_STATE_DIR = path.join(REPO_ROOT, ".fhevm");
const statePaths = (root: string) => {
  const stateDir = root || DEFAULT_STATE_DIR;
  const persistedStateDir = path.join(stateDir, "state");
  const runtimeDir = path.join(stateDir, "runtime");
  const envDir = path.join(runtimeDir, "env");
  const generatedConfigDir = path.join(runtimeDir, "config");
  const addressDir = path.join(runtimeDir, "addresses");
  return {
    STATE_DIR: stateDir,
    PERSISTED_STATE_DIR: persistedStateDir,
    RUNTIME_DIR: runtimeDir,
    ENV_DIR: envDir,
    COMPOSE_OUT_DIR: path.join(runtimeDir, "compose"),
    ADDRESS_DIR: addressDir,
    LOCK_DIR: path.join(persistedStateDir, "locks"),
    GENERATED_CONFIG_DIR: generatedConfigDir,
    STATE_FILE: path.join(persistedStateDir, "state.json"),
    versionsEnvPath: path.join(envDir, "versions.env"),
    relayerConfigPath: path.join(generatedConfigDir, "relayer.yaml"),
    kmsCoreConfigPath: path.join(generatedConfigDir, "kms-core.toml"),
    gatewayAddressesPath: path.join(addressDir, "gateway", ".env.gateway"),
  };
};
let currentStatePaths = statePaths(process.env.FHEVM_STATE_DIR ?? DEFAULT_STATE_DIR);
export let STATE_DIR = currentStatePaths.STATE_DIR;
export let PERSISTED_STATE_DIR = currentStatePaths.PERSISTED_STATE_DIR;
export let RUNTIME_DIR = currentStatePaths.RUNTIME_DIR;
export let ENV_DIR = currentStatePaths.ENV_DIR;
export let COMPOSE_OUT_DIR = currentStatePaths.COMPOSE_OUT_DIR;
export let ADDRESS_DIR = currentStatePaths.ADDRESS_DIR;
export let LOCK_DIR = currentStatePaths.LOCK_DIR;
export let GENERATED_CONFIG_DIR = currentStatePaths.GENERATED_CONFIG_DIR;
export let STATE_FILE = currentStatePaths.STATE_FILE;
export let versionsEnvPath = currentStatePaths.versionsEnvPath;
export let relayerConfigPath = currentStatePaths.relayerConfigPath;
export let kmsCoreConfigPath = currentStatePaths.kmsCoreConfigPath;
export let gatewayAddressesPath = currentStatePaths.gatewayAddressesPath;
export let gatewayAddressesSolidityPath = path.join(currentStatePaths.ADDRESS_DIR, "gateway", "GatewayAddresses.sol");
export let paymentBridgingAddressesSolidityPath = path.join(
  currentStatePaths.ADDRESS_DIR,
  "gateway",
  "PaymentBridgingAddresses.sol",
);
export const setStateDir = (root = process.env.FHEVM_STATE_DIR ?? DEFAULT_STATE_DIR) => {
  currentStatePaths = statePaths(root);
  STATE_DIR = currentStatePaths.STATE_DIR;
  PERSISTED_STATE_DIR = currentStatePaths.PERSISTED_STATE_DIR;
  RUNTIME_DIR = currentStatePaths.RUNTIME_DIR;
  ENV_DIR = currentStatePaths.ENV_DIR;
  COMPOSE_OUT_DIR = currentStatePaths.COMPOSE_OUT_DIR;
  ADDRESS_DIR = currentStatePaths.ADDRESS_DIR;
  LOCK_DIR = currentStatePaths.LOCK_DIR;
  GENERATED_CONFIG_DIR = currentStatePaths.GENERATED_CONFIG_DIR;
  STATE_FILE = currentStatePaths.STATE_FILE;
  versionsEnvPath = currentStatePaths.versionsEnvPath;
  relayerConfigPath = currentStatePaths.relayerConfigPath;
  kmsCoreConfigPath = currentStatePaths.kmsCoreConfigPath;
  gatewayAddressesPath = currentStatePaths.gatewayAddressesPath;
  gatewayAddressesSolidityPath = path.join(currentStatePaths.ADDRESS_DIR, "gateway", "GatewayAddresses.sol");
  paymentBridgingAddressesSolidityPath = path.join(
    currentStatePaths.ADDRESS_DIR,
    "gateway",
    "PaymentBridgingAddresses.sol",
  );
};
const TEMPLATE_DIR = path.join(CLI_DIR, "templates");
const PROFILE_DIR = path.join(CLI_DIR, "profiles");
export const TEMPLATE_ENV_DIR = path.join(TEMPLATE_DIR, "env");
const TEMPLATE_CONFIG_DIR = path.join(TEMPLATE_DIR, "config");
export const TEMPLATE_COMPOSE_DIR = path.join(CLI_DIR, "docker-compose");
const STATIC_CONFIG_DIR = path.join(CLI_DIR, "static", "config");
export const TEMPLATE_RELAYER_CONFIG = path.join(TEMPLATE_CONFIG_DIR, "relayer.yaml");
export const TEMPLATE_KMS_CORE_CONFIG_LEGACY = path.join(
  TEMPLATE_CONFIG_DIR,
  "kms-core-legacy.toml",
);
export const TEMPLATE_KMS_CORE_CONFIG_MODERN = path.join(
  TEMPLATE_CONFIG_DIR,
  "kms-core-modern.toml",
);
export const LATEST_SUPPORTED_PROFILE = path.join(PROFILE_DIR, "latest-supported.json");
export const PROJECT = "fhevm";
export const DEFAULT_HOST_RPC_PORT = 8545;
export const DEFAULT_GATEWAY_RPC_PORT = 8546;
export const DEFAULT_EXTRA_HOST_RPC_PORT = 8547;
export const MINIO_PORT = 9000;
export const POSTGRES_PORT = 5432;
export const DEFAULT_POSTGRES_USER = "postgres";
export const DEFAULT_POSTGRES_PASSWORD = "postgres";
export const DEFAULT_POSTGRES_DB = "coprocessor";
export const PORTS = [3000, 3001, POSTGRES_PORT, 5433, DEFAULT_HOST_RPC_PORT, DEFAULT_GATEWAY_RPC_PORT, DEFAULT_EXTRA_HOST_RPC_PORT, MINIO_PORT, 9001];
export const MINIO_INTERNAL_URL = `http://minio:${MINIO_PORT}`;
export const MINIO_EXTERNAL_URL = `http://localhost:${MINIO_PORT}`;
export const POSTGRES_HOST = `db:${POSTGRES_PORT}`;
export const COPROCESSOR_DB_CONTAINER = "coprocessor-and-kms-db";
export const KMS_CORE_CONTAINER = "kms-core";
export const TEST_SUITE_CONTAINER = "fhevm-test-suite-e2e-debug";
export const KEYGEN_ID_SELECTOR = "0xd52f10eb";
export const CRSGEN_ID_SELECTOR = "0xbaff211e";
export const DEFAULT_CHAIN_ID = "12345";

export const COMPONENTS = [
  "minio",
  "database",
  "core",
  "gateway-node",
  "host-node",
  "gateway-mocked-payment",
  "gateway-sc",
  "host-sc",
  "coprocessor",
  "kms-connector",
  "relayer",
  "test-suite",
] as const;

export const COMPONENT_BY_STEP: Record<StepName, string[]> = {
  "preflight": [],
  "resolve": [],
  "generate": [],
  "base": ["minio", "core", "database", "host-node", "gateway-node"],
  "kms-signer": [],
  "gateway-deploy": ["gateway-mocked-payment", "gateway-sc"],
  "host-deploy": ["host-sc"],
  "discover": [],
  "regenerate": [],
  "validate": [],
  "coprocessor": ["coprocessor"],
  "kms-connector": ["kms-connector"],
  "bootstrap": ["gateway-sc", "host-sc"],
  "relayer": ["relayer"],
  "test-suite": ["test-suite"],
};

export const LOG_TARGETS: Record<string, string> = {
  relayer: "fhevm-relayer",
  coprocessor: "coprocessor-gw-listener",
  "kms-connector": "kms-connector-gw-listener",
  gateway: "gateway-node",
  host: "host-node",
};

export const GROUP_BUILD_COMPONENTS: Record<OverrideGroup, string[]> = {
  "coprocessor": ["coprocessor"],
  "kms-connector": ["kms-connector"],
  "relayer": ["relayer"],
  "gateway-contracts": ["gateway-mocked-payment", "gateway-sc"],
  "host-contracts": ["host-sc"],
  "test-suite": ["test-suite"],
};

export const GROUP_BUILD_SERVICES: Record<OverrideGroup, string[]> = {
  "coprocessor": [
    "coprocessor-db-migration",
    "coprocessor-host-listener",
    "coprocessor-host-listener-poller",
    "coprocessor-host-listener-consumer",
    "coprocessor-gw-listener",
    "coprocessor-tfhe-worker",
    "coprocessor-zkproof-worker",
    "coprocessor-sns-worker",
    "coprocessor-transaction-sender",
  ],
  "kms-connector": [
    "kms-connector-db-migration",
    "kms-connector-gw-listener",
    "kms-connector-kms-worker",
    "kms-connector-tx-sender",
  ],
  "relayer": [
    "relayer-db-migration",
    "relayer",
  ],
  "gateway-contracts": [
    "gateway-deploy-mocked-zama-oft",
    "gateway-set-relayer-mocked-payment",
    "gateway-sc-deploy",
    "gateway-sc-add-network",
    "gateway-sc-add-pausers",
    "gateway-sc-trigger-keygen",
    "gateway-sc-trigger-crsgen",
  ],
  "host-contracts": ["host-sc-deploy", "host-sc-add-pausers"],
  "test-suite": ["test-suite-e2e-debug"],
};

const SERVICE_OVERRIDE_GROUPS = ["coprocessor", "kms-connector", "test-suite"] as const;
const GROUP_PREFIX: Record<OverrideGroup, string> = {
  "coprocessor": "coprocessor-",
  "kms-connector": "kms-connector-",
  "relayer": "relayer-",
  "gateway-contracts": "gateway-",
  "host-contracts": "host-",
  "test-suite": "test-suite-",
};

export const GROUP_SERVICE_SUFFIXES: Record<OverrideGroup, string[]> = Object.fromEntries(
  Object.entries(GROUP_BUILD_SERVICES).map(([group, services]) => [
    group,
    services.map((service) => service.slice(GROUP_PREFIX[group as OverrideGroup].length)),
  ]),
) as Record<OverrideGroup, string[]>;

const IMAGE_SIBLINGS: Record<string, string[]> = {
  "coprocessor-host-listener": ["coprocessor-host-listener-poller"],
  "coprocessor-host-listener-poller": ["coprocessor-host-listener"],
};

export const SCHEMA_COUPLED_GROUPS: OverrideGroup[] = ["coprocessor", "kms-connector"];

/** Resolves per-service override suffixes into full service names plus required siblings. */
export const resolveServiceOverrides = (group: OverrideGroup, suffixes: string[]) => {
  if (!SERVICE_OVERRIDE_GROUPS.includes(group as (typeof SERVICE_OVERRIDE_GROUPS)[number])) {
    throw new Error(
      `Per-service overrides are only supported for ${SERVICE_OVERRIDE_GROUPS.join(", ")}`,
    );
  }
  const names = new Set<string>();
  for (const suffix of suffixes) {
    const fullName = GROUP_PREFIX[group] + suffix;
    if (!GROUP_BUILD_SERVICES[group].includes(fullName)) {
      throw new Error(
        `Unknown service "${suffix}" in group "${group}". Valid: ${GROUP_SERVICE_SUFFIXES[group].join(", ")}`,
      );
    }
    names.add(fullName);
    for (const sibling of IMAGE_SIBLINGS[fullName] ?? []) {
      names.add(sibling);
    }
  }
  return [...names];
};

export const TEST_GREP: Record<string, string> = {
  "paused-host-contracts": "test paused host user input|test paused host HTTP public decrypt|test paused host operators",
  "paused-gateway-contracts":
    "test paused gateway user input|test paused gateway HTTP public decrypt",
  "input-proof": "test user input uint64",
  "input-proof-compute-decrypt": "test add 42 to uint64 input and decrypt",
  "user-decryption": "test user decrypt",
  "delegated-user-decryption": "test delegated user decrypt",
  "public-decryption":
    "test async decrypt (uint.*|ebytes.* trivial|ebytes64 non-trivial|ebytes256 non-trivial with snapshot|addresses|several addresses)",
  "public-decrypt-http-ebool": "test HTTPPublicDecrypt ebool",
  "public-decrypt-http-mixed": "test HTTPPublicDecrypt mixed",
  "random": "generate and decrypt|generating rand in reverting sub-call|upper bound and decrypt",
  "random-subset":
    "64 bits generate and decrypt|generating rand in reverting sub-call|64 bits generate with upper bound and decrypt",
  "operators": "test operator|FHEVM manual operations",
  "hcu-block-cap": "block cap scenarios",
  "erc20": "should transfer tokens between two users.",
  "negative-acl": "negative-acl",
  "multi-chain-isolation": "Multi-Chain State Isolation",
};

export const TEST_PARALLEL: Record<string, boolean> = {
  operators: true,
};

export const LIGHT_TEST_PROFILES = [
  "input-proof",
  "erc20",
] as const;

export const STANDARD_TEST_PROFILES = [
  "paused-host-contracts",
  "paused-gateway-contracts",
  "coprocessor-db-state-revert",
  "input-proof",
  "input-proof-compute-decrypt",
  "user-decryption",
  "delegated-user-decryption",
  "erc20",
  "public-decrypt-http-ebool",
  "public-decrypt-http-mixed",
  "negative-acl",
  "random-subset",
  "multi-chain-isolation",
  "hcu-block-cap",
  "ciphertext-drift",
] as const;

/** Heavy suites are the slowest and most stateful CI checks. */
export const HEAVY_TEST_PROFILES = [
  "operators",
] as const;

export const DEFAULT_TENANT_API_KEY = "00000000-0000-0000-0000-000000000000";
export const COPROCESSOR_WALLET_INDICES = [5, 8, 9, 10, 11] as const;
export const MAX_COPROCESSOR_INSTANCES = COPROCESSOR_WALLET_INDICES.length;

/** Returns the generated env-file path for a component or instance. */
export const envPath = (name: string) => path.join(ENV_DIR, `${name}.env`);
/** Returns the generated compose override path for a component. */
export const composePath = (name: string) => path.join(COMPOSE_OUT_DIR, `${name}.yml`);
/** Returns the template compose path for a component. */
const composeTemplatePath = (name: string) =>
  path.join(TEMPLATE_COMPOSE_DIR, `${name}-docker-compose.yml`);
/** Returns the compose file list used for a component, including overrides when present. */
const composeFiles = (name: string) =>
  existsSync(composePath(name))
    ? [composeTemplatePath(name), composePath(name)]
    : [composeTemplatePath(name)];
/** The implicit single-chain key used when hostChains is omitted. */
export const DEFAULT_HOST_CHAIN_KEY = "host";
export const hostChainAddressesPath = (key: string) =>
  path.join(ADDRESS_DIR, key, ".env.host");
export const defaultHostChainKey = (chains: Array<Pick<HostChainScenario, "key">>) =>
  chains[0]?.key ?? DEFAULT_HOST_CHAIN_KEY;

/**
 * Derives the suffix appended to service names for a chain key.
 * The default chain has no suffix; non-default chains use "-{key}" e.g. "-chain-b".
 */
export const hostChainSuffix = (key: string, defaultKey = DEFAULT_HOST_CHAIN_KEY) =>
  key === defaultKey ? "" : `-${key}`;

/** Derives the host-node container name from a chain key. */
export const hostNodeName = (key: string, defaultKey = DEFAULT_HOST_CHAIN_KEY) =>
  `host-node${hostChainSuffix(key, defaultKey)}`;

/** Derives the host-sc service prefix from a chain key. */
export const hostScName = (key: string, defaultKey = DEFAULT_HOST_CHAIN_KEY) =>
  `host-sc${hostChainSuffix(key, defaultKey)}`;

/** Derives the coprocessor-host compose key from a chain key. */
export const coprocessorHostKey = (key: string) => `coprocessor-${key}`;

/** Returns all derived runtime names for a host chain key in one call. */
export const hostChainNames = (key: string, defaultKey = DEFAULT_HOST_CHAIN_KEY) => ({
  node: hostNodeName(key, defaultKey),
  sc: hostScName(key, defaultKey),
  copro: coprocessorHostKey(key),
  suffix: hostChainSuffix(key, defaultKey),
});
export type HostChainRuntime = HostChainScenario & {
  index: number;
  isDefault: boolean;
  suffix: string;
  node: string;
  sc: string;
  copro: string;
  addressesPath: string;
  addressesSolidityPath: string;
};
export const hostChainRuntime = (
  chain: HostChainScenario,
  index: number,
  defaultKey = DEFAULT_HOST_CHAIN_KEY,
): HostChainRuntime => ({
  ...chain,
  index,
  isDefault: chain.key === defaultKey,
  ...hostChainNames(chain.key, defaultKey),
  addressesPath: hostChainAddressesPath(chain.key),
  addressesSolidityPath: hostChainAddressesSolidityPath(chain.key),
});
export const hostChainRuntimes = (chains: HostChainScenario[]) => {
  const defaultKey = defaultHostChainKey(chains);
  return chains.map((chain, index) => hostChainRuntime(chain, index, defaultKey));
};
export const hostChainAddressesSolidityPath = (key: string) =>
  path.join(ADDRESS_DIR, key, "FHEVMHostAddresses.sol");

/** Builds the docker compose argv prefix for one component. */
export const dockerArgs = (component: string) => [
  "docker",
  "compose",
  "-p",
  PROJECT,
  ...composeFiles(component).flatMap((file) => ["-f", file]),
];
