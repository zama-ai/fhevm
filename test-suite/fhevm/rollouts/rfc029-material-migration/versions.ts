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
// Empty = use the default `@fhevm/sdk` (current SDK, tfhe 1.6.x), matching the
// branch coprocessor (tfhe 1.6.2) + kms-core (1.6.1). A non-empty value pins the
// LEGACY `@zama-fhe/relayer-sdk@x`, whose newest published build is still tfhe
// 1.4 -- deserializing a tfhe-1.6 public key with it fails with
// "expected variant index 0 <= i < 1". (The v0.13 rollouts pin 0.4.2 because they
// test OLD tfhe-1.4 coprocessor images; this rollout runs the branch.)
const relayerSdkVersion = "";

// kms-core image anchor. The RFC-029 migration keygen RPC uses
// copy_compressed_key_to_original + UseExisting + CompressedAll + KeySetAddedInfo, which first
// shipped in kms PR #530 (commit 07b2a8fc, 2026-04-30). The earliest release tag carrying that
// working server impl is the PRE-RELEASE v0.13.20-0; v0.13.20 (tagged 2026-06-11) is its STABLE
// counterpart with a byte-identical proto. We run the stable image: it is wire-compatible with the
// connector (which compiles kms-grpc from tag v0.13.20-0 -- identical proto) and is the v0.13.0
// release pairing the v0.13.0-testnet rollout also uses.
// NB: v0.13.10/.11 predate the feature (no copy_compressed_key_to_original, old wire format);
// v0.13.21 is a proto-identical later stable if a bump is ever wanted.
const kmsCoreImage = "v0.13.20";

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
  `kms-core=${kmsCoreImage} (stable; proto-identical to connector kms-grpc tag v0.13.20-0; has RFC-028 copy_compressed_key_to_original)`,
  "feature=branch-local (RFC-029 coprocessor material-version cutover)",
  "tracks=fhevm-internal#1568",
];
