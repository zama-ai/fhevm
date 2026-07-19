import type { KmsSignDecryptionPermitParameters } from '../../kms/SignedDecryptionPermit-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import type { Eip712Like } from '../../types/kms.js';
import { createUnsignedDecryptionPermitEip712V1 as createUnsignedDecryptionPermitEip712V1_ } from '../../kms/SignedDecryptionPermitV1-p.js';

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
  parameters: KmsSignDecryptionPermitParameters,
): Promise<CreateUnsignedLegacyDecryptionPermitEip712ReturnType> {
  return createUnsignedDecryptionPermitEip712V1_(fhevm, parameters);
}
