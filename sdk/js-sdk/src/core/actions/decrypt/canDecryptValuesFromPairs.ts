/* eslint-disable @typescript-eslint/unified-signatures */
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { canDecryptValuesFromPairs as canDecryptValuesFromPairs_ } from '../../host-contracts/canDecryptValuesFromPairs.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { assertIsEncryptedValueLike, toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

type CanDecryptValuesFromPairsParametersBase = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
};

export type CanDecryptValuesFromPairsWithUserAddressParameters = CanDecryptValuesFromPairsParametersBase & {
  readonly userAddress: string;
};

export type CanDecryptValuesFromPairsWithPermitParameters = CanDecryptValuesFromPairsParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
  readonly transportKeyPair?: TransportKeyPair | undefined;
};

export type CanDecryptValuesFromPairsReturnType = {
  readonly allowed: boolean;
  readonly details: ReadonlyArray<{
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  }>;
};

////////////////////////////////////////////////////////////////////////////////

export async function canDecryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValuesFromPairsWithUserAddressParameters,
): Promise<CanDecryptValuesFromPairsReturnType>;

export async function canDecryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValuesFromPairsWithPermitParameters,
): Promise<CanDecryptValuesFromPairsReturnType>;

export async function canDecryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValuesFromPairsWithUserAddressParameters | CanDecryptValuesFromPairsWithPermitParameters,
): Promise<CanDecryptValuesFromPairsReturnType> {
  const { pairs, ...rest } = parameters;

  const handleContractPairs = pairs.map((pair, i) => {
    assertIsAddress(pair.contractAddress, {});
    assertIsEncryptedValueLike(pair.encryptedValue, { subject: `pairs[${i}].encryptedValue` });
    return {
      handle: toFhevmHandle(pair.encryptedValue),
      contractAddress: addressToChecksummedAddress(pair.contractAddress),
    };
  });

  return canDecryptValuesFromPairs_(fhevm, { ...rest, pairs: handleContractPairs });
}
