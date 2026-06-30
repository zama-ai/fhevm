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
  context: {
    readonly runtime: WithEncrypt;
  },
  parameters: FheEncryptionKeyWasm,
): Promise<FheEncryptionKeyBytes> {
  const tfheVersion = parameters.tfheVersion;

  assertFheEncryptionKeyWasmOwnedBy(parameters, context.runtime, tfheVersion);

  const publicKeyBytes: FheEncryptionPublicKeyBytes = await context.runtime.encrypt.serializeFheEncryptionPublicKey({
    publicKey: parameters.publicKey,
    tfheVersion,
  });

  const crsBytes: FheEncryptionCrsBytes = await context.runtime.encrypt.serializeFheEncryptionCrs({
    crs: parameters.crs,
    tfheVersion,
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
