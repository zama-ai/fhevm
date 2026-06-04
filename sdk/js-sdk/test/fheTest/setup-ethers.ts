import type { FhevmChain } from '@fhevm/sdk/chains';
import type { WasmModuleVersions } from '../../src/core/types/coreFhevmRuntime.js';
import type { FheTestBaseEnv, FheTestChainName } from './setupCommon.js';
import { ethers } from 'ethers';
import { FHETestABI } from './FheTest-abi-v2.js';
import { isCleartext, prepareChains } from './setupCommon.js';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type FheTestEthersConfig = {
  readonly chainName: FheTestChainName;
  readonly fhevmChain: FhevmChain;
  readonly wallet: ethers.HDNodeWallet;
  readonly signer: ethers.Signer;
  readonly alice: {
    readonly wallet: ethers.HDNodeWallet;
    readonly signer: ethers.Signer;
  };
  readonly bob: {
    readonly wallet: ethers.HDNodeWallet;
    readonly signer: ethers.Signer;
  };
  readonly provider: ethers.JsonRpcProvider;
  readonly zamaApiKey: string;
  readonly fheTestAddress: string;
  readonly fheTestContract: ethers.Contract;
  readonly fheEncryptionKeyTfheVersion: string;
  readonly moduleVersions?: WasmModuleVersions | undefined;
};

export type CreateEthersClientFn = (params: {
  chain: FheTestEthersConfig['fhevmChain'];
  provider: FheTestEthersConfig['provider'];
}) => any;

// ---------------------------------------------------------------------------
// Build config
// ---------------------------------------------------------------------------

function _buildConfigs(): FheTestEthersConfig[] {
  const envs: FheTestBaseEnv[] = prepareChains();
  return envs.map(_buildConfig);
}

function _buildConfig(env: FheTestBaseEnv): FheTestEthersConfig {
  const provider = new ethers.JsonRpcProvider(env.rpcUrl);
  const wallet = ethers.HDNodeWallet.fromMnemonic(ethers.Mnemonic.fromPhrase(env.mnemonic));

  const bobWallet = ethers.HDNodeWallet.fromMnemonic(ethers.Mnemonic.fromPhrase(env.mnemonic), "m/44'/60'/0'/0/1");

  // Use a ethers.NonceManager to avoid nonce issues in parallel mode
  const signer = new ethers.NonceManager(wallet.connect(provider));

  const fheTestContract = new ethers.Contract(env.fheTestAddress, FHETestABI, signer);

  // Use a ethers.NonceManager to avoid nonce issues in parallel mode
  const bobSigner = new ethers.NonceManager(bobWallet.connect(provider));

  return {
    chainName: env.chainName,
    fhevmChain: env.fhevmChain,
    wallet,
    signer,
    alice: {
      wallet,
      signer,
    },
    bob: {
      wallet: bobWallet,
      signer: bobSigner,
    },
    provider,
    zamaApiKey: env.zamaApiKey,
    fheTestAddress: env.fheTestAddress,
    fheTestContract,
    fheEncryptionKeyTfheVersion: env.fheEncryptionKeyTfheVersion,
    moduleVersions: env.moduleVersions,
  };
}

// ---------------------------------------------------------------------------
// Singleton — built once, shared across all test files
// ---------------------------------------------------------------------------

let _configs: FheTestEthersConfig[] | undefined;

export function getEthersTestConfigs(): FheTestEthersConfig[] {
  if (_configs === undefined) {
    _configs = _buildConfigs();
  }
  return _configs;
}

export function getEthersTestConfig(): FheTestEthersConfig {
  return getEthersTestConfigs()[0]!;
}

export function areAllEthersTestConfigsCleartext(): boolean {
  return getEthersTestConfigs().every((config) => isCleartext(config.chainName));
}

export function isMultichain(): boolean {
  return getEthersTestConfigs().length > 1;
}
