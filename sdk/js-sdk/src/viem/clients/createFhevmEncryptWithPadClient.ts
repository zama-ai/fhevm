import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncryptWithPad } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptWithPadClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptWithPadActions } from './decorators/encryptWithPad.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptWithPadClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions | undefined;
}): FhevmEncryptWithPadClient<chain, WithEncryptWithPad, publicClient> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptWithPadActions);
}
