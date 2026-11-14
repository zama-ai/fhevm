import { type Interface, Wallet } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { HCUConfigurator, HCULimit } from '../types';
import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

type CommonArgs = {
  broadcast: boolean;
  signerKeyEnv: string;
  useInternalProxyAddress: boolean;
};

/** Who may send a call: the ACL owner, the quota's admin, or the ACL owner or a pauser. */
type Sender = 'owner' | 'quotaAdmin' | 'ownerOrPauser';

// Quota 0 holds every contract nobody assigned; quotas 1 and 2 the allowlisted ones.
const LAST_UNASSIGNED_QUOTA = 2n;
const MAX_UINT48 = (1n << 48n) - 1n;

function quotaTask(name: string, description: string) {
  return task(name, description)
    .addParam('quotaId', 'Quota id, from 0 (the default quota) to 65535', undefined, types.string)
    .addOptionalParam(
      'useInternalProxyAddress',
      'Use the HCULimit and ACL proxy addresses from addresses/.env.host',
      false,
      types.boolean,
    )
    .addFlag('broadcast', 'Send the transaction instead of printing governance calldata')
    .addOptionalParam(
      'signerKeyEnv',
      'Environment variable containing the broadcast signer key',
      'DEPLOYER_PRIVATE_KEY',
    );
}

function parseUint(raw: string, name: string, max: bigint): bigint {
  if (!/^\d+$/.test(raw)) throw new Error(`${name} must be a decimal integer.`);
  const value = BigInt(raw);
  if (value > max) throw new Error(`${name} must be at most ${max}.`);
  return value;
}

function parseAddresses(raw: string, ethers: HardhatRuntimeEnvironment['ethers']): string[] {
  const entries = raw.split(',').map((entry) => entry.trim());
  if (entries.includes('')) {
    throw new Error('Provide a nonempty comma-separated list of addresses without empty entries.');
  }
  const accounts = entries.map((entry) => ethers.getAddress(entry));
  if (accounts.includes(ethers.ZeroAddress)) throw new Error('The zero address is not a valid dApp address.');
  if (new Set(accounts).size !== accounts.length) throw new Error('Duplicate addresses are not allowed.');
  return accounts;
}

function parseAccount(raw: string, ethers: HardhatRuntimeEnvironment['ethers']): string {
  if (raw.includes(',')) throw new Error('Provide exactly one address for --account.');
  return parseAddresses(raw, ethers)[0];
}

async function getHCULimit(hre: HardhatRuntimeEnvironment, options: CommonArgs): Promise<HCULimit> {
  if (options.useInternalProxyAddress) loadHostAddresses();
  const address = hre.ethers.getAddress(getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS'));
  const hcuLimit = (await hre.ethers.getContractAt('HCULimit', address)) as unknown as HCULimit;
  const version = await hcuLimit.getVersion();
  if (!version.startsWith('HCULimit v')) {
    throw new Error(`Contract at ${address} reports version "${version}"; expected HCULimit.`);
  }
  return hcuLimit;
}

async function runQuotaTask(
  hre: HardhatRuntimeEnvironment,
  options: CommonArgs,
  method: 'setQuota' | 'assignQuota' | 'setQuotaAdmin' | 'grantQuota' | 'revokeQuota' | 'pauseQuota' | 'unpauseQuota',
  args: unknown[],
  sender: Sender,
): Promise<void> {
  // HCULimit takes its settings from this address only.
  const target = await (await getHCULimit(hre, options)).getHCUConfiguratorAddress();
  const contract = (await hre.ethers.getContractAt('HCUConfigurator', target)) as unknown as HCUConfigurator;

  const calldata = (contract.interface as Interface).encodeFunctionData(method, args);
  console.log(`Chain ID: ${(await hre.ethers.provider.getNetwork()).chainId}`);
  console.log(`HCUConfigurator proxy: ${target}`);
  console.log(`Call: ${method}`);
  console.log(`Calldata: ${calldata}`);
  if (!options.broadcast) return;

  const signer = new Wallet(getRequiredEnvVar(options.signerKeyEnv), hre.ethers.provider);
  const aclAddress = hre.ethers.getAddress(getRequiredEnvVar('ACL_CONTRACT_ADDRESS'));
  const acl = await hre.ethers.getContractAt('ACL', aclAddress);
  const isOwner = signer.address.toLowerCase() === (await acl.owner()).toLowerCase();
  if (sender === 'owner' && !isOwner) {
    throw new Error(`${signer.address} is not the ACL owner.`);
  }
  if (sender === 'ownerOrPauser' && !isOwner && !(await acl.isPauser(signer.address))) {
    throw new Error(`${signer.address} is neither the ACL owner nor a pauser.`);
  }
  if (sender === 'quotaAdmin') {
    const admin = await contract.getQuotaAdmin(args[0] as bigint);
    if (signer.address.toLowerCase() !== admin.toLowerCase()) {
      throw new Error(`${signer.address} is not the admin of quota ${args[0]}.`);
    }
  }

  const tx = await signer.sendTransaction({ to: target, data: calldata });
  console.log(`Transaction: ${tx.hash}`);
  await tx.wait();
}

quotaTask(
  'task:setQuota',
  "Set a quota's maximum HCU and its decay per block. 281474976710655 leaves it unmetered; 0 and 0 stop it. " +
    'Stopping the default quota refuses every contract not assigned to another quota, and stopping quota 1 or 2 every ' +
    'allowlisted contract nobody assigned, so these require the live dApp contracts to check first.',
)
  .addParam('maxHcu', 'Maximum usage in HCU', undefined, types.string)
  .addParam('decayPerBlock', 'Usage freed per block, in HCU', undefined, types.string)
  .addOptionalParam(
    'requiredAccounts',
    'Comma-separated live dApp contract addresses that must compute in another quota before quota 0, 1 or 2 stops',
    '',
    types.string,
  )
  .setAction(async function ({ quotaId, maxHcu, decayPerBlock, requiredAccounts, ...options }, hre) {
    const id = parseUint(quotaId, 'Quota id', 0xffffn);
    const max = parseUint(maxHcu, 'Maximum HCU', MAX_UINT48);
    const decay = parseUint(decayPerBlock, 'Decay per block', MAX_UINT48);
    if (id <= LAST_UNASSIGNED_QUOTA && max === 0n) {
      const required = parseAddresses(requiredAccounts, hre.ethers);
      const hcuLimit = await getHCULimit(hre, options);
      const quotas = await Promise.all(required.map((account) => hcuLimit.getContractQuota(account)));
      const missing = required.filter((_, index) => quotas[index] === id);
      if (missing.length > 0) {
        throw new Error(`Cannot stop quota ${id}: these required dApps compute in it: ${missing.join(', ')}`);
      }
    }
    await runQuotaTask(hre, options, 'setQuota', [id, max, decay], 'owner');
  });

quotaTask('task:assignQuota', 'Move a dApp contract to a quota; the owner can move it from any quota')
  .addParam('account', 'dApp contract address', undefined, types.string)
  .setAction(async function ({ quotaId, account, ...options }, hre) {
    const args = [parseUint(quotaId, 'Quota id', 0xffffn), parseAccount(account, hre.ethers)];
    await runQuotaTask(hre, options, 'assignQuota', args, 'owner');
  });

quotaTask('task:setQuotaAdmin', "Name a quota's admin, or remove it with the zero address")
  .addParam('admin', 'Admin address', undefined, types.string)
  .setAction(async function ({ quotaId, admin, ...options }, hre) {
    const args = [parseUint(quotaId, 'Quota id', 0xffffn), hre.ethers.getAddress(admin)];
    await runQuotaTask(hre, options, 'setQuotaAdmin', args, 'owner');
  });

quotaTask('task:grantQuota', "Move a dApp contract from the default quota into the admin's quota")
  .addParam('account', 'dApp contract address', undefined, types.string)
  .setAction(async function ({ quotaId, account, ...options }, hre) {
    const args = [parseUint(quotaId, 'Quota id', 0xffffn), parseAccount(account, hre.ethers)];
    await runQuotaTask(hre, options, 'grantQuota', args, 'quotaAdmin');
  });

quotaTask('task:revokeQuota', "Move a dApp contract from the admin's quota back to the default quota")
  .addParam('account', 'dApp contract address', undefined, types.string)
  .setAction(async function ({ quotaId, account, ...options }, hre) {
    const args = [parseUint(quotaId, 'Quota id', 0xffffn), parseAccount(account, hre.ethers)];
    await runQuotaTask(hre, options, 'revokeQuota', args, 'quotaAdmin');
  });

quotaTask('task:pauseQuota', "Stop every FHE operation of a quota's contracts").setAction(async function (
  { quotaId, ...options },
  hre,
) {
  await runQuotaTask(hre, options, 'pauseQuota', [parseUint(quotaId, 'Quota id', 0xffffn)], 'ownerOrPauser');
});

quotaTask('task:unpauseQuota', "Lift a quota's pause").setAction(async function ({ quotaId, ...options }, hre) {
  await runQuotaTask(hre, options, 'unpauseQuota', [parseUint(quotaId, 'Quota id', 0xffffn)], 'owner');
});
