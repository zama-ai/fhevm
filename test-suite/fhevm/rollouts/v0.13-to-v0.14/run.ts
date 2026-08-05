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
  | "final";

// Heavy mode adds the checkpoints that are cheap to run at a phase boundary and expensive to
// debug later. multi-chain-isolation is in every contract phase because this rollout upgrades
// two host chains and per-chain state separation is exactly what the mirror can break.
const heavyPhaseProfiles: Record<RolloutPhase, string[]> = {
  baseline: ["rollout-standard", "multi-chain-isolation"],
  "gateway-contracts": ["rollout-standard"],
  "host-contracts": ["rollout-standard", "multi-chain-isolation", "operators", "hcu-block-cap"],
  relayer: ["rollout-standard", "negative-acl", "unified-user-decryption"],
  "kms-prss-bridge": ["rollout-standard", "public-decryption"],
  kms: ["rollout-standard", "public-decryption", "random-subset"],
  final: [
    "rollout-standard",
    "multi-chain-isolation",
    "operators",
    "random-subset",
    "negative-acl",
    "public-decryption",
    "hcu-block-cap",
    "coprocessor-db-state-revert",
    "rollout-standard",
    "ciphertext-drift-auto-recovery",
    "rollout-standard",
  ],
};

export const resolveRolloutTestMode = (value?: string): RolloutTestMode => {
  const selected = value ?? "rollout-standard";
  if (rolloutTestModes.includes(selected as RolloutTestMode)) {
    return selected as RolloutTestMode;
  }
  throw new Error(`Unsupported ROLLOUT_TEST_PROFILE=${selected}; expected ${rolloutTestModes.join(" or ")}`);
};

export const rolloutPhaseTestProfiles = (phase: RolloutPhase, mode: RolloutTestMode) =>
  mode === "rollout-heavy" ? heavyPhaseProfiles[phase] : ["rollout-standard"];

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
 * The host contracts that move on every host chain, in dependency order.
 *
 * All of these bumped their reinitializer in v0.14 (see the note above): KMSVerifier 4 -> 5,
 * KMSGeneration 2 -> 3, HCULimit 4 -> 5, FHEVMExecutor 5 -> 6, ACL 5 -> 6. ProtocolConfig
 * (2 -> 3) moves separately through its own migration below.
 */
const HOST_CONTRACT_UPGRADES = [
  // KMSVerifier first so it holds the new KMS context before any executor path runs.
  { task: "task:upgradeKMSVerifier", name: "KMSVerifier" },
  { task: "task:upgradeKMSGeneration", name: "KMSGeneration" },
  // HCULimit before FHEVMExecutor: new executor ops call the new HCU checks.
  { task: "task:upgradeHCULimit", name: "HCULimit" },
  { task: "task:upgradeFHEVMExecutor", name: "FHEVMExecutor" },
  { task: "task:upgradeACL", name: "ACL" },
] as const;

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

/**
 * v0.14 makes ProtocolConfig the single source of KMS context truth and requires every
 * non-canonical host chain to mirror the canonical chain's context, so the anchors match
 * across chains. The canonical chain upgrades its proxy in place; the others re-seed their
 * proxy from a live read of the canonical one.
 *
 * The non-canonical deploy container's templated env exposes only its own chain's RPC_URL,
 * so the canonical chain's node has to be named explicitly. `host-node` is the default
 * chain's container on the shared compose network.
 */
const CANONICAL_NODE_RPC_URL = "http://host-node:8545";

const migrateProtocolConfig = async (ctx: RolloutRunContext, targets: HostChainTarget[]) => {
  const canonical = targets.find((target) => target.isCanonical);
  if (!canonical) {
    throw new Error("v0.13-to-v0.14 could not resolve a canonical host chain");
  }

  await upgradeContract(canonical.runTask, "task:upgradeProtocolConfig", "ProtocolConfig");
  await ctx.refreshDiscovery();

  const state = await ctx.readState();
  const canonicalAddress = state.discovery?.hosts?.[canonical.key]?.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  if (!canonicalAddress) {
    throw new Error(
      `host chain "${canonical.key}" has no discovered PROTOCOL_CONFIG_CONTRACT_ADDRESS; the canonical ProtocolConfig must be live before mirroring`,
    );
  }

  for (const target of targets.filter((item) => !item.isCanonical)) {
    console.log(`[contracts] mirror ProtocolConfig onto host chain ${target.key} from ${canonical.key}`);
    await target.runTask(
      [
        "npx hardhat task:deployProtocolConfigFromCanonical",
        `--canonical-rpc-url ${CANONICAL_NODE_RPC_URL}`,
        `--canonical-protocol-config-address ${canonicalAddress}`,
      ].join(" "),
    );
  }

  // Fails loudly if any chain's ProtocolConfig is missing its KMS context or thresholds,
  // which is the failure the "ETH & Polygon anchors match" devnet check was guarding.
  for (const target of targets) {
    await target.runTask("npx hardhat task:assertProtocolConfigReady");
  }
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

  logPhase("00 baseline: boot v0.13.2 on kms-core v0.13.20 with the target test-suite harness");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await testPhase(ctx, "baseline", testMode);

  // The component order below is the documented default:
  // Gateway Contracts -> Host Contracts -> Relayer -> KMS -> Coprocessors -> SDK.
  logPhase("01 gateway contracts: upgrade the gateway chain first");
  await prepareContractMigrationSources(ctx, "gateway", gatewayContractsLock, gatewayContractKeys);
  for (const upgrade of GATEWAY_CONTRACT_UPGRADES) {
    await upgradeContract((command) => ctx.runGatewayContractTask(command), upgrade.task, upgrade.name);
  }
  await ctx.refreshDiscovery();
  await testPhase(ctx, "gateway-contracts", testMode);

  logPhase("02 host contracts: ProtocolConfig anchor, mirror onto every other chain, then the rest");
  await prepareContractMigrationSources(ctx, "host", hostContractsLock, hostContractKeys);
  const targets = await hostChainTargets(ctx);
  await migrateProtocolConfig(ctx, targets);
  for (const target of targets) {
    console.log(`[contracts] host chain ${target.key}`);
    for (const upgrade of HOST_CONTRACT_UPGRADES) {
      await upgradeContract(target.runTask, upgrade.task, upgrade.name);
    }
  }
  await ctx.refreshDiscovery();
  await testPhase(ctx, "host-contracts", testMode);

  logPhase("03 relayer: upgrade relayer after contracts, before the KMS connector moves");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  await testPhase(ctx, "relayer", testMode);

  // Two KMS phases, not one. 0.13.22 is the only kms-core that serves peers on both sides of
  // the PRSS hotfix, so it is a required stop between 0.13.20 and 0.14. The connector stays on
  // 0.13 here: this phase proves the bridge version is a no-op for a pre-hotfix cluster before
  // any 0.14 KMS code is introduced.
  //
  // This is the whole-cluster stop in the release sequence. The node-by-node mixed-version
  // proof for the same hotfix lives in rollouts/v0.13.21-to-v0.13.22-kms-node-by-node, which
  // runs a threshold cluster and forces each upgraded node into the reconstruction quorum.
  logPhase("04 kms PRSS bridge: move kms-core 0.13.20 -> 0.13.22, connector untouched");
  await ctx.upgradeRuntimeGroup("kms-core", { lockFile: kmsPrssBridgeLock });
  await testPhase(ctx, "kms-prss-bridge", testMode);

  logPhase("05 kms: move kms-core 0.13.22 -> 0.14.0-1 and the connector to v0.14 together");
  await ctx.upgradeRuntimeGroup("kms", { lockFile: kmsLock });
  await testPhase(ctx, "kms", testMode);

  logPhase("06 listener-core: upgrade listener-core before the coprocessor consumes it");
  // No test gate here: 0.13 coprocessor listeners do not consume listener-core. The
  // compatibility boundary is the coprocessor upgrade, where consumers switch over.
  await ctx.upgradeRuntimeGroup("listener-core", { lockFile: listenerCoreLock });

  logPhase("07 coprocessor: upgrade coprocessor last");
  await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });
  await testPhase(ctx, "final", testMode);
}

export const phaseOrder = [
  "baseline",
  "gateway-contracts",
  "host-contracts",
  "relayer",
  "kms-prss-bridge",
  "kms",
  "final",
] as const satisfies readonly RolloutPhase[];

export const hostContractUpgradeOrder: readonly string[] = HOST_CONTRACT_UPGRADES.map((upgrade) => upgrade.name);
export const gatewayContractUpgradeOrder: readonly string[] = GATEWAY_CONTRACT_UPGRADES.map(
  (upgrade) => upgrade.name,
);
