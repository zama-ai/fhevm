import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { ensureFrozenContext } from '../../frozenContext/ensureFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initDecrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'decrypt');

  const frozen = await ensureFrozenContext(f);

  await f.runtime.decrypt.initTkmsModule({ tkmsVersion: frozen.tkmsVersion });
}
