import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { relayerModule as cleartextRelayerModule } from '../../core/modules/relayer/cleartext/mock.js';
import { getFhevmRuntimeConfig, hasFhevmRuntimeConfig } from '../internal/config.js';
import { solanaEthereumModule } from '../internal/ethereum.js';
import { createFhevmRuntime } from '../internal/solana-p.js';

////////////////////////////////////////////////////////////////////////////////

let cleartextSolanaRuntime: FhevmRuntime | undefined;

////////////////////////////////////////////////////////////////////////////////

/** The Solana runtime whose relayer serves placeholder key material instead of the network's. */
export function getCleartextSolanaRuntime(): FhevmRuntime {
  if (!hasFhevmRuntimeConfig()) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  cleartextSolanaRuntime ??= createFhevmRuntime({
    ethereum: solanaEthereumModule().ethereum,
    relayer: cleartextRelayerModule().relayer,
    config: getFhevmRuntimeConfig(),
  });

  return cleartextSolanaRuntime;
}
