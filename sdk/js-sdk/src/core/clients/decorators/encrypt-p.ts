import type { FhevmBase, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { fetchFheEncryptionKeyBytes } from '../../key/fetchFheEncryptionKeyBytes.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initEncrypt(
  fhevm: FhevmBase<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>,
): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encrypt');
  await Promise.all([
    // Prefetch the global FheEncryptionKey in bytes format
    fetchFheEncryptionKeyBytes(f, {}),
    f.runtime.encrypt.initTfheModule(),
  ]);
}
