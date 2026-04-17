/* eslint-disable @typescript-eslint/unified-signatures */
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { TypedValue } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { decryptValuesFromPairs as decryptValuesFromPairs_ } from '../../kms/decryptValuesFromPairs.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

type DecryptValuesParametersBase = {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly contractAddress: string;
  readonly transportKeypair: TransportKeypair;
};

export type DecryptSelfValuesParameters = DecryptValuesParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type DecryptDelegatedValuesParameters = DecryptValuesParametersBase & {
  readonly signedPermit: SignedDelegatedDecryptionPermit;
  readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptSelfValuesParameters,
): Promise<DecryptValuesReturnType>;

export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptDelegatedValuesParameters,
): Promise<DecryptValuesReturnType>;

export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptSelfValuesParameters | DecryptDelegatedValuesParameters,
): Promise<DecryptValuesReturnType> {
  const { encryptedValues, contractAddress, ...rest } = parameters;

  assertIsAddress(contractAddress, {});
  const sanitizedContractAddress = addressToChecksummedAddress(contractAddress);

  const sanitizedPairs = encryptedValues.map((ev) => {
    return {
      handle: toFhevmHandle(ev),
      contractAddress: sanitizedContractAddress,
    };
  });

  return decryptValuesFromPairs_(fhevm, { ...rest, pairs: sanitizedPairs });
}
