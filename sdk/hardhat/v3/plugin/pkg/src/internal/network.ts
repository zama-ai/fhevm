// Which kind of node a connection talks to. Hardhat 3 already tells in-process EDR apart from a
// remote node through `networkConfig.type`; everything remote is classified by its LIVE chain id
// (read from the node, never trusted from the config) against the generated chain constants —
// every deployed fhevm host chain, by network group. Detection never gates on the network name:
// users rename freely, and the node task serves a network called `node`.

import { HardhatPluginError } from 'hardhat/plugins';
import type { NetworkConfig } from 'hardhat/types/config';
import type { NetworkConnection } from 'hardhat/types/network';
import type { EthereumProvider } from 'hardhat/types/providers';

import type { FhevmNetworkInfo, FhevmNetworkKind, FhevmPublicChain } from '../types.js';
import { DEVELOPMENT_CHAIN_ID, PLUGIN_ID } from './constants.js';
import { FHEVM_CHAINS, type FhevmNetworkGroup, type FhevmNetworkGroupConstants } from './vendored/fhevm-chains.js';

export async function resolveFhevmNetwork(connection: NetworkConnection<string>): Promise<FhevmNetworkInfo> {
  const { networkName, networkConfig, provider } = connection;
  const chainId = await readChainId(provider);
  assertConfiguredChainId(networkName, networkConfig, chainId);
  const publicChains = publicChainsWithId(chainId);
  return {
    networkName,
    chainId,
    kind: resolveKind(networkConfig.type, chainId, publicChains),
    url: networkConfig.type === 'http' ? await networkConfig.url.getUrl() : undefined,
    publicChains,
  };
}

/** A development node: the plugin may deploy the cleartext stack onto it. */
export function isDevelopmentNetwork(info: FhevmNetworkInfo): boolean {
  return info.kind === 'hardhat' || info.kind === 'localhost';
}

/** A node the SDK talks to in cleartext mode — today exactly the development nodes. */
export function isCleartextNetwork(info: FhevmNetworkInfo): boolean {
  return isDevelopmentNetwork(info);
}

/** A public network served by the real relayer: never deployed to. */
export function isPublicNetwork(info: FhevmNetworkInfo): boolean {
  return info.kind === 'public';
}

/** The registry host chains carrying `chainId`, in network-group order. */
export function publicChainsWithId(chainId: number): FhevmPublicChain[] {
  const groups = Object.entries(FHEVM_CHAINS) as Array<[FhevmNetworkGroup, FhevmNetworkGroupConstants]>;
  return groups.flatMap(([group, constants]) =>
    Object.values(constants.hosts)
      .filter((host) => host.id === chainId)
      .map((host) => ({ group, host })),
  );
}

async function readChainId(provider: EthereumProvider): Promise<number> {
  const hex: unknown = await provider.request({ method: 'eth_chainId' });
  if (typeof hex !== 'string') throw new HardhatPluginError(PLUGIN_ID, `eth_chainId returned ${String(hex)}`);
  return Number(BigInt(hex));
}

// A configured chain id that disagrees with the node is a misconfiguration, not something to guess
// around: the deploy and the SDK would otherwise target the wrong chain.
function assertConfiguredChainId(networkName: string, networkConfig: NetworkConfig, chainId: number): void {
  const configured = networkConfig.chainId;
  if (configured === undefined || configured === chainId) return;
  throw new HardhatPluginError(
    PLUGIN_ID,
    `Network '${networkName}' is configured with chainId ${String(configured)}, but the node reports ${String(chainId)}.`,
  );
}

function resolveKind(
  type: NetworkConfig['type'],
  chainId: number,
  publicChains: readonly FhevmPublicChain[],
): FhevmNetworkKind {
  if (type === 'edr-simulated') return 'hardhat';
  if (chainId === DEVELOPMENT_CHAIN_ID) return 'localhost';
  return publicChains.length > 0 ? 'public' : 'unknown';
}
