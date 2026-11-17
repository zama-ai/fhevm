// Shared code of the HCULimit allowlist test tasks (zama-ai/planning-blockchain#1839).
// The task files load this file while the Hardhat config loads, so it must not import `hardhat` or the SDK at the top level.
import {
  Contract,
  Interface,
  JsonRpcProvider,
  type TransactionReceipt,
  type TransactionResponse,
  type Wallet,
  getAddress,
} from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { FhevmSdk } from '../../test/sdk/fhevm-sdk/sdk';
import type { SdkInstance } from '../../test/sdk/types';

const HCU_LIMIT_ABI = [
  'function getVersion() view returns (string)',
  'function getGlobalHCUCapPerBlock() view returns (uint48)',
  'function getMaxHCUDepthPerTx() view returns (uint48)',
  'function getMaxHCUPerTx() view returns (uint48)',
  'function isAllowlistActivated() view returns (bool)',
  'function isAllowlisted(address) view returns (bool)',
  'function isAllowlister(address) view returns (bool)',
  'function allowlisterDeadline(address) view returns (uint256)',
  'function setAllowlistActivated(bool)',
  'function addToAllowlist(address[])',
  'function removeFromAllowlist(address[])',
  'error NotHostOwner(address)',
  'error CallerNotAllowlisted(address)',
];
// Gas estimation rejects a call that reverts, so a fixed gas limit is necessary to mine it.
const REVERTING_TX_GAS_LIMIT = 1_000_000;
// The SDK and ethers can wait for one hour or more. The allowlist must not stay activated that long.
const TIMEOUT_MINUTES = 3;

let failed = 0;

/** Returns the value of an environment variable, or throws if it is not set. */
export const requireEnv = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`The environment variable ${name} is not set.`);
  return value;
};

const message = (error: unknown) => (error instanceof Error ? error.message : String(error)).split('\n')[0];

/** Prints one timestamped line. */
export const note = (text: string) => console.log(`${new Date().toISOString()} | ${text}`);

/** Prints one PASS or FAIL line and counts the failures. Returns true on PASS. */
export const check = (label: string, expected: unknown, actual: unknown, txHash?: string): boolean => {
  const pass = String(expected) === String(actual);
  if (!pass) failed += 1;
  note(
    `${label} | expected ${expected} | actual ${actual} | ${pass ? 'PASS' : 'FAIL'}${txHash ? ` | transaction ${txHash}` : ''}`,
  );
  return pass;
};

/** Runs `fn`. Logs a thrown error as a failed check, and the step continues. */
export const attempt = async (name: string, fn: () => Promise<void>) => {
  try {
    await fn();
  } catch (error) {
    check(`${name} error`, 'no error', message(error));
  }
};

const setup = async ({ ethers, network }: HardhatRuntimeEnvironment) => {
  const user = new ethers.Wallet(requireEnv('DEPLOYER_PRIVATE_KEY'), ethers.provider);
  const operator = new ethers.Wallet(requireEnv('ALLOWLISTER_PRIVATE_KEY'), ethers.provider);
  const addressA = getAddress(requireEnv('HCU_ALLOWLIST_DAPP_A'));
  // The SDK rejects an address without a checksum, and a revert reason prints an address with one.
  const addressB = getAddress(requireEnv('HCU_ALLOWLIST_DAPP_B'));
  const sepoliaRpcUrl = requireEnv('SEPOLIA_ETH_RPC_URL');
  const { bscTestnet, polygonAmoy, sepolia } = await import('@fhevm/sdk/chains');
  const chain = [bscTestnet, polygonAmoy].find(({ id }) => id === network.config.chainId);
  if (!chain || !('url' in network.config)) {
    throw new Error(`The network ${network.name} is not BNB testnet or Polygon Amoy.`);
  }
  const rpcUrl = network.config.url;
  // The ACL and the FHEVMExecutor give the HCULimit address, so it does not come from the environment.
  const executor: string = await new Contract(
    chain.fhevm.contracts.acl.address,
    ['function getFHEVMExecutorAddress() view returns (address)'],
    ethers.provider,
  ).getFHEVMExecutorAddress();
  const hcuLimitAddress: string = await new Contract(
    executor,
    ['function getHCULimitAddress() view returns (address)'],
    ethers.provider,
  ).getHCULimitAddress();
  return {
    chain,
    sepolia,
    rpcUrl,
    sepoliaRpcUrl,
    user,
    operator,
    addressA,
    addressB,
    hcuLimit: new Contract(hcuLimitAddress, HCU_LIMIT_ABI, user),
    dappA: await ethers.getContractAt('HCUAllowlistHostDapp', addressA, user),
    dappB: await ethers.getContractAt('HCUAllowlistHostDapp', addressB, user),
  } as const;
};

type Context = Awaited<ReturnType<typeof setup>>;

/** Runs one step, then exits with code 1 if a check failed. */
export const run = async (hre: HardhatRuntimeEnvironment, step: (ctx: Context) => Promise<void>) => {
  await attempt('step', async () => step(await setup(hre)));
  note(`The number of failed checks in this step is ${failed}.`);
  process.exit(failed > 0 ? 1 : 0);
};

const withTimeout = <T>(action: string, promise: Promise<T>): Promise<T> =>
  Promise.race([
    promise,
    new Promise<never>((_, reject) =>
      setTimeout(
        () => reject(new Error(`The script waited ${TIMEOUT_MINUTES} minutes for ${action}.`)),
        TIMEOUT_MINUTES * 60_000,
      ),
    ),
  ]);

// Unlike `wait()`, returns the receipt of a reverted transaction instead of throwing.
const mined = async (tx: Promise<TransactionResponse>): Promise<TransactionReceipt> => {
  const { hash, provider } = await tx;
  const poll = async (): Promise<TransactionReceipt> => {
    while (true) {
      const receipt = await provider.getTransactionReceipt(hash);
      if (receipt) return receipt;
      await new Promise((resolve) => setTimeout(resolve, 2_000));
    }
  };
  return withTimeout(`the receipt of transaction ${hash}`, poll());
};

/** Expects a mined transaction with status 1. Returns its receipt. */
export const expectSuccess = async (label: string, tx: Promise<TransactionResponse>) => {
  const receipt = await mined(tx);
  check(label, 'status 1', `status ${receipt.status}`, receipt.hash);
  return receipt;
};

const hcuLimitInterface = new Interface(HCU_LIMIT_ABI);

/** Expects a mined transaction with status 0. Then simulates the call on the state at the end of its block to read the revert reason. */
export const expectRevert = async (label: string, signer: Wallet, to: string, data: string, expectedError: string) => {
  const receipt = await mined(signer.sendTransaction({ to, data, gasLimit: REVERTING_TX_GAS_LIMIT }));
  check(label, 'status 0', `status ${receipt.status}`, receipt.hash);
  let actual = 'the simulation did not revert';
  try {
    await receipt.provider.call({ from: signer.address, to, data, blockTag: receipt.blockNumber });
  } catch (error) {
    // Hardhat throws a ProviderError, not an ethers CALL_EXCEPTION. Its `data` field holds the revert data.
    const revertData = error instanceof Error && 'data' in error ? error.data : undefined;
    const parsed = typeof revertData === 'string' ? hcuLimitInterface.parseError(revertData) : null;
    actual = parsed ? `${parsed.name}(${parsed.args.join(', ')})` : message(error);
  }
  check(`${label} revert reason, simulated at block ${receipt.blockNumber}`, expectedError, actual);
};

type KeyVerification = NonNullable<Parameters<typeof FhevmSdk.create>[0]['keyVerification']>;

/** Creates the SDK of the host chain. Only Sepolia has a KMSGeneration contract, so the SDK checks the FHE key and the CRS against it. */
export const createSdk = async ({ chain, sepolia, rpcUrl, sepoliaRpcUrl }: Context) => {
  const { FhevmSdk } = await import('../../test/sdk/fhevm-sdk/sdk');
  const { contracts, gateway, relayerUrl } = chain.fhevm;
  const sdk = FhevmSdk.create({
    verifyingContractAddressDecryption: gateway.contracts.decryption.address,
    verifyingContractAddressInputVerification: gateway.contracts.inputVerification.address,
    kmsContractAddress: contracts.kmsVerifier.address,
    inputVerifierContractAddress: contracts.inputVerifier.address,
    aclContractAddress: contracts.acl.address,
    protocolConfigAddress: contracts.protocolConfig?.address,
    keyVerification: {
      chain: sepolia,
      // SAFETY: the SDK resolves its own copy of ethers, so tsc rejects this provider for its private fields only. The SDK wrapper makes the same cast.
      client: new JsonRpcProvider(sepoliaRpcUrl) as unknown as KeyVerification['client'],
    },
    relayerUrl,
    rpcUrl,
    gatewayChainId: gateway.id,
    chainId: chain.id,
  });
  return withTimeout('the SDK setup', sdk);
};

/** Publicly decrypts one handle and returns its clear value. */
export const publicDecrypt = async (sdk: SdkInstance, handle: string) =>
  // SAFETY: the handle is a bytes32 that ethers returns as a 0x-prefixed hex string, which is the key type of `clearValues`.
  (await withTimeout('the public decryption', sdk.publicDecrypt([handle]))).clearValues[handle as `0x${string}`];

/** Stores an encrypted value in B and makes it publicly decryptable. Returns the handle of B.stored(). */
export const storeInput = async (ctx: Context, sdk: SdkInstance, value: bigint, prefix: string) => {
  const { dappB, addressB, user } = ctx;
  const input = await withTimeout(
    'the input proof',
    sdk.encryptUint64({ value, contractAddress: addressB, userAddress: user.address }),
  );
  await expectSuccess(
    `${prefix}B.storeInput(encrypted ${value})`,
    dappB.storeInput(input.handles[0], input.inputProof),
  );
  const receipt = await expectSuccess(`${prefix}B.publishStored()`, dappB.publishStored());
  return dappB.stored({ blockTag: receipt.blockNumber });
};
