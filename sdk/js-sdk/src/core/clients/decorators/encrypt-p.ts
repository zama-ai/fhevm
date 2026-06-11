import type { FhevmBase } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  asFhevmClientWith,
  resolveFhevmProtocolVersion,
  resolveFhevmTfheVersion,
  setResolvedProtocolVersion,
  setResolvedTfheVersion,
} from '../../runtime/CoreFhevm-p.js';
import { fetchFheEncryptionKeyBytes } from '../../key/fetchFheEncryptionKeyBytes.js';

////////////////////////////////////////////////////////////////////////////////

export async function _initEncrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encrypt');

  const protocolVersion = await resolveFhevmProtocolVersion(f);
  setResolvedProtocolVersion(f, protocolVersion);

  const tfheVersion = await resolveFhevmTfheVersion(f);

  await Promise.all([
    // Prefetch the global FheEncryptionKey in bytes format
    fetchFheEncryptionKeyBytes(f, {}),
    f.runtime.encrypt.initTfheModule({ tfheVersion }),
  ]);

  setResolvedTfheVersion(f, tfheVersion);
}
