import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { TypedValue } from '../../types/primitives.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { decryptValuesFromPairs as decryptValuesFromPairs_ } from '../../kms/decryptValuesFromPairs.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptValuesFromPairsParameters = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
  readonly transportKeypair: TransportKeypair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type DecryptValuesFromPairsReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesFromPairsParameters,
): Promise<DecryptValuesFromPairsReturnType> {
  const { pairs, ...rest } = parameters;

  const sanitizedPairs = pairs.map((pair) => {
    assertIsAddress(pair.contractAddress, {});
    return {
      handle: toFhevmHandle(pair.encryptedValue),
      contractAddress: addressToChecksummedAddress(pair.contractAddress),
    };
  });

  return decryptValuesFromPairs_(fhevm, {
    ...rest,
    pairs: sanitizedPairs,
  } as Parameters<typeof decryptValuesFromPairs_>[1]);
}
