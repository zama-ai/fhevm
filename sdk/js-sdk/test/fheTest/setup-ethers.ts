import type { FhevmChain } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';
import { FHETestABI } from './FheTest-abi-v2.js';
import { prepareFheTestEnv, type FheTestBaseEnv, type FheTestChainName } from './setupCommon.js';

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
};

// ---------------------------------------------------------------------------
// Build config
// ---------------------------------------------------------------------------

function buildConfig(): FheTestEthersConfig {
  const env: FheTestBaseEnv = prepareFheTestEnv();

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
  };
}

// ---------------------------------------------------------------------------
// Singleton — built once, shared across all test files
// ---------------------------------------------------------------------------

let _config: FheTestEthersConfig | undefined;

export function getEthersTestConfig(): FheTestEthersConfig {
  if (_config === undefined) {
    _config = buildConfig();
  }
  return _config;
}
