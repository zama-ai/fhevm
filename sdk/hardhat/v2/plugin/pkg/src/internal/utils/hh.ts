import type { HardhatRuntimeEnvironment } from 'hardhat/types';
import type { FhevmProvider } from '../types';

import { HardhatFhevmError } from '../../error';
import constants from '../constants';

/**
 * Local replacements for the two `@fhevm/mock-utils` provider helpers this file used.
 * Migration step 6 decides whether they stay here or move somewhere shared.
 */

/** The connected chain id, or `undefined` when the provider cannot be reached. */
async function connectedChainId(provider: {
  send(method: string, params: unknown[]): Promise<unknown>;
}): Promise<number | undefined> {
  try {
    const chainIdHex = (await provider.send('eth_chainId', [])) as string;
    return Number(BigInt(chainIdHex));
  } catch {
    // No network connection, or the method is unsupported. The caller decides what that means.
    return undefined;
  }
}

/**
 * Probes `hardhat_metadata`, which only a Hardhat node answers. `couldNotConnect` is kept distinct
 * from `isHardhat: false` because the caller treats "unreachable" and "reachable but not Hardhat"
 * differently.
 */
async function isHardhatProvider(provider: {
  send(method: string, params: unknown[]): Promise<unknown>;
}): Promise<
  | { couldNotConnect: true; isHardhat?: undefined; chainId?: undefined }
  | { couldNotConnect: false; isHardhat: true; chainId: number }
  | { couldNotConnect: false; isHardhat: false }
> {
  let metadata: unknown;
  try {
    metadata = await provider.send('hardhat_metadata', []);
  } catch {
    return { couldNotConnect: true };
  }

  if (typeof metadata !== 'object' || metadata === null) {
    return { couldNotConnect: false, isHardhat: false };
  }
  const m = metadata as Record<string, unknown>;
  if (m.chainId !== constants.DEVELOPMENT_NETWORK_CHAINID) {
    return { couldNotConnect: false, isHardhat: false };
  }
  if (typeof m.instanceId !== 'string' || m.instanceId.length !== 66) {
    return { couldNotConnect: false, isHardhat: false };
  }

  return { couldNotConnect: false, isHardhat: true, chainId: m.chainId };
}

/**
 * Validates the current `HardhatRuntimeEnvironment` hre object to ensure that
 * essential Hardhat plugins and provider bindings are correctly configured.
 *
 * Specifically:
 * - Verifies that the `@nomicfoundation/hardhat-ethers` plugin is loaded.
 * - Checks consistency between `hre.ethers.provider` and `hre.network.provider`.
 *
 * @param hre - The `HardhatRuntimeEnvironment` object
 * @throws Will throw an error if:
 * - The `@nomicfoundation/hardhat-ethers` plugin is not loaded.
 * - The `hre.ethers.provider` object is inconsistent with the `hre.network.provider` object.
 */
export function checkHardhatRuntimeEnvironment(hre: HardhatRuntimeEnvironment): void {
  const { ethers } = hre as { ethers?: { provider?: unknown } | null };

  if (ethers === undefined || ethers === null) {
    throw new HardhatFhevmError(
      `Missing "@nomicfoundation/hardhat-ethers" plugin. Make sure the "@nomicfoundation/hardhat-ethers" plugin is properly initialized in the hardhat config file.`,
    );
  }

  if (ethers.provider === undefined || ethers.provider === null) {
    throw new HardhatFhevmError(
      `Unexpected "@nomicfoundation/hardhat-ethers" plugin. Unable to access the 'provider' property.`,
    );
  }

  const { _hardhatProvider: hardhatProvider } = ethers.provider as { _hardhatProvider?: unknown };
  if (hardhatProvider === undefined || hardhatProvider === null) {
    return; // wrong version, or no longer exposed
  }

  /**
   * see https://github.com/NomicFoundation/hardhat/blob/d77ecabb19e31f010dc9da2c023253b9da41c147/packages/hardhat-ethers/src/internal/index.ts#L26
   */
  if (hardhatProvider !== hre.network.provider) {
    throw new HardhatFhevmError(`hre.ethers.provider._hardhatProvider !== hre.network.provider`);
  }
}

export async function resolveNetworkConfigChainId(
  hre: HardhatRuntimeEnvironment,
  useEthChainId: boolean,
): Promise<number> {
  if (hre.network.config.chainId === undefined) {
    const chainId: number | undefined = useEthChainId ? await connectedChainId(hre.ethers.provider) : undefined;
    if (chainId === undefined) {
      // No network connection
      if (hre.network.name === 'localhost') {
        return constants.DEVELOPMENT_NETWORK_CHAINID;
      }
      throw new HardhatFhevmError(`Unable to resolve network chainId. Network name: ${hre.network.name}`);
    }
    return chainId;
  }

  return hre.network.config.chainId;
}

export async function getWeb3ClientVersion(provider: FhevmProvider): Promise<unknown> {
  return await provider.send('web3_clientVersion');
}

export async function isHardhatNode(
  networkName: string,
  chainId: number | undefined,
  provider: FhevmProvider,
): Promise<boolean> {
  if (networkName !== 'localhost') {
    return false;
  }

  const res = await isHardhatProvider(provider);
  if (res.couldNotConnect) {
    // If all the conditions are met:
    // - we cannot connect to provider
    // - the network is `localhost`
    // - the chainId is 31337
    // Then we assume that the provider we want to connect to must be the Hardhat Node.
    return chainId === constants.DEVELOPMENT_NETWORK_CHAINID;
  }

  if (!res.isHardhat) {
    return false;
  }

  // We try to connect to a hardhat runtime with the wrong chainId
  // Hardhat node chainId is always 31337
  if (res.chainId !== constants.DEVELOPMENT_NETWORK_CHAINID) {
    return false;
  }

  return chainId === undefined || res.chainId === chainId;
}

export async function checkSupportedNetwork(hre: HardhatRuntimeEnvironment): Promise<boolean> {
  if (hre.network.name === 'hardhat') {
    return true;
  }

  if (await isHardhatNode(hre.network.name, hre.network.config.chainId, hre.ethers.provider)) {
    return true;
  }

  if (hre.network.name === 'localhost') {
    throw new HardhatFhevmError(
      `Unsupported network: The fhevm hardhat plugin only supports the default 'localhost' hardhat node with chainId=${constants.DEVELOPMENT_NETWORK_CHAINID}. Got network 'localhost' with chainId=${hre.network.config.chainId} instead.`,
    );
  }

  throw new HardhatFhevmError(
    `Unsupported network: The fhevm hardhat plugin only supports the 'hardhat' network or the 'localhost' hardhat node with chainId=${constants.DEVELOPMENT_NETWORK_CHAINID}. Got network '${hre.network.name}' with chainId=${hre.network.config.chainId} instead.`,
  );
}
