import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmOptions, Fhevm } from '../../core/types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { SolanaPublicDecryptActions } from './decorators/publicDecrypt.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { solanaPublicDecryptActions } from './decorators/publicDecrypt.js';

export type FhevmSolanaPublicDecryptClient<chain extends FhevmSolanaChain = FhevmSolanaChain> = Fhevm<
  undefined,
  FhevmRuntime,
  undefined
> & { readonly solanaChain: chain } & SolanaPublicDecryptActions;

/** Creates a signer-free client for requesting witness-bound public-decrypt certificate claims. */
export function createFhevmPublicDecryptClient<chain extends FhevmSolanaChain>(parameters: {
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmSolanaPublicDecryptClient<chain> {
  return createFhevmBaseClient(parameters).extend(solanaPublicDecryptActions);
}
