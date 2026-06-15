type Env = Record<string, string>;

export const scenario = "two-of-three";

// Mainnet today: fhevm v0.11.0 / kms-core v0.13.0. Target: fhevm v0.13 / kms
// v0.13.20. v0.12 is only a contract-migration waypoint (host + gateway proxies
// must pass through it); runtime components jump straight to v0.13 in their own
// phases, matching the staged plan for the direct mainnet upgrade.
const v011Tag = "v0.11.0";
const v012Tag = "v0.12.5";
const v013Tag = "v0.13.0";

// The rollout-standard e2e suite and its relayer-sdk pairing only exist in the
// target test-suite image, so the harness is pinned to the target across every
// phase (the test-suite group is overridden at boot, never upgraded in place).
const testSuiteVersion = "v0.13.0";
const relayerSdkVersion = "0.4.2";

// v0.11.1 is the tip of the legacy console/relayer v0.11 line (rc.1 -> rc.2 ->
// v0.11.0 -> v0.11.1), so it's the closest image to what mainnet "v0.11" runs.
// NOTE: this pin does NOT fix the "Ciphertext not ready" readiness flake in
// delegated user-decryption. Verified against the published console/relayer
// images by git ancestry on their build SHAs: rc.1 (sha-1e44d77), v0.11.0
// (sha-8319fd7) and v0.11.1 (sha-d6e1ba8) are identical in the readiness /
// listener / decrypt path. Both console#914 (listener filters) and console#925
// (isUserDecryptionReady compat) are already present in rc.1; the rc.1->v0.11.1
// delta is only config-env-var plumbing (console#928) and an Arbitrum
// estimate-gas fix (console#930). The flake reproduces on the pure-v0.11
// baseline, so it is a pre-existing baseline race, unrelated to version
// skipping. relayer-migrate has no v0.11.1 release, so it stays at v0.11.0.
export const v011 = {
  RELAYER_VERSION: "v0.11.1",
  RELAYER_MIGRATE_VERSION: "v0.11.0",
  GATEWAY_VERSION: v011Tag,
  HOST_VERSION: v011Tag,
  CORE_VERSION: "v0.13.0",
  CONNECTOR_DB_MIGRATION_VERSION: v011Tag,
  CONNECTOR_GW_LISTENER_VERSION: v011Tag,
  CONNECTOR_KMS_WORKER_VERSION: v011Tag,
  CONNECTOR_TX_SENDER_VERSION: v011Tag,
  COPROCESSOR_DB_MIGRATION_VERSION: v011Tag,
  COPROCESSOR_HOST_LISTENER_VERSION: v011Tag,
  COPROCESSOR_GW_LISTENER_VERSION: v011Tag,
  COPROCESSOR_TX_SENDER_VERSION: v011Tag,
  COPROCESSOR_TFHE_WORKER_VERSION: v011Tag,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: v011Tag,
  COPROCESSOR_SNS_WORKER_VERSION: v011Tag,
  LISTENER_CORE_VERSION: v011Tag,
  TEST_SUITE_VERSION: testSuiteVersion,
  RELAYER_SDK_VERSION: relayerSdkVersion,
} satisfies Env;

export const v013 = {
  ...v011,
  RELAYER_VERSION: v013Tag,
  RELAYER_MIGRATE_VERSION: v013Tag,
  GATEWAY_VERSION: v013Tag,
  HOST_VERSION: v013Tag,
  CORE_VERSION: "v0.13.20",
  CONNECTOR_DB_MIGRATION_VERSION: v013Tag,
  CONNECTOR_GW_LISTENER_VERSION: v013Tag,
  CONNECTOR_KMS_WORKER_VERSION: v013Tag,
  CONNECTOR_TX_SENDER_VERSION: v013Tag,
  COPROCESSOR_DB_MIGRATION_VERSION: v013Tag,
  COPROCESSOR_HOST_LISTENER_VERSION: v013Tag,
  COPROCESSOR_GW_LISTENER_VERSION: v013Tag,
  COPROCESSOR_TX_SENDER_VERSION: v013Tag,
  COPROCESSOR_TFHE_WORKER_VERSION: v013Tag,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: v013Tag,
  COPROCESSOR_SNS_WORKER_VERSION: v013Tag,
  LISTENER_CORE_VERSION: v013Tag,
} satisfies Env;

type EnvKey = keyof typeof v011;

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

const withTargetVersions = (...keys: EnvKey[]): Env => ({
  ...v011,
  ...Object.fromEntries(keys.map((key) => [key, v013[key]])),
});

// Contracts move first and reach v0.13 in two hops; every runtime component
// then jumps straight from v0.11 to v0.13. The coprocessor is upgraded last, so
// an old-v0.11 coprocessor runs against fully-migrated v0.13 contracts through
// every intermediate phase — the compatibility boundary this rollout exercises.
export const phaseVersions = {
  baseline: v011,
  // Contract waypoint: only the host/gateway deploy images change to v0.12.
  contractsV012: { ...v011, GATEWAY_VERSION: v012Tag, HOST_VERSION: v012Tag },
  contractsV013: withTargetVersions(...contractKeys),
  relayer: withTargetVersions(...contractKeys, ...relayerKeys),
  kms: withTargetVersions(...contractKeys, ...relayerKeys, ...kmsKeys),
  listenerCore: withTargetVersions(...contractKeys, ...relayerKeys, ...kmsKeys, ...listenerKeys),
  coprocessor: v013,
};

export const versionSources = [
  "rollout=v0.11-to-v0.13",
  `from=${v011Tag}`,
  `via=${v012Tag}`,
  `target=${v013Tag}`,
  "kms-core=v0.13.20",
];
