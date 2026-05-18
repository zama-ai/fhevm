import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmDecryptClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { decryptActions } from './decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmDecryptClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmDecryptClient<chain, WithDecrypt, provider> {
  const c = createFhevmBaseClient(parameters);
  return c.extend(decryptActions);
}
