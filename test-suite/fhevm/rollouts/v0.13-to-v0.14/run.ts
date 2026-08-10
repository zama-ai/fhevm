import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { gatewayContractKeys, hostContractKeys, phaseVersions, scenario, versionSources } from "./versions";

export type RolloutEnv = Record<string, string>;

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

const upgradeContract = async (runTask: (command: string) => Promise<void>, task: string, name: string) => {
  const current = `previous-contracts/${name}.sol:${name}`;
  const next = `contracts/${name}.sol:${name}`;
  console.log(`[contracts] ${name}: ${current} -> ${next}`);
  await runTask(
    [
      "npx hardhat",
      task,
      `--current-implementation ${current}`,
      `--new-implementation ${next}`,
      "--verify-contract false",
      "--use-internal-proxy-address true",
    ].join(" "),
  );
};

const writePhaseVersionLock = (ctx: RolloutRunContext, name: string, versions: RolloutEnv) =>
  ctx.writeVersionLock(name, { versions, sources: versionSources });

const rolloutTestModes = ["rollout-standard", "rollout-heavy"] as const;
type RolloutTestMode = (typeof rolloutTestModes)[number];
type RolloutPhase =
  | "baseline"
  | "gateway-contracts"
  | "host-contracts"
  | "relayer"
  | "kms-prss-bridge"
  | "kms"
  | "coprocessor"
  | "protocol-flip";

// Heavy mode adds the checkpoints that are cheap to run at a phase boundary and expensive to
// debug later. multi-chain-isolation is in every contract phase because this rollout upgrades
// two host chains and per-chain state separation is exactly what the mirror can break.
const heavyPhaseProfiles: Record<RolloutPhase, string[]> = {
  baseline: ["rollout-standard", "multi-chain-isolation"],
  "gateway-contracts": ["rollout-standard"],
  // Nothing that decrypts: the relayer is still a release behind for the width of this phase.
  "host-contracts": ["input-proof", "multi-chain-isolation"],
  "relayer": ["rollout-standard", "multi-chain-isolation", "hcu-block-cap", "negative-acl", "operators"],
  "kms-prss-bridge": ["rollout-standard", "public-decryption"],
  // Runs once per KMS node, so it stays deliberately small: each step pins the reconstruction
  // quorum to the node that just moved, which is the property the step exists to prove.
  kms: ["rollout-standard", "public-decryption"],
  // Runs once per coprocessor operator, at each consensus state the fleet passes through.
  coprocessor: ["rollout-standard", "random-subset"],
  // The ACL moves last and alone, so this gate is the widest. It is also the first phase whose
  // user decryptions go to /v3: the SDK routes every one of them there once the ACL resolves to
  // protocol 0.14, so `rollout-standard` already crosses onto /v3 and the profiles below widen
  // what is exercised over it rather than being the only way to reach it.
  "protocol-flip": [
    "rollout-standard",
    "unified-user-decryption",
    "multi-chain-isolation",
    "operators",
    "random-subset",
    "negative-acl",
    "public-decryption",
    "hcu-block-cap",
    "coprocessor-db-state-revert",
    "ciphertext-drift-auto-recovery",
  ],
};

export const resolveRolloutTestMode = (value?: string): RolloutTestMode => {
  // Unset and empty mean the same thing. A workflow that forwards an omitted input hands this
  // an empty string, and failing the run on that would be a needless way to lose an hour.
  const selected = value?.trim() ? value.trim() : "rollout-standard";
  if (rolloutTestModes.includes(selected as RolloutTestMode)) {
    return selected as RolloutTestMode;
  }
  throw new Error(`Unsupported ROLLOUT_TEST_PROFILE=${selected}; expected ${rolloutTestModes.join(" or ")}`);
};

// Phases whose standard gate is not the usual `rollout-standard` set. Only the host-contracts
// window needs one: it runs with the relayer a release behind, so every decrypting spec in that
// set would fail there, and what the window costs is asserted explicitly in the runbook instead.
const standardPhaseProfiles: Partial<Record<RolloutPhase, readonly string[]>> = {
  "host-contracts": ["input-proof"],
};

export const rolloutPhaseTestProfiles = (phase: RolloutPhase, mode: RolloutTestMode): readonly string[] =>
  mode === "rollout-heavy" ? heavyPhaseProfiles[phase] : (standardPhaseProfiles[phase] ?? ["rollout-standard"]);

const testPhase = async (ctx: RolloutRunContext, phase: RolloutPhase, mode: RolloutTestMode) => {
  const profiles = rolloutPhaseTestProfiles(phase, mode);
  console.log(`[rollout] ${phase} tests (${mode}): ${profiles.join(", ")}`);
  for (const profile of profiles) {
    await ctx.test(profile, { parallel: false });
  }
};

/**
 * Swaps the deploy container's contract sources so `previous-contracts/` holds the version
 * currently live on chain and `contracts/` holds the target. Each surface is prepared in its
 * own phase so the gateway chain can complete before the host chain starts.
 */
const prepareContractMigrationSources = async (
  ctx: RolloutRunContext,
  surface: "host" | "gateway",
  targetLockFile: string,
  allowedVersionKeys: readonly string[],
) => {
  console.log(`[contracts] preserve live ${surface} sources as previous-contracts, activate v0.14 sources`);
  await ctx.snapshotContracts(surface);
  await ctx.applyVersionLock(`v0.14 ${surface} contract migration sources`, {
    lockFile: targetLockFile,
    allowedVersionKeys: [...allowedVersionKeys],
  });
};

/**
 * The gateway contracts that move in this release.
 *
 * Only the contracts v0.14 actually changed can be upgraded. Every upgradeable contract here
 * guards its reinitializer with `reinitializer(REINITIALIZER_VERSION)`, and a fresh deploy runs
 * `initializeFromEmptyProxy` under that same constant — so a proxy deployed from v0.13.2 already
 * sits at the v0.13.2 value. Upgrading a contract whose source (and therefore constant) did not
 * change reverts with OpenZeppelin's `InvalidInitialization()` (`0xf92ee8a9`).
 *
 * Between v0.13.2 and v0.14.0-10 the only gateway contract that changed is Decryption (6 -> 7).
 * GatewayConfig (9), KMSGeneration (6), InputVerification (6) and CiphertextCommits (6) are
 * byte-identical across the two tags, so they must not be touched.
 */
const GATEWAY_CONTRACT_UPGRADES = [{ task: "task:upgradeDecryption", name: "Decryption" }] as const;

/**
 * The host contracts that move on each host chain, in dependency order.
 *
 * All of these bumped their reinitializer in v0.14 (see the note above): KMSVerifier 4 -> 5,
 * KMSGeneration 2 -> 3, HCULimit 4 -> 5, FHEVMExecutor 5 -> 6, ACL 5 -> 6. ProtocolConfig
 * (2 -> 3) moves separately through its own migration below.
 *
 * KMSGeneration lives only on the canonical host chain — key and CRS generation is anchored
 * there and mirrored outward, and the boot flow asserts that every other chain's address file
 * has no KMS_GENERATION_CONTRACT_ADDRESS. Upgrading it elsewhere fails resolving that address.
 */
const HOST_CONTRACT_UPGRADES = [
  // KMSGeneration before KMSVerifier, which is the order the v0.14 devnet upgrade ran on
  // 14 July: ProtocolConfig -> KMSGeneration -> KMSVerifier -> HCULimit -> FHEVMExecutor.
  { task: "task:upgradeKMSGeneration", name: "KMSGeneration", canonicalOnly: true },
  { task: "task:upgradeKMSVerifier", name: "KMSVerifier", canonicalOnly: false },
  // HCULimit before FHEVMExecutor: new executor ops call the new HCU checks.
  { task: "task:upgradeHCULimit", name: "HCULimit", canonicalOnly: false },
  { task: "task:upgradeFHEVMExecutor", name: "FHEVMExecutor", canonicalOnly: false },
] as const;

/**
 * The ACL is deliberately not in the list above: it moves alone, in the last phase.
 *
 * This is a property of the client this harness runs, not of the release, and the difference
 * matters when reading the phase order as a rehearsal. The v0.14 devnet upgrade put the ACL in
 * its host-contracts step on 14 July, together with everything above, and left the relayer on
 * 0.13 until 23 July and the KMS connectors until 27 July — an ordering this runbook could not
 * survive but devnet did.
 *
 * The reason it worked there is that the published clients do not read the ACL. `@fhevm/sdk`
 * 0.13.2 dispatches user decryption on a caller-supplied `parameters.version`, and
 * `@zama-fhe/relayer-sdk` 0.4.4 has no /v3 route at all, so neither changes behaviour when the
 * ACL reports 0.14. Only the 1.1.0-alpha line and current main resolve the protocol version
 * from the on-chain ACL and hard-switch to /v3 — and the harness builds its client from this
 * tree, so it behaves like those.
 *
 * Since /v3 is served only once the relayer and every KMS connector are on 0.14, and every gate
 * runs `user-decryption`, upgrading the ACL in the host phase would make three components
 * load-bearing at once and collapse the per-component phases into one step. Holding it back
 * keeps this harness's client on /v2 while each backend component crosses alone. A deployment
 * whose clients are all published releases does not need that constraint.
 */
const ACL_UPGRADE = { task: "task:upgradeACL", name: "ACL", canonicalOnly: false } as const;

/** The host contract upgrades that apply to a given chain. */
export const hostContractUpgradesForChain = (isCanonical: boolean) =>
  HOST_CONTRACT_UPGRADES.filter((upgrade) => isCanonical || !upgrade.canonicalOnly);

type HostChainTarget = {
  key: string;
  isCanonical: boolean;
  runTask: (command: string) => Promise<void>;
};

/** Resolves one runner per host chain, with the first chain marked canonical. */
const hostChainTargets = async (ctx: RolloutRunContext): Promise<HostChainTarget[]> => {
  const state = await ctx.readState();
  const chains = state.scenario.hostChains;
  if (!chains.length) {
    throw new Error("v0.13-to-v0.14 expects at least one host chain in the active scenario");
  }
  const canonicalKey = chains[0].key;
  return chains.map((chain) => ({
    key: chain.key,
    isCanonical: chain.key === canonicalKey,
    runTask:
      chain.key === canonicalKey
        ? (command: string) => ctx.runHostContractTask(command)
        : (command: string) => ctx.runHostContractTaskOnChain(chain.key, command),
  }));
};

/** Orders host chains canonical-first: it holds the context every other chain must match. */
export const canonicalFirst = <T extends { isCanonical: boolean }>(targets: readonly T[]): T[] =>
  [...targets].sort((a, b) => Number(b.isCanonical) - Number(a.isCanonical));

/**
 * Moves every host chain's ProtocolConfig from v0.13 to v0.14, canonical chain first.
 *
 * v0.14 makes ProtocolConfig the single source of KMS context truth and expects every
 * non-canonical host chain's context to match the canonical chain's. Every chain upgrades its
 * own proxy in place: `task:upgradeProtocolConfig` calls `reinitializeV2`, which rebuilds the
 * epoch-lifecycle shape from that proxy's own stored context, so a chain already mirroring
 * canonical keeps mirroring it.
 *
 * The canonical-seeding path (`task:deployProtocolConfigFromCanonical`) is deliberately not
 * used. It calls `initializeFromCanonical`, guarded by `onlyFromEmptyProxy`, which requires an
 * initialized version of exactly 1 — it bootstraps a brand-new secondary chain onto a live
 * canonical one. Every chain in an N-1 -> N rollout already carries an initialized
 * ProtocolConfig, so that path reverts with `NotInitializingFromEmptyProxy()` (selector
 * 0x6f4f731f). Seeding from canonical is a boot concern; a rollout upgrades in place.
 */
const migrateProtocolConfig = async (ctx: RolloutRunContext, targets: HostChainTarget[]) => {
  if (!targets.some((target) => target.isCanonical)) {
    throw new Error("v0.13-to-v0.14 could not resolve a canonical host chain");
  }

  for (const target of canonicalFirst(targets)) {
    console.log(`[contracts] ProtocolConfig on host chain ${target.key}`);
    await upgradeContract(target.runTask, "task:upgradeProtocolConfig", "ProtocolConfig");
  }
  await ctx.refreshDiscovery();

  // Fails loudly if any chain's ProtocolConfig is missing its KMS context or thresholds,
  // which is the failure the "ETH & Polygon anchors match" devnet check was guarding.
  for (const target of targets) {
    await target.runTask("npx hardhat task:assertProtocolConfigReady");
  }
};

/** Serving KMS node ids, in the order they cross the boundary. */
const kmsNodeIds = async (ctx: RolloutRunContext) => {
  const state = await ctx.readState();
  if (state.scenario.kms.mode !== "threshold") {
    throw new Error("v0.13-to-v0.14 expects a threshold KMS cluster so nodes can be upgraded one at a time");
  }
  return Array.from({ length: state.scenario.kms.committeeSize }, (_, index) => index + 1);
};

/** Coprocessor operator indexes, in the order they cross the boundary. */
const coprocessorInstanceIndexes = async (ctx: RolloutRunContext) => {
  const state = await ctx.readState();
  if (state.scenario.kind !== "coprocessor-consensus") {
    throw new Error("v0.13-to-v0.14 expects a coprocessor-consensus scenario so operators can be upgraded one by one");
  }
  return state.scenario.instances.map((instance) => instance.index);
};

export default async function run(ctx: RolloutRunContext) {
  const testMode = resolveRolloutTestMode(process.env.ROLLOUT_TEST_PROFILE);
  const baselineLock = await writePhaseVersionLock(ctx, "00-baseline", phaseVersions.baseline);
  const gatewayContractsLock = await writePhaseVersionLock(ctx, "01-gateway-contracts", phaseVersions.gatewayContracts);
  const hostContractsLock = await writePhaseVersionLock(ctx, "02-host-contracts", phaseVersions.hostContracts);
  const relayerLock = await writePhaseVersionLock(ctx, "03-relayer", phaseVersions.relayer);
  const kmsPrssBridgeLock = await writePhaseVersionLock(ctx, "04-kms-prss-bridge", phaseVersions.kmsPrssBridge);
  const kmsLock = await writePhaseVersionLock(ctx, "05-kms", phaseVersions.kms);
  const listenerCoreLock = await writePhaseVersionLock(ctx, "06-listener-core", phaseVersions.listenerCore);
  const coprocessorLock = await writePhaseVersionLock(ctx, "07-coprocessor", phaseVersions.coprocessor);
  // Written for the receipt only: this phase moves on-chain state, not an image tag, so the
  // lock is the coprocessor phase's carried forward and is never applied.
  await writePhaseVersionLock(ctx, "08-protocol-flip", phaseVersions.protocolFlip);

  // The harness is overridden to a local build so the @fhevm/sdk it runs is this branch's
  // source rather than the copy baked into the published v0.14.0-9 image. The client is the
  // one component whose behaviour this runbook depends on for its ordering, so it is built
  // from the tree under test.
  logPhase("00 baseline: boot v0.13.2 on kms-core v0.13.20, gates on @fhevm/sdk over /v2");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await testPhase(ctx, "baseline", testMode);

  // The component order follows the default recorded in the QA-owned "Compatibility for
  // breaking changes between components" note — Gateway Contracts -> Host Contracts -> Relayer
  // -> KMS -> Coprocessors -> SDK & Host Library — up to one deliberate deviation: the ACL is
  // pulled out of the host step and moved to the end, for the reason given at ACL_UPGRADE. The
  // v0.14 devnet upgrade did not deviate; it upgraded the ACL with the other host contracts.
  logPhase("01 gateway contracts: upgrade the gateway chain first");
  await prepareContractMigrationSources(ctx, "gateway", gatewayContractsLock, gatewayContractKeys);
  for (const upgrade of GATEWAY_CONTRACT_UPGRADES) {
    await upgradeContract((command) => ctx.runGatewayContractTask(command), upgrade.task, upgrade.name);
  }
  await ctx.refreshDiscovery();
  await testPhase(ctx, "gateway-contracts", testMode);

  // Host contracts cross first, then the relayer, and the stack is degraded in between. What
  // the window costs was established one failure at a time:
  //
  //  - Relayer first is not an option at all. The 0.14 relayer initializes /v2/keyurl by
  //    calling getCurrentKmsContextAndEpoch() on ProtocolConfig, which only exists from 0.14,
  //    so against 0.13 contracts the call reverts with empty data and the relayer exits.
  //  - Host contracts first costs user decryption, with the relayer rejecting it as
  //    `validation_failed ... extraData`. @fhevm/sdk derives the extraData format from host
  //    contract state, not from the protocol version: kmsSignersContextToExtraData emits v0
  //    when the KMS context id is 0, v1 when the epoch id is 0, and v2 otherwise. Upgrading
  //    KMSVerifier and ProtocolConfig gives the client a real context *and* epoch, so it
  //    starts sending v2 extraData, which the 0.13 relayer does not accept.
  //
  // Holding the ACL back does not help here — this is the client's request shape following
  // host contract state, which changes independently of the ACL's protocol version.
  //
  // These are two phases, not one, because an operator cannot land them together. A contract
  // upgrade is a transaction and a relayer upgrade is a deploy; they are separate actions, run
  // by different people, minutes to days apart. The real 0.14 devnet upgrade held this exact
  // intermediate state for nine days — host contracts on 14 July, relayer on 23 July — so a
  // runbook that crosses it atomically rehearses something nobody can perform.
  //
  // The order within the pair is forced. Relayer-first is not available: the 0.14 relayer gates
  // its own startup on the first successful `/v2/keyurl` poll, that poll calls
  // `getCurrentKmsContextAndEpoch()`, and against 0.13 contracts it exits rather than degrading
  // (relayer/src/startup.rs:152, relayer/src/host/keyurl_poller.rs:307).
  //
  // So the window below is entered deliberately and measured rather than skipped. What it costs
  // depends on on-chain state: the extraData version follows the KMS context and epoch ids the
  // client reads, so a stack whose epoch id is still 0 sends v1, which a 0.13 relayer accepts,
  // while this harness migrates ProtocolConfig from scratch, lands a non-zero epoch, and sends
  // v2, which it does not. Devnet crossed its own nine days gated on relayer-sdk 0.4.4, and
  // @fhevm/sdk 0.13.2 — which has this same extraData selection — arrived on 23 July, at the
  // end of that window rather than across it, and was reported compatible with the state it
  // found. Whether that means devnet's epoch was 0 has not been confirmed. Which is why the
  // check below asserts the failure rather than assuming it: if user decryption ever survives
  // this window, expectTestFailure raises and the assumption gets revisited.
  logPhase("02 host contracts: upgrade every host chain, relayer still on 0.13");
  await prepareContractMigrationSources(ctx, "host", hostContractsLock, hostContractKeys);
  const targets = await hostChainTargets(ctx);
  await migrateProtocolConfig(ctx, targets);
  for (const target of canonicalFirst(targets)) {
    console.log(`[contracts] host chain ${target.key}`);
    for (const upgrade of hostContractUpgradesForChain(target.isCanonical)) {
      await upgradeContract(target.runTask, upgrade.task, upgrade.name);
    }
  }
  await ctx.refreshDiscovery();
  // What still works with the relayer a release behind...
  await testPhase(ctx, "host-contracts", testMode);
  // ...and what the window costs: the client now reads a real context and epoch, so it sends v2
  // extraData, which the 0.13 relayer rejects.
  await ctx.expectTestFailure("user-decryption", { errorIncludes: "extraData", parallel: false });

  logPhase("03 relayer: close the window opened above");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  await testPhase(ctx, "relayer", testMode);

  // Two KMS phases, not one. 0.13.22 is the required stop between 0.13.20 and 0.14, for the
  // reason recorded against corePrssBridge in versions.ts. The connector stays on 0.13 here —
  // this lock carries CORE_VERSION alone — so the phase shows the bridge tag boots and serves a
  // cluster whose connectors have not moved, before any 0.14 KMS code is introduced.
  //
  // It shows no more than that. The hotfix is switched by a request-id config this repo cannot
  // express, so every node here runs 0.13.22 at its default setting, and the gate below runs
  // only once the whole cluster is across. The mixed 0.13.20/0.13.22 window — the state that
  // actually failed on devnet — is passed through unobserved. Nothing here proves a mixed
  // cluster reconstructs; the node-by-node runbook next door covers 0.13.21 -> 0.13.22, which
  // is a different transition from this one.
  logPhase("04 kms PRSS bridge: move kms-core 0.13.20 -> 0.13.22 node by node, connector untouched");
  for (const nodeId of await kmsNodeIds(ctx)) {
    await ctx.upgradeKmsNodes([nodeId], { lockFile: kmsPrssBridgeLock });
  }
  await testPhase(ctx, "kms-prss-bridge", testMode);

  // Node by node, each node's connector crossing with its own core. After each node the stack
  // is tested twice: once with the reconstruction quorum pinned to include the node that just
  // moved (proving a mixed cluster reconstructs through it), then normally.
  logPhase("05 kms: move each node's core and connector to v0.14, one node at a time");
  for (const nodeId of await kmsNodeIds(ctx)) {
    await ctx.upgradeKmsNodes([nodeId], { lockFile: kmsLock });
    await ctx.withRequiredKmsNode(nodeId, () => ctx.test("rollout-standard", { parallel: false }));
    await testPhase(ctx, "kms", testMode);
  }

  logPhase("06 listener-core: upgrade listener-core before the coprocessor consumes it");
  // No test gate here: 0.13 coprocessor listeners do not consume listener-core. The
  // compatibility boundary is the coprocessor upgrade, where consumers switch over.
  await ctx.upgradeRuntimeGroup("listener-core", { lockFile: listenerCoreLock });

  // One operator at a time. With 3 coprocessors at threshold 2 the fleet is gated at each
  // consensus state it passes through: one upgraded (below threshold), two (threshold reached),
  // three (all upgraded, and where the bundle lock itself lands).
  logPhase("07 coprocessor: upgrade operators one by one");
  for (const index of await coprocessorInstanceIndexes(ctx)) {
    await ctx.upgradeCoprocessorInstances([index], { lockFile: coprocessorLock });
    await testPhase(ctx, "coprocessor", testMode);
  }

  // Last, and alone, for the harness reason given at ACL_UPGRADE rather than a production one:
  // this tree's client resolves the protocol version from the on-chain ACL, so moving the ACL is
  // what makes it ask for /v3, and everything that has to answer /v3 is already on 0.14 by here.
  // A deployment running published clients moves the ACL with the other host contracts, as the
  // v0.14 devnet upgrade did.
  // No prepareContractMigrationSources here on purpose. The host phase already left the deploy
  // container with previous-contracts/ on 0.13 and contracts/ on 0.14, and no version has moved
  // since. Re-snapshotting now would capture the 0.14 sources as "previous" and upgrade the ACL
  // from 0.14 to 0.14, while the proxy on chain is still 0.13.
  logPhase("08 protocol flip: upgrade the host ACL, so the client follows the chain onto /v3");
  for (const target of canonicalFirst(await hostChainTargets(ctx))) {
    console.log(`[contracts] host chain ${target.key}`);
    await upgradeContract(target.runTask, ACL_UPGRADE.task, ACL_UPGRADE.name);
  }
  await ctx.refreshDiscovery();
  await testPhase(ctx, "protocol-flip", testMode);
}

export const phaseOrder = [
  "baseline",
  "gateway-contracts",
  "host-contracts",
  "relayer",
  "kms-prss-bridge",
  "kms",
  "coprocessor",
  "protocol-flip",
] as const satisfies readonly RolloutPhase[];

export const hostContractUpgradeOrder: readonly string[] = HOST_CONTRACT_UPGRADES.map((upgrade) => upgrade.name);
export const gatewayContractUpgradeOrder: readonly string[] = GATEWAY_CONTRACT_UPGRADES.map(
  (upgrade) => upgrade.name,
);
