import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FheEncryptionKeyWasm, FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';
import { createFheEncryptionKeyWasm } from './FheEncryptionKeyWasm-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionKey(
  context: {
    readonly runtime: WithEncrypt;
    readonly tfheVersion: TfheVersion;
  },
  parameters: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyWasm> {
  const publicKeyNative = await context.runtime.encrypt.deserializeFheEncryptionPublicKey({
    publicKeyBytes: parameters.publicKeyBytes,
    tfheVersion: context.tfheVersion,
  });

  const crsNative = await context.runtime.encrypt.deserializeFheEncryptionCrs({
    crsBytes: parameters.crsBytes,
    tfheVersion: context.tfheVersion,
  });

  return createFheEncryptionKeyWasm(new WeakRef(context.runtime), {
    publicKey: publicKeyNative,
    crs: crsNative,
    metadata: parameters.metadata,
    tfheVersion: context.tfheVersion,
  });
}
