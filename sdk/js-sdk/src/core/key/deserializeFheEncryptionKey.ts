import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FheEncryptionKeyWasm, FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { createFheEncryptionKeyWasm } from './FheEncryptionKeyWasm-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionKey(
  context: {
    readonly runtime: WithEncrypt;
  },
  parameters: {
    readonly keyBytes: FheEncryptionKeyBytes;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<FheEncryptionKeyWasm> {
  const { keyBytes, fhevmContext } = parameters;
  const publicKeyNative = await context.runtime.encrypt.deserializeFheEncryptionPublicKey({
    publicKeyBytes: keyBytes.publicKeyBytes,
    tfheVersion: fhevmContext.tfheVersion,
  });

  const crsNative = await context.runtime.encrypt.deserializeFheEncryptionCrs({
    crsBytes: keyBytes.crsBytes,
    tfheVersion: fhevmContext.tfheVersion,
  });

  return createFheEncryptionKeyWasm(new WeakRef(context.runtime), {
    publicKey: publicKeyNative,
    crs: crsNative,
    metadata: keyBytes.metadata,
    tfheVersion: fhevmContext.tfheVersion,
  });
}
