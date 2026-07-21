import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { NativeSigner } from '../../modules/ethereum/types.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import { signDecryptionPermitV1 as signLegacyDecryptionPermit_ } from '../../kms/SignedDecryptionPermitV1-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

export type SignLegacyDecryptionPermitParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
};

export type SignLegacyDecryptionPermitReturnType = SignedDecryptionPermit;

export async function signLegacyDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignLegacyDecryptionPermitParameters,
): Promise<SignLegacyDecryptionPermitReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return signLegacyDecryptionPermit_(fhevm, { ...parameters, fhevmContext });
}
