import type { ChecksummedAddress } from '../../types/primitives.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { isAllowedForDecryption } from '../../host-contracts/isAllowedForDecryption.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type CanReadPublicValuesParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
};

export type CanReadPublicValuesReturnType = readonly boolean[];

////////////////////////////////////////////////////////////////////////////////

export async function canReadPublicValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanReadPublicValuesParameters,
): Promise<CanReadPublicValuesReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  return isAllowedForDecryption(fhevm, {
    address: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    handles,
  });
}
