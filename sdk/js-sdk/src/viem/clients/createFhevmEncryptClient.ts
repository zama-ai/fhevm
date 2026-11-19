import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptClient } from '../../core/types/fhevmClient.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptActions } from './decorators/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptClient<chain extends FhevmChain, publicClient extends PublicClient>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions<PublicClient> | undefined;
}): FhevmEncryptClient<chain, WithEncrypt, publicClient> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptActions);
}
