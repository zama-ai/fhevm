type Env = Record<string, string>;

// Mainnet upgrade #1 of the staggered plan (Opio, Jun 2026): fhevm v0.11 / kms
// v0.13.0 -> fhevm v0.12 / kms v0.13.10. Both endpoints are already-validated
// version sets: the v0.11 baseline matches the v0.11-to-v0.13 runbook, and the
// v0.12 target matches the v0.12-to-v0.13 runbook's baseline. v0.11 -> v0.12 is a
// plain UUPS contract upgrade (no ProtocolConfig/KMSGeneration state migration --
// that is the v0.12 -> v0.13 hop).
export const scenario = "single";

const fromTag = "v0.11.0";
const toTag = "v0.12.5";

// The relayer rides its own (offset) v0.11.x line that spans fhevm v0.11 and
// v0.12, so it does not change in this hop. v0.11.1 is the tip of that line
// (see the v0.11-to-v0.13 runbook note); testnet v0.12 ran v0.11.0, and v0.11.1
// is a strict superset (estimate-gas hotfix), so it is correct for v0.12 too.
const relayerVersion = "v0.11.1";
const relayerMigrateVersion = "v0.11.0";

// The rollout-standard suite and its relayer-sdk pairing are pinned to the
// target test-suite image across every phase (the test-suite group is overridden
// at boot, never upgraded in place).
const testSuiteVersion = toTag;
const relayerSdkVersion = "0.4.2";

export const from = {
  RELAYER_VERSION: relayerVersion,
  RELAYER_MIGRATE_VERSION: relayerMigrateVersion,
  GATEWAY_VERSION: fromTag,
  HOST_VERSION: fromTag,
  CORE_VERSION: "v0.13.0",
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
  TEST_SUITE_VERSION: testSuiteVersion,
  RELAYER_SDK_VERSION: relayerSdkVersion,
} satisfies Env;

export const to = {
  ...from,
  // relayer keys intentionally unchanged (shared v0.11.x line).
  // LISTENER_CORE_VERSION intentionally unchanged: the standalone listener-core
  // (v2 listener) is a v0.13 component with no v0.11/v0.12 published image. At
  // v0.12 the listener is the coprocessor-bundled host-listener, which moves
  // with the coprocessor group below; listener-core only activates at v0.13.
  GATEWAY_VERSION: toTag,
  HOST_VERSION: toTag,
  CORE_VERSION: "v0.13.10",
  CONNECTOR_DB_MIGRATION_VERSION: toTag,
  CONNECTOR_GW_LISTENER_VERSION: toTag,
  CONNECTOR_KMS_WORKER_VERSION: toTag,
  CONNECTOR_TX_SENDER_VERSION: toTag,
  COPROCESSOR_DB_MIGRATION_VERSION: toTag,
  COPROCESSOR_HOST_LISTENER_VERSION: toTag,
  COPROCESSOR_GW_LISTENER_VERSION: toTag,
  COPROCESSOR_TX_SENDER_VERSION: toTag,
  COPROCESSOR_TFHE_WORKER_VERSION: toTag,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: toTag,
  COPROCESSOR_SNS_WORKER_VERSION: toTag,
} satisfies Env;

type EnvKey = keyof typeof from;

const contractKeys = ["GATEWAY_VERSION", "HOST_VERSION"] as const satisfies readonly EnvKey[];
const kmsKeys = [
  "CORE_VERSION",
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const satisfies readonly EnvKey[];
const withTargetVersions = (...keys: EnvKey[]): Env => ({
  ...from,
  ...Object.fromEntries(keys.map((key) => [key, to[key]])),
});

// Contracts move first (plain UUPS), then kms (core + connector, which needs the
// v0.12 contract ABI), then the coprocessor last. The relayer and the standalone
// listener-core are unchanged in this hop, so neither has a phase of its own.
export const phaseVersions = {
  baseline: from,
  contracts: withTargetVersions(...contractKeys),
  kms: withTargetVersions(...contractKeys, ...kmsKeys),
  coprocessor: to,
};

export const versionSources = [
  "rollout=v0.11-to-v0.12",
  `from=${fromTag}`,
  `target=${toTag}`,
  "kms-core=v0.13.10",
];
