import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import type { Eip712Like } from '../../types/kms.js';
import { createUnsignedDecryptionPermitEip712V1 as createUnsignedDecryptionPermitEip712V1_ } from '../../kms/SignedDecryptionPermitV1-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

export type CreateUnsignedLegacyDecryptionPermitEip712Parameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
};
export type CreateUnsignedLegacyDecryptionPermitEip712ReturnType = Eip712Like;

export async function createUnsignedLegacyDecryptionPermitEip712(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateUnsignedLegacyDecryptionPermitEip712Parameters,
): Promise<CreateUnsignedLegacyDecryptionPermitEip712ReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return createUnsignedDecryptionPermitEip712V1_(fhevm, { ...parameters, fhevmContext });
}
