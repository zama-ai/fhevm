import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncryptTransci } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptTransciClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptTransciActions } from './decorators/encryptTransci.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptTransciClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions<PublicClient> | undefined;
}): FhevmEncryptTransciClient<chain, WithEncryptTransci, publicClient> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptTransciActions);
}
