import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { KmsDelegatedUserDecryptEip712, KmsUserDecryptEip712 } from '../../types/kms.js';
import { serializeSignedDecryptionPermitToJSON } from '../../kms/SignedDecryptionPermit-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SerializeSignedDecryptionPermitParameters = {
  readonly signedPermit: SignedDecryptionPermit;
};

export type SerializeSignedDecryptionPermitReturnType = {
  readonly eip712: KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712;
  readonly signature: string;
  readonly signerAddress: string;
};

/**
 * Serializes a signed decryption permit to a plain object for storage or transmission.
 *
 * The resulting object can be stringified with `JSON.stringify()` and
 * parsed back with `parseSignedDecryptionPermit`.
 */
export function serializeSignedDecryptionPermit(
  _fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SerializeSignedDecryptionPermitParameters,
): SerializeSignedDecryptionPermitReturnType {
  return serializeSignedDecryptionPermitToJSON(parameters.signedPermit);
}
