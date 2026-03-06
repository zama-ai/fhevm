/**
 * Parity tests: legacy CLI env files vs new CLI generators.
 *
 * These tests compare the new CLI's env generators against the legacy
 * hand-maintained .env files (extracted from commit fe6d05c9^) to catch
 * regressions in variable naming, contract address chain separation,
 * boot ordering, and service naming.
 */
import { describe, expect, it } from "bun:test";

import { ENV_GENERATORS } from "./env-mapping";
import { deriveAllKeys } from "./keys";
import {
  createDefaultConfig,
  DEFAULT_MNEMONIC,
  type ContractAddresses,
  type FhevmConfig,
} from "./model";
import { SERVICE_MAP } from "./service-map";
import { BOOT_STEPS } from "../pipeline/steps";

// ---------------------------------------------------------------------------
// Legacy env variable names extracted from test-suite/fhevm/env/staging/
// at commit fe6d05c9^ (the last known-good legacy configuration).
// ---------------------------------------------------------------------------

const LEGACY_ENV_VARS: Record<string, string[]> = {
  coprocessor: [
    "POSTGRES_USER", "POSTGRES_PASSWORD", "DATABASE_URL",
    "AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "AWS_ENDPOINT_URL", "AWS_REGION",
    "KMS_PUBLIC_KEY", "KMS_SERVER_KEY", "KMS_SNS_KEY", "KMS_CRS_KEY",
    "FHE_KEY_ID", "RPC_HTTP_URL", "RPC_WS_URL", "CHAIN_ID",
    "ACL_CONTRACT_ADDRESS", "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
    "GATEWAY_URL", "GATEWAY_WS_URL",
    "TX_SENDER_PRIVATE_KEY", "INPUT_VERIFICATION_ADDRESS",
    "CIPHERTEXT_COMMITS_ADDRESS", "MULTICHAIN_ACL_ADDRESS",
    "KMS_GENERATION_ADDRESS",
  ],
  "kms-core": [
    "KMS_CORE__PUBLIC_VAULT__STORAGE__S3__BUCKET",
    "KMS_CORE__PUBLIC_VAULT__STORAGE__S3__PREFIX",
    "KMS_CORE__PRIVATE_VAULT__STORAGE__FILE__PATH",
    "S3_ENDPOINT", "S3_REGION",
    "OBJECT_FOLDER", "CORE_ADDRESSES",
  ],
  database: ["POSTGRES_USER", "POSTGRES_PASSWORD"],
  "gateway-mocked-payment": [
    "HARDHAT_NETWORK", "RPC_URL", "CHAIN_ID_GATEWAY", "MNEMONIC",
    "DEPLOYER_PRIVATE_KEY", "TX_SENDER_PRIVATE_KEY",
    "ZAMA_OFT_ADDRESS", "PROTOCOL_PAYMENT_ADDRESS",
  ],
  "gateway-node": [
    "HARDHAT_NETWORK", "RPC_URL", "CHAIN_ID_GATEWAY", "MNEMONIC",
  ],
  "gateway-sc": [
    "HARDHAT_NETWORK", "RPC_URL", "CHAIN_ID_GATEWAY",
    "ZAMA_OFT_ADDRESS", "FEES_SENDER_TO_BURNER_ADDRESS",
    "GATEWAY_CONFIG_ADDRESS", "KMS_GENERATION_ADDRESS",
    "PAUSER_SET_ADDRESS", "MNEMONIC", "DEPLOYER_ADDRESS",
    "DEPLOYER_PRIVATE_KEY", "PROTOCOL_NAME", "PROTOCOL_WEBSITE",
    "PUBLIC_DECRYPTION_THRESHOLD", "USER_DECRYPTION_THRESHOLD",
    "KMS_GENERATION_THRESHOLD", "MPC_THRESHOLD", "COPROCESSOR_THRESHOLD",
    "NUM_KMS_NODES", "KMS_TX_SENDER_ADDRESS_0", "KMS_SIGNER_ADDRESS_0",
    "KMS_NODE_IP_ADDRESS_0", "KMS_NODE_STORAGE_URL_0",
    "NUM_COPROCESSORS", "COPROCESSOR_TX_SENDER_ADDRESS_0",
    "COPROCESSOR_SIGNER_ADDRESS_0", "COPROCESSOR_S3_BUCKET_URL_0",
    "NUM_CUSTODIANS", "CUSTODIAN_TX_SENDER_ADDRESS_0",
    "CUSTODIAN_SIGNER_ADDRESS_0", "CUSTODIAN_ENCRYPTION_KEY_0",
    "NUM_HOST_CHAINS", "HOST_CHAIN_CHAIN_ID_0",
    "HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0", "HOST_CHAIN_ACL_ADDRESS_0",
    "HOST_CHAIN_NAME_0", "HOST_CHAIN_WEBSITE_0",
    "NUM_PAUSERS", "PAUSER_ADDRESS_0", "PAUSER_PRIVATE_KEY",
    "INPUT_VERIFICATION_PRICE", "PUBLIC_DECRYPTION_PRICE",
    "USER_DECRYPTION_PRICE",
  ],
  "host-node": [
    "HARDHAT_NETWORK", "RPC_URL", "CHAIN_ID_GATEWAY", "MNEMONIC",
  ],
  "host-sc": [
    "HARDHAT_NETWORK", "RPC_URL", "CHAIN_ID_GATEWAY",
    "PAUSER_SET_CONTRACT_ADDRESS", "MNEMONIC", "DEPLOYER_PRIVATE_KEY",
    "DECRYPTION_ADDRESS", "INPUT_VERIFICATION_ADDRESS",
    "ACL_CONTRACT_ADDRESS",
    "PUBLIC_DECRYPTION_THRESHOLD", "COPROCESSOR_THRESHOLD",
    "NUM_KMS_NODES", "KMS_SIGNER_ADDRESS_0",
    "NUM_COPROCESSORS", "COPROCESSOR_SIGNER_ADDRESS_0",
    "NUM_PAUSERS", "PAUSER_ADDRESS_0", "PAUSER_PRIVATE_KEY",
  ],
  "kms-connector": [
    "PGUSER", "PGPASSWORD", "DATABASE_URL",
    "KMS_CONNECTOR_DATABASE_URL", "KMS_CONNECTOR_DATABASE_POOL_SIZE",
    "KMS_CONNECTOR_GATEWAY_URL", "KMS_CONNECTOR_KMS_CORE_ENDPOINTS",
    "KMS_CONNECTOR_GATEWAY_CHAIN_ID",
    "OTEL_EXPORTER_OTLP_ENDPOINT",
    "KMS_CONNECTOR_DECRYPTION_POLLING_MS",
    "KMS_CONNECTOR_KEY_MANAGEMENT_POLLING_MS",
    "KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS",
    "KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS",
    "KMS_CONNECTOR_GAS_MULTIPLIER_PERCENT",
    "KMS_CONNECTOR_RETRY_INTERVAL_SECS",
    "KMS_CONNECTOR_VERIFY_COPROCESSORS",
    "KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS",
    "KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS",
    "KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS",
    "KMS_CONNECTOR_S3_CONFIG__REGION", "KMS_CONNECTOR_S3_CONFIG__BUCKET",
    "KMS_CONNECTOR_S3_CONFIG__ENDPOINT",
    "KMS_CONNECTOR_PRIVATE_KEY", "KMS_CONNECTOR_HOST_CHAINS",
  ],
  minio: [
    "MINIO_ROOT_USER", "MINIO_ROOT_PASSWORD",
    "ACCESS_KEY", "SECRET_KEY", "MINIO_ENDPOINT",
  ],
  relayer: [
    "DATABASE_URL", "MAX_ATTEMPTS",
    "APP_GATEWAY__TX_ENGINE__PRIVATE_KEY",
    "APP_KEYURL__FHE_PUBLIC_KEY__DATA_ID", "APP_KEYURL__FHE_PUBLIC_KEY__URL",
    "APP_KEYURL__CRS__DATA_ID", "APP_KEYURL__CRS__URL",
    "APP_GATEWAY__BLOCKCHAIN_RPC__WS_URL", "APP_GATEWAY__BLOCKCHAIN_RPC__HTTP_URL",
    "APP_GATEWAY__BLOCKCHAIN_RPC__CHAIN_ID",
    "APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS",
    "APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS",
    "APP_GATEWAY__CONTRACTS__USER_DECRYPT_SHARES_THRESHOLD",
    "APP_STORAGE__SQL_DATABASE_URL", "RUST_LOG",
  ],
  "test-suite": [
    "MNEMONIC", "CHAIN_ID_GATEWAY", "CHAIN_ID_HOST", "RPC_URL",
    "DECRYPTION_ADDRESS", "INPUT_VERIFICATION_ADDRESS",
    "DECRYPTION_ORACLE_ADDRESS",
    "KMS_VERIFIER_CONTRACT_ADDRESS", "ACL_CONTRACT_ADDRESS",
    "INPUT_VERIFIER_CONTRACT_ADDRESS", "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
    "RELAYER_URL",
  ],
};

/**
 * Variables present in legacy env files but intentionally excluded from the
 * new CLI generators, with explanations for each exclusion.
 */
const KNOWN_EXCLUSIONS: Record<string, Record<string, string>> = {
  coprocessor: {
    POSTGRES_USER: "Credentials embedded in DATABASE_URL; no separate vars needed",
    POSTGRES_PASSWORD: "Credentials embedded in DATABASE_URL; no separate vars needed",
    KMS_PUBLIC_KEY: "Runtime discovery from MinIO, not static config",
    KMS_SERVER_KEY: "Runtime discovery from MinIO, not static config",
    KMS_SNS_KEY: "Runtime discovery from MinIO, not static config",
    KMS_CRS_KEY: "Runtime discovery from MinIO, not static config",
    GATEWAY_URL: "Renamed to GATEWAY_WS_URL for clarity; HTTP not needed by coprocessor",
  },
  "kms-core": {
    OBJECT_FOLDER: "Internal kms-core default; not needed in env",
    CORE_ADDRESSES: "Internal kms-core self-reference; not externally configured",
  },
  "gateway-mocked-payment": {
    PROTOCOL_PAYMENT_ADDRESS: "Discovered at runtime after gateway-contracts deploy",
  },
  "gateway-node": {
    HARDHAT_NETWORK: "Injected by the Anvil container image, not needed in env",
    RPC_URL: "Injected by the Anvil container image, not needed in env",
    CHAIN_ID_GATEWAY: "Injected by the Anvil container image, not needed in env",
  },
  "gateway-sc": {
    DEPLOYER_ADDRESS: "Derived from DEPLOYER_PRIVATE_KEY; redundant",
  },
  "host-node": {
    HARDHAT_NETWORK: "Injected by the Anvil container image, not needed in env",
    RPC_URL: "Injected by the Anvil container image, not needed in env",
    CHAIN_ID_GATEWAY: "Injected by the Anvil container image, not needed in env",
  },
  "host-sc": {},
  "kms-connector": {
    PGUSER: "Credentials embedded in DATABASE_URL; no separate vars needed",
    PGPASSWORD: "Credentials embedded in DATABASE_URL; no separate vars needed",
    OTEL_EXPORTER_OTLP_ENDPOINT: "Operational tuning; set via docker-compose, not env file",
    KMS_CONNECTOR_DATABASE_POOL_SIZE: "Operational tuning; uses service default",
    KMS_CONNECTOR_DECRYPTION_POLLING_MS: "Operational tuning; uses service default",
    KMS_CONNECTOR_KEY_MANAGEMENT_POLLING_MS: "Operational tuning; uses service default",
    KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS: "Operational tuning; uses service default",
    KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS: "Operational tuning; uses service default",
    KMS_CONNECTOR_GAS_MULTIPLIER_PERCENT: "Operational tuning; uses service default",
    KMS_CONNECTOR_RETRY_INTERVAL_SECS: "Operational tuning; uses service default",
    KMS_CONNECTOR_VERIFY_COPROCESSORS: "Operational tuning; uses service default",
  },
  relayer: {
    APP_GATEWAY__TX_ENGINE__PRIVATE_KEY: "Moved to relayer YAML config template",
    APP_KEYURL__FHE_PUBLIC_KEY__DATA_ID: "Moved to relayer YAML config template",
    APP_KEYURL__FHE_PUBLIC_KEY__URL: "Moved to relayer YAML config template",
    APP_KEYURL__CRS__DATA_ID: "Moved to relayer YAML config template",
    APP_KEYURL__CRS__URL: "Moved to relayer YAML config template",
    APP_GATEWAY__BLOCKCHAIN_RPC__WS_URL: "Moved to relayer YAML config template",
    APP_GATEWAY__BLOCKCHAIN_RPC__HTTP_URL: "Moved to relayer YAML config template",
    APP_GATEWAY__BLOCKCHAIN_RPC__CHAIN_ID: "Moved to relayer YAML config template",
    APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS: "Moved to relayer YAML config template",
    APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS: "Moved to relayer YAML config template",
    APP_GATEWAY__CONTRACTS__USER_DECRYPT_SHARES_THRESHOLD: "Moved to relayer YAML config template",
    APP_STORAGE__SQL_DATABASE_URL: "Moved to relayer YAML config template",
    RUST_LOG: "Operational tuning; set via docker-compose, not env file",
  },
  "test-suite": {
    DECRYPTION_ORACLE_ADDRESS: "Removed concept; oracle merged into decryption contract",
  },
};

// ---------------------------------------------------------------------------
// Golden contract addresses from legacy env files (deterministic Anvil deploys)
// ---------------------------------------------------------------------------

const GOLDEN_CONTRACTS: ContractAddresses = {
  // Gateway chain contracts
  gatewayConfig: "0x576Ea67208b146E63C5255d0f90104E25e3e04c7",
  kmsGeneration: "0x3b12Fc766Eb598b285998877e8E90F3e43a1F8d2",
  inputVerification: "0x1ceFA8E3F3271358218B52c33929Cf76078004c1",
  decryption: "0x35760912360E875DA50D40a74305575c23D55783",
  pauserSet: "0xfd79448E3cf99F7838B4F19d94C0B5b2471Acfaf",
  multichainAcl: "0xeAC2EfFA07844aB326D92d1De29E136a6793DFFA",
  ciphertextCommits: "0xF0bFB159C7381F7CB332586004d8247252C5b816",
  protocolPayment: "0xacdFB015D1F3D96fBF8BDd3A4b746f4A70123937",
  zamaOft: "0x5ffdaAB0373E62E2ea2944776209aEf29E631A64",
  feesSenderToBurner: "0x0000111122223333444455556666777788889999",

  // Host chain contracts (different addresses — different chain!)
  acl: "0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c",
  fhevmExecutor: "0xcCAe95fF1d11656358E782570dF0418F59fA40e1",
  hostPauserSet: "0x52054F36036811ca418be59e41Fc6DD1b9e4F4c8",
  kmsVerifier: "0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11",
  inputVerifier: "0x857Ca72A957920Fa0FB138602995839866Bd4805",
};

const GOLDEN_FHE_KEY_ID = "421c8116661b2150a46badd3956564ad8d4981718d30fa66a36342bee1b13dbf";

// ---------------------------------------------------------------------------
// Docker-compose container names (ground truth from compose files)
// ---------------------------------------------------------------------------

const DOCKER_COMPOSE_CONTAINER_NAMES = [
  "coprocessor-db-migration",
  "coprocessor-gw-listener",
  "coprocessor-host-listener",
  "coprocessor-host-listener-poller",
  "coprocessor-sns-worker",
  "coprocessor-tfhe-worker",
  "coprocessor-transaction-sender",
  "coprocessor-zkproof-worker",
  "kms-core",
  "coprocessor-and-kms-db",
  "gateway-deploy-mocked-zama-oft",
  "gateway-set-relayer-mocked-payment",
  "gateway-node",
  "gateway-sc-pause",
  "gateway-sc-add-network",
  "gateway-sc-add-pausers",
  "gateway-sc-deploy",
  "gateway-sc-trigger-crsgen",
  "gateway-sc-trigger-keygen",
  "gateway-sc-unpause",
  "host-node",
  "host-sc-pause",
  "host-sc-add-pausers",
  "host-sc-deploy",
  "host-sc-unpause",
  "kms-connector-db-migration",
  "kms-connector-gw-listener",
  "kms-connector-kms-worker",
  "kms-connector-tx-sender",
  "fhevm-minio",
  "fhevm-minio-setup",
  "fhevm-relayer",
  "fhevm-relayer-db",
  "relayer-db-migration",
  "fhevm-test-suite-e2e-debug",
  "jaeger",
  "prometheus",
] as const;

// ---------------------------------------------------------------------------
// Test helper
// ---------------------------------------------------------------------------

function buildGoldenConfig(): FhevmConfig {
  const keys = deriveAllKeys(DEFAULT_MNEMONIC, 1, 1);
  return createDefaultConfig(keys, {
    contracts: GOLDEN_CONTRACTS,
    runtime: {
      minioIp: "172.18.0.2",
      fheKeyId: GOLDEN_FHE_KEY_ID,
    },
  });
}

function stepNumber(name: string): number {
  const step = BOOT_STEPS.find((s) => s.name === name);
  if (!step) throw new Error(`unknown step: ${name}`);
  return step.number;
}

// ---------------------------------------------------------------------------
// 1. Env variable name parity
// ---------------------------------------------------------------------------

describe("env variable name parity", () => {
  const config = buildGoldenConfig();

  for (const [envName, legacyVars] of Object.entries(LEGACY_ENV_VARS)) {
    it(`${envName}: new CLI produces all legacy variable names`, () => {
      const generator = ENV_GENERATORS[envName as keyof typeof ENV_GENERATORS];
      expect(generator).toBeDefined();

      const generated = generator(config);
      const generatedKeys = new Set(Object.keys(generated));
      const exclusions = KNOWN_EXCLUSIONS[envName] ?? {};

      const missing: string[] = [];
      for (const varName of legacyVars) {
        if (generatedKeys.has(varName)) continue;
        if (exclusions[varName]) continue;
        missing.push(varName);
      }

      if (missing.length > 0) {
        throw new Error(
          `${envName}: missing legacy vars in new CLI:\n` +
            missing.map((v) => `  - ${v}`).join("\n") +
            "\n\nIf intentional, add to KNOWN_EXCLUSIONS with an explanation.",
        );
      }
    });
  }

  it("KNOWN_EXCLUSIONS only reference vars that actually existed in legacy", () => {
    for (const [envName, exclusions] of Object.entries(KNOWN_EXCLUSIONS)) {
      const legacyVars = new Set(LEGACY_ENV_VARS[envName] ?? []);
      for (const varName of Object.keys(exclusions)) {
        expect(legacyVars.has(varName)).toBe(true);
      }
    }
  });
});

// ---------------------------------------------------------------------------
// 2. Contract address chain separation
// ---------------------------------------------------------------------------

describe("contract address chain separation", () => {
  const config = buildGoldenConfig();

  it("host-sc uses hostPauserSet, not gateway pauserSet", () => {
    const hostScEnv = ENV_GENERATORS["host-sc"](config);
    expect(hostScEnv.PAUSER_SET_CONTRACT_ADDRESS).toBe(GOLDEN_CONTRACTS.hostPauserSet!);
    expect(hostScEnv.PAUSER_SET_CONTRACT_ADDRESS).not.toBe(GOLDEN_CONTRACTS.pauserSet!);
  });

  it("gateway-sc uses gateway pauserSet, not hostPauserSet", () => {
    const gatewayScEnv = ENV_GENERATORS["gateway-sc"](config);
    expect(gatewayScEnv.PAUSER_SET_ADDRESS).toBe(GOLDEN_CONTRACTS.pauserSet!);
    expect(gatewayScEnv.PAUSER_SET_ADDRESS).not.toBe(GOLDEN_CONTRACTS.hostPauserSet!);
  });

  it("gateway and host PauserSet addresses are distinct", () => {
    expect(GOLDEN_CONTRACTS.pauserSet).not.toBe(GOLDEN_CONTRACTS.hostPauserSet);
  });

  it("coprocessor env uses host-chain RPC URLs (not gateway)", () => {
    const env = ENV_GENERATORS.coprocessor(config);
    expect(env.RPC_HTTP_URL).toBe(config.rpc.hostHttp);
    expect(env.RPC_WS_URL).toBe(config.rpc.hostWs);
    expect(env.RPC_HTTP_URL).toContain("host-node");
    expect(env.RPC_HTTP_URL).not.toContain("gateway-node");
  });

  it("kms-connector uses gateway chain for gateway URL and host chain in HOST_CHAINS", () => {
    const env = ENV_GENERATORS["kms-connector"](config);
    expect(env.KMS_CONNECTOR_GATEWAY_URL).toBe(config.rpc.gatewayHttp);
    expect(env.KMS_CONNECTOR_GATEWAY_URL).toContain("gateway-node");

    const hostChains = JSON.parse(env.KMS_CONNECTOR_HOST_CHAINS);
    expect(hostChains).toHaveLength(1);
    expect(hostChains[0].url).toBe(config.rpc.hostHttp);
    expect(hostChains[0].url).toContain("host-node");
    expect(hostChains[0].chain_id).toBe(config.chainIds.host);
  });

  it("ACL_CONTRACT_ADDRESS is consistent across coprocessor and test-suite", () => {
    const coprocessorEnv = ENV_GENERATORS.coprocessor(config);
    const testSuiteEnv = ENV_GENERATORS["test-suite"](config);

    expect(coprocessorEnv.ACL_CONTRACT_ADDRESS).toBe(GOLDEN_CONTRACTS.acl!);
    expect(testSuiteEnv.ACL_CONTRACT_ADDRESS).toBe(GOLDEN_CONTRACTS.acl!);
    expect(coprocessorEnv.ACL_CONTRACT_ADDRESS).toBe(testSuiteEnv.ACL_CONTRACT_ADDRESS);
  });

  it("FHEVM_EXECUTOR_CONTRACT_ADDRESS is consistent across coprocessor and test-suite", () => {
    const coprocessorEnv = ENV_GENERATORS.coprocessor(config);
    const testSuiteEnv = ENV_GENERATORS["test-suite"](config);

    expect(coprocessorEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS).toBe(GOLDEN_CONTRACTS.fhevmExecutor!);
    expect(testSuiteEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS).toBe(GOLDEN_CONTRACTS.fhevmExecutor!);
  });

  it("INPUT_VERIFICATION_ADDRESS is consistent across coprocessor, host-sc, and test-suite", () => {
    const coprocessorEnv = ENV_GENERATORS.coprocessor(config);
    const hostScEnv = ENV_GENERATORS["host-sc"](config);
    const testSuiteEnv = ENV_GENERATORS["test-suite"](config);

    expect(coprocessorEnv.INPUT_VERIFICATION_ADDRESS).toBe(GOLDEN_CONTRACTS.inputVerification!);
    expect(hostScEnv.INPUT_VERIFICATION_ADDRESS).toBe(GOLDEN_CONTRACTS.inputVerification!);
    expect(testSuiteEnv.INPUT_VERIFICATION_ADDRESS).toBe(GOLDEN_CONTRACTS.inputVerification!);
  });

  it("DECRYPTION_ADDRESS is consistent across host-sc and test-suite", () => {
    const hostScEnv = ENV_GENERATORS["host-sc"](config);
    const testSuiteEnv = ENV_GENERATORS["test-suite"](config);

    expect(hostScEnv.DECRYPTION_ADDRESS).toBe(GOLDEN_CONTRACTS.decryption!);
    expect(testSuiteEnv.DECRYPTION_ADDRESS).toBe(GOLDEN_CONTRACTS.decryption!);
  });

  it("KMS_GENERATION_ADDRESS is consistent across coprocessor and gateway-sc", () => {
    const coprocessorEnv = ENV_GENERATORS.coprocessor(config);
    const gatewayScEnv = ENV_GENERATORS["gateway-sc"](config);

    expect(coprocessorEnv.KMS_GENERATION_ADDRESS).toBe(GOLDEN_CONTRACTS.kmsGeneration!);
    expect(gatewayScEnv.KMS_GENERATION_ADDRESS).toBe(GOLDEN_CONTRACTS.kmsGeneration!);
  });
});

// ---------------------------------------------------------------------------
// 3. Boot order constraints
// ---------------------------------------------------------------------------

describe("boot order constraints", () => {
  it("minio is step 1", () => {
    expect(stepNumber("minio")).toBe(1);
  });

  it("kms-core before kms-signer discovery", () => {
    expect(stepNumber("kms-core")).toBeLessThan(stepNumber("kms-signer"));
  });

  it("postgres before coprocessor and kms-connector", () => {
    expect(stepNumber("postgres")).toBeLessThan(stepNumber("coprocessor"));
    expect(stepNumber("postgres")).toBeLessThan(stepNumber("kms-connector"));
  });

  it("host-node and gateway-node before any contract deployment", () => {
    expect(stepNumber("host-node")).toBeLessThan(stepNumber("gateway-contracts"));
    expect(stepNumber("host-node")).toBeLessThan(stepNumber("host-contracts"));
    expect(stepNumber("gateway-node")).toBeLessThan(stepNumber("gateway-contracts"));
    expect(stepNumber("gateway-node")).toBeLessThan(stepNumber("host-contracts"));
  });

  it("gateway-contracts before host-contracts (host needs gateway addresses)", () => {
    expect(stepNumber("gateway-contracts")).toBeLessThan(stepNumber("host-contracts"));
  });

  it("gateway-contracts and host-contracts before coprocessor (the V0 bug)", () => {
    expect(stepNumber("gateway-contracts")).toBeLessThan(stepNumber("coprocessor"));
    expect(stepNumber("host-contracts")).toBeLessThan(stepNumber("coprocessor"));
  });

  it("kms-connector before coprocessor (coprocessor needs FHE_KEY_ID)", () => {
    expect(stepNumber("kms-connector")).toBeLessThan(stepNumber("coprocessor"));
  });

  it("gateway-mocked-payment before gateway-contracts (provides ZamaOFT)", () => {
    expect(stepNumber("gateway-mocked-payment")).toBeLessThan(stepNumber("gateway-contracts"));
  });

  it("relayer after coprocessor and contracts", () => {
    expect(stepNumber("relayer")).toBeGreaterThan(stepNumber("coprocessor"));
    expect(stepNumber("relayer")).toBeGreaterThan(stepNumber("host-contracts"));
    expect(stepNumber("relayer")).toBeGreaterThan(stepNumber("gateway-contracts"));
  });

  it("test-suite is the final step", () => {
    const maxStep = Math.max(...BOOT_STEPS.map((s) => s.number));
    expect(stepNumber("test-suite")).toBe(maxStep);
  });

  it("host-contracts step contains host-sc-add-pausers in its service list", () => {
    const step = BOOT_STEPS.find((s) => s.name === "host-contracts");
    expect(step).toBeDefined();
    expect(step!.serviceNames).toContain("host-sc-add-pausers");
  });

  it("new CLI covers all legacy step names", () => {
    // Legacy step names → new CLI step names
    const LEGACY_TO_NEW: Record<string, string> = {
      minio: "minio",
      core: "kms-core",
      postgres: "postgres",
      "host-node": "host-node",
      "gateway-node": "gateway-node",
      "gateway-sc": "gateway-contracts",
      "host-sc": "host-contracts",
      "kms-connector": "kms-connector",
      coprocessor: "coprocessor",
      relayer: "relayer",
      "test-suite": "test-suite",
    };

    const newStepNames = new Set(BOOT_STEPS.map((s) => s.name));
    for (const [legacyName, newName] of Object.entries(LEGACY_TO_NEW)) {
      expect(newStepNames.has(newName)).toBe(true);
    }
  });
});

// ---------------------------------------------------------------------------
// 4. Service name parity
// ---------------------------------------------------------------------------

describe("service name parity", () => {
  const serviceMapContainers = new Set(SERVICE_MAP.map((s) => s.containerName));
  const composeContainers = new Set<string>(DOCKER_COMPOSE_CONTAINER_NAMES);

  // Tracing services (jaeger, prometheus) are optional infrastructure not
  // managed by the boot pipeline, so we exclude them from the parity check.
  const TRACING_CONTAINERS = new Set(["jaeger", "prometheus"]);

  it("every SERVICE_MAP containerName exists in docker-compose", () => {
    const missing: string[] = [];
    for (const container of serviceMapContainers) {
      if (!composeContainers.has(container)) {
        missing.push(container);
      }
    }
    expect(missing).toEqual([]);
  });

  it("every docker-compose container_name has a SERVICE_MAP entry", () => {
    const missing: string[] = [];
    for (const container of composeContainers) {
      if (TRACING_CONTAINERS.has(container)) continue;
      if (!serviceMapContainers.has(container)) {
        missing.push(container);
      }
    }
    expect(missing).toEqual([]);
  });

  it("every boot step service name resolves to a valid SERVICE_MAP entry", () => {
    const serviceNames = new Set(SERVICE_MAP.map((s) => s.name));
    const invalid: string[] = [];

    for (const step of BOOT_STEPS) {
      for (const name of step.serviceNames) {
        if (!serviceNames.has(name)) {
          invalid.push(`step ${step.number} (${step.name}): ${name}`);
        }
      }
    }
    expect(invalid).toEqual([]);
  });
});
