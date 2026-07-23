import type { FhevmChain } from '@fhevm/sdk/chains';
import type {
  createFhevmBaseClient as createEthersFhevmBaseClient,
  createFhevmDecryptClient as createEthersFhevmDecryptClient,
  createFhevmEncryptClient as createEthersFhevmEncryptClient,
} from '@fhevm/sdk/ethers';
import type { FhevmDecryptOptions, FhevmEncryptOptions, FhevmOptions } from '../../src/core/types/coreFhevmClient.js';
import type { FhevmModuleVersions } from '../../src/core/types/moduleVersions.js';
import type { FheTestBaseEnv, FheTestChainName } from './setupCommon.js';
import { ethers } from 'ethers';
import { FHETestABI } from './FheTest-abi-v2.js';
import { isCleartext, prepareChains } from './setupCommon.js';

// ---------------------------------------------------------------------------
// FixedNonceManager
// ---------------------------------------------------------------------------
// ethers.js NonceManager.sendTransaction calls this.signer.populateTransaction(tx)
// *before* setting tx.nonce to the manager's value.  populateTransaction therefore
// does its own eth_getTransactionCount query and passes THAT nonce to eth_estimateGas.
// If the chain state changes between the two queries (e.g. the v12 coprocessor
// submitting a tx on Alice's behalf), eth_estimateGas receives a stale nonce and
// fails with "nonce too low".
//
// Fix: resolve the nonce first and pre-populate tx.nonce so that populateTransaction
// skips its own query and eth_estimateGas uses the correct nonce.
class FixedNonceManager extends ethers.NonceManager {
  override async sendTransaction(tx: ethers.TransactionRequest): Promise<ethers.TransactionResponse> {
    const nonce = await this.getNonce('pending');
    this.increment();
    tx = await this.signer.populateTransaction({ ...tx, nonce });
    return await this.signer.sendTransaction(tx);
  }
}

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
  readonly moduleVersions?: FhevmModuleVersions | undefined;
};

type CreateEthersClientParameters = {
  chain: FheTestEthersConfig['fhevmChain'];
  provider: FheTestEthersConfig['provider'];
};

export type CreateEthersBaseClientFn = (
  params: CreateEthersClientParameters & {
    readonly options?: FhevmOptions | undefined;
  },
) => ReturnType<typeof createEthersFhevmBaseClient>;

export type CreateEthersEncryptClientFn = (
  params: CreateEthersClientParameters & {
    readonly options?: FhevmEncryptOptions | undefined;
  },
) => ReturnType<typeof createEthersFhevmEncryptClient>;

export type CreateEthersDecryptClientFn = (
  params: CreateEthersClientParameters & {
    readonly options?: FhevmDecryptOptions | undefined;
  },
) => ReturnType<typeof createEthersFhevmDecryptClient>;

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

  // Use FixedNonceManager to avoid nonce issues in parallel mode and to prevent
  // the stale-nonce race described in the FixedNonceManager class above.
  const signer = new FixedNonceManager(wallet.connect(provider));

  const fheTestContract = new ethers.Contract(env.fheTestAddress, FHETestABI, signer);

  const bobSigner = new FixedNonceManager(bobWallet.connect(provider));

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

export function getEthersClientOptions(
  config: FheTestEthersConfig,
  moduleVersions: FhevmModuleVersions | undefined = config.moduleVersions,
): FhevmOptions | undefined {
  return moduleVersions === undefined ? undefined : { moduleVersions };
}

export function getEthersEncryptClientOptions(
  config: FheTestEthersConfig,
  moduleVersions: FhevmModuleVersions | undefined = config.moduleVersions,
): FhevmEncryptOptions | undefined {
  if (moduleVersions === undefined) {
    return undefined;
  }
  if (moduleVersions === 'auto') {
    return { moduleVersions };
  }
  if (moduleVersions.tfhe === undefined && moduleVersions.checkCompatibility === undefined) {
    return undefined;
  }
  return {
    moduleVersions: {
      tfhe: moduleVersions.tfhe,
      checkCompatibility: moduleVersions.checkCompatibility,
    },
  };
}

export function getEthersDecryptClientOptions(
  moduleVersions: FhevmModuleVersions | undefined,
): FhevmDecryptOptions | undefined {
  if (moduleVersions === undefined) {
    return undefined;
  }
  if (moduleVersions === 'auto') {
    return { moduleVersions };
  }
  if (moduleVersions.kms === undefined && moduleVersions.checkCompatibility === undefined) {
    return undefined;
  }
  return {
    moduleVersions: {
      kms: moduleVersions.kms,
      checkCompatibility: moduleVersions.checkCompatibility,
    },
  };
}
