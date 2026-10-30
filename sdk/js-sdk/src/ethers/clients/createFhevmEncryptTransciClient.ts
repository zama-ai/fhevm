import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncryptTransci } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptTransciClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptTransciActions } from './decorators/encryptTransci.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptTransciClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions<EthersT.ContractRunner> | undefined;
}): FhevmEncryptTransciClient<chain, WithEncryptTransci, provider> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptTransciActions);
}
