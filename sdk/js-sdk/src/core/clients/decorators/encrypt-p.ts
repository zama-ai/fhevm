import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { fetchFheEncryptionKeyBytes } from '../../key/fetchFheEncryptionKeyBytes.js';
import { hyperWasmResolveTfheModuleVersion } from '../../runtime/HyperWasmSolver-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initEncrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encrypt');
  const tfheVersion = await hyperWasmResolveTfheModuleVersion(f);

  await Promise.all([
    // Prefetch the global FheEncryptionKey in bytes format
    fetchFheEncryptionKeyBytes(f, {}),
    f.runtime.encrypt.initTfheModule({ tfheVersion }),
  ]);
}
