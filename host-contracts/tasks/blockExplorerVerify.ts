import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment, RunTaskFunction, TaskArguments } from 'hardhat/types';

import { formatError } from './utils/formatError';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Verifies a single contract on the block explorer, skipping the benign "already verified" response.
//
// `@nomicfoundation/hardhat-verify` rethrows "Already Verified" as a hard error — for the auto-matched
// ERC1967 proxy, and for the deterministic implementation when a prior deploy already verified it.
// When a per-contract `task:verify*` is called straight from a deploy script (the gitops sc-deploy
// pattern), that error combines with `set -eo pipefail` to abort the whole deploy. Genuine failures
// (bad API key, explorer down, bytecode mismatch) are rethrown unchanged.
export async function verifyContract(run: RunTaskFunction, address: string): Promise<void> {
  try {
    await run('verify:verify', { address, constructorArguments: [] });
  } catch (error) {
    if (/already verified/i.test(formatError(error))) {
      console.log(`${address} is already verified — skipping.`);
    } else {
      throw error;
    }
  }
}

task('task:verifyACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationACLAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyFHEVMExecutor')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('FHEVM_EXECUTOR_CONTRACT_ADDRESS');
    const implementationFHEVMExecutorAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationFHEVMExecutorAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyKMSVerifier')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');
    const implementationKMSVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationKMSVerifierAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyInputVerifier')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('INPUT_VERIFIER_CONTRACT_ADDRESS');
    const implementationInputVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationInputVerifierAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyHCULimit')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS');
    const implementationHCULimitAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationHCULimitAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyPauserSet')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const implementationPauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');
    await verifyContract(run, implementationPauserSetAddress);
  });

task('task:verifyProtocolConfig')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('PROTOCOL_CONFIG_CONTRACT_ADDRESS');
    const implementationProtocolConfigAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationProtocolConfigAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyKMSGeneration')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
    const implementationKMSGenerationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationKMSGenerationAddress);
    await verifyContract(run, proxyAddress);
  });

// Verify tasks deployed on every host chain (canonical and secondary).
const SHARED_HOST_VERIFY_TASKS = [
  ['task:verifyACL', 'ACL'],
  ['task:verifyFHEVMExecutor', 'FHEVMExecutor'],
  ['task:verifyKMSVerifier', 'KMSVerifier'],
  ['task:verifyInputVerifier', 'InputVerifier'],
  ['task:verifyHCULimit', 'HCULimit'],
  ['task:verifyPauserSet', 'PauserSet'],
  ['task:verifyProtocolConfig', 'ProtocolConfig'],
] as const;

// Verify tasks deployed only on the canonical host chain.
const CANONICAL_ONLY_VERIFY_TASKS = [['task:verifyKMSGeneration', 'KMSGeneration']] as const;

type HostVerifyTaskName =
  | (typeof SHARED_HOST_VERIFY_TASKS)[number][0]
  | (typeof CANONICAL_ONLY_VERIFY_TASKS)[number][0];

type HostVerifyResult = {
  label: string;
  ok: boolean;
  error?: string;
};

// Verification is best-effort: a re-run legitimately hits "already verified", so a single failure
// must not abort the rest of the batch. We capture each outcome instead of swallowing it, so the
// end-of-run summary can show the operator exactly which contracts failed and the raw error.
async function runVerifyTask(
  hre: HardhatRuntimeEnvironment,
  taskName: HostVerifyTaskName,
  label: string,
  args: TaskArguments,
): Promise<HostVerifyResult> {
  try {
    console.log(`Verify ${label} contract:`);
    await hre.run(taskName, args);
    return { label, ok: true };
  } catch (error) {
    const message = formatError(error);
    console.error(`Verification of ${label} failed: ${message}`);
    return { label, ok: false, error: message };
  }
}

// Prints a per-contract pass/fail summary so the operator can triage at a glance. The raw error is
// repeated for each failure to settle why it failed: "already verified" is benign, explorer/network/
// rate-limit errors are usually transient and safe to retry, anything else warrants investigation.
function printHostVerifySummary(scope: string, results: HostVerifyResult[]): void {
  const failed = results.filter((result) => !result.ok);
  console.log(`\n${scope} contract verification summary (${results.length - failed.length}/${results.length} ok):`);
  for (const result of results) {
    console.log(result.ok ? `  [ok]     ${result.label}` : `  [FAILED] ${result.label}: ${result.error}`);
  }
  if (failed.length === 0) {
    console.log('All contracts verified successfully.');
    return;
  }
  console.log(
    `${failed.length} verification(s) failed: ${failed.map((result) => result.label).join(', ')}. ` +
      `Review the errors above — "already verified" is safe to ignore; transient explorer/network errors can be retried.`,
  );
}

task('task:verifySecondaryHost')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    const args = { useInternalProxyAddress };
    const results: HostVerifyResult[] = [];
    for (const [taskName, label] of SHARED_HOST_VERIFY_TASKS) {
      results.push(await runVerifyTask(hre, taskName, label, args));
    }
    printHostVerifySummary('Secondary host', results);
  });

task('task:verifyCanonicalHost')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    const args = { useInternalProxyAddress };
    const results: HostVerifyResult[] = [];
    for (const [taskName, label] of [...SHARED_HOST_VERIFY_TASKS, ...CANONICAL_ONLY_VERIFY_TASKS]) {
      results.push(await runVerifyTask(hre, taskName, label, args));
    }
    printHostVerifySummary('Canonical host', results);
  });
