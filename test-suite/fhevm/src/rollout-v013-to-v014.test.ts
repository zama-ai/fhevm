import { expect, test } from "bun:test";

import { loadCoprocessorScenario, resolveScenarioFile, resolveScenarioReference } from "./scenario/resolve";
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

test("rehearses the upgrade on the fleet shape testnet actually runs", () => {
  // The canonical ProtocolConfig mirror needs a second host chain to mirror onto, and upgrading
  // participants one at a time needs a real 4-party threshold KMS cluster. The coprocessor count
  // is not free choice: testnet Phase 3 is 3-of-5, and the drift question this rollout answers is
  // about that arithmetic specifically — at 3-of-5 a minority can be outvoted rather than merely
  // absent, which is the state where divergence hides behind a passing gate.
  expect(scenario).toBe("three-of-five-multi-chain-threshold-kms");
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

// An operator lands a contract upgrade and a relayer deploy as separate actions, minutes to days
// apart, so these are two phases rather than one. The order within the pair is forced, not
// chosen, and the intermediate state costs something either way:
//
//  - Relayer first is unavailable: the 0.14 relayer boots by calling getCurrentKmsContextAndEpoch()
//    on ProtocolConfig, which only exists from 0.14, so it exits against 0.13 contracts.
//  - Host contracts first opens a window: @fhevm/sdk derives its extraData format from host
//    contract state, so once ProtocolConfig reports a non-zero context and epoch the client sends
//    v2 extraData and the 0.13 relayer rejects user decryption with `validation_failed`.
// The window is therefore observed rather than skipped, and its cost asserted in run.ts.
test("crosses the host contracts and the relayer as two phases with the window between them", () => {
  const host = phaseOrder.indexOf("host-contracts");
  const relayer = phaseOrder.indexOf("relayer");
  expect(host).toBeGreaterThanOrEqual(0);
  expect(relayer).toBe(host + 1);
  expect(phaseOrder).not.toContain("host-contracts-relayer");
  // The contracts land first, so the relayer finds the ProtocolConfig call it boots on...
  expect(phaseVersions.hostContracts.HOST_VERSION).toBe(to.HOST_VERSION);
  expect(phaseVersions.hostContracts.RELAYER_VERSION).toBe(from.RELAYER_VERSION);
  // ...and the relayer crosses only in the phase after it.
  expect(phaseVersions.relayer.HOST_VERSION).toBe(to.HOST_VERSION);
  expect(phaseVersions.relayer.RELAYER_VERSION).toBe(to.RELAYER_VERSION);
});

// Nothing in the window may decrypt: the client sends v2 extraData once the host contracts
// report a context and epoch, and the 0.13 relayer rejects it.
test("keeps decrypting profiles out of the host-contracts gate", () => {
  for (const mode of ["rollout-standard", "rollout-heavy"] as const) {
    const profiles = rolloutPhaseTestProfiles("host-contracts", mode);
    expect(profiles).not.toContain("rollout-standard");
    expect(profiles).not.toContain("user-decryption");
    expect(profiles).not.toContain("public-decryption");
  }
});

test("ends every phase on the target versions", () => {
  expect(phaseVersions.protocolFlip).toEqual(to);
});

// The v0.14 devnet upgrade ran the host chain in this order on 14 July:
// ProtocolConfig -> KMSGeneration -> KMSVerifier -> HCULimit -> FHEVMExecutor -> ACL.
// ProtocolConfig is migrated separately, ahead of this list; the rest is matched here.
test("orders host contract upgrades the way the v0.14 devnet upgrade ran them", () => {
  const position = (name: string) => hostContractUpgradeOrder.indexOf(name);
  expect(hostContractUpgradeOrder).toEqual(["KMSGeneration", "KMSVerifier", "HCULimit", "FHEVMExecutor"]);
  expect(position("KMSVerifier")).toBeLessThan(position("FHEVMExecutor"));
  expect(position("HCULimit")).toBeLessThan(position("FHEVMExecutor"));
  // The one deviation from devnet: the ACL is not upgraded in this phase at all, it moves alone
  // in the last one, because the client built from this tree reads it to pick /v2 or /v3. The
  // published clients devnet ran do not, which is why devnet could move it here.
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
  // A workflow forwarding an omitted input passes an empty string, which means the same thing.
  expect(resolveRolloutTestMode("")).toBe("rollout-standard");
  expect(resolveRolloutTestMode("  ")).toBe("rollout-standard");
  // Every phase but the host-contracts window, which runs with the relayer a release behind and
  // so cannot run the decrypting specs that set contains.
  for (const phase of phaseOrder.filter((candidate) => candidate !== "host-contracts")) {
    expect(rolloutPhaseTestProfiles(phase, "rollout-standard")).toEqual(["rollout-standard"]);
  }
  expect(rolloutPhaseTestProfiles("host-contracts", "rollout-standard")).toEqual(["input-proof"]);
});

test("covers multi-chain isolation in heavy mode wherever contracts moved", () => {
  expect(rolloutPhaseTestProfiles("host-contracts", "rollout-heavy")).toContain("multi-chain-isolation");
  expect(rolloutPhaseTestProfiles("relayer", "rollout-heavy")).toContain("multi-chain-isolation");
  expect(rolloutPhaseTestProfiles("protocol-flip", "rollout-heavy")).toContain("multi-chain-isolation");
});

// /v3/user-decrypt is only reachable once the ACL reports 0.14, so the dedicated profile sits in
// the protocol-flip phase and nowhere earlier — before it the client resolves 0.13 and asks for
// /v2. It is not the only thing that reaches /v3: once the ACL has moved, the SDK routes every
// user decryption there, so the standard gate crosses onto /v3 too. This pins where the profile
// that exercises the route deliberately belongs.
test("checks the v3 user-decryption route in the protocol-flip phase in heavy mode", () => {
  expect(rolloutPhaseTestProfiles("protocol-flip", "rollout-heavy")).toContain("unified-user-decryption");
  for (const phase of phaseOrder.filter((candidate) => candidate !== "protocol-flip")) {
    expect(rolloutPhaseTestProfiles(phase, "rollout-heavy")).not.toContain("unified-user-decryption");
  }
});

test("rejects unsupported rollout test modes", () => {
  expect(() => resolveRolloutTestMode("standard")).toThrow("Unsupported ROLLOUT_TEST_PROFILE=standard");
});

// The scenario is loaded by name at boot, hours into a CI run. Resolving it here turns a typo or
// a schema slip into a failing unit test instead of a wasted rollout.
test("resolves the 3-of-5 scenario the runbook names, with the testnet Phase 3 arithmetic", async () => {
  const parsed = await loadCoprocessorScenario(scenario);
  // Resolve through the same path boot uses, so defaults and derived fields are the real ones.
  const resolved = resolveScenarioFile(await resolveScenarioReference(scenario), parsed);
  expect(resolved.topology.count).toBe(5);
  expect(resolved.topology.threshold).toBe(3);
  // Threshold strictly below count is what creates an outvoted minority — the state where a
  // divergent operator does not stall consensus and so cannot be seen by a passing gate.
  expect(resolved.topology.threshold).toBeLessThan(resolved.topology.count);
  expect(resolved.hostChains.map((chain) => chain.key)).toEqual(["host", "chain-b"]);
  expect(resolved.kms.mode).toBe("threshold");
  // 4 parties at t=1 is the smallest real threshold cluster (parties == 3t + 1), and it is what
  // gives the KMS staircase four distinct one-node-at-a-time steps.
  expect(resolved.kms.parties).toBe(4);
  expect(resolved.kms.threshold).toBe(1);
  expect(resolved.kms.committeeSize).toBe(4);
  // One instance per operator, so the staircase has five distinct steps to walk.
  expect(resolved.instances).toHaveLength(5);
});
