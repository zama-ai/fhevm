import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { type RolloutEnv, logPhase, upgradeContract } from "../utils";
import { phaseVersions, scenario, versionSources } from "./versions";

const writePhaseVersionLock = (ctx: RolloutRunContext, name: string, versions: RolloutEnv) =>
  ctx.writeVersionLock(name, { versions, sources: versionSources });

const contractVersionKeys = ["GATEWAY_VERSION", "HOST_VERSION"];
const rolloutTestModes = ["rollout-standard", "rollout-heavy"] as const;
type RolloutTestMode = (typeof rolloutTestModes)[number];
type RolloutPhase = "baseline" | "contracts" | "relayer" | "kms" | "final";

const heavyPhaseProfiles: Record<RolloutPhase, string[]> = {
  baseline: ["rollout-standard"],
  contracts: ["rollout-standard", "operators", "random-subset", "hcu-block-cap"],
  relayer: ["rollout-standard", "negative-acl"],
  kms: ["rollout-standard", "public-decryption"],
  final: [
    "rollout-standard",
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

const prepareV013ContractMigrationSources = async (ctx: RolloutRunContext, targetLockFile: string) => {
  console.log("[contracts] preserve v0.12 sources, then activate v0.13 sources");
  await ctx.snapshotContracts("host");
  await ctx.snapshotContracts("gateway");
  await ctx.applyVersionLock("v0.13 contract migration sources", {
    lockFile: targetLockFile,
    allowedVersionKeys: contractVersionKeys,
  });
};

// task:exportKmsMigrationState writes the MIGRATION_* values consumed by the
// host ProtocolConfig and KMSGeneration migration tasks.
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

export const normalizeLocalOneNodeMpcThreshold = (env: RolloutEnv): RolloutEnv => {
  const kmsNodes = JSON.parse(env.MIGRATION_KMS_NODES ?? "[]") as unknown[];
  const thresholds = JSON.parse(env.MIGRATION_KMS_THRESHOLDS ?? "{}") as Record<string, unknown>;

  // Production-like deployments have 13 KMS nodes and currently export mpc=4.
  // The local rollout stack has one KMS node, so legacy GatewayConfig exports
  // the cryptographic threshold t=0. v0.13 ProtocolConfig rejects zero
  // thresholds, so normalize only this synthetic one-node migration case.
  if (kmsNodes.length === 1 && String(thresholds.mpc) === "0") {
    console.log("[contracts] local one-node migration: normalize ProtocolConfig mpc threshold 0 -> 1");
    return { ...env, MIGRATION_KMS_THRESHOLDS: JSON.stringify({ ...thresholds, mpc: "1" }) };
  }

  return env;
};

// True for the local rollout fixture (1 KMS node). The mpc patch above keys off the
// same condition, and task:assertKmsMigrationSucceeded compares the patched ProtocolConfig
// against the un-patched gateway view, which only diverges here.
const isLocalOneKmsNodeFixture = (env: RolloutEnv) =>
  (JSON.parse(env.MIGRATION_KMS_NODES ?? "[]") as unknown[]).length === 1;

type GatewayMigrationContext = {
  kmsGenerationProxy: string;
  gatewayConfigProxy: string;
  migrationEnv: RolloutEnv;
};

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
  return {
    kmsGenerationProxy: gateway.KMS_GENERATION_ADDRESS,
    gatewayConfigProxy: gateway.GATEWAY_CONFIG_ADDRESS,
    migrationEnv: normalizeLocalOneNodeMpcThreshold(parseGatewayMigrationEnv(migrationState.export)),
  };
};

// host-sc-deploy's templated env exposes RPC_URL for the host chain only; the
// migration assertion task additionally needs GATEWAY_RPC_URL to reach the
// gateway-node container on the same compose network.
const GATEWAY_RPC_URL = "http://gateway-node:8546";

export default async function run(ctx: RolloutRunContext) {
  const testMode = resolveRolloutTestMode(process.env.ROLLOUT_TEST_PROFILE);
  const baselineLock = await writePhaseVersionLock(ctx, "00-baseline", phaseVersions.baseline);
  const contractsLock = await writePhaseVersionLock(ctx, "01-contracts", phaseVersions.contracts);
  const relayerLock = await writePhaseVersionLock(ctx, "02-relayer", phaseVersions.relayer);
  const kmsLock = await writePhaseVersionLock(ctx, "03-kms", phaseVersions.kms);
  const listenerCoreLock = await writePhaseVersionLock(ctx, "04-listener-core", phaseVersions.listenerCore);
  const coprocessorLock = await writePhaseVersionLock(ctx, "05-coprocessor", phaseVersions.coprocessor);

  logPhase("00 baseline: boot v0.12.4 with the target test-suite harness");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await testPhase(ctx, "baseline", testMode);

  logPhase("01 contracts: execute the v0.13 migration runbook");
  await prepareV013ContractMigrationSources(ctx, contractsLock);
  // Export gateway state before making gateway KMSGeneration view-only.
  const { kmsGenerationProxy, gatewayConfigProxy, migrationEnv } = await exportGatewayKmsMigrationEnv(ctx);
  // Complete the gateway chain first, matching the v0.13 deployment runbook.
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeGatewayConfig", "GatewayConfig");
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeKMSGeneration", "KMSGeneration");
  // Then complete the host chain migration, matching the v0.13 devnet runbook:
  // deploy ProtocolConfig and KMSGeneration from gateway state, upgrade
  // KMSVerifier next so it picks up the new KMS context before any executor
  // path runs, then upgrade the remaining contracts. HCULimit moves before
  // FHEVMExecutor because new executor ops call new HCU checks.
  await ctx.runHostContractTask("npx hardhat task:deployEmptyProxiesProtocolConfigKMSGeneration");
  await ctx.runHostContractTask("npx hardhat task:deployProtocolConfigFromMigration", { env: migrationEnv });
  await ctx.runHostContractTask("npx hardhat task:deployKMSGenerationFromMigration", { env: migrationEnv });
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeKMSVerifier", "KMSVerifier");
  // Assert the migrated ProtocolConfig, KMSGeneration, and KMSVerifier state against
  // the gateway export, matching the v0.13 devnet runbook before the remaining executor upgrades.
  // task:assertKmsMigrationSucceeded (#2469) does not self-compile; do it explicitly so
  // hre.ethers.getContractAt("ProtocolConfig", ...) can resolve the artifact.
  //
  // Skip on the 1-KMS-node local fixture: normalizeLocalOneNodeMpcThreshold patches
  // mpc 0 -> 1 so the v0.13 ProtocolConfig deploy can succeed, which makes the deployed
  // config diverge from the un-patched gateway view by construction. Production runbooks
  // (n=13, mpc=4) skip the patch and run the assertion as designed.
  if (isLocalOneKmsNodeFixture(migrationEnv)) {
    console.log("[contracts] skipping task:assertKmsMigrationSucceeded for the synthetic 1-KMS-node fixture");
  } else {
    await ctx.runHostContractTask(
      [
        "npx hardhat compile &&",
        "npx hardhat task:assertKmsMigrationSucceeded",
        `--gateway-config-proxy ${gatewayConfigProxy}`,
        `--gateway-kms-generation-proxy ${kmsGenerationProxy}`,
      ].join(" "),
      { env: { GATEWAY_RPC_URL } },
    );
  }
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeHCULimit", "HCULimit");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeFHEVMExecutor", "FHEVMExecutor");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeACL", "ACL");
  await ctx.refreshDiscovery();
  await testPhase(ctx, "contracts", testMode);

  logPhase("02 relayer: upgrade relayer after contracts, before KMS connector consumes versioned extraData");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  await testPhase(ctx, "relayer", testMode);

  logPhase("03 kms: upgrade KMS core and connector together");
  await ctx.upgradeRuntimeGroup("kms", { lockFile: kmsLock });
  await testPhase(ctx, "kms", testMode);

  logPhase("04 listener-core: upgrade listener-core before coprocessor");
  // No test gate here: old coprocessor listeners do not consume listener-core.
  // The compatibility boundary is the coprocessor upgrade, where consumers
  // switch to the new listener path.
  await ctx.upgradeRuntimeGroup("listener-core", { lockFile: listenerCoreLock });
  logPhase("05 coprocessor: upgrade coprocessor");
  await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });
  await testPhase(ctx, "final", testMode);
}
