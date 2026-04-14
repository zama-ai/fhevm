import type {
  SignDecryptionPermitParameters,
  SignSelfDecryptionPermitParameters,
  SignDelegatedDecryptionPermitParameters,
} from '../../kms/SignedDecryptionPermit-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  SignedDecryptionPermit,
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import { signDecryptionPermit as signDecryptionPermit_ } from '../../kms/SignedDecryptionPermit-p.js';

export type {
  SignDecryptionPermitParameters,
  SignSelfDecryptionPermitParameters,
  SignDelegatedDecryptionPermitParameters,
};
export type SignDecryptionPermitReturnType = SignedDecryptionPermit;

export async function signDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignSelfDecryptionPermitParameters,
): Promise<SignedSelfDecryptionPermit>;
export async function signDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignDelegatedDecryptionPermitParameters,
): Promise<SignedDelegatedDecryptionPermit>;
export async function signDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  // The public overloads guarantee the correct pairing.
  // The if/else narrows parameters via delegatorAddress, so TS can match each branch
  // to the correct overload of signDecryptionPermit_ without any casts.
  if (parameters.delegatorAddress !== undefined) {
    return signDecryptionPermit_(fhevm, parameters);
  }
  return signDecryptionPermit_(fhevm, parameters);
}
