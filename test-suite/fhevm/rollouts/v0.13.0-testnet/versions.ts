type Env = Record<string, string>;

// Reproduces the TESTNET v0.12 -> v0.13.0 upgrade (the imminent cutover),
// following Yohan's "0.13.0 migration runbooks" + gitops PR zama-zws/gitops#1063
// step-for-step, to surface gaps in the runbook (NOT to force green).
//
// Single coprocessor, threshold 1, two host chains (host + a Polygon stand-in):
// matches testnet onboarding Phase 1 (Zama-only consensus) during the upgrade
// window, and lets us exercise the Polygon deploy + addHostChain proposal path.
// The split prepare/apply flow lives in run.ts.
export const scenario = "multi-chain";

// EXACT current-testnet tags (the "from"), read from gitops on 2026-06-17. The
// baseline is heterogeneous: KMS core is already promoted to v0.13.10 while the
// rest is v0.12.x, and even the coprocessor workers are mixed. Do not collapse
// these to a single tag -- fidelity to the real testnet state matters here.
//   contracts (gw+host) ...... v0.12.1   (gitops#1063 old image)
//   kms-core ................. v0.13.10  (already on testnet)
//   kms-connector (all) ...... v0.12.0
//   coproc tfhe/zkproof ...... v0.12.3
//   coproc sns/listeners/txs . v0.12.0
//   coproc db-migration ...... v0.12.2
//   relayer / relayer-migrate  v0.11.1 / v0.11.0
const target = "v0.13.0";
const relayerSdkVersion = "0.4.2";

export const from = {
  RELAYER_VERSION: "v0.11.1",
  RELAYER_MIGRATE_VERSION: "v0.11.0",
  GATEWAY_VERSION: "v0.12.1",
  HOST_VERSION: "v0.12.1",
  CORE_VERSION: "v0.13.10",
  CONNECTOR_DB_MIGRATION_VERSION: "v0.12.0",
  CONNECTOR_GW_LISTENER_VERSION: "v0.12.0",
  CONNECTOR_KMS_WORKER_VERSION: "v0.12.0",
  CONNECTOR_TX_SENDER_VERSION: "v0.12.0",
  COPROCESSOR_DB_MIGRATION_VERSION: "v0.12.2",
  COPROCESSOR_HOST_LISTENER_VERSION: "v0.12.0",
  COPROCESSOR_GW_LISTENER_VERSION: "v0.12.0",
  COPROCESSOR_TX_SENDER_VERSION: "v0.12.0",
  COPROCESSOR_TFHE_WORKER_VERSION: "v0.12.3",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.12.3",
  COPROCESSOR_SNS_WORKER_VERSION: "v0.12.0",
  // listener-core (v2) is a v0.13 component with no v0.12 image; it only
  // activates once COPROCESSOR_HOST_LISTENER_VERSION reaches v0.13 (gated by
  // supportsHostListenerConsumer), so the value is inert at the v0.12 baseline.
  LISTENER_CORE_VERSION: target,
  TEST_SUITE_VERSION: target,
  RELAYER_SDK_VERSION: relayerSdkVersion,
} satisfies Env;

// Target ("to"). Contracts v0.13.0 are CONFIRMED by gitops#1063. KMS core ->
// v0.13.20 follows the v0.13.0 release pairing (Amina, 2026-06-11). The runtime
// targets (connector/coprocessor/relayer -> v0.13.0) are INFERRED: gitops has no
// open PR yet that bumps the runtime services, which is itself a runbook gap.
export const to = {
  ...from,
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
const coprocessorKeys = [
  "COPROCESSOR_DB_MIGRATION_VERSION",
  "COPROCESSOR_HOST_LISTENER_VERSION",
  "COPROCESSOR_GW_LISTENER_VERSION",
  "COPROCESSOR_TX_SENDER_VERSION",
  "COPROCESSOR_TFHE_WORKER_VERSION",
  "COPROCESSOR_ZKPROOF_WORKER_VERSION",
  "COPROCESSOR_SNS_WORKER_VERSION",
  "LISTENER_CORE_VERSION",
] as const satisfies readonly EnvKey[];

const withTargetVersions = (...keys: EnvKey[]): Env => ({
  ...from,
  ...Object.fromEntries(keys.map((key) => [key, to[key]])),
});

// Runbook order (devnet precedent, per Amina/Yohan): contracts (prepare + apply
// the proposal effect) -> kms -> coprocessor -> relayer; the Polygon fresh
// deploy + addHostChain proposal effect runs against the second host chain.
export const phaseVersions = {
  baseline: from,
  contracts: withTargetVersions(...contractKeys),
  kms: withTargetVersions(...contractKeys, ...kmsKeys),
  coprocessor: withTargetVersions(...contractKeys, ...kmsKeys, ...coprocessorKeys),
  relayer: to,
};

export const versionSources = [
  "rollout=v0.13.0-testnet",
  "from=v0.12.1-testnet-baseline",
  `target=${target}`,
  "kms-core=v0.13.10->v0.13.20",
  "follows=gitops#1063 + 0.13.0 migration runbooks",
];
