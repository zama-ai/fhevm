import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { fetchFheEncryptionKeyBytes } from '../../key/fetchFheEncryptionKeyBytes.js';
import { ensureFrozenContext } from '../../frozenContext/ensureFrozenContext-p.js';
import { cloneFhevmClientFrozenContext } from '../../frozenContext/FhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initEncrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encrypt');

  const frozen = await ensureFrozenContext(f);

  await Promise.all([
    // Prefetch the global FheEncryptionKey in bytes format
    fetchFheEncryptionKeyBytes(f, { fhevmContext: cloneFhevmClientFrozenContext(frozen) }),
    f.runtime.encrypt.initTfheModule({ tfheVersion: frozen.tfheVersion }),
  ]);
}
