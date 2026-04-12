import { createFheEncryptionKeyWasm } from './FheEncryptionKeyWasm-p.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type {
  FheEncryptionKeyWasm,
  FheEncryptionKeyBytes,
} from '../types/fheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionKey(
  context: { readonly runtime: WithEncrypt },
  parameters: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyWasm> {
  const publicKeyNative =
    await context.runtime.encrypt.deserializeFheEncryptionPublicKey({
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

////////////////////////////////////////////////////////////////////////////////

// export type DeserializeFheEncryptionKeyFromHexParameters =
//   FheEncryptionKeyBytesHex;
// export type DeserializeFheEncryptionKeyFromHexReturnType = FheEncryptionKey;

// export async function deserializeFheEncryptionKeyFromHex(
//   fhevm: Fhevm<FhevmChain | undefined, WithEncrypt, OptionalNativeClient>,
//   parameters: DeserializeFheEncryptionKeyFromHexParameters,
// ): Promise<DeserializeFheEncryptionKeyFromHexReturnType> {
//   const publicKeyNative =
//     await fhevm.runtime.encrypt.deserializeFheEncryptionPublicKey({
//       publicKeyBytes: {
//         id: parameters.publicKeyBytesHex.id,
//         bytes: hexToBytesFaster(parameters.publicKeyBytesHex.bytesHex, {
//           strict: true,
//         }),
//       } as FheEncryptionPublicKeyBytes,
//     });

//   const crsNative = await fhevm.runtime.encrypt.deserializeFheEncryptionCrs({
//     crsBytes: {
//       id: parameters.crsBytesHex.id,
//       capacity: parameters.crsBytesHex.capacity,
//       bytes: hexToBytesFaster(parameters.crsBytesHex.bytesHex, {
//         strict: true,
//       }),
//     } as FheEncryptionCrsBytes,
//   });

//   return createFheEncryptionKeyWasm(new WeakRef(fhevm.runtime), {
//     publicKey: publicKeyNative,
//     crs: crsNative,
//     metadata: parameters.metadata,
//   });
// }
