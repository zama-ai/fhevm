import { ethers as hardhatEthers } from 'hardhat';
import { ethers } from 'ethers';
import { createInstance as createFhevmInstance } from '@zama-fhe/relayer-sdk/node';
import { vars } from 'hardhat/config';
import { getCompatExtraData } from './compat';

const defaultMnemonic =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
const mnemonic: string = process.env.MNEMONIC ?? vars.get('MNEMONIC', defaultMnemonic);

const decryptionAddress = process.env.DECRYPTION_ADDRESS!;
const inputVerificationAddress = process.env.INPUT_VERIFICATION_ADDRESS!;
const relayerUrl = process.env.RELAYER_URL!;
const gatewayChainId = Number(process.env.CHAIN_ID_GATEWAY!);
const publicDecryptTimeoutMs = Number(process.env.RELAYER_SDK_PUBLIC_DECRYPT_TIMEOUT_MS) || 125 * 60 * 1000;
const inputProofTimeoutMs = Number(process.env.RELAYER_SDK_INPUT_PROOF_TIMEOUT_MS) || 245 * 60 * 1000;

export interface ChainConfig {
  rpcUrl: string;
  chainId: number;
  aclAddress: string;
  kmsVerifierAddress: string;
  inputVerifierAddress: string;
}

function requireEnv(name: string): string {
  const value = process.env[name];
  if (!value) throw new Error(`Missing required env var: ${name}`);
  return value;
}

/** Discovers all host chains from indexed env vars (HOST_CHAIN_1_*, HOST_CHAIN_2_*, …). */
function parseHostChains(): ChainConfig[] {
  const primary: ChainConfig = {
    rpcUrl: requireEnv('RPC_URL'),
    chainId: Number(requireEnv('CHAIN_ID_HOST')),
    aclAddress: requireEnv('ACL_CONTRACT_ADDRESS'),
    kmsVerifierAddress: requireEnv('KMS_VERIFIER_CONTRACT_ADDRESS'),
    inputVerifierAddress: requireEnv('INPUT_VERIFIER_CONTRACT_ADDRESS'),
  };
  const chains: ChainConfig[] = [primary];
  for (let i = 1; ; i++) {
    const rpcUrl = process.env[`HOST_CHAIN_${i}_RPC_URL`];
    const chainId = process.env[`HOST_CHAIN_${i}_CHAIN_ID`];
    if (!rpcUrl || !chainId) break;
    chains.push({
      rpcUrl,
      chainId: Number(chainId),
      aclAddress: requireEnv(`HOST_CHAIN_${i}_ACL_CONTRACT_ADDRESS`),
      kmsVerifierAddress: requireEnv(`HOST_CHAIN_${i}_KMS_VERIFIER_CONTRACT_ADDRESS`),
      inputVerifierAddress: requireEnv(`HOST_CHAIN_${i}_INPUT_VERIFIER_CONTRACT_ADDRESS`),
    });
  }
  return chains;
}

/** All discovered host chains. Index 0 is the primary chain. */
export const HOST_CHAINS: ChainConfig[] = parseHostChains();

const providers = new Map<string, ethers.JsonRpcProvider>();

export function getProvider(chain: ChainConfig): ethers.JsonRpcProvider {
  if (!providers.has(chain.rpcUrl)) {
    providers.set(chain.rpcUrl, new ethers.JsonRpcProvider(chain.rpcUrl));
  }
  return providers.get(chain.rpcUrl)!;
}

export type ManagedWallet = ethers.NonceManager & { address: string; reset: () => void };

function wrapWithNonceManager(wallet: ethers.Wallet): ManagedWallet {
  const nm = new ethers.NonceManager(wallet);
  (nm as any).address = wallet.address;
  return nm as ManagedWallet;
}

export interface NamedSigners {
  alice: ManagedWallet;
  bob: ManagedWallet;
  carol: ManagedWallet;
  dave: ManagedWallet;
  eve: ManagedWallet;
}

const signersCache = new Map<string, NamedSigners>();

export function getSigners(chain: ChainConfig): NamedSigners {
  if (!signersCache.has(chain.rpcUrl)) {
    const provider = getProvider(chain);
    const hdNode = ethers.HDNodeWallet.fromMnemonic(
      ethers.Mnemonic.fromPhrase(mnemonic),
      "m/44'/60'/0'/0",
    );
    const mkWallet = (i: number) => wrapWithNonceManager(
      new ethers.Wallet(hdNode.deriveChild(i).privateKey, provider),
    );
    signersCache.set(chain.rpcUrl, {
      alice: mkWallet(0),
      bob: mkWallet(1),
      carol: mkWallet(2),
      dave: mkWallet(3),
      eve: mkWallet(4),
    });
  }
  return signersCache.get(chain.rpcUrl)!;
}

export function getWallet(chain: ChainConfig, index: number): ManagedWallet {
  const provider = getProvider(chain);
  const hdNode = ethers.HDNodeWallet.fromMnemonic(
    ethers.Mnemonic.fromPhrase(mnemonic),
    "m/44'/60'/0'/0",
  );
  return wrapWithNonceManager(
    new ethers.Wallet(hdNode.deriveChild(index).privateKey, provider),
  );
}

export async function createInstance(chain: ChainConfig) {
  const instance = await createFhevmInstance({
    verifyingContractAddressDecryption: decryptionAddress,
    verifyingContractAddressInputVerification: inputVerificationAddress,
    kmsContractAddress: chain.kmsVerifierAddress,
    inputVerifierContractAddress: chain.inputVerifierAddress,
    aclContractAddress: chain.aclAddress,
    network: chain.rpcUrl,
    relayerUrl,
    gatewayChainId,
    chainId: chain.chainId,
  });
  const createEncryptedInput = (contractAddress: string, userAddress: string) => {
    const input = instance.createEncryptedInput(contractAddress, userAddress);
    return {
      ...input,
      encrypt: (options) => input.encrypt({ timeout: inputProofTimeoutMs, ...options }),
    };
  };
  const getExtraData = () => getCompatExtraData(() => instance.getExtraData());
  return {
    ...instance,
    getExtraData,
    createEncryptedInput,
    publicDecrypt: (handles, options) => instance.publicDecrypt(handles, { timeout: publicDecryptTimeoutMs, ...options }),
  };
}

export async function deployContract(
  contractName: string,
  deployer: ethers.Signer,
  ...constructorArgs: unknown[]
): Promise<ethers.Contract> {
  const artifact = await hardhatEthers.getContractFactory(contractName);
  const factory = new ethers.ContractFactory(artifact.interface, artifact.bytecode, deployer);
  const contract = await factory.deploy(...constructorArgs, { gasLimit: 10_000_000 });
  await contract.waitForDeployment();
  return contract as ethers.Contract;
}

export async function evmSnapshot(provider: { send: (method: string, params: unknown[]) => Promise<unknown> }): Promise<string> {
  return provider.send('evm_snapshot', []) as Promise<string>;
}

export async function evmRevert(
  provider: { send: (method: string, params: unknown[]) => Promise<unknown> },
  snapshotId: string,
): Promise<boolean> {
  return provider.send('evm_revert', [snapshotId]) as Promise<boolean>;
}
