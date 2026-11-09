import { type Contract, Wallet } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

type CommonArgs = {
  broadcast: boolean;
  signerKeyEnv: string;
  useInternalProxyAddress: boolean;
};

function allowlistTask(name: string, description: string) {
  return task(name, description)
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

function parseAddresses(raw: string, ethers: HardhatRuntimeEnvironment['ethers']): string[] {
  const entries = raw.split(',').map((entry) => entry.trim());
  if (entries.some((entry) => entry.length === 0)) {
    throw new Error('Provide a nonempty comma-separated list of addresses without empty entries.');
  }
  const accounts = entries.map((entry) => ethers.getAddress(entry));
  if (accounts.some((account) => account === ethers.ZeroAddress)) {
    throw new Error('The zero address is not a valid allowlist account.');
  }
  if (new Set(accounts.map((account) => account.toLowerCase())).size !== accounts.length) {
    throw new Error('Duplicate addresses are not allowed.');
  }
  return accounts;
}

function parseAccount(raw: string, ethers: HardhatRuntimeEnvironment['ethers']): string {
  if (raw.includes(',')) throw new Error('Provide exactly one address for --account.');
  return parseAddresses(raw, ethers)[0];
}

async function runAllowlistTask(
  hre: HardhatRuntimeEnvironment,
  options: CommonArgs,
  method: 'setAllowlistActivated' | 'addToAllowlist' | 'removeFromAllowlist' | 'addAllowlister' | 'removeAllowlister',
  args: unknown[],
  preflight?: (contract: Contract) => Promise<void>,
): Promise<void> {
  if (options.useInternalProxyAddress) loadHostAddresses();
  const target = hre.ethers.getAddress(getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS'));
  const contract = (await hre.ethers.getContractAt('HCULimit', target)) as unknown as Contract;
  const version = await contract.getFunction('getVersion').staticCall();
  if (!version.startsWith('HCULimit v')) {
    throw new Error(`Contract at ${target} reports version "${version}"; expected HCULimit.`);
  }
  if (preflight) await preflight(contract);

  const calldata = contract.interface.encodeFunctionData(method, args);
  console.log(`Chain ID: ${(await hre.ethers.provider.getNetwork()).chainId}`);
  console.log(`HCULimit proxy: ${target}`);
  console.log(`Call: ${method}`);
  console.log(`Calldata: ${calldata}`);
  if (!options.broadcast) return;

  const signer = new Wallet(getRequiredEnvVar(options.signerKeyEnv), hre.ethers.provider);
  const aclAddress = hre.ethers.getAddress(getRequiredEnvVar('ACL_CONTRACT_ADDRESS'));
  const acl = await hre.ethers.getContractAt('ACL', aclAddress);
  const owner = await acl.owner();
  const isOwner = signer.address.toLowerCase() === owner.toLowerCase();
  if (
    !isOwner &&
    (method !== 'addToAllowlist' || !(await contract.getFunction('isAllowlister').staticCall(signer.address)))
  ) {
    throw new Error(
      `${signer.address} is not the ACL owner${method === 'addToAllowlist' ? ' or an active allowlister' : ''}.`,
    );
  }

  const tx = await signer.sendTransaction({ to: target, data: calldata });
  console.log(`Transaction: ${tx.hash}`);
  await tx.wait();
}

allowlistTask('task:setAllowlistActivated', 'Activate or deactivate the HCULimit computation allowlist')
  .addParam('activated', 'Whether to activate the computation allowlist', undefined, types.boolean)
  .addOptionalParam(
    'requiredAccounts',
    'Comma-separated live dApp contract addresses to check before activation',
    '',
    types.string,
  )
  .setAction(async function ({ activated, requiredAccounts, ...options }, hre) {
    const required = activated ? parseAddresses(requiredAccounts, hre.ethers) : [];
    await runAllowlistTask(hre, options, 'setAllowlistActivated', [activated], async (contract) => {
      if (!activated) return;
      const membership = await Promise.all(
        required.map((account) => contract.getFunction('isAllowlisted').staticCall(account)),
      );
      const missing = required.filter((_, index) => !membership[index]);
      if (missing.length > 0) {
        throw new Error(`Cannot activate: these required dApps are not allowlisted: ${missing.join(', ')}`);
      }
    });
  });

allowlistTask('task:addToAllowlist', 'Admit dApp contracts to HCULimit computation')
  .addParam('accounts', 'Comma-separated dApp contract addresses', undefined, types.string)
  .setAction(async function ({ accounts, ...options }, hre) {
    await runAllowlistTask(hre, options, 'addToAllowlist', [parseAddresses(accounts, hre.ethers)]);
  });

allowlistTask('task:removeFromAllowlist', 'Remove dApp contracts from the HCULimit computation allowlist')
  .addParam('accounts', 'Comma-separated dApp contract addresses', undefined, types.string)
  .setAction(async function ({ accounts, ...options }, hre) {
    await runAllowlistTask(hre, options, 'removeFromAllowlist', [parseAddresses(accounts, hre.ethers)]);
  });

allowlistTask('task:addAllowlister', 'Grant or replace a delegated HCULimit allowlister deadline')
  .addParam('account', 'Allowlister address', undefined, types.string)
  .addParam('deadline', 'Future Unix timestamp in seconds', undefined, types.string)
  .setAction(async function ({ account, deadline, ...options }, hre) {
    const allowlister = parseAccount(account, hre.ethers);
    if (!/^\d+$/.test(deadline)) throw new Error('Deadline must be a decimal Unix timestamp in seconds.');
    const timestamp = BigInt(deadline);
    const latestBlock = await hre.ethers.provider.getBlock('latest');
    if (!latestBlock || timestamp <= BigInt(latestBlock.timestamp)) {
      throw new Error('Allowlister deadline must be after the latest block timestamp.');
    }
    await runAllowlistTask(hre, options, 'addAllowlister', [allowlister, timestamp]);
  });

allowlistTask('task:removeAllowlister', 'Clear a delegated HCULimit allowlister deadline')
  .addParam('account', 'Allowlister address', undefined, types.string)
  .setAction(async function ({ account, ...options }, hre) {
    await runAllowlistTask(hre, options, 'removeAllowlister', [parseAccount(account, hre.ethers)]);
  });
