import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { isAllowedForDecryption } from '../../host-contracts/isAllowedForDecryption.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type CanReadPublicValueParameters = {
  readonly encryptedValue: EncryptedValueLike;
};

export type CanReadPublicValueReturnType = boolean;

////////////////////////////////////////////////////////////////////////////////

export async function canReadPublicValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanReadPublicValueParameters,
): Promise<CanReadPublicValueReturnType> {
  const results = await isAllowedForDecryption(fhevm, {
    address: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    handles: [toFhevmHandle(parameters.encryptedValue)],
  });

  return results[0] === true;
}
