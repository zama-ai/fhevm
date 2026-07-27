import type { SignDecryptionPermitParameters } from '../../kms/SignedDecryptionPermit-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { signDecryptionPermit as signDecryptionPermit_ } from '../../kms/SignedDecryptionPermit-p.js';

export type { SignDecryptionPermitParameters };
export type SignDecryptionPermitReturnType = SignedDecryptionPermit;

export async function signDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  return signDecryptionPermit_(fhevm, parameters);
}
