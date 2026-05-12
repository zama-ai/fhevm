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

type CanDecryptValuesParametersBase = {
  readonly encryptedValues: EncryptedValueLike[];
  readonly contractAddress: string;
};

export type CanDecryptValuesWithUserAddressParameters = CanDecryptValuesParametersBase & {
  readonly userAddress: string;
};

export type CanDecryptValuesWithPermitParameters = CanDecryptValuesParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
  readonly transportKeyPair?: TransportKeyPair | undefined;
};

export type CanDecryptValuesReturnType = {
  readonly allowed: boolean;
  readonly details: ReadonlyArray<{
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  }>;
};

////////////////////////////////////////////////////////////////////////////////

export async function canDecryptValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValuesWithUserAddressParameters,
): Promise<CanDecryptValuesReturnType>;

export async function canDecryptValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValuesWithPermitParameters,
): Promise<CanDecryptValuesReturnType>;

export async function canDecryptValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValuesWithUserAddressParameters | CanDecryptValuesWithPermitParameters,
): Promise<CanDecryptValuesReturnType> {
  const { encryptedValues, contractAddress, ...rest } = parameters;

  assertIsAddress(contractAddress, {});
  const sanitizedContractAddress = addressToChecksummedAddress(contractAddress);

  const handleContractPairs = encryptedValues.map((encryptedValue, i) => {
    assertIsEncryptedValueLike(encryptedValue, { subject: `encryptedValues[${i}]` });
    return {
      handle: toFhevmHandle(encryptedValue),
      contractAddress: sanitizedContractAddress,
    };
  });

  return canDecryptValuesFromPairs_(fhevm, {
    ...rest,
    pairs: handleContractPairs,
  });
}
