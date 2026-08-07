import { expect, test } from "bun:test";

import {
  canonicalFirst,
  gatewayContractUpgradeOrder,
  hostContractUpgradeOrder,
  hostContractUpgradesForChain,
  phaseOrder,
  resolveRolloutTestMode,
  rolloutPhaseTestProfiles,
} from "../rollouts/v0.13-to-v0.14/run";
import {
  connectorKeys,
  coprocessorKeys,
  from,
  hostContractsTargetTag,
  phaseVersions,
  scenario,
  targetTag,
  testSuiteTargetTag,
  to,
} from "../rollouts/v0.13-to-v0.14/versions";

test("rehearses the upgrade on a multi-chain topology with consensus-capable coprocessors", () => {
  // The canonical ProtocolConfig mirror needs a second host chain to mirror onto, the
  // drift/consensus checks need 3 coprocessors at threshold 2, and upgrading participants one
  // at a time needs a real 4-party threshold KMS cluster.
  expect(scenario).toBe("two-of-three-multi-chain-threshold-kms");
});

test("keeps the kms-core PRSS bridge as a mandatory stop between 0.13.20 and 0.14", () => {
  expect(phaseVersions.baseline.CORE_VERSION).toBe("v0.13.20");
  expect(phaseVersions.kmsPrssBridge.CORE_VERSION).toBe("v0.13.22");
  expect(phaseVersions.kms.CORE_VERSION).toBe("v0.14.0-1");
  // A cluster must never jump straight from the pre-hotfix to the post-hotfix core.
  expect(phaseVersions.relayer.CORE_VERSION).toBe(phaseVersions.baseline.CORE_VERSION);
});

test("moves kms-core alone in the PRSS bridge phase, leaving every connector on 0.13", () => {
  for (const key of connectorKeys) {
    expect(phaseVersions.kmsPrssBridge[key]).toBe(from[key]);
  }
  // ...and moves them with the core only in the kms phase.
  for (const key of connectorKeys) {
    expect(phaseVersions.kms[key]).toBe(to[key]);
  }
});

test("upgrades the gateway chain strictly before the host chain", () => {
  expect(phaseVersions.gatewayContracts.GATEWAY_VERSION).toBe(to.GATEWAY_VERSION);
  expect(phaseVersions.gatewayContracts.HOST_VERSION).toBe(from.HOST_VERSION);
  expect(phaseVersions.hostContracts.HOST_VERSION).toBe(to.HOST_VERSION);
});

test("splits listener-core from the coprocessor image bump", () => {
  expect(phaseVersions.listenerCore.LISTENER_CORE_VERSION).toBe(to.LISTENER_CORE_VERSION);
  expect(phaseVersions.listenerCore.COPROCESSOR_DB_MIGRATION_VERSION).toBe(from.COPROCESSOR_DB_MIGRATION_VERSION);
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).toBe(to.COPROCESSOR_DB_MIGRATION_VERSION);
});

test("runs the harness at the target tag from the first phase, and moves only the client", () => {
  expect(phaseVersions.baseline.TEST_SUITE_VERSION).toBe(to.TEST_SUITE_VERSION);
  expect(phaseVersions.protocolFlip.TEST_SUITE_VERSION).toBe(to.TEST_SUITE_VERSION);
});

// An empty RELAYER_SDK_VERSION selects the in-repo @fhevm/sdk, and it is the client for every
// phase. No published @zama-fhe/relayer-sdk can be used here at all: kms-core mints the public
// key at bootstrap on tfhe-rs 1.6, and neither 0.4.4 (node-tfhe 1.4.0-alpha.3) nor 0.5.0-rc.1
// (1.5.4) can deserialize that key, so they fail before the baseline gate rather than at any
// upgrade boundary.
test("runs every phase on the in-repo @fhevm/sdk", () => {
  for (const env of Object.values(phaseVersions)) {
    expect(env.RELAYER_SDK_VERSION).toBe("");
  }
});

// No released 0.14 ships consensus-detector or upgrade-controller — the crates first appear
// after v0.14.0-11 — so neither end of this rollout runs them and no phase may name a tag for
// them. The surrounding stack resolves from main, where both are published, so a phase that
// left either key out would inherit main's tag; the empty string is what keeps them off the
// bundle, and a tag would be pulled and fail as an unknown manifest.
test("never asks a released 0.14 for components it does not ship", () => {
  for (const env of Object.values(phaseVersions)) {
    expect(env.COPROCESSOR_CONSENSUS_DETECTOR_VERSION).toBe("");
    expect(env.COPROCESSOR_UPGRADE_CONTROLLER_VERSION).toBe("");
  }
});

test("moves the ACL strictly last, after every backend component is on target", () => {
  expect(phaseOrder[phaseOrder.length - 1]).toBe("protocol-flip");
  // The phase before it already has the whole backend at its target versions...
  for (const key of coprocessorKeys) {
    expect(phaseVersions.coprocessor[key]).toBe(to[key]);
  }
  // ...so the protocol-flip phase moves no image at all: it upgrades on-chain state only.
  expect(phaseVersions.protocolFlip).toEqual(phaseVersions.coprocessor);
});

test("pins host-contracts and the harness one tag back, where images actually exist", () => {
  // fhevm publishes images only for the components a tag touched. v0.14.0-10 has no
  // host-contracts or test-suite/e2e image, so pinning them there boots an unpullable ref.
  expect(to.HOST_VERSION).toBe(hostContractsTargetTag);
  expect(to.TEST_SUITE_VERSION).toBe(testSuiteTargetTag);
  expect(to.HOST_VERSION).not.toBe(targetTag);
  // Everything else does publish at the newest pre-release.
  expect(to.GATEWAY_VERSION).toBe(targetTag);
  expect(to.RELAYER_VERSION).toBe(targetTag);
  expect(to.COPROCESSOR_TFHE_WORKER_VERSION).toBe(targetTag);
  expect(to.LISTENER_CORE_VERSION).toBe(targetTag);
});

test("keeps every phase lock cumulative", () => {
  const ordered = [
    phaseVersions.baseline,
    phaseVersions.gatewayContracts,
    phaseVersions.hostContracts,
    phaseVersions.relayer,
    phaseVersions.kms,
    phaseVersions.listenerCore,
    phaseVersions.coprocessor,
    phaseVersions.protocolFlip,
  ];
  // Once a key reaches its target it never goes back. The PRSS bridge phase is excluded
  // because its CORE_VERSION is deliberately an intermediate, not a target.
  for (let index = 1; index < ordered.length; index += 1) {
    for (const [key, value] of Object.entries(ordered[index - 1])) {
      if (value === to[key as keyof typeof to]) {
        expect(ordered[index][key]).toBe(value);
      }
    }
  }
});

// Host contracts and the relayer cross in one phase, gated only once, because neither order
// survives a gate between them:
//
//  - Relayer first: the 0.14 relayer boots by calling getCurrentKmsContextAndEpoch() on
//    ProtocolConfig, which only exists from 0.14, so it exits against 0.13 contracts.
//  - Host contracts first: @fhevm/sdk derives its extraData format from host contract state,
//    so once KMSVerifier and ProtocolConfig report a KMS context and epoch the client sends
//    v2 extraData and the 0.13 relayer rejects user decryption with `validation_failed`.
test("crosses the host contracts and the relayer in a single gated phase", () => {
  expect(phaseOrder).toContain("host-contracts-relayer");
  expect(phaseOrder).not.toContain("host-contracts");
  expect(phaseOrder).not.toContain("relayer");
  // The contract half still lands first inside the phase, so the relayer finds the
  // ProtocolConfig call it boots on...
  expect(phaseVersions.hostContracts.HOST_VERSION).toBe(to.HOST_VERSION);
  expect(phaseVersions.hostContracts.RELAYER_VERSION).toBe(from.RELAYER_VERSION);
  // ...and the relayer lock follows immediately, with no gate observing the state between.
  expect(phaseVersions.relayer.HOST_VERSION).toBe(to.HOST_VERSION);
  expect(phaseVersions.relayer.RELAYER_VERSION).toBe(to.RELAYER_VERSION);
});

test("ends every phase on the target versions", () => {
  expect(phaseVersions.protocolFlip).toEqual(to);
});

test("orders host contract upgrades so verification and limits land before the executor", () => {
  const position = (name: string) => hostContractUpgradeOrder.indexOf(name);
  expect(position("KMSVerifier")).toBeLessThan(position("FHEVMExecutor"));
  expect(position("HCULimit")).toBeLessThan(position("FHEVMExecutor"));
  // The ACL is not upgraded in this phase at all — it moves alone in the last one, because it
  // is what @fhevm/sdk reads to decide between /v2 and /v3.
  expect(hostContractUpgradeOrder).not.toContain("ACL");
});

// Every upgradeable contract guards its reinitializer with `reinitializer(REINITIALIZER_VERSION)`,
// and a fresh deploy runs `initializeFromEmptyProxy` under the same constant. A contract whose
// source did not change between the two tags therefore carries an unchanged constant, and
// upgrading it reverts with OpenZeppelin's `InvalidInitialization()` (0xf92ee8a9). Between
// v0.13.2 and v0.14.0-10 only Decryption (6 -> 7) changed on the gateway side.
test("upgrades only the gateway contract whose reinitializer v0.14 bumped", () => {
  expect(gatewayContractUpgradeOrder).toEqual(["Decryption"]);
  expect(gatewayContractUpgradeOrder).not.toContain("GatewayConfig");
  expect(gatewayContractUpgradeOrder).not.toContain("KMSGeneration");
});

// ProtocolConfig is upgraded in place on every chain, canonical first, because canonical holds
// the KMS context the other chains' stored state must match.
test("migrates the canonical host chain's ProtocolConfig before the others", () => {
  const ordered = canonicalFirst([
    { isCanonical: false, key: "chain-b" },
    { isCanonical: true, key: "host" },
    { isCanonical: false, key: "chain-c" },
  ]);
  expect(ordered.map((target) => target.key)).toEqual(["host", "chain-b", "chain-c"]);
});

// KMSGeneration is anchored on the canonical host chain only; the boot flow asserts every other
// chain's address file has no KMS_GENERATION_CONTRACT_ADDRESS, so upgrading it elsewhere fails
// resolving that address.
test("upgrades KMSGeneration on the canonical host chain only", () => {
  const names = (isCanonical: boolean): string[] =>
    hostContractUpgradesForChain(isCanonical).map((upgrade) => upgrade.name);
  expect(names(true)).toContain("KMSGeneration");
  expect(names(false)).not.toContain("KMSGeneration");
  // Everything else moves on every chain.
  for (const name of ["KMSVerifier", "HCULimit", "FHEVMExecutor"]) {
    expect(names(false)).toContain(name);
  }
});

test("gates every phase on rollout-standard by default", () => {
  expect(resolveRolloutTestMode(undefined)).toBe("rollout-standard");
  for (const phase of phaseOrder) {
    expect(rolloutPhaseTestProfiles(phase, "rollout-standard")).toEqual(["rollout-standard"]);
  }
});

test("covers multi-chain isolation in heavy mode wherever contracts moved", () => {
  expect(rolloutPhaseTestProfiles("host-contracts-relayer", "rollout-heavy")).toContain("multi-chain-isolation");
  expect(rolloutPhaseTestProfiles("protocol-flip", "rollout-heavy")).toContain("multi-chain-isolation");
});

// /v3/user-decrypt is only reachable once the ACL reports 0.14, so the route is checked in the
// protocol-flip phase and nowhere earlier — before it the client resolves 0.13 and asks for /v2.
test("checks the v3 user-decryption route in the protocol-flip phase in heavy mode", () => {
  expect(rolloutPhaseTestProfiles("protocol-flip", "rollout-heavy")).toContain("unified-user-decryption");
  for (const phase of phaseOrder.filter((candidate) => candidate !== "protocol-flip")) {
    expect(rolloutPhaseTestProfiles(phase, "rollout-heavy")).not.toContain("unified-user-decryption");
  }
});

test("rejects unsupported rollout test modes", () => {
  expect(() => resolveRolloutTestMode("standard")).toThrow("Unsupported ROLLOUT_TEST_PROFILE=standard");
});
