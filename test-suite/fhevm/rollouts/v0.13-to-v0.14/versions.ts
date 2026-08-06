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
// v0.13.2 is the latest *stable* 0.13.x — it is the release marked Latest on GitHub. v0.13.3
// exists as a bare tag with published images but no release, so it is deliberately not used.
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
// The -> 0.13.22 -> 0.14.0-1 tail is mandatory, not cosmetic. 0.14.0-1 always applies the
// PRSS hotfix and the 0.13.x line never does, so a cluster running both at once cannot
// reconstruct shares — it fails user decryption with a Gao decoding error. 0.13.22 is the
// only version that can serve alongside peers on either side of the hotfix, which makes it
// the required bridge. Skipping it is what forced the devnet rollback.
//
// The baseline is 0.13.11 rather than 0.13.20 because kms-core is what mints the FHE key
// material at bootstrap, and the two sit on opposite sides of a tfhe-rs serialization break:
//
//   kms-core v0.13.11   tfhe = "=1.5.4"
//   kms-core v0.13.20-0 tfhe = "=1.6.1"
//
// tfhe-rs 1.6 added a second variant to CompactCiphertextListExpansionKindVersions, which
// lives inside the compact public key. @zama-fhe/relayer-sdk 0.4.4 carries node-tfhe
// 1.4.0-alpha.3, whose copy of that enum has one variant, so it rejects any key a 1.6 KMS
// generated with "invalid value: integer 1, expected variant index 0 <= i < 1" — surfaced by
// the SDK as the misleading "Impossible to fetch public key: wrong relayer url.".
//
// Booting on 0.13.20 therefore fails the baseline gate before a single component is
// upgraded. Booting on 0.13.11 mints key material the old client can read, and crossing the
// tfhe boundary later in the run is safe because an upgrade never regenerates keys — which
// is also why mainnet/testnet apps on 0.4.x are unaffected by 0.14. The v0.12-to-v0.13
// runbook already relies on exactly this: it boots kms-core v0.13.10, upgrades to v0.13.20-0
// mid-run, and stays green on relayer-sdk 0.4.2 throughout.
const coreFrom = "v0.13.11";
const corePrssBridge = "v0.13.22";
const coreTo = "v0.14.0-1";

// The client library the e2e gates call the relayer with — its own version line, unrelated
// to the monorepo tags above. Two packages can occupy this slot and `instance.ts` picks
// between them at runtime:
//
//   RELAYER_SDK_VERSION non-empty -> @zama-fhe/relayer-sdk at exactly that npm version
//   RELAYER_SDK_VERSION empty     -> @fhevm/sdk, the in-repo workspace SDK
//
// 0.4.4 is the current npm `latest` of @zama-fhe/relayer-sdk. It resolves relayer routes to
// /v1 or /v2 only — there is no /v3 anywhere in the published package — so it keeps calling
// /v2/user-decrypt no matter which protocol version the host contracts report. That is what
// lets every backend component cross the 0.13 -> 0.14 boundary on its own: the client does
// not change its request shape underneath the rollout.
//
// @fhevm/sdk is the opposite: it reads the on-chain ACL version, maps it to a protocol
// version, and switches to /v3/user-decrypt from protocol 0.14. Running it from the first
// phase makes host contracts, relayer and KMS connector all become load-bearing the moment
// the ACL lands, which is why the client moves last and alone.
const relayerSdkFrom = "0.4.4";
// Empty selects @fhevm/sdk, which the harness image builds from its own source tree. At the
// harness tag used here (v0.14.0-9) that workspace package is version 1.1.0-alpha.9 — the
// 0.14 SDK, and the first client in this rollout to exercise /v3.
const relayerSdkTo = "";

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
  // The harness image runs at its target tag from the first phase, so every phase is measured
  // by the same e2e suite and only the client library underneath it moves. The 0.13 harness
  // could not host this rollout anyway: at v0.13.2 test-suite/e2e depends solely on
  // @fhevm/sdk and the runtime switch between the two clients does not exist yet.
  TEST_SUITE_VERSION: testSuiteTargetTag,
  RELAYER_SDK_VERSION: relayerSdkFrom,
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
  RELAYER_SDK_VERSION: relayerSdkTo,
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
export const sdkKeys = ["RELAYER_SDK_VERSION"] as const satisfies readonly EnvKey[];

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
  | "sdk";

/**
 * Every phase lock is cumulative: it carries all earlier phases' target versions.
 *
 * The order is the documented component order, and the client SDK moves last on purpose.
 * Two directional constraints make that ordering load-bearing rather than conventional:
 *
 *  - The 0.14 relayer cannot boot against 0.13 host contracts. It initializes `/v2/keyurl` by
 *    calling `getCurrentKmsContextAndEpoch()` on ProtocolConfig, which only exists from 0.14,
 *    so against 0.13 the call reverts with empty data and the relayer exits. Host contracts
 *    therefore have to land before the relayer.
 *  - A client that follows the on-chain protocol version pulls the rest of the stack forward
 *    with it. @fhevm/sdk switches to /v3/user-decrypt as soon as the host ACL reports 0.14,
 *    and /v3 needs both the 0.14 relayer and the 0.14 KMS connector to be serving. Holding
 *    the gates on @zama-fhe/relayer-sdk 0.4.4, which only ever calls /v2, keeps each backend
 *    phase independently testable; the `sdk` phase then moves the client on its own, against
 *    a stack that is already fully on 0.14.
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
  sdk: to,
};

export const versionSources = [
  "rollout=v0.13-to-v0.14",
  `from=${fromTag}`,
  `target=${targetTag}`,
  `kms-core=${coreFrom}->${corePrssBridge}->${coreTo}`,
  `relayer-sdk=${relayerSdkFrom}->@fhevm/sdk`,
];
