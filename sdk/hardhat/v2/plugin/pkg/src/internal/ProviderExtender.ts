import type { EIP1193Provider, HardhatConfig, ProviderExtender } from 'hardhat/types';

import { FhevmProviderExtender } from './provider/FhevmProviderExtender';

/**
 * Hardhat ProviderExtender
 * Called at Hardhat initialization
 *
 * The startup `eth_blockNumber` round-trip that used to happen here is gone with the JS mock engine:
 * it only existed to seed the block number of the in-memory cleartext DB.
 */
export const providerExtender: ProviderExtender = async (
  provider: EIP1193Provider,
  config: HardhatConfig,
  network: string,
  // eslint-disable-next-line @typescript-eslint/require-await
) => {
  return new FhevmProviderExtender(provider, config, network);
};
