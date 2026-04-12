import { assertFheEncryptionKeyWasmOwnedBy } from './FheEncryptionKeyWasm-p.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
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

  const publicKeyBytes: FheEncryptionPublicKeyBytes =
    await context.runtime.encrypt.serializeFheEncryptionPublicKey({
      publicKey: parameters.publicKey,
    });

  const crsBytes: FheEncryptionCrsBytes =
    await context.runtime.encrypt.serializeFheEncryptionCrs({
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

////////////////////////////////////////////////////////////////////////////////

// export type SerializeFheEncryptionKeyToHexParameters = FheEncryptionKey;
// export type SerializeFheEncryptionKeyToHexReturnType = FheEncryptionKeyBytesHex;

// export async function serializeFheEncryptionKeyToHex(
//   fhevm: Fhevm<FhevmChain | undefined, WithEncrypt, OptionalNativeClient>,
//   parameters: SerializeFheEncryptionKeyToHexParameters,
// ): Promise<SerializeFheEncryptionKeyToHexReturnType> {
//   assertFheEncryptionKeyOwnedBy(parameters, fhevm.runtime);

//   const publicKeyBytes: FheEncryptionPublicKeyBytes =
//     await fhevm.runtime.encrypt.serializeFheEncryptionPublicKey({
//       publicKey: parameters.publicKey,
//     });

//   const crsBytes: FheEncryptionCrsBytes =
//     await fhevm.runtime.encrypt.serializeFheEncryptionCrs({
//       crs: parameters.crs,
//     });

//   const metadata: FheEncryptionKeyMetadata = Object.freeze({
//     ...parameters.metadata,
//   });

//   return Object.freeze({
//     publicKeyBytesHex: {
//       id: publicKeyBytes.id,
//       bytesHex: bytesToHexLarge(publicKeyBytes.bytes, false),
//     } as FheEncryptionPublicKeyBytesHex,
//     crsBytesHex: {
//       id: crsBytes.id,
//       capacity: crsBytes.capacity,
//       bytesHex: bytesToHexLarge(crsBytes.bytes, false),
//     } as FheEncryptionCrsBytesHex,
//     metadata,
//   });
// }
