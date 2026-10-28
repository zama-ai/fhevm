import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncryptTransci } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptTransciClient } from '../../core/types/fhevmClient.js';
import { createFhevmCleartextBaseClient } from './createFhevmCleartextBaseClient.js';
import { encryptTransciActions } from './decorators/encryptTransci.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextEncryptTransciClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions | undefined;
}): FhevmEncryptTransciClient<chain, WithEncryptTransci, publicClient> {
  const c = createFhevmCleartextBaseClient(parameters);

  return c.extend(encryptTransciActions);
}
