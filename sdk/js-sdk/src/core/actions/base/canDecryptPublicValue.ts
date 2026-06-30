import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { isAllowedForDecryption } from '../../host-contracts/isAllowedForDecryption.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type CanDecryptPublicValueParameters = {
  readonly encryptedValue: EncryptedValueLike;
};

export type CanDecryptPublicValueReturnType = boolean;

////////////////////////////////////////////////////////////////////////////////

export async function canDecryptPublicValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptPublicValueParameters,
): Promise<CanDecryptPublicValueReturnType> {
  const results = await isAllowedForDecryption(fhevm, {
    aclAddress: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    handles: [toFhevmHandle(parameters.encryptedValue)],
  });

  return results[0] === true;
}
