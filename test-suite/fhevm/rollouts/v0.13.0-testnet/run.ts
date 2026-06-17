import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { phaseVersions, scenario, versionSources } from "./versions";

export type RolloutEnv = Record<string, string>;

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

// Apply a contract upgrade with the owner key (the DAO-execution stand-in: the
// rollout can't run governance, so the proxy owner upgrades directly).
const upgradeContract = async (runTask: (command: string) => Promise<void>, task: string, name: string) => {
  const current = `previous-contracts/${name}.sol:${name}`;
  const next = `contracts/${name}.sol:${name}`;
  console.log(`[apply] ${name}: ${current} -> ${next}`);
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

// Run the runbook's prepare step: deploy the new implementation and emit the DAO
// upgrade calldata WITHOUT mutating the proxy. These prepareUpgrade*/prepareDeploy*
// tasks have never run in a container (per Amina/Yohan); exercising them here is
// the highest-value part of this rollout. Output lands in the receipt/logs as the
// command audit trail Yohan asked for.
const prepareUpgrade = async (runTask: (command: string) => Promise<void>, task: string, name: string) => {
  const current = `previous-contracts/${name}.sol:${name}`;
  const next = `contracts/${name}.sol:${name}`;
  console.log(`[prepare] ${name}: ${task}`);
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

const contractVersionKeys = ["GATEWAY_VERSION", "HOST_VERSION"];

const prepareContractMigrationSources = async (ctx: RolloutRunContext, targetLockFile: string) => {
  console.log("[contracts] preserve v0.12 sources as previous-contracts, activate v0.13 sources");
  await ctx.snapshotContracts("host");
  await ctx.snapshotContracts("gateway");
  await ctx.applyVersionLock("v0.13 contract migration sources", {
    lockFile: targetLockFile,
    allowedVersionKeys: contractVersionKeys,
  });
};

const parseGatewayMigrationEnv = (value: unknown): RolloutEnv => {
  if (!value || typeof value !== "object") {
    throw new Error("migration-state.json is missing its export object");
  }
  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>).map(([key, item]) => {
      if (item === undefined || item === null) {
        throw new Error(`migration-state.json export.${key} is empty`);
      }
      return [key, String(item)];
    }),
  );
};

// One-node fixture: legacy GatewayConfig exports cryptographic threshold mpc=0,
// which v0.13 ProtocolConfig rejects. Normalize only that synthetic case.
export const needsLocalOneNodeMpcNormalization = (env: RolloutEnv): boolean => {
  const kmsNodes = JSON.parse(env.MIGRATION_KMS_NODES ?? "[]") as unknown[];
  const thresholds = JSON.parse(env.MIGRATION_KMS_THRESHOLDS ?? "{}") as Record<string, unknown>;
  return kmsNodes.length === 1 && String(thresholds.mpc) === "0";
};

export const normalizeLocalOneNodeMpcThreshold = (env: RolloutEnv): RolloutEnv => {
  if (!needsLocalOneNodeMpcNormalization(env)) {
    return env;
  }
  const thresholds = JSON.parse(env.MIGRATION_KMS_THRESHOLDS ?? "{}") as Record<string, unknown>;
  console.log("[contracts] local one-node migration: normalize ProtocolConfig mpc threshold 0 -> 1");
  return { ...env, MIGRATION_KMS_THRESHOLDS: JSON.stringify({ ...thresholds, mpc: "1" }) };
};

type GatewayMigrationContext = {
  kmsGenerationProxy: string;
  gatewayConfigProxy: string;
  migrationEnv: RolloutEnv;
  mpcThresholdNormalized: boolean;
};

// Phase 1 of the runbook: export the gateway KMS migration state. Writes the
// MIGRATION_* values consumed by the host ProtocolConfig/KMSGeneration tasks.
const exportGatewayKmsMigrationEnv = async (ctx: RolloutRunContext): Promise<GatewayMigrationContext> => {
  const state = await ctx.readState();
  const gateway = state.discovery?.gateway;
  if (!gateway?.KMS_GENERATION_ADDRESS || !gateway.GATEWAY_CONFIG_ADDRESS) {
    throw new Error("gateway KMSGeneration and GatewayConfig addresses must be discovered before contract migration");
  }
  const output = "kms-migration-state.json";
  await ctx.runGatewayContractTask(
    [
      "npx hardhat compile &&",
      "npx hardhat task:exportKmsMigrationState",
      `--kms-generation-proxy ${gateway.KMS_GENERATION_ADDRESS}`,
      `--gateway-config-proxy ${gateway.GATEWAY_CONFIG_ADDRESS}`,
      `--output /app/addresses/${output}`,
    ].join(" "),
  );
  const stateFile = path.join(ctx.stateDir(), "runtime", "addresses", "gateway", output);
  const migrationState = JSON.parse(await Bun.file(stateFile).text()) as { export?: unknown };
  const rawEnv = parseGatewayMigrationEnv(migrationState.export);
  return {
    kmsGenerationProxy: gateway.KMS_GENERATION_ADDRESS,
    gatewayConfigProxy: gateway.GATEWAY_CONFIG_ADDRESS,
    migrationEnv: normalizeLocalOneNodeMpcThreshold(rawEnv),
    mpcThresholdNormalized: needsLocalOneNodeMpcNormalization(rawEnv),
  };
};

const GATEWAY_RPC_URL = "http://gateway-node:8546";

const assertKmsMigration = async (
  ctx: RolloutRunContext,
  { gatewayConfigProxy, kmsGenerationProxy, mpcThresholdNormalized }: GatewayMigrationContext,
) => {
  const assertCommand = [
    "npx hardhat compile &&",
    "npx hardhat task:assertKmsMigrationSucceeded",
    `--gateway-config-proxy ${gatewayConfigProxy}`,
    `--gateway-kms-generation-proxy ${kmsGenerationProxy}`,
    "--use-internal-proxy-address true",
  ].join(" ");
  if (!mpcThresholdNormalized) {
    await ctx.runHostContractTask(assertCommand, { env: { GATEWAY_RPC_URL } });
    return;
  }
  const wrapped = [
    `if out=$(${assertCommand} 2>&1); then printf '%s\\n' "$out";`,
    `else status=$?; printf '%s\\n' "$out";`,
    `printf '%s\\n' "$out" | grep -q 'ProtocolConfig MPC threshold mismatch: expected 0, got 1' || exit $status;`,
    `echo '[runbook] tolerated synthetic-fixture mpc threshold mismatch';`,
    "fi",
  ].join(" ");
  await ctx.runHostContractTask(wrapped, { env: { GATEWAY_RPC_URL } });
};

const gw = (ctx: RolloutRunContext) => (command: string) => ctx.runGatewayContractTask(command);
const host = (ctx: RolloutRunContext) => (command: string) => ctx.runHostContractTask(command);

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await writePhaseVersionLock(ctx, "00-baseline", phaseVersions.baseline);
  const contractsLock = await writePhaseVersionLock(ctx, "01-contracts", phaseVersions.contracts);
  const kmsLock = await writePhaseVersionLock(ctx, "02-kms", phaseVersions.kms);
  const coprocessorLock = await writePhaseVersionLock(ctx, "03-coprocessor", phaseVersions.coprocessor);
  const relayerLock = await writePhaseVersionLock(ctx, "04-relayer", phaseVersions.relayer);

  logPhase("00 baseline: boot exact testnet v0.12 state (KMS core already v0.13.10)");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await ctx.test("rollout-standard", { parallel: false });

  // ---- Contract migration, following gitops#1063 + the 0.13.0 runbook ----
  logPhase("01 contracts: testnet v0.12 -> v0.13.0 migration (prepare, then apply the proposal effect)");
  await prepareContractMigrationSources(ctx, contractsLock);

  // RUNBOOK GAP (finding #1, confirmed by run 27684221770): the Notion runbook's
  // "Step 0 -- assert no pending key-management request on the HOST KMSGeneration,
  // before anything else" is not runnable at the v0.12 starting state. At v0.12
  // KMSGeneration lives on the GATEWAY; the host KMSGeneration proxy only exists
  // after deployEmptyProxies + deployKMSGenerationFromMigration, so
  // `task:assertNoPendingKeyManagementRequest` (a host task) cannot resolve an
  // address as step 0 and aborts. The intent (no pending keygen before migrating)
  // must target the gateway KMSGeneration, or move post-migration. Skipped here;
  // the post-migration state is validated by task:assertKmsMigrationSucceeded.

  // Phase 1: export gateway migration state (before gateway KMSGeneration goes view-only).
  const migrationCtx = await exportGatewayKmsMigrationEnv(ctx);
  const { migrationEnv } = migrationCtx;

  // Phase 2+3: PREPARE (the never-tested-in-container steps). Deploy impls + emit
  // DAO calldata without mutating proxies. Gateway then host.
  logPhase("01.prepare gateway: prepareUpgrade* (GatewayConfig, KMSGeneration, Decryption, CiphertextCommits, InputVerification)");
  await prepareUpgrade(gw(ctx), "task:prepareUpgradeGatewayConfig", "GatewayConfig");
  await prepareUpgrade(gw(ctx), "task:prepareUpgradeKMSGeneration", "KMSGeneration");
  await prepareUpgrade(gw(ctx), "task:prepareUpgradeDecryption", "Decryption");
  await prepareUpgrade(gw(ctx), "task:prepareUpgradeCiphertextCommits", "CiphertextCommits");
  await prepareUpgrade(gw(ctx), "task:prepareUpgradeInputVerification", "InputVerification");

  logPhase("01.prepare host: empty proxies, prepareDeploy*FromMigration, prepareUpgrade* (KMSVerifier, HCULimit, FHEVMExecutor, ACL)");
  await ctx.runHostContractTask("npx hardhat task:deployEmptyProxiesProtocolConfigKMSGeneration");
  // prepareDeploy* build initializeFromMigration calldata FROM ENV (gap #2:
  // a stale env yields a proposal with wrong signers/thresholds vs the export).
  await ctx.runHostContractTask("npx hardhat task:prepareDeployProtocolConfigFromMigration --verify-contract false", { env: migrationEnv });
  await ctx.runHostContractTask("npx hardhat task:prepareDeployKMSGenerationFromMigration --verify-contract false", { env: migrationEnv });
  await prepareUpgrade(host(ctx), "task:prepareUpgradeKMSVerifier", "KMSVerifier");
  await prepareUpgrade(host(ctx), "task:prepareUpgradeHCULimit", "HCULimit");
  await prepareUpgrade(host(ctx), "task:prepareUpgradeFHEVMExecutor", "FHEVMExecutor");
  await prepareUpgrade(host(ctx), "task:prepareUpgradeACL", "ACL");

  // Phase 5: APPLY the proposal effect (owner-key stand-in for DAO execution),
  // gateway-before-host (MASSIVE WARNING in the runbook: gateway proposal first).
  logPhase("01.apply gateway proposal effect: GatewayConfig + KMSGeneration (view-only)");
  await upgradeContract(gw(ctx), "task:upgradeGatewayConfig", "GatewayConfig");
  await upgradeContract(gw(ctx), "task:upgradeKMSGeneration", "KMSGeneration");

  logPhase("01.apply host proposal effect: ProtocolConfig + KMSGeneration migration, then KMSVerifier/HCULimit/FHEVMExecutor/ACL");
  await ctx.runHostContractTask("npx hardhat task:deployProtocolConfigFromMigration", { env: migrationEnv });
  await ctx.runHostContractTask("npx hardhat task:deployKMSGenerationFromMigration", { env: migrationEnv });

  // Polygon (runbook Phase 7): fresh-deploy the new host chain. Must happen
  // BEFORE refreshDiscovery, which (with HOST_VERSION>=0.13) requires a
  // PROTOCOL_CONFIG_CONTRACT_ADDRESS on EVERY host chain. Polygon gets
  // ProtocolConfig-from-migration but no standalone KMSGeneration
  // (--with-kms-generation false), matching the gitops Eth/Polygon asymmetry.
  // Ownership transfer (Phase 7.14/8) is intentionally skipped: the rollout owns
  // the deployer key (no DAO/multisig), and transferring would lock later steps.
  // Run the Polygon fresh-deploy as ONE command in a single chain-b container
  // (mirroring the gitops polygon-sc-deploy job): each runHostContractTaskOnChain
  // is an ephemeral container, so compile + the deploy tasks must share one
  // invocation and the deployed addresses must be re-exported between tasks.
  // `npx hardhat compile` first because deployEmptyUUPSProxies needs the PauserSet
  // artifact (the primary host migration tasks self-compile; this fresh-deploy path
  // does not).
  logPhase("01.polygon: fresh v0.13.0 host-contract deploy on the 2nd chain (chain-b)");
  // Order mirrors gitops Phase 7 and is load-bearing: the upgradeable contracts
  // (e.g. KMSGeneration) reference `protocolConfigAdd` from FHEVMHostAddresses.sol,
  // so a full compile fails until the empty proxies are deployed and that address
  // file is populated. Compile only immutable contracts first (PauserSet etc.),
  // deploy the empty proxies, then full-compile, then deploy the rest.
  const polygonDeploy = [
    "npx hardhat compile:specific --contract contracts/immutable",
    "npx hardhat task:deployEmptyUUPSProxies --with-kms-generation false",
    "export $(cat addresses/.env.host | xargs)",
    "npx hardhat task:deployPauserSet",
    "npx hardhat compile:specific --contract contracts",
    "npx hardhat task:deployACL",
    "npx hardhat task:deployFHEVMExecutor",
    "npx hardhat task:deployProtocolConfigFromMigration",
    "npx hardhat task:deployKMSVerifier",
    "npx hardhat task:deployInputVerifier",
    "npx hardhat task:deployHCULimit",
  ].join(" && ");
  await ctx.runHostContractTaskOnChain("chain-b", polygonDeploy, { env: migrationEnv });

  await ctx.refreshDiscovery();
  await upgradeContract(host(ctx), "task:upgradeKMSVerifier", "KMSVerifier");
  await assertKmsMigration(ctx, migrationCtx);
  await upgradeContract(host(ctx), "task:upgradeHCULimit", "HCULimit");
  await upgradeContract(host(ctx), "task:upgradeFHEVMExecutor", "FHEVMExecutor");
  await upgradeContract(host(ctx), "task:upgradeACL", "ACL");
  await ctx.refreshDiscovery();
  await ctx.test("rollout-standard", { parallel: false });

  // ---- Runtime components (gitops has no PR for these yet -- inferred v0.13.0) ----
  logPhase("02 kms: core v0.13.10 -> v0.13.20 and connector v0.12.0 -> v0.13.0");
  await ctx.upgradeRuntimeGroup("kms", { lockFile: kmsLock });
  await ctx.test("rollout-standard", { parallel: false });

  logPhase("03 coprocessor: v0.12.x -> v0.13.0 (host-listener-consumer activates)");
  await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });
  await ctx.test("rollout-standard", { parallel: false });

  logPhase("04 relayer: v0.11.1 -> v0.13.0");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  await ctx.test("rollout-standard", { parallel: false });

  // ---- Polygon: addHostChain proposal effect (Phase 9 of the runbook) ----
  // LIMITATION: the rollout boots the 2nd host chain via the multi-chain scenario,
  // so we exercise the addHostChain proposal-effect task against the gateway, but
  // the runbook's fresh Polygon host-contract deploy (Phase 7) + ownership transfer
  // need rollout multi-chain contract-deploy support that isn't wired yet -- that
  // gap is itself a finding. We run addHostChainsToGatewayConfig to test the
  // registration path; flagged in the gap report.
  // FINDING / modeling limit: the rollout's multi-chain scenario boots chain-b as
  // an already-registered host chain, so addHostChain reverts with
  // HostChainAlreadyRegistered (selector 0x96a56828). The runbook's "add a new
  // host chain" proposal-effect therefore can't be freshly exercised here (the
  // desired end-state -- chain-b registered -- already holds). Tolerate that one
  // revert; any other failure still fails the rollout.
  logPhase("05 polygon: addHostChainsToGatewayConfig (registration proposal effect)");
  const addHostChain = [
    "if out=$(npx hardhat task:addHostChainsToGatewayConfig 2>&1); then printf '%s\\n' \"$out\";",
    "else status=$?; printf '%s\\n' \"$out\";",
    "printf '%s\\n' \"$out\" | grep -qiE 'HostChainAlreadyRegistered|0x96a56828' || exit $status;",
    "echo '[runbook] tolerated HostChainAlreadyRegistered: chain-b pre-registered by the rollout scenario';",
    "fi",
  ].join(" ");
  await ctx.runGatewayContractTask(addHostChain);
  await ctx.test("rollout-standard", { parallel: false });
}
