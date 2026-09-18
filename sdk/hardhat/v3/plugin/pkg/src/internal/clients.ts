// viem clients over hardhat 3's in-process EIP-1193 provider — the same construction as
// @nomicfoundation/hardhat-viem: `custom()` transport, and on a development node the fast-polling,
// uncached, no-retry defaults that make an automining chain feel instant.

import type { EthereumProvider } from 'hardhat/types/providers';
import {
  type Account,
  type Chain,
  type PublicClient,
  type Transport,
  type WalletClient,
  createPublicClient,
  createWalletClient,
  custom,
} from 'viem';
import { hardhat } from 'viem/chains';

const DEVELOPMENT_CLIENT_PARAMS = { pollingInterval: 50, cacheTime: 0 } as const;
const DEVELOPMENT_TRANSPORT_PARAMS = { retryCount: 0 } as const;

/** A development chain for viem: hardhat's preset, id taken from the node so anvil fits too. */
export async function developmentChain(provider: EthereumProvider): Promise<Chain> {
  const chainId: unknown = await provider.request({ method: 'eth_chainId' });
  if (typeof chainId !== 'string') throw new Error(`eth_chainId returned ${String(chainId)}`);
  return { ...hardhat, id: Number(BigInt(chainId)) };
}

export function developmentPublicClient(provider: EthereumProvider, chain: Chain): PublicClient {
  return createPublicClient({
    chain,
    transport: custom(provider, DEVELOPMENT_TRANSPORT_PARAMS),
    ...DEVELOPMENT_CLIENT_PARAMS,
  });
}

export function developmentWalletClient(
  provider: EthereumProvider,
  chain: Chain,
  account: Account,
): WalletClient<Transport, Chain, Account> {
  return createWalletClient({
    account,
    chain,
    transport: custom(provider, DEVELOPMENT_TRANSPORT_PARAMS),
    ...DEVELOPMENT_CLIENT_PARAMS,
  });
}
