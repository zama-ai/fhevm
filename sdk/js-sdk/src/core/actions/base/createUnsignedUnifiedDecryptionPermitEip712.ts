import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import type { Eip712Like } from '../../types/kms.js';
import { createUnsignedDecryptionPermitEip712V2 as createUnsignedDecryptionPermitEip712V2_ } from '../../kms/SignedDecryptionPermitV2-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

export type CreateUnsignedUnifiedDecryptionPermitEip712Parameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly transportKeyPair: TransportKeyPair;
};
export type CreateUnsignedUnifiedDecryptionPermitEip712ReturnType = Eip712Like;

export async function createUnsignedUnifiedDecryptionPermitEip712(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateUnsignedUnifiedDecryptionPermitEip712Parameters,
): Promise<CreateUnsignedUnifiedDecryptionPermitEip712ReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return createUnsignedDecryptionPermitEip712V2_(fhevm, { ...parameters, fhevmContext });
}
