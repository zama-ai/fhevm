import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient, type FhevmViemEncryptOptions } from './createFhevmBaseClient.js';
import { encryptActions } from './decorators/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptClient<chain extends FhevmChain, publicClient extends PublicClient>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmViemEncryptOptions | undefined;
}): FhevmEncryptClient<chain, WithEncrypt, publicClient> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptActions);
}
