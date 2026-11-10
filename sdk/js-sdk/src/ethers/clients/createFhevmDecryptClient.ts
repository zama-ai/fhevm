import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmDecryptClient } from '../../core/types/fhevmClient.js';
import type { FhevmDecryptOptions } from '../../core/types/coreFhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { decryptActions } from './decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmDecryptClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmDecryptOptions<EthersT.ContractRunner> | undefined;
}): FhevmDecryptClient<chain, WithDecrypt, provider> {
  const c = createFhevmBaseClient({
    ...parameters,
    // Decrypt clients never use the key: do not keep a copy of it.
    options: parameters.options === undefined ? undefined : { ...parameters.options, fheEncryptionKey: undefined },
  });
  return c.extend(decryptActions);
}
