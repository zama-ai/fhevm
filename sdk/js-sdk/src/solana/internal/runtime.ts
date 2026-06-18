import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { getFhevmRuntimeConfig, hasFhevmRuntimeConfig } from './config.js';
import { solanaEthereumModule } from './ethereum.js';
import { relayerModule } from '../../core/modules/relayer/module/index.js';
import { createFhevmRuntime } from './solana-p.js';

////////////////////////////////////////////////////////////////////////////////

let solanaFhevmRuntime: FhevmRuntime | undefined;

////////////////////////////////////////////////////////////////////////////////

export function getSolanaRuntime(): FhevmRuntime {
  if (!hasFhevmRuntimeConfig()) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  const em = solanaEthereumModule();
  const rm = relayerModule();

  solanaFhevmRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    relayer: rm.relayer,
    config: getFhevmRuntimeConfig(),
  });

  return solanaFhevmRuntime;
}
