import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { NativeSigner } from '../../modules/ethereum/types.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import { signDecryptionPermitV2 as signUnifiedDecryptionPermit_ } from '../../kms/SignedDecryptionPermitV2-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

export type SignUnifiedDecryptionPermitParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
};

export type SignUnifiedDecryptionPermitReturnType = SignedDecryptionPermit;

export async function signUnifiedDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: SignUnifiedDecryptionPermitParameters,
): Promise<SignUnifiedDecryptionPermitReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return signUnifiedDecryptionPermit_(fhevm, { ...parameters, fhevmContext });
}
