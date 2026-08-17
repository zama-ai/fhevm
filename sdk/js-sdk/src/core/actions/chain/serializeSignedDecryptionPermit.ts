import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { Eip712Like } from '../../types/kms.js';
import { serializeSignedDecryptionPermitToJSON } from '../../kms/SignedDecryptionPermit-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SerializeSignedDecryptionPermitParameters = {
  readonly signedPermit: SignedDecryptionPermit;
};

export type SerializeSignedDecryptionPermitReturnType = {
  readonly version: number;
  readonly eip712: Eip712Like;
  readonly signature: string;
  readonly signerAddress: string;
};

/**
 * Serializes a signed decryption permit to a plain object for storage or transmission.
 *
 * The resulting object can be stringified with `JSON.stringify()` and
 * parsed back with `parseSignedDecryptionPermit`.
 */
export async function serializeSignedDecryptionPermit(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SerializeSignedDecryptionPermitParameters,
): Promise<SerializeSignedDecryptionPermitReturnType> {
  // context is not needed
  await initPublicAction(fhevm);
  return serializeSignedDecryptionPermitToJSON(parameters.signedPermit);
}
