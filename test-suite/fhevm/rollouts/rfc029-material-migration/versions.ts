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
// Topology: 5 coprocessors, threshold 3, a real 4-party threshold-mode KMS,
// and two host chains (L1 + a Polygon stand-in, chain-b). The 5-coprocessor
// shape makes the zero-divergence assertion meaningful (any per-operation
// material-version split across the fleet breaks consensus); the two-host-chain
// shape exercises a per-chain cutover block (H_C) on each chain plus the single
// gateway cutover block (G). Defined in scenarios/rfc029-cutover.yaml.
export const scenario = "rfc029-cutover";

// Single coherent target: the RFC-029 cutover is a coprocessor-INTERNAL material
// switch shipped in one build, not a version upgrade -- so the surrounding stack
// is pinned to the v0.13.0 floor while the coprocessor/connector/host-contracts
// images are built locally from this branch (via the scenario's local
// coprocessor instances + ctx.up overrides in run.ts).
const target = "v0.13.0";
const relayerSdkVersion = "0.4.2";

// kms-core image anchor. MUST be the same kms commit the connector compiles its
// gRPC proto against (kms-connector/Cargo.toml pins kms-grpc rev 1edf3a0), so
// the RFC-029 migration keygen RPC (KeyGenRequest + KeySetAddedInfo /
// copy_compressed_key_to_original, UseExisting + CompressedAll) is proto-compatible
// end to end. `ghcr.io/zama-ai/kms/core-service:1edf3a0` is published and
// pullable, and 1edf3a0 ("explicit num_parties #619", 2026-05-29) carries the
// RFC-028 keygen-from-existing implementation. (NOTE: an earlier note referenced
// "43fb606" -- that is not a kms commit and has no image; 1edf3a0 is the correct,
// connector-matched anchor.)
const kmsCoreImage = "1edf3a0";

export const versions = {
  RELAYER_VERSION: target,
  RELAYER_MIGRATE_VERSION: target,
  GATEWAY_VERSION: target,
  HOST_VERSION: target,
  CORE_VERSION: kmsCoreImage,
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
  `kms-core=${kmsCoreImage} (matches connector kms-grpc pin 1edf3a0)`,
  "feature=branch-local (RFC-029 coprocessor material-version cutover)",
  "tracks=fhevm-internal#1568",
];
