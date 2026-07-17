import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { NativeSigner } from '../../modules/ethereum/types.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import { signDecryptionPermit as signDecryptionPermit_ } from '../../kms/SignedDecryptionPermit-p.js';

export type SignDecryptionPermitParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
};
export type SignDecryptionPermitReturnType = SignedDecryptionPermit;

export async function signDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignDecryptionPermitParameters,
): Promise<SignedDecryptionPermit> {
  return signDecryptionPermit_(fhevm, parameters);
}
