import type { FhevmChain } from '@fhevm/sdk/chains';
import type {
  createFhevmBaseClient as createViemFhevmBaseClient,
  createFhevmDecryptClient as createViemFhevmDecryptClient,
  createFhevmEncryptClient as createViemFhevmEncryptClient,
  createFhevmEncryptWithPadClient as createViemFhevmEncryptWithPadClient,
} from '@fhevm/sdk/viem';
import type { FhevmDecryptOptions, FhevmEncryptOptions, FhevmOptions } from '../../src/core/types/coreFhevmClient.js';
import type { FheTestBaseEnv, FheTestChainName } from './setupCommon.js';
import { createPublicClient, http, type PublicClient, type Transport, type Chain } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import {
  mainnet as viemMainnet,
  sepolia as viemSepolia,
  anvil as viemAnvil,
  polygonAmoy as viemPolygonAmoy,
} from 'viem/chains';
import { isCleartext, prepareChains } from './setupCommon.js';

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
  readonly protocolVersion: FheTestBaseEnv['protocolVersion'];
  readonly fheEncryptionKeyTfheVersion: string;
};

type CreateViemClientParameters = {
  chain: FheTestViemConfig['fhevmChain'];
  publicClient: FheTestViemConfig['publicClient'];
};

export type CreateViemBaseClientFn = (
  params: CreateViemClientParameters & {
    readonly options?: FhevmOptions | undefined;
  },
) => ReturnType<typeof createViemFhevmBaseClient>;

export type CreateViemEncryptClientFn = (
  params: CreateViemClientParameters & {
    readonly options?: FhevmEncryptOptions | undefined;
  },
) => ReturnType<typeof createViemFhevmEncryptClient>;

export type CreateViemEncryptWithPadClientFn = (
  params: CreateViemClientParameters & {
    readonly options?: FhevmEncryptOptions | undefined;
  },
) => ReturnType<typeof createViemFhevmEncryptWithPadClient>;

export type CreateViemDecryptClientFn = (
  params: CreateViemClientParameters & {
    readonly options?: FhevmDecryptOptions | undefined;
  },
) => ReturnType<typeof createViemFhevmDecryptClient>;

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
        : env.chainName === 'polygon_devnet'
          ? viemPolygonAmoy
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
    protocolVersion: env.protocolVersion,
    fheEncryptionKeyTfheVersion: env.fheEncryptionKeyTfheVersion,
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

/**
 * The latest block's timestamp, in seconds. Permit start times are checked against the chain's clock, which on an
 * idle anvil lags the wall clock, so a permit built from `Date.now()` can look like it starts in the future.
 */
export async function latestBlockTimestamp(config: Pick<FheTestViemConfig, 'publicClient'>): Promise<number> {
  const block = await config.publicClient.getBlock();
  return Number(block.timestamp);
}
