import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith, getFheEncryptionKeyProvider } from '../../runtime/CoreFhevm-p.js';
import { fetchFheEncryptionKeyBytes } from '../../key/fetchFheEncryptionKeyBytes.js';
import { ensureFrozenContext } from '../../frozenContext/ensureFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initEncrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encrypt');

  const frozen = await ensureFrozenContext(f);

  await Promise.all([
    // Prefetch the global FheEncryptionKey in bytes format
    fetchFheEncryptionKeyBytes(getFheEncryptionKeyProvider(f)),
    f.runtime.encrypt.initTfheModule({ tfheVersion: frozen.tfheVersion }),
  ]);
}
