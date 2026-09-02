// Stands up the canonical localhost cleartext FHEVM stack: ONE call to @fhevm/host-contracts-cleartext's
// `deploy()`, driven by the vendored viem adapters, once its three preconditions hold — the deployer
// (account 5 of the package mnemonic) is funded, sits at nonce 0, and doubles as admin. The addresses
// are precomputed by the package from (deployer, start nonce) and passed in, so a wrong nonce fails on
// the first address instead of landing a clean stack somewhere else. Idempotent: a chain whose ACL
// already carries code is left untouched.

import { type Deployed, deploy, precomputeAddresses } from '@fhevm/host-contracts-cleartext/ts';
import { HardhatPluginError } from 'hardhat/plugins';
import type { EthereumProvider } from 'hardhat/types/providers';
import {
  type Address,
  type HDAccount,
  type PublicClient,
  getAddress,
  isAddress,
  parseAbi,
  parseEther,
  toHex,
} from 'viem';
import { mnemonicToAccount } from 'viem/accounts';

import { developmentChain, developmentPublicClient, developmentWalletClient } from './clients.js';
import { FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME, LOCALHOST_DEPLOYER, PLUGIN_ID } from './constants.js';
import { createViemEthereumAdaptersFromClients, createViemEthereumUtils } from './vendored/viemEthereumLib.js';

// Generous on purpose: instrumented (coverage) bytecode multiplies the deploy's gas.
const DEPLOYER_BALANCE = parseEther('10000');

export type LocalhostAddresses = Omit<Deployed, 'aclOwnerAddress'>;

/** Where the local stack lands, derived by the package from the deployer and its start nonce. */
export function precomputeLocalhostAddresses(): LocalhostAddresses {
  const { fhevmAddresses, cleartextAddresses, pauserSetAddress } = precomputeAddresses({
    ethUtils: createViemEthereumUtils(),
    from: LOCALHOST_DEPLOYER.address,
    startNonce: BigInt(LOCALHOST_DEPLOYER.startNonce),
  });
  return { fhevmAddresses, cleartextAddresses, pauserSetAddress };
}

export async function deployCleartextStack(provider: EthereumProvider): Promise<Deployed> {
  const chain = await developmentChain(provider);
  const publicClient = developmentPublicClient(provider, chain);
  const precomputed = precomputeLocalhostAddresses();

  if (await hasCode(publicClient, precomputed.fhevmAddresses.aclAddress)) {
    return { ...precomputed, aclOwnerAddress: await readAclOwner(publicClient, precomputed.fhevmAddresses.aclAddress) };
  }

  const account = await resolveDeployer(provider, publicClient);
  const adapters = createViemEthereumAdaptersFromClients({
    publicClient,
    walletClient: developmentWalletClient(provider, chain, account),
  });
  const deployed = await deploy({
    ethProvider: adapters.provider,
    ethUtils: adapters.utils,
    deployer: adapters.signer,
    // Deployer and admin are one account.
    admin: adapters.signer,
    precomputed,
  });
  // Checksummed like every other address here; `deploy()` hands the owner back lower-case.
  return { ...deployed, aclOwnerAddress: getAddress(deployed.aclOwnerAddress) };
}

async function hasCode(client: PublicClient, address: string): Promise<boolean> {
  const code = await client.getCode({ address: address as Address });
  return code !== undefined && code !== '0x';
}

const ACL_OWNER_ABI = parseAbi(['function owner() view returns (address)']);

async function readAclOwner(client: PublicClient, aclAddress: string): Promise<string> {
  const owner: unknown = await client.readContract({
    address: aclAddress as Address,
    abi: ACL_OWNER_ABI,
    functionName: 'owner',
  });
  if (typeof owner !== 'string' || !isAddress(owner)) {
    throw new HardhatPluginError(PLUGIN_ID, `ACL at ${aclAddress} returned an invalid owner.`);
  }
  return owner;
}

// Derived from the public package mnemonic, funded through the dev-node cheat, and required to be at
// the start nonce: at any other nonce the stack would land off the addresses ZamaConfig compiles in.
async function resolveDeployer(provider: EthereumProvider, client: PublicClient): Promise<HDAccount> {
  const account = mnemonicToAccount(LOCALHOST_DEPLOYER.mnemonic, { path: LOCALHOST_DEPLOYER.path });
  if (account.address !== LOCALHOST_DEPLOYER.address) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Unexpected ${FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME} deployer: derived ${account.address} at ${LOCALHOST_DEPLOYER.path}, expected ${LOCALHOST_DEPLOYER.address}.`,
    );
  }

  await setBalance(provider, account.address, DEPLOYER_BALANCE);

  const nonce = await client.getTransactionCount({ address: account.address });
  if (nonce !== LOCALHOST_DEPLOYER.startNonce) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `The cleartext deployer ${account.address} is at nonce ${String(nonce)}, expected ${String(LOCALHOST_DEPLOYER.startNonce)}. ` +
        `Every host contract address is CREATE(deployer, nonce), so the stack only lands on the addresses ` +
        `'@fhevm/solidity/config/ZamaConfig.sol' compiles into your contracts if this account has sent nothing yet. ` +
        `Restart the node, or make sure nothing else sends from this account.`,
    );
  }
  return account;
}

// anvil aliases the `hardhat_*` cheats, but not every node does: fall back to the `anvil_*` spelling.
async function setBalance(provider: EthereumProvider, address: string, balance: bigint): Promise<void> {
  const params = [address, toHex(balance)];
  try {
    await provider.request({ method: 'hardhat_setBalance', params });
  } catch {
    try {
      await provider.request({ method: 'anvil_setBalance', params });
    } catch {
      throw new HardhatPluginError(
        PLUGIN_ID,
        `Unable to fund the cleartext deployer ${address}: the node supports neither 'hardhat_setBalance' nor 'anvil_setBalance'.`,
      );
    }
  }
}
