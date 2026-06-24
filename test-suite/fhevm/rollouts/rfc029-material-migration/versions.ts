type Env = Record<string, string>;

// Models the RFC-029 key-material version cutover (fhevm-internal#1568).
//
// Unlike v0.13.0-testnet, this is NOT a version upgrade: the cutover is a
// coprocessor-INTERNAL material switch (legacy ServerKey -> migrated
// CompressedXofKeySet) that ships in a single build. So every component
// stays pinned at one coherent target; the rollout's phases are migration
// ACTIONS (publish v1 material, publish the schedule, cross the cutover
// blocks), not image-tag bumps.
//
// Topology mirrors testnet: one coprocessor, threshold 1, two host chains
// (L1 + a Polygon stand-in, chain-b). The two-host-chain shape is the whole
// point -- it exercises a per-chain cutover block (H_C) on each chain plus
// the single gateway cutover block (G).
export const scenario = "multi-chain";

// Single coherent target. The feature under test lives on the current build
// (`target: "latest-main"` in the rollout up-options); these tags pin the
// surrounding stack to the v0.13.0 floor so the lock file is concrete.
const target = "v0.13.0";
const relayerSdkVersion = "0.4.2";

export const versions = {
  RELAYER_VERSION: target,
  RELAYER_MIGRATE_VERSION: target,
  GATEWAY_VERSION: target,
  HOST_VERSION: target,
  CORE_VERSION: "v0.13.20",
  CONNECTOR_DB_MIGRATION_VERSION: target,
  CONNECTOR_GW_LISTENER_VERSION: target,
  CONNECTOR_KMS_WORKER_VERSION: target,
  CONNECTOR_TX_SENDER_VERSION: target,
  COPROCESSOR_DB_MIGRATION_VERSION: target,
  COPROCESSOR_HOST_LISTENER_VERSION: target,
  COPROCESSOR_GW_LISTENER_VERSION: target,
  COPROCESSOR_TX_SENDER_VERSION: target,
  COPROCESSOR_TFHE_WORKER_VERSION: target,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: target,
  COPROCESSOR_SNS_WORKER_VERSION: target,
  LISTENER_CORE_VERSION: target,
  TEST_SUITE_VERSION: target,
  RELAYER_SDK_VERSION: relayerSdkVersion,
} satisfies Env;

// One phase: boot the whole stack at the target, then drive the cutover from
// within run.ts (no inter-phase tag changes).
export const phaseVersions = {
  baseline: versions,
};

export const versionSources = [
  "rollout=rfc029-material-migration",
  `target=${target}`,
  "feature=latest-main (RFC-029 coprocessor material-version cutover)",
  "tracks=fhevm-internal#1568",
];
