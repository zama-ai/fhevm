/* eslint-disable @typescript-eslint/require-await */
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type {
  BuildWithProofPackedReturnTypeParameters,
  BuildWithProofPackedReturnType,
  ParseTFHEProvenCompactCiphertextListParameters,
  ParseTFHEProvenCompactCiphertextListReturnType,
  SerializeFheEncryptionCrsParameters,
  SerializeFheEncryptionCrsReturnType,
  SerializeFheEncryptionKeyParameters,
  SerializeFheEncryptionKeyReturnType,
  SerializeFheEncryptionPublicKeyParameters,
  SerializeFheEncryptionPublicKeyReturnType,
  EncryptModuleFactory,
  DeserializeFheEncryptionPublicKeyParameters as DeserializeFheEncryptionPublicKeyParameters,
  DeserializeFheEncryptionPublicKeyReturnType as DeserializeFheEncryptionPublicKeyReturnType,
  DeserializeFheEncryptionCrsParameters,
  DeserializeFheEncryptionCrsReturnType,
} from './types.js';

////////////////////////////////////////////////////////////////////////////////
// parseTFHEProvenCompactCiphertextList
////////////////////////////////////////////////////////////////////////////////

export async function parseTFHEProvenCompactCiphertextList(
  _parameters: ParseTFHEProvenCompactCiphertextListParameters,
): Promise<ParseTFHEProvenCompactCiphertextListReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPacked(
  _parameters: BuildWithProofPackedReturnTypeParameters,
): Promise<BuildWithProofPackedReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionKey(
  _parameters: SerializeFheEncryptionKeyParameters,
): Promise<SerializeFheEncryptionKeyReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionPublicKey(
  _parameters: SerializeFheEncryptionPublicKeyParameters,
): Promise<SerializeFheEncryptionPublicKeyReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionCrs(
  _parameters: SerializeFheEncryptionCrsParameters,
): Promise<SerializeFheEncryptionCrsReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// deserializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionCrs(
  _parameters: DeserializeFheEncryptionCrsParameters,
): Promise<DeserializeFheEncryptionCrsReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// deserializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionPublicKey(
  _parameters: DeserializeFheEncryptionPublicKeyParameters,
): Promise<DeserializeFheEncryptionPublicKeyReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// mockTfheActions
////////////////////////////////////////////////////////////////////////////////

export const encryptModule: EncryptModuleFactory = (_runtime: FhevmRuntime) => {
  return Object.freeze({
    encrypt: Object.freeze({
      initTfheModule: () => Promise.resolve(),
      getTfheModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      parseTFHEProvenCompactCiphertextList,
      buildWithProofPacked,
      serializeFheEncryptionKey,
      serializeFheEncryptionPublicKey,
      serializeFheEncryptionCrs,
      deserializeFheEncryptionPublicKey,
      deserializeFheEncryptionCrs,
    }),
  });
};
