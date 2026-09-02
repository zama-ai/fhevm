// Which kind of node a connection talks to. Hardhat 3 already tells in-process EDR apart from a
// remote node through `networkConfig.type`; everything remote is classified by its LIVE chain id
// (read from the node, never trusted from the config), with `@fhevm/sdk/chains` as the source of
// the public ids. Detection never gates on the network name: users rename freely, and the node task
// serves a network called `node`.

import { mainnet, polygon, polygonAmoy, sepolia } from '@fhevm/sdk/chains';
import { HardhatPluginError } from 'hardhat/plugins';
import type { NetworkConfig } from 'hardhat/types/config';
import type { NetworkConnection } from 'hardhat/types/network';
import type { EthereumProvider } from 'hardhat/types/providers';

import { DEVELOPMENT_CHAIN_ID, PLUGIN_ID } from './constants.js';

export type FhevmNetworkKind =
  /** In-process EDR chain: ours to prepare. */
  | 'hardhat'
  /** A remote development node on the development chain id: `hardhat node` or anvil. */
  | 'localhost'
  | 'sepolia'
  | 'mainnet'
  | 'polygon'
  | 'polygon-amoy'
  | 'unknown';

export type FhevmNetworkInfo = {
  readonly networkName: string;
  readonly chainId: number;
  readonly kind: FhevmNetworkKind;
  /** The remote node's URL; undefined in process. */
  readonly url: string | undefined;
};

export async function resolveFhevmNetwork(connection: NetworkConnection<string>): Promise<FhevmNetworkInfo> {
  const { networkName, networkConfig, provider } = connection;
  const chainId = await readChainId(provider);
  assertConfiguredChainId(networkName, networkConfig, chainId);
  return {
    networkName,
    chainId,
    kind: resolveKind(networkConfig.type, chainId),
    url: networkConfig.type === 'http' ? await networkConfig.url.getUrl() : undefined,
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
  return info.kind !== 'hardhat' && info.kind !== 'localhost' && info.kind !== 'unknown';
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

function resolveKind(type: NetworkConfig['type'], chainId: number): FhevmNetworkKind {
  if (type === 'edr-simulated') return 'hardhat';
  if (chainId === DEVELOPMENT_CHAIN_ID) return 'localhost';
  if (chainId === mainnet.id) return 'mainnet';
  if (chainId === sepolia.id) return 'sepolia';
  if (chainId === polygon.id) return 'polygon';
  if (chainId === polygonAmoy.id) return 'polygon-amoy';
  return 'unknown';
}
