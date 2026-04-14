import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FheEncryptionKeyWasm, FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { createFheEncryptionKeyWasm } from './FheEncryptionKeyWasm-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionKey(
  context: { readonly runtime: WithEncrypt },
  parameters: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyWasm> {
  const publicKeyNative = await context.runtime.encrypt.deserializeFheEncryptionPublicKey({
    publicKeyBytes: parameters.publicKeyBytes,
  });

  const crsNative = await context.runtime.encrypt.deserializeFheEncryptionCrs({
    crsBytes: parameters.crsBytes,
  });

  return createFheEncryptionKeyWasm(new WeakRef(context.runtime), {
    publicKey: publicKeyNative,
    crs: crsNative,
    metadata: parameters.metadata,
  });
}
