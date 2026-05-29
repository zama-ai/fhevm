import type { FhevmChain } from '@fhevm/sdk/chains';
import { createPublicClient, http, type PublicClient, type Transport, type Chain } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import { mainnet as viemMainnet, sepolia as viemSepolia, anvil as viemAnvil } from 'viem/chains';
import { isCleartext, prepareChains, type FheTestBaseEnv, type FheTestChainName } from './setupCommon.js';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type FheTestViemConfig = {
  readonly chainName: FheTestChainName;
  readonly fhevmChain: FhevmChain;
  readonly publicClient: PublicClient<Transport, Chain>;
  readonly account: ReturnType<typeof mnemonicToAccount>;
  readonly alice: {
    readonly account: ReturnType<typeof mnemonicToAccount>;
  };
  readonly bob: {
    readonly account: ReturnType<typeof mnemonicToAccount>;
  };
  readonly zamaApiKey: string;
  readonly fheTestAddress: string;
};

export type CreateViemClientFn = (params: {
  chain: FheTestViemConfig['fhevmChain'];
  publicClient: FheTestViemConfig['publicClient'];
}) => any;

// ---------------------------------------------------------------------------
// Build config
// ---------------------------------------------------------------------------

function _buildConfigs(): FheTestViemConfig[] {
  const envs: FheTestBaseEnv[] = prepareChains();
  return envs.map(_buildConfig);
}

function _buildConfig(env: FheTestBaseEnv): FheTestViemConfig {
  const viemChain =
    env.chainName === 'mainnet'
      ? viemMainnet
      : env.chainName === 'sepolia' || env.chainName === 'devnet' || env.chainName === 'testnet'
        ? viemSepolia
        : env.chainName.startsWith('localstack')
          ? { ...viemAnvil, id: env.fhevmChain.id }
          : viemAnvil;

  const account = mnemonicToAccount(env.mnemonic);
  const bobAccount = mnemonicToAccount(env.mnemonic, {
    path: "m/44'/60'/0'/0/1",
  });

  const publicClient: PublicClient<Transport, Chain> = createPublicClient({
    chain: viemChain,
    transport: http(env.rpcUrl),
  });

  return {
    chainName: env.chainName,
    fhevmChain: env.fhevmChain,
    publicClient,
    account,
    alice: {
      account,
    },
    bob: {
      account: bobAccount,
    },
    zamaApiKey: env.zamaApiKey,
    fheTestAddress: env.fheTestAddress,
  };
}

// ---------------------------------------------------------------------------
// Singleton — built once, shared across all test files
// ---------------------------------------------------------------------------

let _configs: FheTestViemConfig[] | undefined;

export function getViemTestConfigs(): FheTestViemConfig[] {
  if (_configs === undefined) {
    _configs = _buildConfigs();
  }
  return _configs;
}

export function getViemTestConfig(): FheTestViemConfig {
  return getViemTestConfigs()[0]!;
}

export function areAllViemTestConfigsCleartext(): boolean {
  return getViemTestConfigs().every((config) => isCleartext(config.chainName));
}

export function isMultichain(): boolean {
  return getViemTestConfigs().length > 1;
}
