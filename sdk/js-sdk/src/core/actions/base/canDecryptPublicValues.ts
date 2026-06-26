import type { ChecksummedAddress } from '../../types/primitives.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { isAllowedForDecryption } from '../../host-contracts/isAllowedForDecryption.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type CanDecryptPublicValuesParameters = {
  readonly encryptedValues: readonly EncryptedValueLike[];
};

export type CanDecryptPublicValuesReturnType = readonly boolean[];

////////////////////////////////////////////////////////////////////////////////

export async function canDecryptPublicValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptPublicValuesParameters,
): Promise<CanDecryptPublicValuesReturnType> {
  const handles = parameters.encryptedValues.map(toFhevmHandle);

  return isAllowedForDecryption(fhevm, {
    aclAddress: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    handles,
  });
}
