/**
 * Devnet Coprocessor Single-Signer cutover — local rollout runbook.
 *
 * Validates the DAO cutover `GatewayConfig.updateCoprocessors([zama], 1)` against
 * the local stack: after the cutover the excluded partner coprocessors' tx-senders
 * must decode the gateway revert (NotCoprocessorTxSender) as a non-retryable config
 * error and STOP retrying, while the protocol stays healthy on the single Zama
 * signer. The cutover happens while the gateway is still v0.12.5; the v0.13 upgrade
 * phases then run in lockstep and the same soak gate is re-asserted on v0.13.
 *
 * Built on top of the v0.12-to-v0.13 rollout (#2404, now on main). The v0.13
 * contract-migration sequence is currently duplicated locally (see
 * executeV013ContractMigration) rather than shared with rollouts/v0.12-to-v0.13/run.ts.
 * Extracting a shared module imported by both runbooks is a sensible follow-up —
 * flagged for review.
 */
import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { phaseVersions, scenario, versionSources } from "./versions";

export type RolloutEnv = Record<string, string>;

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

const writePhaseVersionLock = (ctx: RolloutRunContext, name: string, versions: RolloutEnv) =>
  ctx.writeVersionLock(name, { versions, sources: versionSources });

const contractVersionKeys = ["GATEWAY_VERSION", "HOST_VERSION"];

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

// ---------------------------------------------------------------------------
// Single-signer cutover.
//
// runContractTask runs the gateway-sc-deploy image's BAKED-IN code (no local
// source mount), so a new hardhat task added to this repo would not exist in any
// released gateway-contracts image. The fhevm-cli package also has no ethers
// dependency. We therefore drive the cutover through the blessed
// ctx.runGatewayContractTask primitive with an inline hardhat script — the same
// "inline sh -lc script" pattern #2404's assertKmsMigration already uses. A
// minimal human-readable ABI is used so the script needs no compiled artifacts
// and runs unchanged on both v0.12.5 and v0.13.
//
// On v0.12.5, updateCoprocessors is `onlyOwner` WITHOUT the v0.13
// `whenInputVerificationPaused` interlock (present at GatewayConfig.sol:333 on
// v0.13, absent on v0.12.5). So no pause is required here — and pausing all
// gateway contracts would starve the input-verification path the soak depends
// on. We therefore intentionally skip the devnet runbook's pause/unpause parity
// step at the v0.12.5 baseline.
// ---------------------------------------------------------------------------
const SINGLE_SIGNER_CUTOVER_SCRIPT = String.raw`
const hre = require("hardhat");
const { ethers } = hre;

async function main() {
  const env = process.env;
  const gatewayConfigAddress = env.GATEWAY_CONFIG_ADDRESS;
  if (!gatewayConfigAddress) throw new Error("GATEWAY_CONFIG_ADDRESS missing from gateway-sc env");
  const deployerKey = env.DEPLOYER_PRIVATE_KEY;
  if (!deployerKey) throw new Error("DEPLOYER_PRIVATE_KEY missing from gateway-sc env");

  const zama = [{
    txSenderAddress: env.COPROCESSOR_TX_SENDER_ADDRESS_0,
    signerAddress: env.COPROCESSOR_SIGNER_ADDRESS_0,
    s3BucketUrl: env.COPROCESSOR_S3_BUCKET_URL_0,
  }];
  if (!zama[0].txSenderAddress || !zama[0].signerAddress || !zama[0].s3BucketUrl) {
    throw new Error("COPROCESSOR_{TX_SENDER,SIGNER}_ADDRESS_0 / _S3_BUCKET_URL_0 missing from gateway-sc env");
  }

  const owner = new ethers.Wallet(deployerKey, ethers.provider);
  const abi = [
    "function updateCoprocessors((address txSenderAddress, address signerAddress, string s3BucketUrl)[] newCoprocessors, uint256 newCoprocessorThreshold)",
    "function getCoprocessorSigners() view returns (address[])",
  ];
  const gatewayConfig = new ethers.Contract(gatewayConfigAddress, abi, owner);

  const before = await gatewayConfig.getCoprocessorSigners();
  console.log("[cutover] coprocessor signers before: " + JSON.stringify(before));
  console.log("[cutover] updateCoprocessors([zama], 1) from owner " + owner.address);
  const receipt = await (await gatewayConfig.updateCoprocessors(zama, 1)).wait();
  const after = await gatewayConfig.getCoprocessorSigners();
  console.log("[cutover] tx " + receipt.hash + "; coprocessor signers after: " + JSON.stringify(after));
  if (after.length !== 1) {
    throw new Error("expected exactly 1 coprocessor signer after cutover, got " + after.length);
  }
}

main().then(() => process.exit(0)).catch((error) => { console.error(error); process.exit(1); });
`;

const runSingleSignerCutover = async (ctx: RolloutRunContext) => {
  // The script must live UNDER /app so `require("hardhat")` resolves via
  // /app/node_modules (Node walks up from the script's directory — /tmp has no
  // node_modules). /app/addresses is the writable, mounted dir the migration
  // tasks already use as scratch. --no-compile keeps it fast: the minimal ABI
  // needs no build.
  const scriptPath = "/app/addresses/single-signer-cutover.cjs";
  const command = [
    `cat > ${scriptPath} <<'CUTOVER_EOF'`,
    SINGLE_SIGNER_CUTOVER_SCRIPT,
    "CUTOVER_EOF",
    `npx hardhat run --no-compile ${scriptPath}`,
  ].join("\n");
  await ctx.runGatewayContractTask(command);
};

// ---------------------------------------------------------------------------
// Host-side cutover — REQUIRED for the cutover to be complete.
//
// The gateway updateCoprocessors above only changes the GATEWAY chain. The
// relayer-SDK verifies input proofs against the HOST chain's InputVerifier
// (it reads InputVerifier.getThreshold()/getCoprocessorSigners()), NOT
// GatewayConfig. Verified locally: a gateway-only cutover leaves the host
// InputVerifier at the old N-of-M set, so input proofs fail with
// `RelayerThresholdCoprocessorSignerError: threshold is not reached` and the
// protocol is NOT healthy on the single signer. We therefore mirror the cutover
// on the host InputVerifier via defineNewContext(newSigners, newThreshold),
// which is `onlyACLOwner` (host-contracts/contracts/InputVerifier.sol:174) — the
// host deployer is the ACL owner. The InputVerifier address is resolved at
// runtime from the mounted FHEVMHostAddresses.sol.
const SINGLE_SIGNER_HOST_CUTOVER_SCRIPT = String.raw`
const hre = require("hardhat");
const { ethers } = hre;

async function main() {
  const env = process.env;
  const inputVerifierAddress = env.INPUT_VERIFIER_ADDRESS;
  if (!inputVerifierAddress) throw new Error("INPUT_VERIFIER_ADDRESS not resolved from FHEVMHostAddresses.sol");
  const deployerKey = env.DEPLOYER_PRIVATE_KEY;
  if (!deployerKey) throw new Error("DEPLOYER_PRIVATE_KEY missing from host-sc env");
  const signer0 = env.COPROCESSOR_SIGNER_ADDRESS_0;
  if (!signer0) throw new Error("COPROCESSOR_SIGNER_ADDRESS_0 missing from host-sc env");

  const owner = new ethers.Wallet(deployerKey, ethers.provider);
  const abi = [
    "function defineNewContext(address[] newSignersSet, uint256 newThreshold)",
    "function getCoprocessorSigners() view returns (address[])",
    "function getThreshold() view returns (uint256)",
  ];
  const inputVerifier = new ethers.Contract(inputVerifierAddress, abi, owner);

  const before = await inputVerifier.getCoprocessorSigners();
  console.log("[host-cutover] InputVerifier signers before: " + JSON.stringify(before) + " threshold " + (await inputVerifier.getThreshold()).toString());
  console.log("[host-cutover] defineNewContext([zama], 1) from ACL owner " + owner.address);
  const receipt = await (await inputVerifier.defineNewContext([signer0], 1)).wait();
  const after = await inputVerifier.getCoprocessorSigners();
  console.log("[host-cutover] tx " + receipt.hash + "; InputVerifier signers after: " + JSON.stringify(after) + " threshold " + (await inputVerifier.getThreshold()).toString());
  if (after.length !== 1) {
    throw new Error("expected exactly 1 host coprocessor signer after cutover, got " + after.length);
  }
}

main().then(() => process.exit(0)).catch((error) => { console.error(error); process.exit(1); });
`;

const runHostSingleSignerCutover = async (ctx: RolloutRunContext) => {
  const scriptPath = "/app/addresses/host-single-signer-cutover.cjs";
  const command = [
    // Resolve the deployed InputVerifier address from the mounted host addresses.
    `INPUT_VERIFIER_ADDRESS=$(grep inputVerifierAdd /app/addresses/FHEVMHostAddresses.sol | grep -oE '0x[0-9a-fA-F]{40}')`,
    `cat > ${scriptPath} <<'CUTOVER_EOF'`,
    SINGLE_SIGNER_HOST_CUTOVER_SCRIPT,
    "CUTOVER_EOF",
    `INPUT_VERIFIER_ADDRESS=$INPUT_VERIFIER_ADDRESS npx hardhat run --no-compile ${scriptPath}`,
  ].join("\n");
  await ctx.runHostContractTask(command);
};

// ---------------------------------------------------------------------------
// v0.13 contract migration — currently duplicated from rollouts/v0.12-to-v0.13/run.ts;
// see that runbook for the per-step rationale comments. Candidate for extraction into a
// shared module (see PR description).
// ---------------------------------------------------------------------------
const prepareV013ContractMigrationSources = async (ctx: RolloutRunContext, targetLockFile: string) => {
  console.log("[contracts] preserve v0.12 sources, then activate v0.13 sources");
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

const executeV013ContractMigration = async (ctx: RolloutRunContext, contractsLock: string) => {
  await prepareV013ContractMigrationSources(ctx, contractsLock);
  const migrationCtx = await exportGatewayKmsMigrationEnv(ctx);
  const { migrationEnv } = migrationCtx;
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeGatewayConfig", "GatewayConfig");
  await upgradeContract((command) => ctx.runGatewayContractTask(command), "task:upgradeKMSGeneration", "KMSGeneration");
  await ctx.runHostContractTask("npx hardhat task:deployEmptyProxiesProtocolConfigKMSGeneration");
  await ctx.runHostContractTask("npx hardhat task:deployProtocolConfigFromMigration", { env: migrationEnv });
  await ctx.runHostContractTask("npx hardhat task:deployKMSGenerationFromMigration", { env: migrationEnv });
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
  const contractsLock = await writePhaseVersionLock(ctx, "01-contracts", phaseVersions.contracts);
  const relayerLock = await writePhaseVersionLock(ctx, "02-relayer", phaseVersions.relayer);
  const kmsLock = await writePhaseVersionLock(ctx, "03-kms", phaseVersions.kms);
  const listenerCoreLock = await writePhaseVersionLock(ctx, "04-listener-core", phaseVersions.listenerCore);
  const coprocessorLock = await writePhaseVersionLock(ctx, "05-coprocessor", phaseVersions.coprocessor);

  logPhase("00 baseline: boot v0.12.5 (two-of-three coprocessors) with the target test-suite harness");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });

  logPhase("00 baseline gate: multi-signer write-path is healthy before the cutover");
  await ctx.test("rollout-standard", { parallel: false });

  // The cutover is TWO-SIDED: the gateway GatewayConfig governs on-chain tx-sender
  // consensus, while the host InputVerifier governs the input-proof signature
  // threshold the relayer-SDK enforces. Both must move to the single Zama signer
  // or the input path breaks (verified locally).
  logPhase("cutover (gateway): GatewayConfig.updateCoprocessors([zama], 1) on the v0.12.5 gateway");
  await runSingleSignerCutover(ctx);
  logPhase("cutover (host): InputVerifier.defineNewContext([zama], 1) on the host chain");
  await runHostSingleSignerCutover(ctx);

  logPhase("phase 1 gate: partner-on-v0.12.5, excluded by the single-signer gateway");
  await ctx.test("single-signer-soak", { parallel: false });

  logPhase("01 contracts: execute the v0.13 migration runbook");
  await executeV013ContractMigration(ctx, contractsLock);
  logPhase("02 relayer: upgrade relayer");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  logPhase("03 kms: upgrade KMS core and connector together");
  await ctx.upgradeRuntimeGroup("kms", { lockFile: kmsLock });
  logPhase("04 listener-core: upgrade listener-core before coprocessor");
  await ctx.upgradeRuntimeGroup("listener-core", { lockFile: listenerCoreLock });
  logPhase("05 coprocessor: upgrade coprocessor");
  await ctx.upgradeRuntimeGroup("coprocessor", { lockFile: coprocessorLock });

  logPhase("phase 2 gate: partner-on-v0.13, still excluded by the single-signer gateway");
  await ctx.test("single-signer-soak", { parallel: false });
}
