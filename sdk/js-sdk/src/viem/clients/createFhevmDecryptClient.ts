import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmDecryptClient } from '../../core/types/fhevmClient.js';
import type { FhevmDecryptOptions } from '../../core/types/coreFhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { decryptActions } from './decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmDecryptClient<chain extends FhevmChain, publicClient extends PublicClient>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmDecryptOptions<PublicClient> | undefined;
}): FhevmDecryptClient<chain, WithDecrypt, publicClient> {
  const c = createFhevmBaseClient({
    ...parameters,
    // Decrypt clients never use the key: do not keep a copy of it.
    options: parameters.options === undefined ? undefined : { ...parameters.options, fheEncryptionKey: undefined },
  });
  return c.extend(decryptActions);
}
