import type { KmsSignDecryptionPermitParameters } from '../../kms/SignedDecryptionPermit-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TransportKeyPair } from '../decrypt/index.js';
import type { Eip712Like } from '../../types/kms.js';
import { createUnsignedDecryptionPermitEip712V2 as _createUnsignedDecryptionPermitEip712V2 } from '../../kms/SignedDecryptionPermitV2-p.js';

export type CreateUnsignedDecryptionPermitEip712Parameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly transportKeyPair: TransportKeyPair;
};
export type CreateUnsignedDecryptionPermitEip712ReturnType = Eip712Like;

export async function createUnsignedDecryptionPermitEip712(
  fhevm: Fhevm<FhevmChain>,
  parameters: KmsSignDecryptionPermitParameters,
): Promise<CreateUnsignedDecryptionPermitEip712ReturnType> {
  return _createUnsignedDecryptionPermitEip712V2(fhevm, parameters);
}
