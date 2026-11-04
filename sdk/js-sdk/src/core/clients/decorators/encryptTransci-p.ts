import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { ensureFrozenContext } from '../../frozenContext/ensureFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initEncryptTransci(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encryptTransci');

  await ensureFrozenContext(f);

  await f.runtime.encryptTransci.initTkmsModule();
}
