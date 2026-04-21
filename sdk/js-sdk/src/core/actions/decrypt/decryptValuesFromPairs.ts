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

type DecryptValuesFromPairsParametersBase = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
  readonly transportKeypair: TransportKeypair;
};

export type DecryptSelfValuesFromPairsParameters = DecryptValuesFromPairsParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type DecryptDelegatedValuesFromPairsParameters = DecryptValuesFromPairsParametersBase & {
  readonly signedPermit: SignedDelegatedDecryptionPermit;
  readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesFromPairsReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptSelfValuesFromPairsParameters,
): Promise<DecryptValuesFromPairsReturnType>;

export async function decryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptDelegatedValuesFromPairsParameters,
): Promise<DecryptValuesFromPairsReturnType>;

export async function decryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptSelfValuesFromPairsParameters | DecryptDelegatedValuesFromPairsParameters,
): Promise<DecryptValuesFromPairsReturnType> {
  const { pairs, ...rest } = parameters;

  const sanitizedPairs = pairs.map((pair) => {
    assertIsAddress(pair.contractAddress, {});
    return {
      handle: toFhevmHandle(pair.encryptedValue),
      contractAddress: addressToChecksummedAddress(pair.contractAddress),
    };
  });

  return decryptValuesFromPairs_(fhevm, { ...rest, pairs: sanitizedPairs });
}
