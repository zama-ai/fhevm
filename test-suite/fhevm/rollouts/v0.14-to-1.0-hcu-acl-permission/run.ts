/**
 * RFC 040 rollout: HCULimit and ACL upgrade from the deployed 0.14 release, then the rollback.
 *
 * The ACL retypes its persisted permission from `bool` to `Permission { allowed, depth, storedBlock }` in the same
 * slot. OpenZeppelin's check refuses that retype, so the upgrade task accepts it through STORAGE_EXCEPTIONS
 * (host-contracts/tasks/utils/storageExceptions.ts). This runbook checks the stored bits that the exception lets
 * through: permissions granted before the upgrade still decrypt after it, a permission granted after it records its
 * block, and both still decrypt once the proxies point back at the 0.14 implementations.
 */
import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { resolveRolloutSolidityRef } from "../../src/commands/rollout-solidity";
import { runRetainedMaterial } from "../../src/consensus/retained-material-run";
import { castCall } from "../../src/flow/readiness";
import { castSend, loadHostOwner } from "../../src/kms-onchain";
import { STATE_DIR, defaultHostChainKey } from "../../src/layout";
import { hostReachableRpcUrl } from "../../src/utils/fs";
import { run } from "../../src/utils/process";
import { migrationBaselineVersions, migrationVersions } from "../v0.14-to-v0.15-gpu-key-migration/versions";

const BASELINE_TAG = process.env.HCU_ROLLOUT_BASELINE_TAG?.trim() || "v0.14.5";
const versionSources = ["rollout=v0.14-to-1.0-hcu-acl-permission", `baseline=${BASELINE_TAG}`];
const ERC1967_IMPLEMENTATION_SLOT = "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";

// HCULimit first: the new ACL calls HCULimit.cacheDepth on every grant (RFC 040, rollout step 2).
const upgradePlan = [
  ["task:upgradeHCULimit", "HCULimit"],
  ["task:upgradeACL", "ACL"],
] as const;

const upgradeCommand = (task: string, contract: string) =>
  [
    `npx hardhat ${task}`,
    `--current-implementation previous-contracts/${contract}.sol:${contract}`,
    `--new-implementation contracts/${contract}.sol:${contract}`,
    "--verify-contract false",
    "--use-internal-proxy-address true",
  ].join(" ");

const logPhase = (label: string) => console.log(`\n[hcu-acl-permission] ${label}`);

type Host = { rpcUrl: string; acl: string; hcuLimit: string };

const readHost = async (ctx: RolloutRunContext): Promise<Host> => {
  const state = await ctx.readState();
  const key = defaultHostChainKey(state.scenario.hostChains);
  const rpcUrl = state.discovery?.endpoints.hosts[key]?.http;
  const contracts = state.discovery?.hosts[key];
  if (!rpcUrl || !contracts?.ACL_CONTRACT_ADDRESS || !contracts.HCU_LIMIT_CONTRACT_ADDRESS) {
    throw new Error(`host chain ${key} is missing its RPC endpoint, ACL or HCULimit address`);
  }
  return {
    rpcUrl: hostReachableRpcUrl(rpcUrl),
    acl: contracts.ACL_CONTRACT_ADDRESS,
    hcuLimit: contracts.HCU_LIMIT_CONTRACT_ADDRESS,
  };
};

const implementationOf = async (host: Host, proxy: string) => {
  const word = (await run(["cast", "storage", proxy, ERC1967_IMPLEMENTATION_SLOT, "--rpc-url", host.rpcUrl])).stdout.trim();
  return `0x${word.slice(-40)}`;
};

const versionOf = (host: Host, proxy: string) => castCall(host.rpcUrl, proxy, "getVersion()(string)");

const expectVersion = async (host: Host, proxy: string, expected: string) => {
  const version = await versionOf(host, proxy);
  if (!version.includes(expected)) throw new Error(`${proxy} runs ${version}, expected ${expected}`);
  console.log(`[hcu-acl-permission] ${proxy} runs ${version}`);
};

/** The handle and contract of a retained fixture, as its seed phase recorded them. */
const retainedOutput = async (ctx: RolloutRunContext, label: string) => {
  const state = await ctx.readState();
  const chainId = state.scenario.hostChains[0].chainId;
  const fixture = (await Bun.file(path.join(STATE_DIR, "runtime", "retained-material", `${label}-${chainId}.json`)).json()) as {
    address: string;
    output: string;
  };
  return fixture;
};

const storedDepth = async (host: Host, handle: string, account: string) => {
  const out = await castCall(host.rpcUrl, host.acl, "getStoredDepth(bytes32,address)(uint48,uint48)", handle, account);
  const [depth, storedBlock] = out.split(/\s+/).map((value) => BigInt(value));
  return { depth, storedBlock };
};

export default async function runHcuAclPermission(ctx: RolloutRunContext) {
  const versions = migrationVersions({ RFC029_BASELINE_FHEVM_TAG: BASELINE_TAG });
  const solidityLibraryRevision = await resolveRolloutSolidityRef(versions.baselineTag);
  const test = (profile: string) => ctx.test(profile, { parallel: false, solidityLibraryRevision });

  const mainLock = await ctx.resolveVersionLock("hcu-main-snapshot", { versions: {}, sources: versionSources });
  const main = (await Bun.file(mainLock).json()) as { env: Record<string, string>; sources: string[] };
  const baselineLock = await ctx.writeVersionLock("hcu-00-baseline", {
    versions: migrationBaselineVersions(main.env, versions.baseline),
    sources: main.sources,
  });

  logPhase(`00 boot ${BASELINE_TAG} and seed permissions with the bool ACL`);
  // The retained-material check compares coprocessors, so it needs at least two.
  await ctx.up({ lockFile: baselineLock, overrides: [{ group: "test-suite" }], scenario: "two-of-two" });
  await runRetainedMaterial(await ctx.readState(), "seed", "hcu-before-upgrade");
  const host = await readHost(ctx);
  const previousImplementations = {
    acl: await implementationOf(host, host.acl),
    hcuLimit: await implementationOf(host, host.hcuLimit),
  };
  const previousVersions = { acl: await versionOf(host, host.acl), hcuLimit: await versionOf(host, host.hcuLimit) };

  logPhase("01 upgrade HCULimit, then ACL, through the upgrade tasks and their OpenZeppelin check");
  await ctx.snapshotContracts("host");
  await ctx.applyVersionLock("RFC 040 host contracts", {
    lockFile: baselineLock,
    allowedVersionKeys: ["HOST_VERSION"],
    overrides: [{ group: "host-contracts" }],
  });
  for (const [task, contract] of upgradePlan) {
    await ctx.runHostContractTask(upgradeCommand(task, contract));
  }
  await expectVersion(host, host.hcuLimit, "v0.6.0");
  await expectVersion(host, host.acl, "v0.7.0");

  logPhase("02 old permissions still read as allowed; a new permission records its block");
  await runRetainedMaterial(await ctx.readState(), "verify", "hcu-before-upgrade");
  const before = await retainedOutput(ctx, "hcu-before-upgrade");
  const old = await storedDepth(host, before.output, before.address);
  if (old.depth !== 0n || old.storedBlock !== 0n) {
    throw new Error(`a permission granted before the upgrade reads depth ${old.depth} at block ${old.storedBlock}`);
  }
  await runRetainedMaterial(await ctx.readState(), "seed", "hcu-after-upgrade");
  const after = await retainedOutput(ctx, "hcu-after-upgrade");
  const fresh = await storedDepth(host, after.output, after.address);
  if (fresh.storedBlock === 0n) throw new Error("a permission granted after the upgrade recorded no block");
  console.log(`[hcu-acl-permission] new permission: depth ${fresh.depth}, block ${fresh.storedBlock}`);
  await test("rollout-standard");

  logPhase("03 roll back as ACT-029: owner upgradeToAndCall to the 0.14 implementations, ACL first");
  const owner = await loadHostOwner();
  await castSend(host.rpcUrl, host.acl, owner, "upgradeToAndCall(address,bytes)", previousImplementations.acl, "0x");
  await castSend(host.rpcUrl, host.hcuLimit, owner, "upgradeToAndCall(address,bytes)", previousImplementations.hcuLimit, "0x");
  await expectVersion(host, host.acl, previousVersions.acl);
  await expectVersion(host, host.hcuLimit, previousVersions.hcuLimit);

  logPhase("04 the bool ACL reads permissions from before and after the upgrade");
  await runRetainedMaterial(await ctx.readState(), "verify", "hcu-before-upgrade");
  await runRetainedMaterial(await ctx.readState(), "verify", "hcu-after-upgrade");
  await test("rollout-standard");
}
