// The @fhevm/sdk client behind `connection.fhevm`, created once per connection inside `newConnection`
// (the client's `ready` is async and the public getter is not). A development connection gets the
// cleartext client over the stack it just prepared: no relayer, no WASM. Public networks wait on the
// network-group decision (which gateway serves a chain listed under two) before they get a client.

import type { Deployed } from '@fhevm/host-contracts-cleartext/ts';
import { hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createFhevmCleartextClient } from '@fhevm/sdk/viem/cleartext';
import type { NetworkConnection } from 'hardhat/types/network';

import type { FhevmClient, FhevmNetworkInfo } from '../types.js';
import { cleartextChain } from './chains.js';
import { developmentChain, developmentPublicClient } from './clients.js';

export async function createSdkClient(
  connection: NetworkConnection<string>,
  network: FhevmNetworkInfo,
  stack: Deployed | undefined,
): Promise<FhevmClient | undefined> {
  if (stack === undefined) return undefined;
  ensureRuntimeConfig();
  const chain = await developmentChain(connection.provider);
  const publicClient = developmentPublicClient(connection.provider, chain);
  const client = createFhevmCleartextClient({ publicClient, chain: cleartextChain(network.chainId, stack) });
  // Resolves the on-chain context every action needs; without it the first call fails.
  await client.ready;
  return client;
}

// A process-wide singleton in the SDK, which refuses to build a client until it is set. Relayer auth
// (the API key) joins here when public networks land.
function ensureRuntimeConfig(): void {
  if (hasFhevmRuntimeConfig()) return;
  setFhevmRuntimeConfig({});
}
