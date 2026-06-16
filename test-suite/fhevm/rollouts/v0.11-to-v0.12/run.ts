import type { RolloutRunContext } from "../../src/commands/rollout-run";
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

type RolloutPhase = "baseline" | "contracts" | "kms" | "final";

// Single coprocessor / Zama-only consensus, so the cross-node ciphertext-digest
// split-brain can't fire; gate each phase with the standard rollout suite.
const testPhase = async (ctx: RolloutRunContext, phase: RolloutPhase) => {
  console.log(`[rollout] ${phase} tests: rollout-standard`);
  await ctx.test("rollout-standard", { parallel: false });
};

// v0.11 -> v0.12 is a plain UUPS upgrade with no state migration, so the live
// (v0.11) deploy containers still hold the sources we want as the upgrade
// baseline -- the default snapshot captures them correctly.
const prepareContractMigrationSources = async (ctx: RolloutRunContext, targetLockFile: string) => {
  await ctx.snapshotContracts("host");
  await ctx.snapshotContracts("gateway");
  await ctx.applyVersionLock("contract migration sources", {
    lockFile: targetLockFile,
    allowedVersionKeys: contractVersionKeys,
  });
};

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await writePhaseVersionLock(ctx, "00-baseline", phaseVersions.baseline);
  const contractsLock = await writePhaseVersionLock(ctx, "01-contracts", phaseVersions.contracts);
  const kmsLock = await writePhaseVersionLock(ctx, "02-kms", phaseVersions.kms);
  const coprocessorLock = await writePhaseVersionLock(ctx, "03-coprocessor", phaseVersions.coprocessor);

  logPhase("00 baseline: boot v0.11 (single coprocessor, Zama-only consensus)");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await testPhase(ctx, "baseline");

  // Plain UUPS implementation swaps, no state migration. Order mirrors the
  // tested v0.12 devnet upgrade: host HCULimit -> FHEVMExecutor -> ACL
  // (FHEVMExecutor depends on the new HCU checks), then gateway GatewayConfig
  // -> Decryption, then host KMSVerifier (adds getCurrentKmsContextId, which the
  // v0.12 KMS connector requires).
  logPhase("01 contracts: v0.11 -> v0.12 plain UUPS upgrades (no state migration)");
  await prepareContractMigrationSources(ctx, contractsLock);
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeHCULimit", "HCULimit");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeFHEVMExecutor", "FHEVMExecutor");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeACL", "ACL");
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeGatewayConfig", "GatewayConfig");
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeDecryption", "Decryption");
  await upgradeContract((command) => ctx.runHostContractTask(command), "task:upgradeKMSVerifier", "KMSVerifier");
  await ctx.refreshDiscovery();
  await testPhase(ctx, "contracts");

  // The relayer is unchanged across this hop (shared v0.11.x line), so it has no
  // phase. KMS moves next: core v0.13.0 -> v0.13.10 and the connector v0.11 ->
  // v0.12, which depends on the now-upgraded v0.12 host/gateway contract ABI.
  logPhase("02 kms: core v0.13.0 -> v0.13.10 and connector v0.11 -> v0.12 together");
  await ctx.upgradeRuntimeGroup("kms", { lockFile: kmsLock });
  await testPhase(ctx, "kms");

  // No listener-core phase: the standalone listener-core (v2) is a v0.13
  // component; at v0.12 the listener rides with the coprocessor's host-listener,
  // which moves in the coprocessor group below.
  logPhase("03 coprocessor: v0.11 -> v0.12 (last; carries the host-listener)");
  await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });
  await testPhase(ctx, "final");
}
