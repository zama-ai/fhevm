import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { getFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './config.js';
import { cleartextEthereumModule } from './ethereum-ct.js';
import { relayerModule as cleartextRelayerModule } from '../../core/modules/relayer/cleartext/mock.js';
import { createFhevmRuntime } from './ethers-p.js';

////////////////////////////////////////////////////////////////////////////////

let ethersFhevmRuntime: FhevmRuntime | undefined;

////////////////////////////////////////////////////////////////////////////////

export function getCleartextEthersRuntime(): FhevmRuntime {
  if (!hasFhevmRuntimeConfig()) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  const em = cleartextEthereumModule();
  const rm = cleartextRelayerModule();

  ethersFhevmRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    relayer: rm.relayer,
    config: getFhevmRuntimeConfig(),
  });

  return ethersFhevmRuntime;
}
