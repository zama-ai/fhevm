type Env = Record<string, string>;

// 3 coprocessors (threshold 2) across two host chains. This is the local stand-in for the
// devnet topology the v0.14 upgrade was rehearsed on (Ethereum + Polygon), and it is the
// smallest scenario where the canonical ProtocolConfig mirror has a second chain to mirror
// onto — the "ETH & Polygon anchors match" check from the devnet report.
export const scenario = "two-of-three-multi-chain";

// fhevm monorepo tags: latest stable 0.13.x -> latest 0.14.0 pre-release.
//
// v0.13.2 publishes an image for every component in this bundle, so the baseline is uniform.
const fromTag = "v0.13.2";
// The target is NOT uniform. fhevm only builds container images for the components a tag
// actually touched, so a pre-release tag carries images for some components and not others.
// v0.14.0-10 published everything except host-contracts and test-suite/e2e, whose newest
// published image is still v0.14.0-9. Pinning those two at v0.14.0-10 boots into an
// unpullable image, so they are pinned one tag back on purpose.
//
// Re-derive after any new 0.14 pre-release, e.g.:
//   docker manifest inspect ghcr.io/zama-ai/fhevm/host-contracts:<tag>
export const targetTag = "v0.14.0-10";
export const hostContractsTargetTag = "v0.14.0-9";
export const testSuiteTargetTag = "v0.14.0-9";

// kms-core ships on its own tag line (the `zama-ai/kms` repo), so it does not follow the
// monorepo tags. Release 0.13.0 serves KMS 0.13.20 and release 0.14.0 serves KMS 0.14.0.
//
// The 0.13.20 -> 0.13.22 -> 0.14.0-1 sequence is mandatory, not cosmetic. 0.14.0-1 always
// applies the PRSS hotfix and 0.13.20 never does, so a cluster running both at once cannot
// reconstruct shares — it fails user decryption with a Gao decoding error. 0.13.22 is the
// only version that can serve alongside peers on either side of the hotfix, which makes it
// the required bridge. Skipping it is what forced the devnet rollback.
const coreFrom = "v0.13.20";
const corePrssBridge = "v0.13.22";
const coreTo = "v0.14.0-1";

export const from = {
  GATEWAY_VERSION: fromTag,
  HOST_VERSION: fromTag,
  RELAYER_VERSION: fromTag,
  RELAYER_MIGRATE_VERSION: fromTag,
  CORE_VERSION: coreFrom,
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
  // The harness runs at its target tag from the first phase, so every phase is measured by
  // the same e2e suite. RELAYER_SDK_VERSION is deliberately unset: empty means the harness
  // uses @fhevm/sdk, which is the reference client for live environments.
  TEST_SUITE_VERSION: testSuiteTargetTag,
} satisfies Env;

export const to = {
  ...from,
  GATEWAY_VERSION: targetTag,
  HOST_VERSION: hostContractsTargetTag,
  RELAYER_VERSION: targetTag,
  RELAYER_MIGRATE_VERSION: targetTag,
  CORE_VERSION: coreTo,
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

// One group per step of the documented component order:
// Gateway Contracts -> Host Contracts -> Relayer -> KMS -> Coprocessors -> SDK.
export const gatewayContractKeys = ["GATEWAY_VERSION"] as const satisfies readonly EnvKey[];
export const hostContractKeys = ["HOST_VERSION"] as const satisfies readonly EnvKey[];
export const relayerKeys = ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"] as const satisfies readonly EnvKey[];
export const coreKeys = ["CORE_VERSION"] as const satisfies readonly EnvKey[];
export const connectorKeys = [
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const satisfies readonly EnvKey[];
export const listenerKeys = ["LISTENER_CORE_VERSION"] as const satisfies readonly EnvKey[];
export const coprocessorKeys = [
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

export type RolloutPhaseKey =
  | "baseline"
  | "gatewayContracts"
  | "relayer"
  | "hostContracts"
  | "kmsPrssBridge"
  | "kms"
  | "listenerCore"
  | "coprocessor";

/**
 * Every phase lock is cumulative: it carries all earlier phases' target versions.
 *
 * The relayer moves before the host contracts, not after. The SDK resolves the protocol version
 * from the on-chain ACL version and routes user decryption to `/v2/user-decrypt` below protocol
 * 0.14 and `/v3/user-decrypt` from 0.14 on. Upgrading the host ACL therefore switches every
 * client to /v3 at once, and the 0.13 relayer only serves /v2 — so contracts-then-relayer breaks
 * decryption for the whole window between the two. The 0.14 relayer serves both routes, so
 * relayer-first is backward compatible with the still-0.13 ACL: expand, then migrate.
 */
export const phaseVersions: Record<RolloutPhaseKey, Env> = {
  baseline: from,
  gatewayContracts: withTargetVersions(...gatewayContractKeys),
  relayer: withTargetVersions(...gatewayContractKeys, ...relayerKeys),
  hostContracts: withTargetVersions(...gatewayContractKeys, ...relayerKeys, ...hostContractKeys),
  // kms-core only, and only as far as the PRSS bridge. The connector stays on 0.13 here:
  // this phase exists to prove the bridge version serves a pre-hotfix cluster unchanged.
  kmsPrssBridge: {
    ...withTargetVersions(...gatewayContractKeys, ...hostContractKeys, ...relayerKeys),
    CORE_VERSION: corePrssBridge,
  },
  kms: withTargetVersions(...gatewayContractKeys, ...hostContractKeys, ...relayerKeys, ...coreKeys, ...connectorKeys),
  listenerCore: withTargetVersions(
    ...gatewayContractKeys,
    ...hostContractKeys,
    ...relayerKeys,
    ...coreKeys,
    ...connectorKeys,
    ...listenerKeys,
  ),
  coprocessor: to,
};

export const versionSources = [
  "rollout=v0.13-to-v0.14",
  `from=${fromTag}`,
  `target=${targetTag}`,
  `kms-core=${coreFrom}->${corePrssBridge}->${coreTo}`,
];
