import type { FhevmChain } from '@fhevm/sdk/chains';
import { createPublicClient, http, type PublicClient, type Transport, type Chain } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import { sepolia as viemSepolia, mainnet as viemMainnet, anvil as viemAnvil } from 'viem/chains';
import { getBaseEnv, type FheTestBaseEnv, type FheTestChainName } from './setupCommon.js';

// Re-export for convenience
export type { FheTestChainName } from './setupCommon.js';

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

// ---------------------------------------------------------------------------
// Build config
// ---------------------------------------------------------------------------

function buildConfig(): FheTestViemConfig {
  const env: FheTestBaseEnv = getBaseEnv();

  const viemChain =
    env.chainName === 'sepolia' || env.chainName === 'devnet'
      ? viemSepolia
      : env.chainName === 'mainnet'
        ? viemMainnet
        : env.chainName === 'localstack'
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

let _config: FheTestViemConfig | undefined;

export function getViemTestConfig(): FheTestViemConfig {
  if (_config === undefined) {
    _config = buildConfig();
  }
  return _config;
}
