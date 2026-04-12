import type { RelayerPublicDecryptOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { PublicDecryptionProof } from '../../types/publicDecryptionProof.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { publicDecrypt } from './publicDecrypt.js';

export type ReadPublicValueParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type ReadPublicValueReturnType = PublicDecryptionProof;

export async function readPublicValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: ReadPublicValueParameters,
): Promise<ReadPublicValueReturnType> {
  return publicDecrypt(fhevm, parameters);
}
