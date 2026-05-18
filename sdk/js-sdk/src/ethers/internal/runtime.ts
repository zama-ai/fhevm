import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { getFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './config.js';
import { ethereumModule } from './ethereum.js';
import { relayerModule } from '../../core/modules/relayer/module/index.js';
import { createFhevmRuntime } from './ethers-p.js';

////////////////////////////////////////////////////////////////////////////////

let viemFhevmRuntime: FhevmRuntime | undefined;

////////////////////////////////////////////////////////////////////////////////

export function getEthersRuntime(): FhevmRuntime {
  if (!hasFhevmRuntimeConfig()) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  const em = ethereumModule();
  const rm = relayerModule();

  viemFhevmRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    relayer: rm.relayer,
    config: getFhevmRuntimeConfig(),
  });

  return viemFhevmRuntime;
}
