type Env = Record<string, string>;

// 4 KMS parties, 3 coprocessors (threshold 2), two host chains.
//
// Two chains are the local stand-in for the devnet topology the v0.14 upgrade was rehearsed
// on (Ethereum + Polygon), and the smallest shape where the canonical ProtocolConfig mirror
// has a second chain to mirror onto — the "ETH & Polygon anchors match" check from the
// devnet report. 4 KMS parties and 3 coprocessors are what make the participants upgradeable
// one at a time: each KMS node crosses with its own connector, and 3 coprocessors at
// threshold 2 make the consensus states observable as instances cross (below threshold, at
// threshold, all upgraded).
export const scenario = "two-of-three-multi-chain-threshold-kms";

// fhevm monorepo tags: latest stable 0.13.x -> latest 0.14.0 pre-release.
//
// v0.13.2 is the newest 0.13.x on the remote and the release marked Latest on GitHub. Clones
// made before 2026-08 may still hold a local v0.13.3 tag; it is not on origin (`git ls-remote
// --tags origin` lists only v0.13.0, v0.13.1, v0.13.2), so nothing can be pulled from it.
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

// The client library the e2e gates call the relayer with — its own version line, unrelated
// to the monorepo tags above. Two packages can occupy this slot and `instance.ts` picks
// between them at runtime:
//
//   RELAYER_SDK_VERSION non-empty -> @zama-fhe/relayer-sdk at exactly that npm version
//   RELAYER_SDK_VERSION empty     -> @fhevm/sdk, the in-repo workspace SDK
//
// Empty selects @fhevm/sdk, the in-repo workspace SDK, which the harness image builds from
// its own source tree. It is the client for every phase, and it cannot be @zama-fhe/relayer-sdk.
//
// No published @zama-fhe/relayer-sdk can run this rollout at all, at any phase, because of the
// key material rather than the protocol. kms-core mints the FHE public key at bootstrap, and
// every kms-core paired with fhevm 0.13+ is on tfhe-rs 1.6 (v0.13.20 -> 1.6.1, v0.14.0-1 ->
// 1.6.2). tfhe-rs 1.6 added a second variant to CompactCiphertextListExpansionKindVersions,
// which lives inside the compact public key; the enum derives VersionsDispatch, so a 1.6
// writer always emits the new variant. relayer-sdk 0.4.4 carries node-tfhe 1.4.0-alpha.3 and
// 0.5.0-rc.1 carries 1.5.4 — both have the one-variant enum, so both reject the key with
// "invalid value: integer 1, expected variant index 0 <= i < 1" before the first gate runs.
//
// Deployed networks are unaffected: their key material predates tfhe-rs 1.6 and an upgrade
// never regenerates it. That compatibility can only be reproduced on a stack seeded with
// legacy key material, which this harness cannot do — it always mints its own. The same
// constraint is already recorded in .github/workflows/preview-env-deploy.yml, where the
// routine e2e runs the @fhevm/sdk suite for exactly this reason.
const relayerSdkVersion = "";

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
  // No released 0.14 ships consensus-detector or upgrade-controller: the crates first appear
  // after v0.14.0-11, so neither end of this rollout runs them. They are listed rather than
  // omitted because the surrounding stack resolves from main, where both are published, and a
  // key this map does not mention is inherited from that snapshot. Empty means "not part of
  // this release", and the lock writer drops it so the bundle simply has no such component.
  COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "",
  COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "",
  LISTENER_CORE_VERSION: fromTag,
  // The harness never moves: it is pinned here for the lock's sake, but `run.ts` overrides the
  // test-suite group to a local build, so what actually runs is this branch's e2e suite and
  // this branch's @fhevm/sdk at every phase. That is deliberate — the client is the component
  // whose behaviour decides this runbook's ordering, so it is built from the tree under test
  // rather than taken from a published image. The 0.13 harness could not host this rollout
  // anyway: at v0.13.2 test-suite/e2e depends solely on @fhevm/sdk and the runtime switch
  // between the two client libraries does not exist yet.
  TEST_SUITE_VERSION: testSuiteTargetTag,
  RELAYER_SDK_VERSION: relayerSdkVersion,
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
  // consensus-detector and upgrade-controller stay unset here too — see `from`. The target end
  // of this rollout is a released 0.14 tag, which has no such images to pull.
  LISTENER_CORE_VERSION: targetTag,
} satisfies Env;

type EnvKey = keyof typeof from;

// One group per step of the documented component order:
// Gateway Contracts -> Host Contracts -> Relayer -> KMS -> Listener -> Coprocessors -> SDK.
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
// There is deliberately no key group for the final phase. The client never moves, so no SDK
// version step exists; what that phase changes is the host ACL, which is on-chain state rather
// than an image tag, and that is what moves the protocol version @fhevm/sdk reads off chain.

const withTargetVersions = (...keys: EnvKey[]): Env => ({
  ...from,
  ...Object.fromEntries(keys.map((key) => [key, to[key]])),
});

export type RolloutPhaseKey =
  | "baseline"
  | "gatewayContracts"
  | "hostContracts"
  | "relayer"
  | "kmsPrssBridge"
  | "kms"
  | "listenerCore"
  | "coprocessor"
  | "protocolFlip";

/**
 * Every phase lock is cumulative: it carries all earlier phases' target versions.
 *
 * The order is the documented component order, and the host ACL moves last on purpose.
 * Two directional constraints make that ordering load-bearing rather than conventional:
 *
 *  - The 0.14 relayer cannot boot against 0.13 host contracts. It initializes `/v2/keyurl` by
 *    calling `getCurrentKmsContextAndEpoch()` on ProtocolConfig, which only exists from 0.14,
 *    so against 0.13 the call reverts with empty data and the relayer exits. Host contracts
 *    therefore have to land before the relayer.
 *  - The client follows the on-chain protocol version, so the ACL is what pulls the rest of
 *    the stack forward. @fhevm/sdk maps the ACL version to a protocol version and switches to
 *    /v3/user-decrypt from 0.14, and /v3 needs both the 0.14 relayer and the 0.14 KMS
 *    connector to be serving. Every gate runs `user-decryption`, so upgrading the ACL early
 *    would make the relayer and connector load-bearing from that moment and collapse the
 *    per-component phases into one step. Holding the ACL back keeps the client on /v2 while
 *    each backend component crosses on its own; the `protocolFlip` phase then upgrades the
 *    ACL against a stack that is already fully on 0.14, and the client follows it onto /v3.
 */
export const phaseVersions: Record<RolloutPhaseKey, Env> = {
  baseline: from,
  gatewayContracts: withTargetVersions(...gatewayContractKeys),
  hostContracts: withTargetVersions(...gatewayContractKeys, ...hostContractKeys),
  relayer: withTargetVersions(...gatewayContractKeys, ...hostContractKeys, ...relayerKeys),
  // kms-core only, and only as far as the PRSS bridge. The connector stays on 0.13 here:
  // this phase exists to prove the bridge version serves a pre-hotfix cluster unchanged.
  kmsPrssBridge: {
    ...withTargetVersions(...gatewayContractKeys, ...hostContractKeys, ...relayerKeys),
    CORE_VERSION: corePrssBridge,
  },
  // Applied node by node: each party's core and its own connector cross together, because a
  // connector only talks to its own core and the pair cannot straddle the boundary.
  kms: withTargetVersions(...gatewayContractKeys, ...hostContractKeys, ...relayerKeys, ...coreKeys, ...connectorKeys),
  listenerCore: withTargetVersions(
    ...gatewayContractKeys,
    ...hostContractKeys,
    ...relayerKeys,
    ...coreKeys,
    ...connectorKeys,
    ...listenerKeys,
  ),
  // Applied one operator at a time, so the fleet is observed below, at, and above the
  // consensus threshold rather than jumping straight to fully upgraded.
  coprocessor: withTargetVersions(
    ...gatewayContractKeys,
    ...hostContractKeys,
    ...relayerKeys,
    ...coreKeys,
    ...connectorKeys,
    ...listenerKeys,
    ...coprocessorKeys,
  ),
  // No version moves here: this phase upgrades the host ACL, which is on-chain state rather
  // than an image tag. The lock is the coprocessor phase's, carried forward unchanged.
  protocolFlip: to,
};

export const versionSources = [
  "rollout=v0.13-to-v0.14",
  `from=${fromTag}`,
  `target=${targetTag}`,
  `kms-core=${coreFrom}->${corePrssBridge}->${coreTo}`,
  "client=@fhevm/sdk",
];
