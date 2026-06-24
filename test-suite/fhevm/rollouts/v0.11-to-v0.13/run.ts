import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { STANDARD_TEST_PROFILES } from "../../src/layout";
import { phaseVersions, scenario, versionSources } from "./versions";

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

const contractVersionKeys = ["GATEWAY_VERSION", "HOST_VERSION"];

// A gate after every single hop, no batching — each phase change is validated
// in isolation so a regression points at the component that introduced it.
type RolloutPhase =
  | "baseline"
  | "contracts-v012"
  | "contracts-v013"
  | "relayer"
  | "kms"
  | "listener-core"
  | "final";

// Slow/stateful or topology-bound profiles. They mutate DB state or need a
// multi-chain topology, so they make poor per-hop smoke gates and run only at
// the final gate, via the `standard` aggregate whose built-in guards self-skip
// the ones this rollout's single-chain topology can't satisfy.
const FINAL_ONLY_PROFILES: readonly string[] = [
  "coprocessor-db-state-revert",
  "ciphertext-drift-auto-recovery",
  "multi-chain-isolation",
];

// Every intermediate hop runs the complete `fhevm-cli test standard` coverage
// minus the final-only profiles — derived from STANDARD_TEST_PROFILES so it
// tracks the suite. This is deliberately the full standard suite, not the much
// thinner shared `rollout-standard` subset (which omits negative-acl,
// hcu-block-cap, the paused-contract profiles, etc.).
export const PER_HOP_TEST_PROFILES = STANDARD_TEST_PROFILES.filter(
  (profile) => !FINAL_ONLY_PROFILES.includes(profile),
);

// Per-hop gates run each non-final-only profile by name (multi-chain-isolation
// is excluded, so none hit the named-profile precondition that throws on a
// single-chain stack). The final gate runs the `standard` aggregate so it also
// covers the stateful/topology-bound profiles with their built-in skip guards.
export const rolloutPhaseProfiles = (phase: RolloutPhase): string[] =>
  phase === "final" ? ["standard"] : [...PER_HOP_TEST_PROFILES];

const testPhase = async (ctx: RolloutRunContext, phase: RolloutPhase) => {
  const profiles = rolloutPhaseProfiles(phase);
  console.log(`[rollout] ${phase} tests: ${profiles.join(", ")}`);
  for (const profile of profiles) {
    await ctx.test(profile, { parallel: false });
  }
};

// Preserves the currently-deployed contract sources as `previous-contracts`,
// then activates the next version's sources for the upgrade tasks. For the
// second hop the on-chain implementation is v0.12, but the persistent deploy
// container still runs the v0.11 boot image — so snapshot from the locked
// (v0.12) image to capture the correct upgrade baseline.
const prepareContractMigrationSources = async (
  ctx: RolloutRunContext,
  targetLockFile: string,
  options: { fromLockedImage?: boolean } = {},
) => {
  await ctx.snapshotContracts("host", options);
  await ctx.snapshotContracts("gateway", options);
  await ctx.applyVersionLock("contract migration sources", {
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

// Production-like deployments have 13 KMS nodes and currently export mpc=4.
// The local rollout stack has one KMS node, so legacy GatewayConfig exports
// the cryptographic threshold t=0. v0.13 ProtocolConfig rejects zero
// thresholds, so normalize only this synthetic one-node migration case.
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

// host-sc-deploy's templated env exposes RPC_URL for the host chain only; the
// migration assertion task additionally needs GATEWAY_RPC_URL to reach the
// gateway-node container on the same compose network.
const GATEWAY_RPC_URL = "http://gateway-node:8546";

const assertKmsMigration = async (
  ctx: RolloutRunContext,
  { gatewayConfigProxy, kmsGenerationProxy, mpcThresholdNormalized }: GatewayMigrationContext,
) => {
  // task:assertKmsMigrationSucceeded (#2469) does not self-compile; do it
  // explicitly so hre.ethers.getContractAt("ProtocolConfig", ...) resolves.
  const assertCommand = [
    "npx hardhat compile &&",
    "npx hardhat task:assertKmsMigrationSucceeded",
    `--gateway-config-proxy ${gatewayConfigProxy}`,
    `--gateway-kms-generation-proxy ${kmsGenerationProxy}`,
    "--use-internal-proxy-address true",
  ].join(" ");

  if (!mpcThresholdNormalized) {
    // Production-like deployments (n=13, mpc=4): clean assertion path.
    await ctx.runHostContractTask(assertCommand, { env: { GATEWAY_RPC_URL } });
    return;
  }

  // 1-KMS-node fixture only: normalize() patched mpc 0 -> 1, so the on-chain
  // ProtocolConfig necessarily diverges from the un-patched gateway view on
  // exactly this field. Tolerate that one error; any other failure (including
  // any other mpc-mismatch value) still fails the rollout.
  const wrapped = [
    `if out=$(${assertCommand} 2>&1); then printf '%s\\n' "$out";`,
    `else status=$?; printf '%s\\n' "$out";`,
    `printf '%s\\n' "$out" | grep -q 'ProtocolConfig MPC threshold mismatch: expected 0, got 1' || exit $status;`,
    `echo '[runbook] tolerated synthetic-fixture mpc threshold mismatch';`,
    "fi",
  ].join(" ");
  await ctx.runHostContractTask(wrapped, { env: { GATEWAY_RPC_URL } });
};

// Hop 1 (v0.11 -> v0.12): plain UUPS implementation swaps, no state migration.
// Order mirrors the tested v0.12 devnet upgrade: host HCULimit -> FHEVMExecutor
// -> ACL (FHEVMExecutor depends on the new HCU checks), then gateway
// GatewayConfig -> Decryption, then host KMSVerifier (context-aware signers).
const migrateContractsV011ToV012 = async (ctx: RolloutRunContext, v012Lock: string) => {
  logPhase("01a contracts: v0.11 -> v0.12 UUPS upgrades (no state migration)");
  // The baseline deploy containers still run the v0.11 image, so the default
  // snapshot captures the v0.11 sources we want as the upgrade baseline here.
  await prepareContractMigrationSources(ctx, v012Lock);
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeHCULimit", "HCULimit");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeFHEVMExecutor", "FHEVMExecutor");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeACL", "ACL");
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeGatewayConfig", "GatewayConfig");
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeDecryption", "Decryption");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeKMSVerifier", "KMSVerifier");
  await ctx.refreshDiscovery();
};

// Hop 2 (v0.12 -> v0.13): the full state migration from the v0.12-to-v0.13
// runbook -- export gateway KMS state, deploy ProtocolConfig and migrate
// KMSGeneration onto the host, then upgrade the remaining proxies. The v0.12
// previous-contracts come from the locked (v0.12) deploy image, since the
// persistent containers still run the v0.11 boot image.
const migrateContractsV012ToV013 = async (ctx: RolloutRunContext, v013Lock: string) => {
  logPhase("01b contracts: v0.12 -> v0.13 state migration");
  await prepareContractMigrationSources(ctx, v013Lock, { fromLockedImage: true });
  // Export gateway state before making gateway KMSGeneration view-only.
  const migrationCtx = await exportGatewayKmsMigrationEnv(ctx);
  const { migrationEnv } = migrationCtx;
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
  // Refresh discovery so downstream runtime env (test-suite, listener-core, etc.) picks up
  // the new host proxy addresses written by the migration deploy tasks.
  await ctx.refreshDiscovery();
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeKMSVerifier", "KMSVerifier");
  await assertKmsMigration(ctx, migrationCtx);
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeHCULimit", "HCULimit");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeFHEVMExecutor", "FHEVMExecutor");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeACL", "ACL");
  await ctx.refreshDiscovery();
};

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await writePhaseVersionLock(ctx, "00-baseline", phaseVersions.baseline);
  const contractsV012Lock = await writePhaseVersionLock(ctx, "01a-contracts-v012", phaseVersions.contractsV012);
  const contractsV013Lock = await writePhaseVersionLock(ctx, "01b-contracts-v013", phaseVersions.contractsV013);
  const relayerLock = await writePhaseVersionLock(ctx, "02-relayer", phaseVersions.relayer);
  const kmsLock = await writePhaseVersionLock(ctx, "03-kms", phaseVersions.kms);
  const listenerCoreLock = await writePhaseVersionLock(ctx, "04-listener-core", phaseVersions.listenerCore);
  const coprocessorLock = await writePhaseVersionLock(ctx, "05-coprocessor", phaseVersions.coprocessor);

  logPhase("00 baseline: boot v0.11.0 with the target test-suite harness");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await testPhase(ctx, "baseline");

  // Contracts always hop through v0.12; gate each hop independently so the
  // intermediate states (v0.11 services on v0.12, then on v0.13, contracts) are
  // each exercised, not just the end state.
  logPhase("01a contracts: v0.11 -> v0.12 (services still v0.11)");
  await migrateContractsV011ToV012(ctx, contractsV012Lock);
  await testPhase(ctx, "contracts-v012");

  logPhase("01b contracts: v0.12 -> v0.13 (services still v0.11)");
  await migrateContractsV012ToV013(ctx, contractsV013Lock);
  await testPhase(ctx, "contracts-v013");

  logPhase("02 relayer: straight v0.11 -> v0.13, before KMS connector consumes versioned extraData");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  await testPhase(ctx, "relayer");

  logPhase("03 kms: straight kms-core v0.13.0 -> v0.13.20 and connector v0.11 -> v0.13 together");
  await ctx.upgradeRuntimeGroup("kms", { lockFile: kmsLock });
  await testPhase(ctx, "kms");

  logPhase("04 listener-core: straight v0.11 -> v0.13 (still an old coprocessor against it)");
  await ctx.upgradeRuntimeGroup("listener-core", { lockFile: listenerCoreLock });
  await testPhase(ctx, "listener-core");

  logPhase("05 coprocessor: straight v0.11 -> v0.13 (closes the old-coprocessor / v0.13-contracts window)");
  await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });
  await testPhase(ctx, "final");
}
