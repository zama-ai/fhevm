type Env = Record<string, string>;

export const scenario = "two-of-three";

const fromTag = "v0.12.4";
const targetTag = "v0.13.0-1";
// Pre-RC validation pin for PR #2469 (KMS migration verification script, release/0.13.x merge).
// Revert to targetTag once v0.13.0-2 (or its successor) is published.
const targetContractsSha = "9f8332e0";
const relayerSdkVersion = "0.4.2";

export const from = {
  RELAYER_VERSION: "v0.11.0",
  RELAYER_MIGRATE_VERSION: "v0.11.0",
  GATEWAY_VERSION: fromTag,
  HOST_VERSION: fromTag,
  CORE_VERSION: "v0.13.10",
  CONNECTOR_DB_MIGRATION_VERSION: fromTag,
  CONNECTOR_GW_LISTENER_VERSION: fromTag,
  CONNECTOR_KMS_WORKER_VERSION: fromTag,
  CONNECTOR_TX_SENDER_VERSION: fromTag,
  COPROCESSOR_DB_MIGRATION_VERSION: fromTag,
  COPROCESSOR_HOST_LISTENER_VERSION: fromTag,
  COPROCESSOR_GW_LISTENER_VERSION: fromTag,
  COPROCESSOR_TX_SENDER_VERSION: fromTag,
  COPROCESSOR_TFHE_WORKER_VERSION: fromTag,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: fromTag,
  COPROCESSOR_SNS_WORKER_VERSION: fromTag,
  LISTENER_CORE_VERSION: fromTag,
  TEST_SUITE_VERSION: targetTag,
  RELAYER_SDK_VERSION: relayerSdkVersion,
} satisfies Env;

export const to = {
  ...from,
  RELAYER_VERSION: targetTag,
  RELAYER_MIGRATE_VERSION: targetTag,
  GATEWAY_VERSION: targetContractsSha,
  HOST_VERSION: targetContractsSha,
  CORE_VERSION: "v0.13.20-0",
  CONNECTOR_DB_MIGRATION_VERSION: targetTag,
  CONNECTOR_GW_LISTENER_VERSION: targetTag,
  CONNECTOR_KMS_WORKER_VERSION: targetTag,
  CONNECTOR_TX_SENDER_VERSION: targetTag,
  COPROCESSOR_DB_MIGRATION_VERSION: targetTag,
  COPROCESSOR_HOST_LISTENER_VERSION: targetTag,
  COPROCESSOR_GW_LISTENER_VERSION: targetTag,
  COPROCESSOR_TX_SENDER_VERSION: targetTag,
  COPROCESSOR_TFHE_WORKER_VERSION: targetTag,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: targetTag,
  COPROCESSOR_SNS_WORKER_VERSION: targetTag,
  LISTENER_CORE_VERSION: targetTag,
} satisfies Env;

type EnvKey = keyof typeof from;

const relayerKeys = ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"] as const satisfies readonly EnvKey[];
const contractKeys = ["GATEWAY_VERSION", "HOST_VERSION"] as const satisfies readonly EnvKey[];
const kmsKeys = [
  "CORE_VERSION",
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const satisfies readonly EnvKey[];
const listenerKeys = ["LISTENER_CORE_VERSION"] as const satisfies readonly EnvKey[];
const coprocessorKeys = [
  "COPROCESSOR_DB_MIGRATION_VERSION",
  "COPROCESSOR_HOST_LISTENER_VERSION",
  "COPROCESSOR_GW_LISTENER_VERSION",
  "COPROCESSOR_TX_SENDER_VERSION",
  "COPROCESSOR_TFHE_WORKER_VERSION",
  "COPROCESSOR_ZKPROOF_WORKER_VERSION",
  "COPROCESSOR_SNS_WORKER_VERSION",
] as const satisfies readonly EnvKey[];

const withTargetVersions = (...keys: EnvKey[]): Env => ({
  ...from,
  ...Object.fromEntries(keys.map((key) => [key, to[key]])),
});

export const phaseVersions = {
  baseline: from,
  contracts: withTargetVersions(...contractKeys),
  relayer: withTargetVersions(...contractKeys, ...relayerKeys),
  kms: withTargetVersions(...contractKeys, ...relayerKeys, ...kmsKeys),
  listenerCore: withTargetVersions(...contractKeys, ...relayerKeys, ...kmsKeys, ...listenerKeys),
  coprocessor: to,
};

export const versionSources = [
  `rollout=v0.12-to-v0.13`,
  `target=${targetTag}`,
  `contracts=${targetContractsSha}`,
  `kms-core=v0.13.20-0`,
];
