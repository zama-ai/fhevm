import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import { assertFheEncryptionKeyWasmOwnedBy } from './FheEncryptionKeyWasm-p.js';
import {
  type FheEncryptionCrsBytes,
  type FheEncryptionKeyWasm,
  type FheEncryptionKeyBytes,
  type FheEncryptionKeyMetadata,
  type FheEncryptionPublicKeyBytes,
} from '../types/fheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionKeyWasm(
  context: { readonly runtime: WithEncrypt },
  parameters: FheEncryptionKeyWasm,
): Promise<FheEncryptionKeyBytes> {
  assertFheEncryptionKeyWasmOwnedBy(parameters, context.runtime);

  const publicKeyBytes: FheEncryptionPublicKeyBytes = await context.runtime.encrypt.serializeFheEncryptionPublicKey({
    publicKey: parameters.publicKey,
  });

  const crsBytes: FheEncryptionCrsBytes = await context.runtime.encrypt.serializeFheEncryptionCrs({
    crs: parameters.crs,
  });

  const metadata: FheEncryptionKeyMetadata = Object.freeze({
    ...parameters.metadata,
  });

  return Object.freeze({
    publicKeyBytes: publicKeyBytes,
    crsBytes: crsBytes,
    metadata,
  });
}
