import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { getFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './config.js';
import { cleartextEthereumModule } from './ethereum-ct.js';
import { relayerModule as cleartextRelayerModule } from '../../core/modules/relayer/cleartext/mock.js';
import { createFhevmRuntime } from './viem-p.js';

////////////////////////////////////////////////////////////////////////////////

let viemFhevmRuntime: FhevmRuntime | undefined;

////////////////////////////////////////////////////////////////////////////////

export function getCleartextViemRuntime(): FhevmRuntime {
  if (!hasFhevmRuntimeConfig()) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  const em = cleartextEthereumModule();
  const rm = cleartextRelayerModule();

  viemFhevmRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    relayer: rm.relayer,
    config: getFhevmRuntimeConfig(),
  });

  return viemFhevmRuntime;
}
