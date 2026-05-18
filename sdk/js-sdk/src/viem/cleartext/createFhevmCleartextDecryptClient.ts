import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmDecryptClient } from '../../core/types/fhevmClient.js';
import { createFhevmCleartextBaseClient } from './createFhevmCleartextBaseClient.js';
import { decryptActions } from './decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextDecryptClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmDecryptClient<chain, WithDecrypt, publicClient> {
  const c = createFhevmCleartextBaseClient(parameters);
  return c.extend(decryptActions);
}
